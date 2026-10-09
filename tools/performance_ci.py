#!/usr/bin/env python3
"""Bounded performance regression checks for macOS and Linux.

Correctness assertions determine the exit status. Wall time and peak RSS are
observations, never speed thresholds. Known open audit failures are labeled
separately, and unrelated errors in those fixtures still fail the run. Every
compiler, timer, converter and helper runs in an owned process group. All TeX
inputs, home directories and caches live in one temporary fixture directory.
"""

import argparse
from collections import Counter
import hashlib
import json
import math
import os
from pathlib import Path
import platform
import re
import shutil
import signal
import struct
import subprocess
import sys
import tempfile
import time
import zlib

from audit_runtime import Audit, CASES as AUDIT_CASES, png_chunk
from benchmark_runtime import document, pad, png

REPO = Path(__file__).resolve().parent.parent
QUICK_CASES = ('lookup', 'lint', 'input-identity', 'png', 'preview', 'runtime', 'images')
FULL_CASES = (*AUDIT_CASES, 'runtime', 'images', 'watch-retention')
MAX_STDOUT_BYTES = 1024 * 1024
LOG_TAIL_BYTES = 8192
HASH_CHUNK_BYTES = 1024 * 1024
STATUSES = ('passed', 'failed', 'known_failure', 'unexpected_pass', 'observation', 'skipped')
KNOWN_ISSUES = {
    'lookup-precedence': 'New higher-priority inputs do not invalidate the build cache.',
    'source-boundaries': 'Textual end markers can hide active source changes.',
    'edit-race': 'Changes during compilation can publish a stale PDF as fresh.',
    'format-cache': 'Replacing a compressed format can reuse its stale raw companion.',
    'cancel': 'Cancellation can leave the engine running.',
}
OPEN_SCALING = {
    'format-many': 'Repeated prefix scans and string edits can be quadratic.',
    'lint-long': 'Column lookup can repeat prefix scans.',
    'lint-slashes': 'Escape checks can repeat backslash scans.',
    'unrelated-sidecars': 'Settled caches load unrelated sidecar outputs.',
    'pdf-shared-resources': 'Shared resources are cloned for each page.',
    'expansion-scopes': 'Mutating local groups clone the expansion state.',
    'aux-concurrency': 'Auxiliary converters have no shared concurrency ceiling.',
}


def normal_engine_input_error(values):
    """CLI exit 1 alone cannot distinguish a TeX error from a child crash.

    compiler.rs reports the child's ExitStatus in this line. Require its exact
    normal exit status, rather than accepting a signal or a Rust panic exit.
    """
    statuses = re.findall(r'(?m)^(?:Error: )?TeX engine failed with status ([^\r\n]+)\r?$',
                          values.get('stderr_tail', ''))
    return values.get('code') == 1 and not values.get('timeout') and statuses == ['exit status: 1']


def verdict(case, values):
    """Classify fixture evidence. Only an exact known reproduction is an xfail."""
    checks = []

    def check(name, condition):
        checks.append({'name': name, 'passed': bool(condition)})

    if values.get('skipped'):
        return {'status': 'skipped', 'reason': values.get('reason', 'Dependency unavailable'),
                'assertions': checks}
    if 'exception' in values:
        check('fixture completed without an infrastructure error', False)
    elif case in ('lookup-precedence', 'database-identity', 'input-identity', 'source-boundaries'):
        expected = 'NEW-CONTENT-LONGER' if case == 'input-identity' else (
            'NEW-CHOICE' if case in ('lookup-precedence', 'database-identity') else 'NEW-CONTENT')
        old = 'OLD-CHOICE' if case in ('lookup-precedence', 'database-identity') else 'OLD-CONTENT'
        check('forced control contains the new text', expected in values.get('forced_text', ''))
        if case == 'input-identity':
            check('fixture preserved source mtime', values.get('mtime_preserved'))
            check('fixture changed physical size', values.get('size_changed'))
        if case == 'source-boundaries':
            check('fixture changed source mtime', values.get('mtime_changed'))
        if 'located' in values:
            check('resolver locates the replacement', values['located'].get('code') == 0)
        control_ok = all(item['passed'] for item in checks)
        fresh = values.get('next_build', {}).get('skipped') is False and \
            expected in values.get('text', '') and old not in values.get('text', '')
        check('ordinary build rebuilds and contains the new text', fresh)
        known = control_ok and values.get('stale_cache_hit') is True
    elif case == 'edit-race':
        check('engine consumed the old source before the edit', 'OLD-CONTENT' in values.get('first_text', ''))
        control_ok = all(item['passed'] for item in checks)
        check('next build contains the new text', 'NEW-CONTENT' in values.get('next_text', '')
              and 'OLD-CONTENT' not in values.get('next_text', ''))
        known = control_ok and values.get('stale_cache_hit') is True
    elif case == 'format-cache':
        check('initial format emits old text', 'OLD-FORMAT' in values.get('first_text', ''))
        check('fresh raw companion emits new text', 'NEW-FORMAT' in values.get('refreshed_text', ''))
        control_ok = all(item['passed'] for item in checks)
        check('replacement format emits new text', 'NEW-FORMAT' in values.get('second_text', '')
              and 'OLD-FORMAT' not in values.get('second_text', ''))
        known = control_ok and values.get('stale_raw_companion') is True
    elif case == 'cancel':
        check('engine child started', values.get('child_started'))
        check('parent exited after termination', values.get('parent_exited'))
        control_ok = all(item['passed'] for item in checks)
        check('engine child exits with its parent', values.get('child_survived') is False)
        known = control_ok and values.get('child_survived') is True
    elif case == 'pdf-parent-cycle':
        check('cyclic PDF import returns a normal engine input error', normal_engine_input_error(values))
        # TeX inserts line breaks at print width, including inside words. Drop
        # only those breaks, retaining the diagnostic's literal spaces.
        engine_log = values.get('engine_log', '').replace('\r', '').replace('\n', '')
        check('cyclic PDF import identifies the parent-chain cycle',
              'xpdf: cyclic PDF page Parent chain' in engine_log)
    elif case.startswith('png-invalid-'):
        check('invalid metadata returns a normal engine input error', normal_engine_input_error(values))
        check('error identifies the invalid metadata', values.get('expected_error', '')
              in values.get('engine_log', '') and bool(values.get('expected_error')))
    elif case.startswith('png-palette-'):
        invalid = int(case.rsplit('-', 1)[1]) > 768
        check('palette fixture returns the expected status', values.get('code') == (1 if invalid else 0)
              and not values.get('timeout'))
        if invalid:
            check('invalid palette returns a normal engine input error', normal_engine_input_error(values))
            check('palette rejection identifies metadata bounds', 'invalid PNG PLTE length' in values.get('engine_log', ''))
    elif case == 'unicode-preview':
        check('watcher completed Unicode preview prewarming', values.get('prewarmed'))
        check('watcher remains alive after prewarming', values.get('alive'))
    elif case == 'expansion-invalid-input':
        check('invalid expansion exits normally', values.get('code') == 0 and not values.get('timeout'))
        check('invalid expansion returns ExpandError', 'expansion_error=' in values.get('stdout', ''))
    elif case == 'expansion-scopes':
        check('scope expansion succeeds', values.get('code') == 0 and not values.get('timeout'))
        match = re.search(r'output_tokens=(\d+)', values.get('stdout', ''))
        check('scope expansion emits the expected finite token count',
              match is not None and int(match[1]) == 2 * values['depth'] + 1)
    elif case == 'symlink-dag':
        check('finite missing-file DAG lookup returns not found', values.get('code') == 1
              and not values.get('timeout'))
        check('lookup identifies the expected missing fixture input', bool(re.search(
            r'(?m)^Error: TeX input missing\.sty was not found; use --report-json to inspect its search paths\r?$',
            values.get('stderr_tail', ''))))
    elif case == 'symlink-alias-semantics':
        check('named alias component selects its target rather than a decoy', values.get('code') == 0
              and values.get('alias_found') and not values.get('timeout'))
    elif case in ('format-many', 'lint-long', 'lint-slashes'):
        check('CLI returns the expected diagnostic status', values.get('code') ==
              (1 if case == 'format-many' else 0) and not values.get('timeout'))
        check('read-only check preserves the source bytes', values.get('source_unchanged'))
    elif case == 'lint-correctness':
        check('format check reports two fixes per math line', values.get('fixes_available') == 2 * values['lines'])
        check('format check writes no changes', values.get('fixes_applied') == 0
              and values.get('files_changed') == [] and values.get('source_unchanged'))
        check('format check reports required edits as exit 1', values.get('code') == 1)
        check('format check counts expected warnings', values.get('warning_count') == 2 * values['lines'])
    elif case == 'runtime-cache-hit':
        check('first build compiles', values.get('first', {}).get('skipped') is False)
        check('unchanged second build skips TeX', values.get('second', {}).get('skipped') is True
              and values.get('second', {}).get('tex_runs') == 0)
    elif case == 'watch-retention':
        check('watcher renders every finite edit', values.get('completed_edits') == values.get('requested_edits'))
        check('watcher remains alive', values.get('alive'))
    elif case == 'executable-integrity':
        check('selected executable matches its starting SHA-256', values.get('unchanged')
              and values.get('sha256_start') is not None)
    elif case == 'deep-inputs':
        # TeX has a finite input stack. Its normal capacity error is acceptable.
        check('deep input returns success or a normal capacity error',
              values.get('code') in (0, 1) and not values.get('timeout'))
        if values.get('code') == 1:
            check('deep input rejection is a normal engine input error', normal_engine_input_error(values))
            check('deep input rejection identifies TeX capacity',
                  'capacity exceeded' in values.get('stdout', '') + values.get('stderr_tail', ''))
    else:
        check('fixture command succeeds', values.get('code') == 0 and not values.get('timeout'))

    issue = KNOWN_ISSUES.get(case)
    passed = all(item['passed'] for item in checks)
    if issue:
        status = 'unexpected_pass' if passed else ('known_failure' if locals().get('known', False) else 'failed')
    else:
        status = 'passed' if passed else 'failed'
    result = {'status': status, 'assertions': checks}
    if issue:
        result['known_issue'] = issue
    for prefix, problem in OPEN_SCALING.items():
        if case.startswith(prefix):
            result['open_scaling_issue'] = problem
            if result['status'] == 'passed':
                result['status'] = 'observation'
            break
    return result


def without_timings(value):
    """Keep engine-reported phase timings outside correctness evidence."""
    if isinstance(value, dict):
        return {key: without_timings(item) for key, item in value.items()
                if key not in ('seconds', 'peak_mib', 'rss_available', 'status_source', 'rss_unavailable_reason')
                and not key.endswith('_ms')}
    if isinstance(value, list):
        return [without_timings(item) for item in value]
    return value


def executable_sha256(path):
    """Hash without retaining a release executable in memory."""
    digest = hashlib.sha256()
    with path.open('rb') as handle:
        while True:
            chunk = handle.read(HASH_CHUNK_BYTES)
            if not chunk:
                break
            digest.update(chunk)
    return digest.hexdigest()


def timing_fields(value, prefix=''):
    fields = {}
    if isinstance(value, dict):
        for key, item in value.items():
            path = f'{prefix}.{key}' if prefix else key
            if key in ('seconds', 'peak_mib') or key.endswith('_ms'):
                fields[path] = item
            else:
                fields.update(timing_fields(item, path))
    elif isinstance(value, list):
        for index, item in enumerate(value):
            fields.update(timing_fields(item, f'{prefix}[{index}]'))
    return fields


def summarize(report):
    counts = Counter(row['correctness']['status'] for row in report['results'])
    return {status: counts[status] for status in STATUSES}


def failed_report(report, args):
    counts = summarize(report)
    return bool(counts['failed'] or (args.strict_known_failures and counts['known_failure'])
                or (args.fail_on_skip and counts['skipped']))


def watch_build_messages(stderr):
    return len(re.findall(r'^(?:built|cached) ', stderr, re.M))


def watch_ready(stderr, previous, initial):
    # A published PDF can precede cache publication and watcher prewarming.
    # Waiting for completion messages avoids turning retention edits into the
    # separate, deliberately controlled edit-during-build race fixture.
    return watch_build_messages(stderr) > previous and (not initial or bool(
        re.search(r'^(?:built|cached) .*\.tekai-hmr-warm[/\\]main\.pdf', stderr, re.M)))


class PerformanceCI(Audit):
    """Reuse audit fixtures, replacing their platform-specific command runner."""

    def __init__(self, args, work):
        self.args, self.work = args, work
        self.args.quick = args.profile == 'quick'
        self.owned = {}
        self.counter = 0
        self.timer = None
        self.timer_probed = False
        self.timer_reason = 'RSS timer was not requested'
        self.pdftext = shutil.which('pdftotext')
        self.ps = shutil.which('ps')
        self.report = {'schema_version': 1, 'profile': args.profile, 'binary': str(args.engine),
                       'platform': platform.system(), 'architecture': platform.machine(),
                       'python': platform.python_version(), 'command_timeout_seconds': args.timeout,
                       'results': [], 'observations': [], 'timing_policy': 'Observations only, no speed thresholds',
                       'fixture_policy': 'Temporary inputs, home, output and caches; owned process groups',
                       'limitations': ['Peak RSS is optional and includes the supervised command tree as reported by the system timer.',
                                       'CLI completion does not establish linear work or a leak-free runtime. Rust operation-count and retention tests provide those gates.',
                                       'Known open correctness failures and open scaling costs are not passing regression claims.',
                                       'No installed TeX, real-paper fidelity comparison, or Windows process-tree support is exercised.']}
        # Prevent the host's TeX settings, CLI configuration or runner override
        # from redirecting a probe to user files or an installed engine.
        self.env = {key: value for key, value in os.environ.items()
                    if not key.startswith(('TEKAI_', 'TEX', 'BIB', 'KPATHSEA'))
                    and key not in ('WEB2C', 'INDEXSTYLE')}
        for name in ('home', 'tmp', 'xdg-cache', 'xdg-config', 'xdg-data'):
            (work / name).mkdir()
        self.env.update(HOME=str(work / 'home'), USERPROFILE=str(work / 'home'),
                        TMPDIR=str(work / 'tmp'), TMP=str(work / 'tmp'), TEMP=str(work / 'tmp'),
                        XDG_CACHE_HOME=str(work / 'xdg-cache'), XDG_CONFIG_HOME=str(work / 'xdg-config'),
                        XDG_DATA_HOME=str(work / 'xdg-data'), APPDATA=str(work / 'xdg-data'),
                        LOCALAPPDATA=str(work / 'xdg-cache'), PATH='', TEKAI_TEXMF_MODE='bundled',
                        TEKAI_ENGINE_CACHE=str(work / 'runtime-cache'))
        self.persist()

    def capture_executables(self, selected_cases):
        selected = [('engine', self.args.engine)]
        if 'expansion' in selected_cases:
            selected.append(('expansion', self.args.expansion_engine))
        self.report['executables'] = []
        for role, path in selected:
            row = {'role': role, 'path': str(path), 'sha256_start': None, 'sha256_end': None}
            self.report['executables'].append(row)
            try:
                row['sha256_start'] = executable_sha256(path)
            except FileNotFoundError:
                if role != 'expansion':
                    raise
                # Optional expansion absence is reported by its fixture or the
                # required-dependency check. Appearance mid-run still fails.
                row['unavailable_at_start'] = True
        self.persist()

    def verify_executables(self):
        for row in self.report.get('executables', []):
            try:
                row['sha256_end'] = executable_sha256(Path(row['path']))
            except FileNotFoundError:
                row['sha256_end'] = None
            except OSError as error:
                row['unchanged'] = False
                self.record('executable-integrity', role=row['role'], path=row['path'],
                            exception=f'Could not recheck selected executable: {error}')
                continue
            row['unchanged'] = row['sha256_start'] == row['sha256_end']
            if row.get('unavailable_at_start') and row['sha256_end'] is None:
                continue
            self.record('executable-integrity', **row)

    def persist(self):
        self.report['summary'] = summarize(self.report)
        self.args.output.parent.mkdir(parents=True, exist_ok=True)
        self.args.output.write_text(json.dumps(self.report, indent=2) + '\n', encoding='utf-8')
        lines = ['# Performance CI', '', f"Profile `{self.args.profile}`.", '',
                 'Correctness and timings are separate. Known failures are open bugs, not passes.', '',
                 '| Outcome | Count |', '| --- | ---: |']
        lines.extend(f'| {status} | {count} |' for status, count in self.report['summary'].items())
        lines.extend(['', '| Case | Outcome | Details |', '| --- | --- | --- |'])
        for row in self.report['results']:
            result = row['correctness']
            failures = ', '.join(item['name'] for item in result['assertions'] if not item['passed'])
            detail = result.get('reason') or failures or result.get('open_scaling_issue', '')
            lines.append(f"| {row['case']} | {result['status']} | {detail.replace('|', '/').replace(chr(10), ' ')} |")
        lines.extend(['', '## Timing and RSS observations', '',
                      '| Command | Wall seconds | Peak MiB | Timed out |', '| --- | ---: | ---: | --- |'])
        for row in self.report['observations']:
            rss = f"{row['peak_rss_mib']:.2f}" if row['peak_rss_mib'] is not None else 'unavailable'
            lines.append(f"| {row['command_id']} | {row['elapsed_seconds']:.4f} | {rss} | {row['timed_out']} |")
        lines.extend(['', '## Limitations', '', *(f'- {item}' for item in self.report['limitations']), ''])
        self.args.output.with_suffix('.md').write_text('\n'.join(lines), encoding='utf-8')

    def record(self, case, **values):
        result = verdict(case, values)
        normalized = dict(values)
        stdout = normalized.get('stdout', '')
        if isinstance(stdout, str):
            try:
                structured = json.loads(stdout)
                if isinstance(structured, dict):
                    normalized['stdout'] = structured
            except ValueError:
                pass
        timings = timing_fields(normalized)
        if isinstance(stdout, str):
            expansion = re.search(r'expansion_ms=([0-9.]+)', stdout)
            if expansion:
                timings['expansion_ms'] = float(expansion[1])
                normalized['stdout'] = re.sub(r'\s*expansion_ms=[0-9.]+', '', stdout)
        if timings:
            self.report.setdefault('fixture_timing_observations', []).append(
                {'case': case, 'result_index': len(self.report['results']), 'fields': timings})
        self.report['results'].append({'case': case, 'correctness': result,
                                       'evidence': without_timings(normalized)})
        self.persist()
        print(f"{result['status']}: {case}", flush=True)

    def start(self, command, project, env, measured=False):
        self.counter += 1
        prefix = self.work / f'command-{self.counter:04d}'
        stdout_path, stderr_path = prefix.with_suffix('.stdout'), prefix.with_suffix('.stderr')
        with stdout_path.open('wb') as stdout, stderr_path.open('wb') as stderr:
            process = subprocess.Popen(list(map(str, command)), cwd=project, env=env,
                                       stdout=stdout, stderr=stderr, start_new_session=True)
        process.capture_paths = (stdout_path, stderr_path)
        process.command_id = self.counter
        process.invocation = list(map(str, command))
        process.started_at = time.monotonic()
        process.observe = True
        process.timed_out = False
        self.owned[process.pid] = process
        return process

    def stop(self, process):
        if self.owned.get(process.pid) is not process:
            return
        try:
            os.killpg(process.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        finally:
            # Files replace pipes, so an orphan cannot keep communicate open.
            process.wait(timeout=5)
        self.owned.pop(process.pid)
        if process.observe:
            peak = self.rss(self.captured(process)[1])
            self.report['observations'].append({'command_id': process.command_id,
                'command': process.invocation, 'elapsed_seconds': time.monotonic() - process.started_at,
                'peak_rss_mib': peak, 'rss_available': peak is not None,
                'rss_unavailable_reason': None if peak is not None else self.timer_reason,
                'return_code': None if process.timed_out else process.returncode,
                'timed_out': process.timed_out})
            self.persist()

    def close(self):
        errors = []
        for process in list(self.owned.values()):
            try:
                self.stop(process)
            except (OSError, subprocess.SubprocessError) as error:
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

    @staticmethod
    def rss(stderr):
        bsd = re.search(r'(\d+)\s+maximum resident set size', stderr)
        gnu = re.search(r'__TEKAI_RSS_KIB__=(\d+)', stderr)
        return int(bsd[1]) / 1048576 if bsd else (int(gnu[1]) / 1024 if gnu else None)

    def probe_timer(self):
        if self.timer_probed:
            return
        self.timer_probed = True
        timer = Path('/usr/bin/time')
        if not timer.is_file():
            self.timer_reason = 'System RSS timer is unavailable'
            return
        prefix = [str(timer), '-l'] if platform.system() == 'Darwin' else \
            [str(timer), '--format=__TEKAI_RSS_KIB__=%M']
        result = self.execute([*prefix, sys.executable, '-c', 'pass'], self.work, self.env,
                              timeout=min(2, self.args.timeout), observe=False)
        if result.get('code') == 0 and self.rss(result.get('stderr_tail', '')) is not None:
            self.timer = prefix
        else:
            self.timer_reason = result.get('stderr_tail') or result.get('launch_error') or 'RSS timer probe failed'

    def execute(self, command, project, env, timeout=None, observe=True):
        try:
            process = self.start(command, project, env)
        except OSError as error:
            return {'code': None, 'launch_error': str(error), 'stdout': '', 'stderr_tail': str(error)}
        result = {'command_id': process.command_id}
        process.observe = observe
        try:
            process.wait(timeout=timeout or self.args.timeout)
        except subprocess.TimeoutExpired:
            result['timeout'] = True
            process.timed_out = True
        finally:
            self.stop(process)
        stdout, stderr, truncated = self.captured(process)
        result.update(code=None if result.get('timeout') else process.returncode,
                      stdout=stdout, stderr_tail=stderr, stdout_truncated=truncated)
        return result

    def run(self, command, project, env=None, measured=False):
        if measured:
            self.probe_timer()
        prefix = self.timer if measured and self.timer else []
        return self.execute([*prefix, *command], project, env or self.environment(project))

    @staticmethod
    def parsed(result):
        if result.get('code') != 0 or result.get('stdout_truncated'):
            raise RuntimeError('Fixture build failed: ' + repr(result))
        value = json.loads(result['stdout'])
        if not isinstance(value, dict) or not isinstance(value.get('skipped'), bool) \
                or not isinstance(value.get('tex_runs'), int):
            raise ValueError('Build report is missing skipped/tex_runs fields')
        return value

    def text(self, project):
        result = self.run([self.pdftext, project / 'build/main.pdf', '-'], project)
        if result.get('code') != 0 or result.get('stdout_truncated'):
            raise RuntimeError('PDF text extraction failed: ' + repr(result))
        return result['stdout']

    def process_inspection_available(self, case):
        if not self.ps:
            self.record(case, skipped=True, reason='ps process inspection is unavailable')
            return False
        result = self.run([self.ps, '-p', str(os.getpid()), '-o', 'pid='], self.work)
        if result.get('code') != 0:
            self.record(case, skipped=True, reason='ps process inspection is denied or unsupported')
            return False
        return True

    def snapshot(self, process):
        result = self.run([self.ps, '-axo', 'pid=,ppid=,pgid=,stat=,command='], self.work)
        if result.get('code') != 0 or result.get('stdout_truncated'):
            raise RuntimeError('Owned-group process inspection failed')
        rows = []
        for line in result['stdout'].splitlines():
            fields = line.strip().split(None, 4)
            if len(fields) == 5 and fields[0].isdigit() and fields[2] == str(process.pid):
                rows.append({'pid': int(fields[0]), 'ppid': int(fields[1]),
                             'stat': fields[3], 'command': fields[4]})
        return rows

    def lookup(self):
        super().lookup()
        project = self.project('alias-semantics')
        # The CLI searches its project before TEXINPUTS. Put the target outside
        # that project so an implicit project lookup cannot mask alias spelling.
        tree = self.work / 'alias-tree'
        tree.mkdir()
        physical = self.work / 'alias-physical'
        physical.mkdir()
        (physical / 'choice.tex').write_text('alias\n', encoding='utf-8')
        alias = tree / 'alias'
        alias.symlink_to(physical, target_is_directory=True)
        decoy = tree / 'a-decoy'
        decoy.mkdir()
        (decoy / 'choice.tex').write_text('wrong target\n', encoding='utf-8')
        result = self.run([self.args.engine, 'locate', 'choice.tex', '--directory', project], project,
                          self.environment(project, TEXINPUTS=str(tree) + '//alias//'))
        # The locate CLI canonicalizes its result. Resolver unit tests cover
        # lexical alias spelling; this CLI assertion covers the chosen identity.
        self.record('symlink-alias-semantics', alias_found=(alias / 'choice.tex').is_file()
                    and result.get('stdout', '').strip() == str((physical / 'choice.tex').resolve()),
                    output_path_policy='CLI canonicalizes paths; lexical spelling is a Rust unit-test gate', **result)

    def lint(self):
        project = self.project('lint-correctness')
        source = project / 'main.tex'
        content = '$x$\n' * 4
        source.write_text(content, encoding='utf-8')
        result = self.run([self.args.engine, 'format', source, '--check', '--report-json', '--allow-warnings'], project)
        data = json.loads(result['stdout']) if result.get('code') == 1 else {}
        self.record('lint-correctness', lines=4, source_unchanged=source.read_text() == content,
                    **data, **result)
        sizes = [1000, 4000] if self.args.quick else [5000, 10000, 20000, 40000, 80000]
        for kind in ('format-many', 'lint-long', 'lint-slashes'):
            for size in sizes:
                project = self.project(f'{kind}-{size}')
                source = project / 'main.tex'
                content = '$x$\n' * size if kind == 'format-many' else \
                    ('x' if kind == 'lint-long' else '\\') * size + '\n'
                source.write_text(content, encoding='utf-8')
                command = [self.args.engine, 'format', source, '--check', '--quiet', '--allow-warnings'] \
                    if kind == 'format-many' else [self.args.engine, 'lint', source, '--allow-warnings']
                result = self.run(command, project, measured=True)
                self.record(kind, size=size, source_unchanged=source.read_text() == content, **result)
                if result.get('timeout'):
                    break

    def png(self):
        header = b'\x89PNG\r\n\x1a\n' + png_chunk(b'IHDR', struct.pack('>IIBBBBB', 1, 1, 8, 2, 0, 0, 0))
        valid = header + png_chunk(b'PLTE', bytes(3)) + png_chunk(b'IDAT', zlib.compress(bytes(4))) + png_chunk(b'IEND', b'')
        self.media('png-valid-palette', valid, 'png')
        declarations = [('PLTE', 0), ('PLTE', 4), ('PLTE', 769), ('PLTE', 24 * 1024 * 1024),
                        ('tRNS', 24 * 1024 * 1024)]
        for kind, size in declarations:
            name = f'png-invalid-{kind}-{size}'
            project = self.project(name, '\\documentclass{article}\n\\usepackage{graphicx}\n'
                                   '\\begin{document}\\includegraphics[width=1cm]{image.png}\\end{document}\n')
            # A huge declaration does not need a huge fixture payload.
            (project / 'image.png').write_bytes(header + struct.pack('>I', size) + kind.encode('ascii'))
            result = self.run(self.build_command(project, '--once', '--force'), project, measured=True)
            log = project / 'build/main.log'
            expected = 'invalid PNG PLTE length' if kind == 'PLTE' else 'invalid or duplicate PNG tRNS length'
            self.record(name, declared_bytes=size, expected_error=expected,
                        engine_log=log.read_text(errors='replace')[-LOG_TAIL_BYTES:] if log.is_file() else '', **result)
        if not self.args.quick:
            # Keep the audit's real 24-MiB input as an RSS observation as well.
            super().png()

    def media(self, name, content, extension):
        project = self.project(name, '\\documentclass{article}\n\\usepackage{graphicx}\n'
                               '\\begin{document}\\includegraphics[page=1]{image.'
                               + extension + '}\\end{document}\n')
        (project / f'image.{extension}').write_bytes(content)
        result = self.run(self.build_command(project, '--once'), project, measured=True)
        if result.get('code') == 0:
            result['build_report'] = self.parsed(result)
            result.pop('stdout')
        log = project / 'build/main.log'
        self.record(name, input_bytes=len(content),
                    engine_log=log.read_text(errors='replace')[-LOG_TAIL_BYTES:] if log.is_file() else '', **result)

    def preview(self):
        project = self.project('unicode-preview', '\\documentclass{article}\n\\begin{document}\n'
                               + 'x' * 8190 + 'é more text\n\\end{document}\n')
        command = [self.args.engine, 'watch', project / 'main.tex', '--out-dir',
                   project / 'build', '--no-lint', '--preview']
        process = self.start(command, project, self.environment(project))
        started = time.monotonic()
        ready = False
        try:
            deadline = started + self.args.timeout
            while process.poll() is None and time.monotonic() < deadline:
                _, stderr, _ = self.captured(process)
                ready = bool(re.search(r'^(?:built|cached) .*\.tekai-hmr-warm[/\\]main\.pdf', stderr, re.M))
                if ready:
                    break
                time.sleep(0.02)
            self.record('unicode-preview', prewarmed=ready, alive=process.poll() is None,
                        stderr_tail=self.captured(process)[1])
        finally:
            process.timed_out = not ready and process.poll() is None
            self.stop(process)

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
        process = self.start(command, project, self.environment(project))
        try:
            deadline = time.monotonic() + self.args.timeout
            while not (project / 'build/audit.marker').is_file():
                if process.poll() is not None or time.monotonic() >= deadline:
                    raise RuntimeError('Edit-race input-consumption marker did not appear')
                time.sleep(0.002)
            (project / 'main.tex').write_text(source.replace('OLD-CONTENT', 'NEW-CONTENT'), encoding='utf-8')
            process.wait(timeout=max(0.001, deadline - time.monotonic()))
            stdout, stderr, truncated = self.captured(process)
            first = self.parsed({'code': process.returncode, 'stdout': stdout, 'stderr_tail': stderr,
                                 'stdout_truncated': truncated})
            first_text = self.text(project)
        except subprocess.TimeoutExpired:
            process.timed_out = True
            raise
        finally:
            self.stop(process)
        following = self.parsed(self.run(command, project))
        following_text = self.text(project)
        self.record('edit-race', first=first, first_text=first_text, next_build=following,
                    next_text=following_text, stale_cache_hit=following.get('skipped') is True
                    and 'OLD-CONTENT' in following_text)

    def cancel(self):
        if not self.process_inspection_available('cancel'):
            return
        project = self.project('cancel', '\\documentclass{article}\n\\newcount\\auditcount\n'
                               '\\begin{document}\n\\loop\\advance\\auditcount by1'
                               '\\ifnum\\auditcount<100000000\\repeat\n\\end{document}\n')
        process = self.start(self.build_command(project), project, self.environment(project))
        try:
            deadline = time.monotonic() + self.args.timeout
            child = None
            while process.poll() is None and time.monotonic() < deadline:
                child = next((row for row in self.snapshot(process)
                              if row['ppid'] == process.pid and '__tekai-engine' in row['command']), None)
                if child:
                    break
                time.sleep(0.02)
            if child is None:
                raise RuntimeError('Cancellation fixture did not start an engine child')
            process.terminate()
            process.wait(timeout=max(0.001, deadline - time.monotonic()))
            # The brief grace period measures cancellation, not compiler speed.
            time.sleep(0.2)
            survivors = [row for row in self.snapshot(process)
                         if row['pid'] == child['pid'] and not row['stat'].startswith('Z')]
            self.record('cancel', child_started=True, parent_exited=True,
                        child_survived=bool(survivors), child_snapshot=survivors)
        finally:
            self.stop(process)

    def aux_concurrency(self):
        # Count fixture-owned markers instead of sampling all host processes.
        # This gives a deterministic peak overlap measurement across platforms.
        from audit_runtime import pdf_bytes
        count = 8 if self.args.quick else 32
        project = self.project('aux-concurrency', '\\documentclass{article}\n\\begin{document}\nHello\n\\iffalse\n'
                               + '\n'.join(f'\\includegraphics{{image{i}.eps}}' for i in range(count))
                               + '\n\\fi\n\\end{document}\n')
        programs = project / 'programs'
        programs.mkdir()
        converter = programs / 'epstopdf'
        blank = pdf_bytes([b'<< /Type /Catalog /Pages 2 0 R >>',
                           b'<< /Type /Pages /Kids [3 0 R] /Count 1 >>',
                           b'<< /Type /Page /Parent 2 0 R /MediaBox [0 0 10 10] /Contents 4 0 R >>',
                           b'<< /Length 0 >>\nstream\n\nendstream'])
        converter.write_text(f'#!{sys.executable}\nimport os, sys, time\nfrom pathlib import Path\n'
                             f'root = Path({str(project)!r})\n'
                             'marker = root / ("active-" + str(os.getpid()))\nmarker.touch()\n'
                             'try:\n time.sleep(0.5)\n'
                             ' output = next(arg.split("=", 1)[1] for arg in sys.argv[1:] if arg.startswith("--outfile="))\n'
                             f' Path(output).write_bytes({blank!r})\nfinally:\n marker.unlink(missing_ok=True)\n', encoding='utf-8')
        converter.chmod(0o700)
        for index in range(count):
            (project / f'image{index}.eps').write_text('%!PS-Adobe-3.0 EPSF-3.0\n%%BoundingBox: 0 0 10 10\nshowpage\n')
        env = self.environment(project, PATH=str(programs))
        process = self.start(self.build_command(project, '--external-tools', '--force'), project, env)
        peak = 0
        try:
            deadline = time.monotonic() + self.args.timeout
            while process.poll() is None and time.monotonic() < deadline:
                peak = max(peak, len(list(project.glob('active-*'))))
                time.sleep(0.01)
            timed_out = process.poll() is None
            process.timed_out = timed_out
            stdout, stderr, _ = self.captured(process)
            self.record('aux-concurrency', jobs=count, peak_converters=peak,
                        concurrency_gate=False, code=None if timed_out else process.returncode,
                        timeout=timed_out, stdout=stdout, stderr_tail=stderr)
        finally:
            self.stop(process)

    def expansion(self):
        binary = self.args.expansion_engine
        if not binary.is_file() or not os.access(binary, os.X_OK):
            self.record('expansion', skipped=True, reason='Build the audit_expansion example or supply --expansion-engine')
            return
        project = self.project('expansion')
        for depth in ([1, 64] if self.args.quick else [1, 16, 64]):
            for mode in ('read-only', 'local'):
                result = self.run([binary, 'scopes', '1000', str(depth), '32', mode], project, measured=True)
                self.record('expansion-scopes', depth=depth, definitions=1000, replacement_tokens=32, mode=mode, **result)
        for mode in ('division-overflow', 'division-overflow-edef', 'hex-unicode'):
            self.record('expansion-invalid-input', input=mode, **self.run([binary, mode], project))

    def runtime(self):
        sizes = [0, 128] if self.args.quick else [0, 1000, 4000]
        for count in sizes:
            project = self.project(f'runtime-nested-{count}')
            document(project)
            pad(project, count)
            command = self.build_command(project, '--once')
            first = self.parsed(self.run(command, project, measured=True))
            second = self.parsed(self.run(command, project, measured=True))
            self.record('runtime-cache-hit', directories=count, first=first, second=second)
        for kind in ('home', 'site'):
            for count in (sizes[0], sizes[-1]):
                tree = self.work / f'shared-{kind}-{count}'
                tree.mkdir()
                pad(tree / 'doc', count)
                (tree / 'ls-R').write_text('% ls-R\n', encoding='utf-8')
                project = self.project(f'shared-project-{kind}-{count}')
                document(project)
                env = self.environment(project, TEKAI_TEXMF_MODE='shared',
                                       TEXMFHOME=str(tree if kind == 'home' else self.work / 'absent-home'),
                                       TEXMFLOCAL=str(tree if kind == 'site' else self.work / 'absent-site'))
                command = self.build_command(project, '--once')
                first = self.parsed(self.run(command, project, env, measured=True))
                second = self.parsed(self.run(command, project, env, measured=True))
                self.record('runtime-cache-hit', tree_kind=kind, directories=count, first=first, second=second)

    def images(self):
        for count in ([1, 4] if self.args.quick else [1, 8, 32]):
            project = self.project(f'decoded-images-{count}')
            for index in range(count):
                png(project / f'i{index}.png', index)
            body = '\n'.join(f'\\includegraphics[width=1cm]{{i{index}.png}}\\newpage' for index in range(count))
            document(project, '\\documentclass{article}\n\\usepackage{graphicx}\n\\begin{document}\n' + body + '\n\\end{document}\n')
            result = self.run(self.build_command(project, '--once'), project, measured=True)
            self.record('decoded-images', images=count, **result)

    def watch_retention(self):
        if not self.needs_pdftext('watch-retention'):
            return
        project = self.project('watch-retention')
        requested = 10
        for index in range(requested):
            (project / f'part{index}.tex').write_text(('%' + 'x' * 1022 + '\n') * 256 + f'EDIT-{index}\n')
        process = None
        completed = 0
        previous_messages = 0
        last_text = ''
        last_text_error = ''
        try:
            for index in range(requested):
                # Change the preamble too. A snippet-only hot preview strips
                # input commands, so it would not consume each rotated include.
                document(project, '\\documentclass{article}\n'
                         + f'\\newcommand{{\\audititeration}}{{{index}}}\n\\begin{{document}}\n'
                         + f'\\input{{part{index}}}\n\\end{{document}}\n')
                if process is None:
                    process = self.start([self.args.engine, 'watch', project / 'main.tex', '--preview',
                                          '--root', project, '--out-dir', project / 'build', '--no-lint', '--quiet'],
                                         project, self.environment(project))
                deadline = time.monotonic() + self.args.timeout
                matched = False
                while process.poll() is None and time.monotonic() < deadline:
                    stderr = self.captured(process)[1]
                    if watch_ready(stderr, previous_messages, index == 0) and (project / 'build/main.pdf').is_file():
                        result = self.run([self.pdftext, project / 'build/main.pdf', '-'], project)
                        last_text, last_text_error = result['stdout'], result['stderr_tail']
                        if result.get('code') == 0 and f'EDIT-{index}' in result['stdout']:
                            matched = True
                            previous_messages = watch_build_messages(stderr)
                            break
                    time.sleep(0.05)
                if not matched:
                    process.timed_out = process.poll() is None
                    break
                completed += 1
                if self.ps:
                    result = self.run([self.ps, '-o', 'rss=', '-p', str(process.pid)], project)
                    if result.get('code') == 0 and result['stdout'].strip().isdigit():
                        self.report.setdefault('watch_rss_observations', []).append(
                            {'edit': index, 'rss_mib': int(result['stdout'].strip()) / 1024})
            self.record('watch-retention', requested_edits=requested, completed_edits=completed,
                        alive=process.poll() is None, retention_gate=False,
                        last_pdf_text=last_text, last_pdf_text_error=last_text_error,
                        stderr_tail=self.captured(process)[1])
        finally:
            if process is not None:
                self.stop(process)


def parser():
    result = argparse.ArgumentParser(description=__doc__)
    result.add_argument('--engine', type=Path, default=REPO / 'target/release/tekai')
    result.add_argument('--profile', choices=('quick', 'full'), default='quick')
    result.add_argument('--output', type=Path, default=REPO / 'target/performance-ci/report.json')
    result.add_argument('--timeout', type=float, default=30, help='Per-command safety timeout, greater than 0 and at most 60 seconds')
    result.add_argument('--case', action='append', choices=FULL_CASES, help='Run only the named fixture family; repeat to select several')
    result.add_argument('--expansion-engine', type=Path, help='Defaults to ENGINE_PARENT/examples/audit_expansion')
    result.add_argument('--require-pdftotext', action='store_true')
    result.add_argument('--require-expansion', action='store_true')
    result.add_argument('--fail-on-skip', action='store_true')
    result.add_argument('--strict-known-failures', action='store_true', help='Also fail on the exact known open bugs')
    return result


def main(argv=None):
    argument_parser = parser()
    args = argument_parser.parse_args(argv)
    if os.name != 'posix':
        argument_parser.error('The runtime checks require POSIX process groups, supported on Linux and macOS')
    if not math.isfinite(args.timeout) or not 0 < args.timeout <= 60:
        argument_parser.error('--timeout must be finite, greater than 0 and at most 60 seconds')
    args.engine, args.output = args.engine.resolve(), args.output.resolve()
    args.expansion_engine = (args.expansion_engine or args.engine.parent / 'examples/audit_expansion').resolve()
    if not args.engine.is_file() or not os.access(args.engine, os.X_OK):
        argument_parser.error('Build a release tekai binary or supply an executable --engine')
    cases = list(dict.fromkeys(args.case or (QUICK_CASES if args.profile == 'quick' else FULL_CASES)))
    # Report missing dependencies instead of exiting before artifact creation.
    with tempfile.TemporaryDirectory(prefix='tekai-performance-ci-') as temporary:
        audit = PerformanceCI(args, Path(temporary).resolve())
        audit.report['selected_cases'] = cases
        interrupted = False
        previous_handler = signal.getsignal(signal.SIGTERM)

        def terminate(_number, _frame):
            raise KeyboardInterrupt

        signal.signal(signal.SIGTERM, terminate)
        try:
            audit.capture_executables(cases)
            if args.require_pdftotext and not audit.pdftext:
                audit.record('required-pdftotext', exception='pdftotext is required but unavailable')
            elif args.require_expansion and (not args.expansion_engine.is_file() or not os.access(args.expansion_engine, os.X_OK)):
                audit.record('required-expansion', exception='audit_expansion is required but unavailable')
            else:
                project = audit.project('warmup', '\\documentclass{article}\n\\begin{document}Warmup\\end{document}\n')
                warmup = audit.run(audit.build_command(project, '--once'), project)
                audit.record('warmup', **warmup)
                if warmup.get('code') == 0:
                    for case in cases:
                        cleanup_failed = False
                        try:
                            getattr(audit, case.replace('-', '_'))()
                        except (OSError, ValueError, RuntimeError, subprocess.SubprocessError) as error:
                            audit.record(case, exception=f'{type(error).__name__}: {error}')
                        finally:
                            try:
                                audit.close()
                            except RuntimeError as error:
                                audit.record('cleanup', exception=str(error))
                                cleanup_failed = True
                        if cleanup_failed:
                            break
        except KeyboardInterrupt:
            interrupted = True
            audit.record('interrupted', exception='Run interrupted; terminating all owned process groups')
        except (OSError, ValueError, RuntimeError, subprocess.SubprocessError) as error:
            audit.record('runner', exception=f'{type(error).__name__}: {error}')
        finally:
            try:
                audit.close()
            except RuntimeError as error:
                audit.record('cleanup', exception=str(error))
            finally:
                try:
                    audit.verify_executables()
                finally:
                    signal.signal(signal.SIGTERM, previous_handler)
                    audit.persist()
        return 130 if interrupted else int(failed_report(audit.report, args))


if __name__ == '__main__':
    sys.exit(main())
