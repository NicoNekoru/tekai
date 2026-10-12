#!/usr/bin/env python3
"""Reproduce performance and lifetime audit cases using isolated fixtures.

This is a diagnostic runner, not a timing-based CI gate. Requires macOS and a
release tekai binary. Cache correctness cases also require pdftotext. Every
started build owns a process group which is terminated on timeout. The runner
reaps its direct child.
"""

import argparse
import gzip
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import shutil
import signal
import struct
import subprocess
import tempfile
import time
import zlib

from owned_process import OwnedProcess

REPO = Path(__file__).resolve().parent.parent
CASES = ('lookup', 'lint', 'cache', 'edit-race', 'input-identity', 'source-boundaries', 'format-cache',
         'cancel', 'preview', 'pdf', 'png', 'deep-inputs', 'aux-concurrency', 'expansion')
SEARCH_VARIABLES = (
    'TEXINPUTS', 'BIBINPUTS', 'BSTINPUTS', 'TEXFONTS', 'TFMFONTS', 'AFMFONTS',
    'T1FONTS', 'TTFONTS', 'OPENTYPEFONTS', 'VFFONTS', 'ENCFONTS', 'SFDFONTS',
    'TEXFONTMAPS', 'PKFONTS', 'GFFONTS', 'TEXFORMATS', 'TEXMFHOME', 'TEXMFLOCAL',
    'TEKAI_ENGINE_FORMATS',
)
MAX_STDOUT_BYTES = 1024 * 1024
LOG_TAIL_BYTES = 8192


def stop_group(process):
    process.finish()


def pdf_bytes(objects):
    output = bytearray(b'%PDF-1.4\n')
    offsets = [0]
    for number, obj in enumerate(objects, 1):
        offsets.append(len(output))
        output.extend(f'{number} 0 obj\n'.encode() + obj + b'\nendobj\n')
    xref = len(output)
    output.extend(f'xref\n0 {len(offsets)}\n0000000000 65535 f \n'.encode())
    for offset in offsets[1:]:
        output.extend(f'{offset:010} 00000 n \n'.encode())
    output.extend(f'trailer\n<< /Size {len(offsets)} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n'.encode())
    return output


def png_chunk(kind, data):
    return struct.pack('>I', len(data)) + kind + data + struct.pack('>I', zlib.crc32(kind + data))


class Audit:
    def __init__(self, args, work):
        self.args = args
        self.work = work
        self.counter, self.owned = 0, {}
        machine = platform.machine()
        try:
            metadata = self.run(['/usr/sbin/sysctl', '-n', 'machdep.cpu.brand_string'],
                                work, env=os.environ.copy(), timeout=2)
            if metadata.get('code') == 0 and not metadata.get('stdout_truncated'):
                machine = metadata['stdout'].strip() or machine
        except (OSError, subprocess.SubprocessError):
            # CPU model metadata must not prevent scoped fixture diagnostics.
            pass
        self.report = {'machine': machine,
            'binary': str(args.engine), 'quick': args.quick, 'timeout_seconds': args.timeout,
            'results': []}
        self.env = dict(os.environ, TEKAI_TEXMF_MODE='bundled',
                        TEKAI_ENGINE_CACHE=str(work / 'runtime-cache'))
        for variable in SEARCH_VARIABLES:
            self.env.pop(variable, None)
        self.pdftext = shutil.which('pdftotext')
        self.rss_probe = None

    def record(self, case, **values):
        row = {'case': case, **values}
        self.report['results'].append(row)
        self.args.report.write_text(json.dumps(self.report, indent=2) + '\n')
        print(json.dumps(row), flush=True)

    def project(self, name, source=None):
        project = self.work / name
        project.mkdir()
        if source is not None:
            (project / 'main.tex').write_text(source)
        return project

    def environment(self, project, **values):
        cache = self.work / 'caches' / project.name
        return dict(self.env, TEKAI_AUX_CACHE=str(cache / 'aux'),
                    TEKAI_BIBTEX_CACHE=str(cache / 'bibtex'),
                    TEKAI_FORMAT_CACHE=str(cache / 'format'), **values)

    def start(self, command, project, env, measured=False):
        owner = OwnedProcess()
        if measured:
            command = ['/usr/bin/time', '-l', *command]
        self.counter += 1
        prefix = self.work / f'command-{self.counter:04d}'
        paths = prefix.with_suffix('.stdout'), prefix.with_suffix('.stderr')
        process = None
        with paths[0].open('wb') as stdout, paths[1].open('wb') as stderr:
            try:
                process = subprocess.Popen(list(map(str, command)), cwd=project, env=env,
                                           stdout=stdout, stderr=stderr, start_new_session=True)
            finally:
                if process is not None:
                    try:
                        owner.attach(process)
                        owner.capture_paths = paths
                    finally:
                        owner._process = process
                        self.owned[process.pid] = owner
        return owner

    def stop(self, process):
        stop_group(process)
        self.owned.pop(process.pid, None)

    def close(self):
        errors = []
        for process in list(self.owned.values()):
            try:
                self.stop(process)
            except (OSError, subprocess.SubprocessError, RuntimeError) as error:
                errors.append(str(error))
        if errors:
            raise RuntimeError('Owned process cleanup failed: ' + ', '.join(errors))

    @staticmethod
    def captured(process):
        stdout_path, stderr_path = process.capture_paths
        with stdout_path.open('rb') as handle:
            stdout = handle.read(MAX_STDOUT_BYTES + 1)
        with stderr_path.open('rb') as handle:
            handle.seek(max(0, stderr_path.stat().st_size - LOG_TAIL_BYTES))
            stderr = handle.read(LOG_TAIL_BYTES)
        return stdout[:MAX_STDOUT_BYTES].decode('utf-8', errors='replace'), \
            stderr.decode('utf-8', errors='replace'), len(stdout) > MAX_STDOUT_BYTES

    def rss_timer_available(self):
        if self.rss_probe is None:
            try:
                process = self.start(['/usr/bin/true'], self.work, self.env, measured=True)
            except OSError as error:
                self.rss_probe = {'available': False, 'stderr_tail': str(error)}
                return False
            try:
                process.wait_for_exit(timeout=2)
                _, stderr, _ = self.captured(process)
                available = process.returncode == 0 and 'maximum resident set size' in stderr
                self.rss_probe = {'available': available, 'stderr_tail': stderr[-1000:]}
            except subprocess.TimeoutExpired:
                self.rss_probe = {'available': False, 'stderr_tail': 'System timer probe timed out'}
            finally:
                self.stop(process)
        return self.rss_probe['available']

    def run(self, command, project, env=None, measured=False, timeout=None):
        use_timer = measured and self.rss_timer_available()
        started = time.monotonic()
        process = self.start(command, project, env or self.environment(project), use_timer)
        try:
            process.wait_for_exit(timeout=self.args.timeout if timeout is None else timeout)
            stdout, stderr, truncated = self.captured(process)
            maximum = re.search(r'(\d+)\s+maximum resident set size', stderr)
            result = {'seconds': process.completion - started, 'code': process.returncode,
                      'peak_mib': int(maximum[1]) / 1048576 if maximum else None,
                      'stdout': stdout, 'stderr_tail': stderr[-2000:], 'stdout_truncated': truncated}
            if measured:
                result['rss_available'] = maximum is not None
                result['status_source'] = 'timer' if use_timer else 'command'
                if not use_timer:
                    result['rss_unavailable_reason'] = self.rss_probe['stderr_tail']
            return result
        except subprocess.TimeoutExpired:
            return {'seconds': time.monotonic() - started, 'timeout': True}
        finally:
            self.stop(process)

    def build_command(self, project, *extra):
        return [self.args.engine, 'build', project / 'main.tex', '--out-dir',
                project / 'build', '--quiet', '--report-json', *extra]

    def text(self, project):
        result = self.run([self.pdftext, project / 'build/main.pdf', '-'], project)
        if result.get('code') != 0 or result.get('stdout_truncated'):
            raise RuntimeError(f'PDF text extraction failed: {result}')
        return result['stdout']

    def needs_pdftext(self, case):
        if self.pdftext is not None:
            return True
        self.record(case, skipped=True, reason='pdftotext is unavailable')
        return False

    def process_inspection_available(self, case):
        try:
            result = self.run(['ps', '-p', str(os.getpid()), '-o', 'pid='], self.work, timeout=2)
            if result.get('code') != 0 or result.get('stdout_truncated'):
                raise RuntimeError(str(result))
            return True
        except (OSError, subprocess.SubprocessError, RuntimeError) as error:
            self.record(case, skipped=True, reason=f'Process inspection is unavailable: {error}')
            return False

    @staticmethod
    def parsed(result):
        if result.get('code') != 0 or result.get('stdout_truncated'):
            raise RuntimeError(f'Fixture build failed: {result}')
        return json.loads(result['stdout'])

    def lookup(self):
        if self.needs_pdftext('lookup-precedence'):
            project = self.project('shadow', '\\documentclass{article}\n\\begin{document}\n'
                                   '\\input{choice}\n\\end{document}\n')
            for directory in ('a', 'z'):
                (project / directory).mkdir()
            (project / 'z/choice.tex').write_text('OLD-CHOICE\n')
            env = self.environment(project, TEXINPUTS=f'{project}//:')
            command = self.build_command(project)
            self.parsed(self.run(command, project, env))
            (project / 'a/choice.tex').write_text('NEW-CHOICE\n')
            located = self.run([self.args.engine, 'locate', 'choice.tex', '--directory', project], project, env)
            next_build = self.parsed(self.run(command, project, env))
            text = self.text(project)
            forced = self.parsed(self.run([*command, '--force'], project, env))
            forced_text = self.text(project)
            self.record('lookup-precedence', next_build=next_build, located=located,
                        text=text, forced=forced, forced_text=forced_text,
                        stale_cache_hit=next_build['skipped'] and 'OLD-CHOICE' in text
                        and 'NEW-CHOICE' in forced_text)

            project = self.project('database', '\\documentclass{article}\n\\usepackage{auditchoice}\n'
                                   '\\begin{document}\n\\auditchoice\n\\end{document}\n')
            tree = self.work / 'outside-site'
            for directory, word in [('a', 'NEW-CHOICE'), ('z', 'OLD-CHOICE')]:
                path = tree / 'tex/latex' / directory
                path.mkdir(parents=True)
                (path / 'auditchoice.sty').write_text('\\newcommand{\\auditchoice}{' + word + '}\n')
            database = tree / 'ls-R'
            database.write_text('./tex/latex/z:\nauditchoice.sty\n')
            env = self.environment(project, TEXMFLOCAL=str(tree),
                                   TEXMFHOME=str(self.work / 'absent'))
            env['TEKAI_TEXMF_MODE'] = 'shared'
            command = self.build_command(project)
            self.parsed(self.run(command, project, env))
            identity = database.stat()
            replacement = tree / 'ls-R.replacement'
            replacement.write_text('./tex/latex/a:\nauditchoice.sty\n')
            os.utime(replacement, ns=(identity.st_atime_ns, identity.st_mtime_ns))
            replacement.replace(database)
            located = self.run([self.args.engine, 'locate', 'auditchoice.sty', '--directory', project], project, env)
            next_build = self.parsed(self.run(command, project, env))
            text = self.text(project)
            forced = self.parsed(self.run([*command, '--force'], project, env))
            forced_text = self.text(project)
            self.record('database-identity', next_build=next_build, located=located,
                        text=text, forced=forced, forced_text=forced_text,
                        stale_cache_hit=next_build['skipped'] and 'OLD-CHOICE' in text
                        and 'NEW-CHOICE' in forced_text)

        for depth in ([4, 12] if self.args.quick else [4, 8, 12, 16]):
            project = self.project(f'dag-{depth}')
            physical = project / 'physical'
            root = project / 'root'
            root.mkdir()
            for level in range(depth + 1):
                directory = physical / str(level)
                directory.mkdir(parents=True)
                if level < depth:
                    for name in ('left', 'right'):
                        (directory / name).symlink_to(physical / str(level + 1), target_is_directory=True)
            (root / 'start').symlink_to(physical / '0', target_is_directory=True)
            result = self.run([self.args.engine, 'locate', 'missing.sty', '--directory', root],
                              project, self.environment(project, TEXINPUTS=f'{root}//'), measured=True)
            self.record('symlink-dag', depth=depth, physical_directories=depth + 3, **result)

    def lint(self):
        for kind in ('format-many', 'lint-long', 'lint-slashes'):
            for size in ([10000, 40000] if self.args.quick else [5000, 10000, 20000, 40000, 80000]):
                project = self.project(f'{kind}-{size}')
                source = project / 'main.tex'
                source.write_text('$x$\n' * size if kind == 'format-many' else
                                  ('x' if kind == 'lint-long' else '\\') * size + '\n')
                command = [self.args.engine, 'format', source, '--check', '--quiet', '--allow-warnings'] \
                    if kind == 'format-many' else [self.args.engine, 'lint', source, '--allow-warnings']
                result = self.run(command, project, measured=True)
                result.pop('stdout', None)
                self.record(kind, size=size, **result)
                if result.get('timeout'):
                    break

    def cache(self):
        for count in ([0, 16] if self.args.quick else [0, 4, 16]):
            project = self.project(f'cache-{count}', '\\documentclass{article}\n'
                                   '\\begin{document}Hello\\end{document}\n')
            output = project / 'build'
            output.mkdir()
            block = b' ' * (8 * 1024 * 1024)
            for index in range(count):
                (output / f'unrelated-{index}.bbl').write_bytes(block)
            result = self.run(self.build_command(project, '--force'), project, measured=True)
            if result.get('code') == 0:
                result['build_report'] = json.loads(result.pop('stdout'))
            self.record('unrelated-sidecars', unrelated_mib=count * 8, **result)

    def edit_race(self):
        if not self.needs_pdftext('edit-race'):
            return
        source = ('\\documentclass{article}\n\\newwrite\\auditfile\n\\newcount\\auditcount\n'
                  '\\begin{document}\nOLD-CONTENT\\par\n'
                  '\\immediate\\openout\\auditfile=audit.marker\n'
                  '\\immediate\\write\\auditfile{started}\n\\immediate\\closeout\\auditfile\n'
                  '\\loop\\advance\\auditcount by1\\ifnum\\auditcount<1000000\\repeat\n'
                  '\\end{document}\n')
        project = self.project('edit-race', source)
        command = self.build_command(project)
        env = self.environment(project)
        process = self.start(command, project, env)
        try:
            deadline = time.monotonic() + self.args.timeout
            while not (project / 'build/audit.marker').exists():
                if time.monotonic() > deadline or process.poll_exit() is not None:
                    raise RuntimeError('Edit-race marker did not appear')
                time.sleep(0.002)
            (project / 'main.tex').write_text(source.replace('OLD-CONTENT', 'NEW-CONTENT'))
            process.wait_for_exit(timeout=self.args.timeout)
            stdout, stderr, truncated = self.captured(process)
            first = {'code': process.returncode, 'stdout': stdout, 'stderr_tail': stderr,
                     'stdout_truncated': truncated, 'timeout': False}
            generation_rejected = (
                f'inputs or search directories changed during compilation at {project / "main.tex"}.' in stderr
                and 'The PDF may be outdated, rerun the build' in stderr)
            state_exists = any((project / f'build/.tekai-main.state.{extension}').exists()
                               for extension in ('toml', 'json'))
            first_text = self.text(project)
            next_build = self.parsed(self.run(command, project, env))
            next_text = self.text(project)
            self.record('edit-race', first=first, first_text=first_text,
                        generation_rejected=generation_rejected, state_exists_after_race=state_exists,
                        next_build=next_build, next_text=next_text,
                        stale_cache_hit=next_build['skipped'] and 'OLD-CONTENT' in next_text)
        finally:
            self.stop(process)

    def cancel(self):
        if not self.process_inspection_available('cancel'):
            return
        project = self.project('cancel', '\\documentclass{article}\n\\newcount\\auditcount\n'
                               '\\begin{document}\n\\loop\\advance\\auditcount by1'
                               '\\ifnum\\auditcount<100000000\\repeat\n\\end{document}\n')
        process = self.start(self.build_command(project), project, self.environment(project))
        child = None
        try:
            deadline = time.monotonic() + self.args.timeout
            while child is None and time.monotonic() < deadline and process.poll_exit() is None:
                inspected = self.run(['ps', '-axo', 'pid=,ppid=,command='], self.work)
                if inspected.get('code') != 0 or inspected.get('stdout_truncated'):
                    raise RuntimeError('Cancellation process inspection failed')
                lines = inspected['stdout'].splitlines()
                for line in lines:
                    fields = line.strip().split(None, 2)
                    if len(fields) == 3 and fields[1] == str(process.pid) and '__tekai-engine' in fields[2]:
                        child = int(fields[0])
                        break
                if child is None:
                    time.sleep(0.02)
            if child is None:
                raise RuntimeError('Cancellation fixture did not start an engine child')
            process.terminate()
            process.wait_for_exit(timeout=self.args.timeout)
            time.sleep(0.5)
            inspected = self.run(['ps', '-p', str(child), '-o', 'pid=,ppid=,%cpu=,stat=,command='], self.work)
            if inspected.get('stdout_truncated') or inspected.get('timeout') or inspected.get('code') not in (0, 1):
                raise RuntimeError('Cancellation child inspection failed')
            snapshot = inspected['stdout'].strip()
            self.record('cancel', parent_code=process.returncode,
                        child_survived=bool(snapshot), child_snapshot=snapshot)
        finally:
            self.stop(process)

    def input_identity(self):
        if not self.needs_pdftext('input-identity'):
            return
        source = '\\documentclass{article}\n\\begin{document}\nOLD-CONTENT\n\\end{document}\n'
        for mode in ('in-place', 'replacement'):
            project = self.project(f'input-identity-{mode}', source)
            command = self.build_command(project)
            self.parsed(self.run(command, project))
            path = project / 'main.tex'
            identity = path.stat()
            changed = source.replace('OLD-CONTENT', 'NEW-CONTENT-LONGER')
            target = path if mode == 'in-place' else project / 'replacement.tex'
            target.write_text(changed)
            os.utime(target, ns=(identity.st_atime_ns, identity.st_mtime_ns))
            if mode == 'replacement':
                target.replace(path)
            current = path.stat()
            next_build = self.parsed(self.run(command, project))
            text = self.text(project)
            forced = self.parsed(self.run([*command, '--force'], project))
            forced_text = self.text(project)
            self.record('input-identity', mode=mode, size_changed=current.st_size != identity.st_size,
                        inode_changed=current.st_ino != identity.st_ino,
                        mtime_preserved=current.st_mtime_ns == identity.st_mtime_ns,
                        next_build=next_build, text=text, forced=forced, forced_text=forced_text,
                        stale_cache_hit=next_build['skipped'] and 'OLD-CONTENT' in text
                        and 'NEW-CONTENT-LONGER' in forced_text)

    def format_cache(self):
        if not self.needs_pdftext('format-cache'):
            return
        project = self.project('format-cache', '\\audittext\\par\\end\n')
        env = self.environment(project, TEXFORMATS=f'{project}:')
        base = [self.args.engine, '__tekai-engine', '-no-shell-escape',
                '-interaction=nonstopmode', '-halt-on-error']
        for name, word in [('old', 'OLD-FORMAT'), ('new', 'NEW-FORMAT')]:
            source = ('\\catcode123=1\\catcode125=2'
                      '\\pdfoutput=1\\font\\auditfont=cmr10\\auditfont'
                      '\\hsize=100pt\\vsize=100pt\\output={\\shipout\\box255}'
                      '\\def\\audittext{' + word + '}\\dump')
            result = self.run([*base, '-ini', '-etex', f'-jobname={name}',
                               f'-output-directory={project}', source], project, env)
            if result.get('code') != 0:
                raise RuntimeError(f'Format fixture generation failed: {result}')
        active = project / 'active.fmt'
        raw = project / 'active.fmt.raw'
        # Exercise gzip materialization even if dump compression defaults change.
        old = (project / 'old.fmt').read_bytes()
        old = old if old.startswith(b'\x1f\x8b') else gzip.compress(old, mtime=0)
        new = (project / 'new.fmt').read_bytes()
        new = new if new.startswith(b'\x1f\x8b') else gzip.compress(new, mtime=0)
        active.write_bytes(old)
        # An adjacent companion belongs to the installation, not our cache.
        poison = b'not a valid format companion\n'
        raw.write_bytes(poison)
        raw.chmod(0o400)
        active.chmod(0o400)
        identity = active.stat()
        (project / 'build').mkdir()
        command = [*base, '-fmt=active', '-jobname=main',
                   f'-output-directory={project / "build"}', 'main.tex']
        first = self.run(command, project, env)
        if first.get('code') != 0:
            raise RuntimeError(f'Format cache fixture failed: {first}')
        first_text = self.text(project)
        replacement = project / 'replacement.fmt'
        replacement.write_bytes(new)
        os.utime(replacement, ns=(identity.st_atime_ns, identity.st_mtime_ns))
        replacement.chmod(0o400)
        replacement.replace(active)
        second = self.run(command, project, env)
        if second.get('code') != 0:
            raise RuntimeError(f'Replaced format fixture failed: {second}')
        second_text = self.text(project)
        atomic_mtime_preserved = active.stat().st_mtime_ns == identity.st_mtime_ns
        active.chmod(0o600)
        active.write_bytes(old)
        os.utime(active, ns=(identity.st_atime_ns, identity.st_mtime_ns))
        inplace = self.run(command, project, env)
        if inplace.get('code') != 0:
            raise RuntimeError(f'In-place format fixture failed: {inplace}')
        inplace_text = self.text(project)
        inplace_mtime_preserved = active.stat().st_mtime_ns == identity.st_mtime_ns
        active.write_bytes(new)
        os.utime(active, ns=(identity.st_atime_ns, identity.st_mtime_ns))
        # Corruption recovery must work in our private cache, without touching
        # either the compressed installation source or its adjacent companion.
        cache = Path(env['TEKAI_ENGINE_CACHE']) / 'raw-formats-v1'
        cached_raw = cache / (hashlib.sha256(new).hexdigest() + '.fmt.raw')
        if not cached_raw.is_file():
            raise RuntimeError('Format fixture did not publish its content-addressed raw cache')
        cached_raw.write_bytes(b'corrupted private format\n')
        refreshed = self.run(command, project, env)
        if refreshed.get('code') != 0:
            raise RuntimeError(f'Refreshed format fixture failed: {refreshed}')
        refreshed_text = self.text(project)
        self.record('format-cache', first_text=first_text, second_text=second_text,
                    inplace_text=inplace_text, refreshed_text=refreshed_text,
                    atomic_mtime_preserved=atomic_mtime_preserved,
                    inplace_mtime_preserved=inplace_mtime_preserved,
                    adjacent_companion_unchanged=raw.read_bytes() == poison,
                    private_cache_recovered=cached_raw.is_file()
                    and cached_raw.read_bytes() != b'corrupted private format\n')

    def source_boundaries(self):
        if not self.needs_pdftext('source-boundaries'):
            return
        for mode in ('conditional', 'macro', 'endinput'):
            preamble = '\\documentclass{article}\n'
            if mode == 'conditional':
                preamble += '\\iffalse\\end{document}\\fi\n'
            elif mode == 'macro':
                preamble += '\\newcommand{\\auditfinish}{\\end{document}}\n'
            source = preamble + '\\begin{document}\nOLD-CONTENT\n\\end{document}\n'
            project = self.project(f'source-boundaries-{mode}', source)
            path = project / 'main.tex'
            if mode == 'endinput':
                path.write_text(preamble + '\\begin{document}\\input{part}\\end{document}\n')
                path = project / 'part.tex'
                source = '\\iffalse\\endinput\\fi\nOLD-CONTENT\n'
                path.write_text(source)
            command = self.build_command(project)
            self.parsed(self.run(command, project))
            identity = path.stat()
            path.write_text(source.replace('OLD-CONTENT', 'NEW-CONTENT'))
            current = path.stat()
            if current.st_mtime_ns == identity.st_mtime_ns:
                raise RuntimeError('Source boundary fixture needs distinct modification times')
            next_build = self.parsed(self.run(command, project))
            text = self.text(project)
            forced = self.parsed(self.run([*command, '--force'], project))
            forced_text = self.text(project)
            self.record('source-boundaries', mode=mode, mtime_changed=True,
                        next_build=next_build, text=text, forced=forced, forced_text=forced_text,
                        stale_cache_hit=next_build['skipped'] and 'OLD-CONTENT' in text
                        and 'NEW-CONTENT' in forced_text)

    def preview(self):
        project = self.project('unicode-preview', '\\documentclass{article}\n\\begin{document}\n'
                               + 'x' * 8190 + 'é more text\n\\end{document}\n')
        command = [self.args.engine, 'watch', project / 'main.tex', '--out-dir',
                   project / 'build', '--no-lint', '--preview']
        result = self.run(command, project)
        self.record('unicode-preview', aborted=result.get('code', 0) < 0, **result)

    def media(self, name, content, extension):
        project = self.project(name, '\\documentclass{article}\n\\usepackage{graphicx}\n'
                               '\\begin{document}\n\\includegraphics[page=1]{image.'
                               + extension + '}\n\\end{document}\n')
        (project / f'image.{extension}').write_bytes(content)
        result = self.run(self.build_command(project, '--once'), project, measured=True)
        if result.get('code') == 0:
            result['build_report'] = json.loads(result.pop('stdout'))
        self.record(name, input_bytes=len(content), **result)

    def pdf(self):
        for count in ([512, 8192] if self.args.quick else [128, 512, 2048, 8192]):
            self.media(f'pdf-dictionary-{count}', pdf_bytes([
                b'<< /Type /Catalog /Pages 2 0 R >>',
                b'<< /Type /Pages /Kids [3 0 R] /Count 1 >>',
                b'<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 200] /Resources << /Audit 5 0 R >> /Contents 4 0 R >>',
                b'<< /Length 0 >>\nstream\n\nendstream',
                b'<< ' + b' '.join(f'/Key{i} {i}'.encode() for i in range(count)) + b' >>',
            ]), 'pdf')
        for count in ([1, 256] if self.args.quick else [1, 16, 64, 256]):
            objects = [
                b'<< /Type /Catalog /Pages 2 0 R >>',
                b'<< /Type /Pages /Kids [' + b' '.join(f'{5 + i} 0 R'.encode() for i in range(count))
                + b'] /Count ' + str(count).encode() + b' /MediaBox [0 0 200 200] /Resources 3 0 R >>',
                b'<< /Audit << ' + b' '.join(f'/Key{i} {i}'.encode() for i in range(2048)) + b' >> >>',
                b'<< /Length 0 >>\nstream\n\nendstream',
            ]
            objects.extend([b'<< /Type /Page /Parent 2 0 R /Contents 4 0 R >>'] * count)
            self.media(f'pdf-shared-resources-{count}', pdf_bytes(objects), 'pdf')
        self.media('pdf-parent-cycle', pdf_bytes([
            b'<< /Type /Catalog /Pages 2 0 R >>',
            b'<< /Type /Pages /Kids [3 0 R] /Count 1 /Parent 2 0 R >>',
            b'<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 200] /Contents 4 0 R >>',
            b'<< /Length 0 >>\nstream\n\nendstream',
        ]), 'pdf')

    def png(self):
        for length in ([3, 24 * 1024 * 1024] if self.args.quick else [3, 3 * 1024 * 1024, 24 * 1024 * 1024]):
            content = b'\x89PNG\r\n\x1a\n'
            content += png_chunk(b'IHDR', struct.pack('>IIBBBBB', 1, 1, 8, 2, 0, 0, 0))
            content += png_chunk(b'PLTE', bytes(length))
            content += png_chunk(b'IDAT', zlib.compress(b'\x00\x00\x00\x00'))
            content += png_chunk(b'IEND', b'')
            self.media(f'png-palette-{length}', content, 'png')

    def deep_inputs(self):
        for count in ([64, 4096] if self.args.quick else [64, 512, 4096]):
            project = self.project(f'deep-inputs-{count}', '\\documentclass{article}\n'
                                   '\\begin{document}\\input{part0}\\end{document}\n')
            for index in range(count):
                (project / f'part{index}.tex').write_text(
                    f'\\input{{part{index + 1}}}\n' if index + 1 < count else 'Body\n')
            result = self.run(self.build_command(project, '--once'), project, measured=True)
            self.record('deep-inputs', phase='build', files=count, **result)
            command = [self.args.engine, 'check', project / 'main.tex', '--out-dir',
                       project / 'build', '--once', '--allow-warnings', '--quiet', '--report-json']
            result = self.run(command, project, measured=True)
            self.record('deep-inputs', phase='check', files=count, **result)

    def aux_concurrency(self):
        if not self.process_inspection_available('aux-concurrency'):
            return
        for count in ([4, 32] if self.args.quick else [4, 16, 32]):
            source = ('\\documentclass{article}\n\\begin{document}\nHello\n\\iffalse\n'
                      + '\n'.join(f'\\includegraphics{{image{i}.eps}}' for i in range(count))
                      + '\n\\fi\n\\end{document}\n')
            project = self.project(f'aux-concurrency-{count}', source)
            program_dir = project / 'programs'
            program_dir.mkdir()
            program = program_dir / 'epstopdf'
            blank_pdf = pdf_bytes([
                b'<< /Type /Catalog /Pages 2 0 R >>',
                b'<< /Type /Pages /Kids [3 0 R] /Count 1 >>',
                b'<< /Type /Page /Parent 2 0 R /MediaBox [0 0 10 10] /Contents 4 0 R >>',
                b'<< /Length 0 >>\nstream\n\nendstream',
            ])
            # The stand-in performs no conversion. It delays briefly to expose
            # peak concurrency, then writes a tiny valid PDF to the requested path.
            program.write_text('#!/usr/bin/env python3\nimport sys\nimport time\n'
                               'from pathlib import Path\ntime.sleep(0.5)\n'
                               'output = next(arg.split("=", 1)[1] for arg in sys.argv[1:] '
                               'if arg.startswith("--outfile="))\n'
                               f'Path(output).write_bytes({blank_pdf!r})\n')
            program.chmod(0o700)
            for index in range(count):
                (project / f'image{index}.eps').write_text(
                    '%!PS-Adobe-3.0 EPSF-3.0\n%%BoundingBox: 0 0 10 10\nshowpage\n')
            env = self.environment(project, PATH=str(program_dir) + os.pathsep + os.environ.get('PATH', ''))
            command = self.build_command(project, '--external-tools', '--force')
            process = self.start(command, project, env)
            peak = 0
            started = time.monotonic()
            try:
                deadline = started + self.args.timeout
                while process.poll_exit() is None and time.monotonic() < deadline:
                    inspected = self.run(['ps', '-axo', 'pid=,ppid=,command='], self.work)
                    if inspected.get('code') != 0 or inspected.get('stdout_truncated'):
                        raise RuntimeError('Auxiliary process inspection failed')
                    lines = inspected['stdout'].splitlines()
                    active = 0
                    for line in lines:
                        fields = line.strip().split(None, 2)
                        if len(fields) == 3 and fields[1] == str(process.pid) and str(program) in fields[2]:
                            active += 1
                    peak = max(peak, active)
                    time.sleep(0.01)
                if process.poll_exit() is None:
                    self.record('aux-concurrency', jobs=count, peak_converters=peak, timeout=True)
                else:
                    stdout, stderr, truncated = self.captured(process)
                    self.record('aux-concurrency', jobs=count, peak_converters=peak,
                                cpu_count=os.cpu_count(), seconds=process.completion - started,
                                code=process.returncode, stdout=stdout, stderr_tail=stderr[-2000:],
                                stdout_truncated=truncated)
            finally:
                self.stop(process)

    def expansion(self):
        binary = REPO / 'target/release/examples/audit_expansion'
        if not binary.is_file():
            self.record('expansion', skipped=True,
                        reason='Build the tekai-pdftex audit_expansion release example first')
            return
        project = self.project('expansion')
        for depth in ([1, 64] if self.args.quick else [1, 16, 64]):
            for mode in ('read-only', 'local'):
                result = self.run([binary, 'scopes', '1000', str(depth), '32', mode],
                                  project, measured=True)
                self.record('expansion-scopes', fixture_binary=str(binary), definitions=1000,
                            depth=depth, replacement_tokens=32, mode=mode, **result)
        for mode in ('division-overflow', 'division-overflow-edef', 'hex-unicode'):
            result = self.run([binary, mode], project)
            self.record('expansion-invalid-input', input=mode, fixture_binary=str(binary), **result)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--engine', type=Path, default=REPO / 'target/release/tekai')
    parser.add_argument('--case', action='append', choices=CASES)
    parser.add_argument('--quick', action='store_true', help='Use endpoints rather than all scaling samples')
    parser.add_argument('--timeout', type=float, default=15)
    parser.add_argument('--report', type=Path, default=REPO / 'target/runtime-performance/second-audit.json')
    args = parser.parse_args()
    args.engine = args.engine.resolve()
    args.report = args.report.resolve()
    if os.uname().sysname != 'Darwin':
        parser.error('RSS measurements require macOS')
    if not args.engine.is_file() or not os.access(args.engine, os.X_OK):
        parser.error('build a release tekai binary or supply --engine')
    if not 0 < args.timeout <= 60:
        parser.error('--timeout must be positive and at most 60 seconds')
    args.report.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix='tekai-second-audit-') as temporary:
        audit = Audit(args, Path(temporary).resolve())
        # Bundle extraction is outside the timing samples. Keep every cache
        # writable inside this fixture, never in the user's shared cache.
        try:
            warmup = audit.project('warmup', '\\documentclass{article}\n'
                                  '\\begin{document}Warmup\\end{document}\n')
            result = audit.run(audit.build_command(warmup, '--once'), warmup)
            if result.get('code') != 0:
                raise RuntimeError(f'Isolated runtime warmup failed: {result}')
            for case in args.case or CASES:
                getattr(audit, case.replace('-', '_'))()
        finally:
            audit.close()


if __name__ == '__main__':
    main()
