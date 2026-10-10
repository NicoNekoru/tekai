#!/usr/bin/env python3
"""Bounded same-runner paired comparison, using generated ordinary inputs.

Baseline and candidate have separate binaries, inputs, homes and caches.
Timings are advisory unless --gate is selected. A gate can be inconclusive.
"""

import argparse
import hashlib
import json
import math
import os
from pathlib import Path
import platform
import re
import shutil
import signal
import statistics
import subprocess
import sys
import tempfile
import threading
import time

from performance_ci import PerformanceCI
from runtime_fixtures import document, pad, png  # Keep the existing helper imports usable.

REPO = Path(__file__).resolve().parent.parent
CASES = ('nested-lookup', 'image-compile', 'warm-build-cache')
LABELS = ('baseline', 'candidate')
MAX_CAPTURE_BYTES = 1024 * 1024
CALIBRATION_HEADROOM = 2.0
NOISE_LIMIT = 0.10
ORDER_BIAS_LIMIT = 0.05
FAMILY_ALPHA = 0.05
SEARCH_ENV_VARS = frozenset(('WEB2C', 'INDEXSTYLE', 'BSTINPUTS', 'TFMFONTS', 'AFMFONTS',
                            'ENCFONTS', 'SFDFONTS', 'PKFONTS', 'GFFONTS', 'VFFONTS',
                            'T1FONTS', 'TTFONTS', 'OPENTYPEFONTS'))


def executable_sha256(path):
    digest = hashlib.sha256()
    with path.open('rb') as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b''):
            digest.update(chunk)
    return digest.hexdigest()


def fixture_sha256(project):
    """Hash relative input paths and bytes, excluding separate output/cache dirs."""
    digest = hashlib.sha256()
    for path in sorted(project.rglob('*')):
        if path.is_file():
            digest.update(path.relative_to(project).as_posix().encode('utf-8') + b'\0')
            digest.update(bytes.fromhex(executable_sha256(path)))
    return digest.hexdigest()


def paired_order(pair_index, case_index=0):
    return LABELS if (pair_index + case_index) % 2 == 0 else tuple(reversed(LABELS))


def median_interval(values, alpha):
    """Exact two-sided sign interval, with no normal approximation or randomness.

    For order statistic k, noncoverage is at most twice the Binomial(n, .5)
    lower tail through k-1. Coverage assumes independent balanced blocks with
    a stable population median. Ties make the interval conservative.
    """
    if not values or any(not math.isfinite(value) or value <= 0 for value in values):
        raise ValueError('Median interval needs finite positive observations')
    if not 0 < alpha < 1:
        raise ValueError('Interval alpha must be between zero and one')
    ordered = sorted(values)
    n, lower_tail, selected = len(ordered), 0, None
    for k in range(1, (n + 1) // 2 + 1):
        lower_tail += math.comb(n, k - 1)
        noncoverage = 2 * lower_tail / 2 ** n
        if noncoverage <= alpha:
            selected = {'lower': ordered[k - 1], 'upper': ordered[n - k],
                        'coverage_at_least': 1 - noncoverage, 'order_statistic': k}
        else:
            break
    return selected


def relative_mad(values):
    center = statistics.median(values)
    return statistics.median(abs(value - center) for value in values) / center


def calibrated_iterations(warmups, min_seconds, ceiling):
    fastest = min(statistics.median(item[label]['seconds'] for item in warmups) for label in LABELS)
    if not math.isfinite(fastest) or fastest <= 0:
        raise ValueError('Calibration observations must be finite and positive')
    required = math.ceil(min_seconds / fastest)
    if required > ceiling:
        raise CalibrationInfeasible({'minimum_iterations': required, 'iteration_ceiling': ceiling,
                                    'fastest_warmup_median_seconds': fastest,
                                    'predicted_ceiling_batch_seconds': fastest * ceiling,
                                    'minimum_batch_seconds': min_seconds})
    return min(ceiling, max(1, math.ceil(min_seconds * CALIBRATION_HEADROOM / fastest)))


def analyze_pairs(samples, threshold, min_seconds, case_count=len(CASES)):
    """Keep insufficient precision and unstable measurements inconclusive."""
    if not math.isfinite(threshold) or not 0 < threshold <= 1:
        raise ValueError('Practical threshold must be finite and in (0, 1]')
    if not math.isfinite(min_seconds) or min_seconds <= 0 or case_count < 1:
        raise ValueError('Duration and comparison count must be positive')
    if len(samples) % 2 or not samples:
        raise ValueError('Paired samples must contain complete balanced blocks')
    values = {label: [] for label in LABELS}
    ratios, orders = [], {label: [] for label in LABELS}
    for index, sample in enumerate(samples):
        order = tuple(sample['order'])
        if set(order) != set(LABELS) or len(order) != 2:
            raise ValueError('Each sample must execute each binary once')
        if index % 2 and order != tuple(reversed(samples[index - 1]['order'])):
            raise ValueError('Adjacent pairs must balance execution order')
        for label in LABELS:
            elapsed = sample[label]['seconds']
            if not math.isfinite(elapsed) or elapsed <= 0:
                raise ValueError('Elapsed times must be finite and positive')
            values[label].append(elapsed)
        ratio = values['candidate'][-1] / values['baseline'][-1]
        ratios.append(ratio)
        orders[order[0]].append(ratio)
    # BC/CB neighbors form one inference unit. Taking their geometric mean
    # cancels a fixed order effect; the two pairs are not independent samples.
    blocks = [math.exp((math.log(ratios[n]) + math.log(ratios[n + 1])) / 2)
              for n in range(0, len(ratios), 2)]
    interval = median_interval(blocks, FAMILY_ALPHA / case_count)
    noise = {label: relative_mad(values[label]) for label in LABELS}
    order_medians = {label: statistics.median(orders[label]) for label in LABELS}
    order_bias = max(order_medians.values()) / min(order_medians.values()) - 1
    issues = []
    if any(min(items) < min_seconds for items in values.values()):
        issues.append('At least one batch is shorter than the predeclared minimum duration')
    if any(value > NOISE_LIMIT for value in noise.values()):
        issues.append('Per-binary relative median absolute deviation exceeds the noise limit')
    if order_bias > ORDER_BIAS_LIMIT:
        issues.append('Baseline-first and candidate-first ratios differ beyond the order-bias limit')
    if interval is None:
        issues.append('Too few balanced blocks for a finite multiplicity-adjusted median interval')
    boundary = 1 + threshold
    if issues:
        status = 'inconclusive'
    elif interval['lower'] > boundary:
        status = 'fail'
    elif interval['upper'] <= boundary:
        status = 'pass'
    else:
        status = 'inconclusive'
        issues.append('Median-ratio uncertainty interval crosses the practical slowdown boundary')
    return {'status': status, 'median_ratio': statistics.median(blocks),
            'paired_ratios': ratios, 'balanced_block_ratios': blocks,
            'median_ratio_interval': interval, 'relative_mad': noise,
            'order_median_ratios': order_medians, 'order_bias': order_bias,
            'reasons': issues, 'practical_slowdown_boundary': boundary}


def comparison_exit(status, gate):
    if status == 'error':
        return 1
    return {'pass': 0, 'fail': 1, 'inconclusive': 2}[status] if gate else 0


class BenchmarkError(RuntimeError):
    pass


class CalibrationInfeasible(ValueError):
    def __init__(self, details):
        super().__init__('Calibration iteration ceiling cannot reach the minimum batch duration')
        self.details = details


class Supervisor(PerformanceCI):
    """Reuse PerformanceCI's tested process ownership, launch and group cleanup.

    A blocking waiter timestamps completion. Popen.wait with a timeout polls at
    up to 50 ms, distorting short cache-hit samples. Capture files keep orphaned
    children from holding pipes open. Per-command/whole-run limits bound work.
    """
    def __init__(self, work, timeout, deadline):
        self.work, self.timeout, self.deadline = work, timeout, deadline
        self.owned, self.counter = {}, 0
        self.report = {'observations': []}
        self.timer_reason = 'RSS is not measured by the paired benchmark'

    def persist(self):
        # Inherited stop normally sees observe=False. A signal between launch
        # and that assignment can still safely finish its observation cleanup.
        pass

    def remaining(self):
        remaining = self.deadline - time.monotonic()
        if remaining <= 0:
            raise BenchmarkError('Whole-run deadline exceeded')
        return remaining

    def execute(self, command, cwd, env):
        allowance = min(self.timeout, self.remaining())
        started = time.monotonic()
        process = self.start(command, cwd, env)
        process.observe = False
        process.started_at = started
        completed, observation = threading.Event(), {}

        def wait():
            try:
                observation['code'] = process.wait()
                observation['seconds'] = time.monotonic() - process.started_at
            finally:
                completed.set()

        waiter = threading.Thread(target=wait, daemon=True)
        waiter.start()
        failure = None
        try:
            while not completed.wait(min(0.05, max(0.001, process.started_at + allowance - time.monotonic()))):
                if any(path.stat().st_size > MAX_CAPTURE_BYTES for path in process.capture_paths):
                    failure = 'Command capture exceeded the finite output limit'
                    break
                if time.monotonic() >= process.started_at + allowance:
                    failure = 'Command deadline exceeded'
                    break
        finally:
            # The inherited method kills the exact owned group even after a
            # successful parent exit, then reaps and unregisters the process.
            self.stop(process)
            waiter.join(timeout=5)
        if waiter.is_alive():
            raise BenchmarkError('Process waiter did not finish after owned-group cleanup')
        stdout, stderr, truncated = self.captured(process)
        if observation.get('seconds', 0) > allowance:
            failure = failure or 'Command deadline exceeded'
        if truncated or any(path.stat().st_size > MAX_CAPTURE_BYTES for path in process.capture_paths):
            failure = failure or 'Command capture exceeded the finite output limit'
        if failure or observation.get('code') != 0:
            raise BenchmarkError(f'{failure or "Command failed"}; return code {observation.get("code")}\n'
                                 + stdout[-8192:] + '\n' + stderr)
        for path in process.capture_paths:
            path.unlink()
        return {'command_id': process.command_id, 'seconds': observation['seconds'],
                'stdout': stdout, 'stderr_tail': stderr}


def isolated_environment(root):
    env = {key: value for key, value in os.environ.items()
           if not key.startswith(('TEKAI_', 'TEX', 'BIB', 'KPATHSEA'))
           and key not in SEARCH_ENV_VARS}
    locations = {name: root / name for name in ('home', 'tmp', 'xdg-cache', 'xdg-config', 'xdg-data')}
    for path in locations.values():
        path.mkdir(parents=True)
    env.update(HOME=str(locations['home']), USERPROFILE=str(locations['home']),
               TMPDIR=str(locations['tmp']), TMP=str(locations['tmp']), TEMP=str(locations['tmp']),
               XDG_CACHE_HOME=str(locations['xdg-cache']), XDG_CONFIG_HOME=str(locations['xdg-config']),
               XDG_DATA_HOME=str(locations['xdg-data']), APPDATA=str(locations['xdg-data']),
               LOCALAPPDATA=str(locations['xdg-cache']), PATH='', TEKAI_TEXMF_MODE='bundled')
    for kind in ('ENGINE', 'FORMAT', 'AUX', 'BIBTEX'):
        path = root / ('cache-' + kind.lower())
        path.mkdir()
        env['TEKAI_' + kind + '_CACHE'] = str(path)
    return env


def make_fixture(root, binary, case):
    project, out = root / 'project', root / 'out'
    document(project)
    out.mkdir()
    env = isolated_environment(root)
    if case in ('nested-lookup', 'warm-build-cache'):
        pad(project, 1000)
    if case == 'nested-lookup':
        nested = project / 'content/ordinary'
        nested.mkdir(parents=True)
        (nested / 'needle.tex').write_text('Ordinary nested lookup.\n', encoding='utf-8')
        document(project, '\\documentclass{article}\n\\begin{document}\\input{needle}\\end{document}\n')
    if case == 'image-compile':
        for index in range(8):
            png(project / f'i{index}.png', index)
        body = '\n'.join(f'\\includegraphics[width=1cm]{{i{index}.png}}\\newpage' for index in range(8))
        document(project, '\\documentclass{article}\n\\usepackage{graphicx}\n\\begin{document}\n' + body + '\n\\end{document}\n')
    env['TEXINPUTS'] = f'{project}//:{out}//:'
    command = [binary, 'build', project / 'main.tex', '--out-dir', out, '--once', '--quiet', '--report-json'] \
        if case == 'warm-build-cache' else [binary, '__tekai-engine', '-interaction=nonstopmode',
          '-halt-on-error', '-no-shell-escape', f'-output-directory={out}', 'main.tex']
    return {'root': root, 'project': project, 'out': out, 'env': env,
            'command': command, 'input_sha256': fixture_sha256(project)}


def check_pdf(path):
    with path.open('rb') as handle:
        header = handle.read(5)
        handle.seek(max(0, path.stat().st_size - 64))
        tail = handle.read(64)
    if header != b'%PDF-' or not tail.rstrip().endswith(b'%%EOF'):
        raise BenchmarkError('Fixture did not produce a complete PDF')


def check_uniform_pnm(path, pixel, dimensions=(1024, 1024)):
    """Verify every decoded pixel of a finite fixture-owned Poppler PPM/PGM."""
    magic = b'P6' if len(pixel) == 3 else b'P5'
    with path.open('rb') as handle:
        if handle.readline(16).strip() != magic:
            raise BenchmarkError('Decoded image has the wrong color component count')
        line = handle.readline(1024)
        comment_lines = 0
        while line.startswith(b'#') and comment_lines < 16:
            line = handle.readline(1024)
            comment_lines += 1
        if line.split() != [str(value).encode('ascii') for value in dimensions] \
                or handle.readline(16).strip() != b'255':
            raise BenchmarkError('Decoded image has unexpected dimensions or sample depth')
        remaining = dimensions[0] * dimensions[1] * len(pixel)
        chunk_size = 65536 * len(pixel)
        expected = pixel * 65536
        while remaining:
            count = min(remaining, chunk_size)
            if handle.read(count) != expected[:count]:
                raise BenchmarkError('Decoded image pixels differ from the expected fixture color or alpha')
            remaining -= count
        if handle.read(1):
            raise BenchmarkError('Decoded image has trailing pixel data')


def check_fixture_output(supervisor, fixture, case):
    pdf = fixture['out'] / 'main.pdf'
    if case == 'image-compile':
        if pdf.stat().st_size > 64 * 1024 * 1024:
            raise BenchmarkError('Image PDF exceeds the finite fixture size limit')
        info = supervisor.execute([fixture['pdfinfo'], pdf], fixture['project'], fixture['env'])
        image_list = supervisor.execute([fixture['pdfimages'], '-list', pdf], fixture['project'], fixture['env'])
        pages = re.findall(r'(?m)^Pages:\s+(\d+)\s*$', info['stdout'])
        images = [line.split() for line in image_list['stdout'].splitlines()
                  if re.match(r'^\s*\d+\s+\d+\s+', line)]
        # Poppler interprets compressed object streams independently of Tekai.
        # Each page must contain its 1024px color image and alpha mask.
        valid = pages == ['8'] and len(images) == 16 and all(
            len(fields) >= 8 and fields[2] in ('image', 'smask')
            and fields[3:5] == ['1024', '1024'] and fields[7] == '8'
            and fields[5:7] == (['rgb', '3'] if fields[2] == 'image' else ['gray', '1']) for fields in images)
        for kind in ('image', 'smask'):
            valid = valid and sorted(int(fields[0]) for fields in images if len(fields) >= 8 and fields[2] == kind) == list(range(1, 9))
        if not valid:
            raise BenchmarkError('Image fixture lacks the expected eight pages and 1024px color/alpha images')
        decoded = fixture['out'] / 'decoded-verification'
        decoded.mkdir(exist_ok=True)
        prefix = decoded / 'pixels'
        extracted = supervisor.execute([fixture['pdfimages'], '-f', '1', '-l', '8', pdf, prefix],
                                       fixture['project'], fixture['env'])
        paths = []
        for fields in images:
            page = int(fields[0]) - 1
            kind = fields[2]
            # Poppler's default PPM export expands a gray soft mask to RGB.
            # The independent -list control above still requires a gray source.
            path = decoded / f'pixels-{int(fields[1]):03d}.ppm'
            pixel = bytes((page * 7 % 256, page * 13 % 256, 90)) if kind == 'image' else bytes((128, 128, 128))
            check_uniform_pnm(path, pixel)
            paths.append(path)
        if set(decoded.iterdir()) != set(paths):
            raise BenchmarkError('Image verifier emitted unexpected output files')
        for path in paths:
            path.unlink()
        return {'pages': 8, 'image_objects': len(images), 'dimensions': [1024, 1024],
                'rgb_by_page': [[page * 7 % 256, page * 13 % 256, 90] for page in range(8)],
                'alpha': 128, 'every_decoded_pixel_verified': True,
                'verifier_command_ids': [info['command_id'], image_list['command_id'], extracted['command_id']]}
    result = supervisor.execute([fixture['pdftotext'], pdf, '-'], fixture['project'], fixture['env'])
    expected = 'Ordinary nested lookup.' if case == 'nested-lookup' else 'Probe.'
    text = ' '.join(result['stdout'].split())
    if expected not in text:
        raise BenchmarkError('Fixture PDF does not contain independently extracted expected text')
    return {'expected_text': expected, 'text_verified': True, 'verifier_command_id': result['command_id']}


def execute_fixture(supervisor, fixture, case, initializing=False, validate_output=True):
    pdf = fixture['out'] / 'main.pdf'
    if case != 'warm-build-cache':
        pdf.unlink(missing_ok=True)
    result = supervisor.execute(fixture['command'], fixture['project'], fixture['env'])
    if case == 'warm-build-cache':
        try:
            report = json.loads(result['stdout'])
        except ValueError as error:
            raise BenchmarkError('Build-cache fixture returned invalid JSON') from error
        expected_runs = 1 if initializing else 0
        if not isinstance(report, dict) or report.get('skipped') is not (not initializing) \
                or type(report.get('tex_runs')) is not int or report['tex_runs'] != expected_runs:
            raise BenchmarkError('Build-cache fixture did not take the expected compile/cache-hit path')
    check_pdf(pdf)
    if validate_output:
        result['output_validation'] = check_fixture_output(supervisor, fixture, case)
    return result


def batch(supervisor, fixture, case, iterations):
    observations = []
    for index in range(iterations):
        result = execute_fixture(supervisor, fixture, case, validate_output=index == iterations - 1)
        observations.append({'command_id': result['command_id'], 'seconds': result['seconds']})
    return {'seconds': sum(item['seconds'] for item in observations),
            'iterations': iterations, 'commands': observations,
            'output_validation': result['output_validation']}


def load_metadata(path):
    if path.stat().st_size > MAX_CAPTURE_BYTES:
        raise ValueError('Metadata exceeds the finite input limit')
    metadata = json.loads(path.read_text(encoding='utf-8'))
    if not isinstance(metadata, dict):
        raise ValueError('Metadata must be a JSON object')
    for field in ('runner_label', 'toolchain'):
        if not isinstance(metadata.get(field), str) or not metadata[field].strip():
            raise ValueError('Metadata requires a nonempty ' + field + ' string')
    for label in LABELS:
        side = metadata.get(label)
        if not isinstance(side, dict) or any(not isinstance(side.get(field), str) or not side[field].strip()
                                             for field in ('revision', 'build_command')):
            raise ValueError('Metadata requires ' + label + ' revision and build_command strings')
    return metadata


def persist(report, path):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(report, indent=2, allow_nan=False) + '\n', encoding='utf-8')
    lines = ['# Paired runtime comparison', '',
             'Mode ' + ('explicit statistical gate.' if report['gate'] else 'advisory timing evidence.'),
             '', 'Outcome `' + report['status'] + '`.', '',
             '| Case | Outcome | Candidate / baseline | Median interval |',
             '| --- | --- | ---: | --- |']
    for case in report['results']:
        analysis = case.get('analysis', {})
        interval = analysis.get('median_ratio_interval')
        bounds = f"{interval['lower']:.4f} to {interval['upper']:.4f}" if interval else 'unavailable'
        ratio = f"{analysis['median_ratio']:.4f}" if 'median_ratio' in analysis else 'unavailable'
        lines.append(f"| {case['case']} | {analysis.get('status', 'incomplete')} | {ratio} | {bounds} |")
    lines.extend(['', '## Decision policy', '', report['policy']['method'], '',
                  f"Practical slowdown threshold {report['policy']['relative_threshold']:.1%}.", '',
                  'A pass supports only these warm, generated workloads on this runner. '
                  'It does not establish universal speed, peak memory, asymptotic complexity or fidelity.', '',
                  'Intervals assume independent balanced blocks with a stable population median. '
                  'Shared-runner drift and scheduling correlations can violate that assumption. '
                  'Noise checks detect some violations, not all of them.', ''])
    for case in report['results']:
        lines.extend(f"- {case['case']}, {reason}" for reason in case.get('analysis', {}).get('reasons', []))
    lines.extend('- ' + error for error in report['errors'])
    path.with_suffix('.md').write_text('\n'.join(lines) + '\n', encoding='utf-8')


def parser():
    result = argparse.ArgumentParser(description=__doc__)
    result.add_argument('--baseline', type=Path, required=True)
    result.add_argument('--candidate', type=Path, required=True)
    result.add_argument('--metadata', type=Path, required=True,
                        help='JSON with runner_label, toolchain and per-side revision/build_command')
    result.add_argument('--output', type=Path, default=REPO / 'target/runtime-performance/report.json')
    result.add_argument('--pairs', type=int, default=40, help='Even paired batch count, 16 to 60')
    result.add_argument('--warmups', type=int, default=2, help='Per-side per-case calibration warmups, 2 to 10')
    result.add_argument('--timeout', type=float, default=30, help='Per-command deadline, (0, 60] seconds')
    result.add_argument('--budget', type=float, default=900, help='Whole-run deadline, (0, 1800] seconds')
    result.add_argument('--threshold', type=float, default=0.10, help='Predeclared relative practical slowdown, (0, 1]')
    result.add_argument('--min-sample-seconds', type=float, default=1.0, help='Minimum aggregate batch duration, [0.05, 2] seconds')
    result.add_argument('--warm-cache-min-sample-seconds', type=float, default=1.0,
                        help='Longer minimum for startup-sensitive cache-hit batches, [0.25, 4] seconds')
    result.add_argument('--max-iterations', type=int, default=256, help='Matched batch iteration ceiling, 1 to 256')
    result.add_argument('--gate', action='store_true', help='Exit 0 pass, 1 demonstrated regression/error, 2 inconclusive')
    return result


def main(argv=None):
    argument_parser = parser()
    args = argument_parser.parse_args(argv)
    if os.name != 'posix' or platform.system() not in ('Linux', 'Darwin'):
        argument_parser.error('Requires macOS or Linux owned POSIX process groups')
    for name, upper in (('timeout', 60), ('budget', 1800), ('threshold', 1)):
        value = getattr(args, name)
        if not math.isfinite(value) or not 0 < value <= upper:
            argument_parser.error(f'--{name} must be finite and in (0, {upper}]')
    if not math.isfinite(args.min_sample_seconds) or not 0.05 <= args.min_sample_seconds <= 2:
        argument_parser.error('--min-sample-seconds must be finite and in [0.05, 2]')
    if not math.isfinite(args.warm_cache_min_sample_seconds) or not 0.25 <= args.warm_cache_min_sample_seconds <= 4:
        argument_parser.error('--warm-cache-min-sample-seconds must be finite and in [0.25, 4]')
    if not 16 <= args.pairs <= 60 or args.pairs % 2 or not 2 <= args.warmups <= 10 \
            or not 1 <= args.max_iterations <= 256:
        argument_parser.error('Requires even --pairs 16..60, --warmups 2..10 and --max-iterations 1..256')
    args.output = args.output.resolve()
    if args.output.suffix.lower() != '.json':
        argument_parser.error('--output requires a .json extension to keep distinct JSON and Markdown reports')
    try:
        metadata = load_metadata(args.metadata.resolve())
    except (OSError, ValueError) as error:
        argument_parser.error(str(error))
    sources = {label: getattr(args, label).resolve() for label in LABELS}
    if any(not path.is_file() or not os.access(path, os.X_OK) for path in sources.values()):
        argument_parser.error('Both selected binary paths must be executable regular files')
    for output_path in (args.output, args.output.with_suffix('.md')):
        for input_path in (*sources.values(), args.metadata.resolve()):
            if output_path == input_path or (output_path.exists() and output_path.samefile(input_path)):
                argument_parser.error('Output paths must not overwrite selected inputs or aliases')
    report = {'schema_version': 2, 'gate': args.gate, 'status': 'incomplete', 'metadata': metadata,
              'machine': {'system': platform.system(), 'release': platform.release(),
                          'architecture': platform.machine(), 'processor': platform.processor(),
                          'logical_cpu_count': os.cpu_count(), 'python': platform.python_version()},
              'policy': {'relative_threshold': args.threshold, 'familywise_confidence': 1 - FAMILY_ALPHA,
                         'pairs_per_case': args.pairs, 'warmups_per_side': args.warmups,
                         'min_sample_seconds': args.min_sample_seconds, 'max_iterations': args.max_iterations,
                         'warm_cache_min_sample_seconds': args.warm_cache_min_sample_seconds,
                         'calibration_headroom': CALIBRATION_HEADROOM,
                         'per_command_deadline_seconds': args.timeout, 'whole_run_deadline_seconds': args.budget,
                         'noise_relative_mad_limit': NOISE_LIMIT, 'order_bias_limit': ORDER_BIAS_LIMIT,
                         'method': 'Adjacent opposite-order pairs form geometric-mean ratio blocks. '
                         'Exact binomial sign/order-statistic intervals bound their population median, '
                         'with Bonferroni correction across the three predeclared cases. '
                         'Fail requires the entire interval above the practical boundary and no noise flags. '
                         'Pass requires the entire interval at or below it and no noise flags. '
                         'Otherwise the comparison is inconclusive.'},
              'executables': [], 'results': [], 'errors': []}
    previous_handler = signal.getsignal(signal.SIGTERM)

    def terminate(_number, _frame):
        raise KeyboardInterrupt

    signal.signal(signal.SIGTERM, terminate)
    started = time.monotonic()
    try:
        with tempfile.TemporaryDirectory(prefix='tekai-paired-benchmark-') as temporary:
            work = Path(temporary).resolve()
            supervisor = Supervisor(work, args.timeout, started + args.budget)
            binaries = {}
            try:
                verifiers = {name: shutil.which(name) for name in ('pdftotext', 'pdfinfo', 'pdfimages')}
                if not all(verifiers.values()):
                    raise BenchmarkError('Poppler pdftotext, pdfinfo and pdfimages are required for independent fixture verification')
                report['output_verifiers'] = {name: {'path': path, 'sha256_start': executable_sha256(Path(path))}
                                              for name, path in verifiers.items()}
                for label, source in sources.items():
                    supervisor.remaining()
                    dest = work / label / 'bin' / 'tekai'
                    dest.parent.mkdir(parents=True)
                    row = {'label': label, 'source_path': str(source), 'isolated_path': str(dest),
                           'source_sha256_start': executable_sha256(source)}
                    report['executables'].append(row)
                    shutil.copy2(source, dest)
                    row['sha256_start'] = executable_sha256(dest)
                    if row['sha256_start'] != row['source_sha256_start']:
                        raise BenchmarkError('Selected binary changed during its isolated copy')
                    binaries[label] = dest
                persist(report, args.output)
                for case_index, case in enumerate(CASES):
                    supervisor.remaining()
                    fixtures = {label: make_fixture(work / label / case, binaries[label], case) for label in LABELS}
                    for fixture in fixtures.values():
                        fixture.update(verifiers)
                    row = {'case': case, 'fixture_sha256': {label: fixtures[label]['input_sha256'] for label in LABELS},
                           'fixture_roots': {label: str(fixtures[label]['root']) for label in LABELS},
                           'min_sample_seconds': args.warm_cache_min_sample_seconds if case == 'warm-build-cache'
                           else args.min_sample_seconds,
                           'warmups': [], 'samples': []}
                    report['results'].append(row)
                    if len(set(row['fixture_sha256'].values())) != 1:
                        raise BenchmarkError('Per-binary source fixtures do not have equal content')
                    row['initialization'] = {}
                    for label in paired_order(0, case_index):
                        initial = execute_fixture(supervisor, fixtures[label], case, initializing=True)
                        row['initialization'][label] = {'command_id': initial['command_id'], 'seconds': initial['seconds'],
                                                       'output_validation': initial['output_validation']}
                    for warmup_index in range(args.warmups):
                        warmup = {'order': paired_order(warmup_index, case_index)}
                        for label in warmup['order']:
                            warmup[label] = batch(supervisor, fixtures[label], case, 1)
                        row['warmups'].append(warmup)
                    try:
                        iterations = calibrated_iterations(row['warmups'], row['min_sample_seconds'], args.max_iterations)
                    except CalibrationInfeasible as error:
                        row['calibration_infeasible'] = error.details
                        row['analysis'] = {'status': 'inconclusive', 'reasons': [str(error)]}
                    else:
                        row['iterations_per_batch'] = iterations
                        persist(report, args.output)
                        for pair_index in range(args.pairs):
                            sample = {'pair_index': pair_index, 'order': paired_order(pair_index, case_index),
                                      'iterations': iterations}
                            # Keep partial pairs so a later failure cannot erase the
                            # successful side's observation from the final artifact.
                            row['samples'].append(sample)
                            for label in sample['order']:
                                sample[label] = batch(supervisor, fixtures[label], case, iterations)
                            sample['ratio'] = sample['candidate']['seconds'] / sample['baseline']['seconds']
                            persist(report, args.output)
                        row['analysis'] = analyze_pairs(row['samples'], args.threshold, row['min_sample_seconds'])
                    row['fixture_sha256_end'] = {label: fixture_sha256(fixtures[label]['project']) for label in LABELS}
                    row['fixture_unchanged'] = row['fixture_sha256'] == row['fixture_sha256_end']
                    if not row['fixture_unchanged']:
                        raise BenchmarkError('Compiler changed or removed benchmark fixture inputs')
                    print(case + ' ' + row['analysis']['status'], flush=True)
                    persist(report, args.output)
            except (OSError, ValueError, RuntimeError, subprocess.SubprocessError) as error:
                report['errors'].append(f'{type(error).__name__}: {error}')
            except KeyboardInterrupt:
                report['errors'].append('Run interrupted; terminating owned process groups')
            finally:
                try:
                    supervisor.close()
                except (OSError, RuntimeError, subprocess.SubprocessError) as error:
                    report['errors'].append('Cleanup failed: ' + str(error))
                for row in report['executables']:
                    for path_key, hash_key in (('source_path', 'source_sha256_end'), ('isolated_path', 'sha256_end')):
                        try:
                            row[hash_key] = executable_sha256(Path(row[path_key]))
                        except OSError:
                            row[hash_key] = None
                    row['unchanged'] = row.get('sha256_start') == row['sha256_end'] \
                        and row['source_sha256_start'] == row['source_sha256_end']
                    if not row['unchanged']:
                        report['errors'].append('Executable content changed or disappeared: ' + row['label'])
                for name, verifier in report.get('output_verifiers', {}).items():
                    try:
                        verifier['sha256_end'] = executable_sha256(Path(verifier['path']))
                    except OSError:
                        verifier['sha256_end'] = None
                    verifier['unchanged'] = verifier['sha256_start'] == verifier['sha256_end']
                    if not verifier['unchanged']:
                        report['errors'].append('Output verifier changed or disappeared: ' + name)
    finally:
        signal.signal(signal.SIGTERM, previous_handler)
    if report['errors']:
        report['status'] = 'error'
    else:
        outcomes = [row['analysis']['status'] for row in report['results']]
        report['status'] = 'fail' if 'fail' in outcomes else ('inconclusive' if 'inconclusive' in outcomes else 'pass')
    report['elapsed_seconds'] = time.monotonic() - started
    report['exit_code'] = comparison_exit(report['status'], args.gate)
    persist(report, args.output)
    print('Comparison ' + report['status'] + '; report ' + str(args.output), flush=True)
    return report['exit_code']


if __name__ == '__main__':
    sys.exit(main())
