#!/usr/bin/env python3
"""Bounded same-runner paired comparison, using generated ordinary inputs.

Each artifact crosses two neutral slots with independent binaries and caches.
Timings are advisory unless --gate is selected. A gate can be inconclusive.
"""

import argparse
import ctypes
import gc
from fractions import Fraction
import hashlib
import json
import math
import os
from pathlib import Path
import platform
import queue
import re
import resource
import shutil
import signal
import statistics
import subprocess
import sys
import tempfile
import threading
import time

from performance_ci import PerformanceCI
from performance_build import VERSION
from benchmark_sampling import (DEFAULT_PAIRS, MAX_PAIRS, DEFAULT_BUDGET, MAX_BUDGET,
                                SCHEMA_VERSION, MAX_REPORT_BYTES, sign_interval_plan, prospective_power_study)
from runtime_fixtures import dependency_document, document, pad, png  # Keep existing helper imports usable.

REPO = Path(__file__).resolve().parent.parent
CASES = ('cli-startup', 'nested-lookup', 'image-compile', 'warm-build-cache')
LABELS = ('baseline', 'candidate')
SLOTS = ('s0', 's1')
REPLICAS = ('r0', 'r1')
PAIRS_PER_UNIT = 4
MAX_CAPTURE_BYTES = 1024 * 1024
MAX_STATE_BYTES = 8 * 1024 * 1024
MAX_JOURNAL_RECORD_BYTES = 8 * 1024 * 1024
CACHE_DEPENDENCY_COUNT = 1024
EMPTY_CONFIG_BYTES = b'# Isolated benchmark configuration. No inherited options.\n'
EMPTY_CONFIG_SHA256 = hashlib.sha256(EMPTY_CONFIG_BYTES).hexdigest()
CALIBRATION_HEADROOM = 2.0
CALIBRATION_PAIRS = 2
CALIBRATION_ITERATIONS = 32
MAX_ITERATIONS = 512
NOISE_LIMIT = 0.10
ORDER_BIAS_LIMIT = 0.05
FAMILY_ALPHA = 0.05
TIMING_METADATA = {
    'clock': 'time.monotonic', 'unit': 'seconds',
    'parent_timing_fields': ['start', 'launch_end', 'completion'],
    'command_timing_fields': ['start', 'capture_start', 'capture_files_open', 'popen_start', 'popen_return',
                             'capture_files_closed', 'launch_end', 'waiter_dispatch', 'waiter_accepted',
                             'completion', 'cleanup_start', 'cleanup_end'],
    'command_scope': 'Parent capture setup through blocking-wait completion, including launch and completion scheduling.',
    'launch_scope': 'Parent capture setup and Popen return, not isolated loader time.',
    'cleanup_scope': 'Owned-group kill/reap, completed-wait confirmation, bounded capture reads and capture-file removal.',
    'unit_scope': 'Unit boundaries also include between-command cleanup and output verification, outside scored command sums.',
    'checkpoint_scope': 'Atomic checkpoint writes occur between units, outside command and unit timestamps.',
    'waiter_scope': 'One owned persistent worker blocks in process.wait. Acceptance and completion include worker scheduling.',
    'used_for_gate': False,
}
SEARCH_ENV_VARS = frozenset(('WEB2C', 'INDEXSTYLE', 'BSTINPUTS', 'TFMFONTS', 'AFMFONTS',
                            'ENCFONTS', 'SFDFONTS', 'PKFONTS', 'GFFONTS', 'VFFONTS',
                            'T1FONTS', 'TTFONTS', 'OPENTYPEFONTS'))
ISOLATED_ENVIRONMENT_KEYS = ('HOME', 'USERPROFILE', 'TMPDIR', 'TMP', 'TEMP', 'XDG_CACHE_HOME',
                             'XDG_CONFIG_HOME', 'XDG_DATA_HOME', 'APPDATA', 'LOCALAPPDATA', 'PATH',
                             'LC_ALL', 'LANG', 'TZ', 'TEKAI_TEXMF_MODE', 'TEXINPUTS')


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


def replica_for(label, slot):
    """Each artifact occupies each equal-length replica position once."""
    return REPLICAS[(LABELS.index(label) + SLOTS.index(slot)) % 2]


def slot_pair(slot, pair_index, case_index=0):
    if slot not in SLOTS or type(pair_index) is not int or pair_index < 0 \
            or type(case_index) is not int or not 0 <= case_index < len(CASES):
        raise ValueError('Slot schedule requires valid slots and exact nonnegative integer indices')
    return {'slot': slot, 'order': paired_order(pair_index, SLOTS.index(slot) + case_index),
            'replicas': {label: replica_for(label, slot) for label in LABELS}}


def crossover_pair(pair_index, case_index=0):
    """Complementary ABBA/BAAB quads form one eight-batch inference unit."""
    if type(pair_index) is not int or pair_index < 0:
        raise ValueError('Crossover pair index must be an exact nonnegative integer')
    within = pair_index % PAIRS_PER_UNIT
    unit = pair_index // PAIRS_PER_UNIT
    quad = within // 2
    slot = SLOTS[within % 2]
    return {'pair_index': pair_index, 'unit_index': unit, 'quad_index': quad,
            **slot_pair(slot, quad + unit, case_index)}


def cell_order(case_index=0):
    """Balanced creation/initialization order for all four owned cells."""
    return [{'slot': row['slot'], 'label': label, 'replica': row['replicas'][label]}
            for row in (crossover_pair(index, case_index) for index in range(2)) for label in row['order']]


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
    plan = sign_interval_plan(len(ordered), alpha)
    if plan is None:
        return None
    k = plan['order_statistic']
    return {'lower': ordered[k - 1], 'upper': ordered[len(ordered) - k], **plan}


def relative_mad(values):
    center = statistics.median(values)
    if not math.isfinite(center) or center <= 0:
        raise ValueError('Noise calculation needs a finite positive median')
    result = statistics.median(abs(value - center) for value in values) / center
    if not math.isfinite(result) or result < 0:
        raise ValueError('Noise calculation produced an unusable ratio')
    return result


def calibration_details(pilots, min_seconds, ceiling):
    """Normalize complete fixed pilots without consuming inference or telemetry."""
    def positive_seconds(value):
        if type(value) not in (int, float):
            raise ValueError('Calibration durations must be finite positive numbers')
        try:
            value = float(value)
        except OverflowError as error:
            raise ValueError('Calibration durations must be finite positive numbers') from error
        if not math.isfinite(value) or value <= 0:
            raise ValueError('Calibration durations must be finite positive numbers')
        return value

    min_seconds = positive_seconds(min_seconds)
    if type(ceiling) is not int or not 1 <= ceiling <= MAX_ITERATIONS:
        raise ValueError(f'Calibration ceiling must be an integer from 1 to {MAX_ITERATIONS}')
    if not isinstance(pilots, (list, tuple)) or len(pilots) != CALIBRATION_PAIRS * len(SLOTS):
        raise ValueError('Calibration requires exactly two complete balanced pilot pairs per slot')
    rates = {slot: {label: [] for label in LABELS} for slot in SLOTS}
    for index, pilot in enumerate(pilots):
        if not isinstance(pilot, dict) or not {'pair_index', 'slot', 'replicas', 'order', 'iterations', *LABELS} <= pilot.keys() \
                or not isinstance(pilot['order'], (list, tuple)) \
                or any(type(label) is not str for label in pilot['order']):
            raise ValueError('Calibration requires complete pilot rows and raw role batches')
        order = tuple(pilot['order'])
        slot = SLOTS[index % len(SLOTS)]
        if type(pilot['pair_index']) is not int or pilot['pair_index'] != index // len(SLOTS) \
                or pilot['slot'] != slot \
                or pilot['replicas'] != {label: replica_for(label, slot) for label in LABELS} \
                or len(order) != 2 or set(order) != set(LABELS) \
                or (index % 2 and order != tuple(reversed(pilots[index - 1]['order']))) \
                or (index >= len(SLOTS) and order != tuple(reversed(pilots[index - len(SLOTS)]['order']))):
            raise ValueError('Calibration pilot indices and role orders must be balanced')
        if type(pilot['iterations']) is not int or pilot['iterations'] != CALIBRATION_ITERATIONS:
            raise ValueError('Calibration pilots require exactly 32 iterations per side')
        for label in LABELS:
            observed = pilot[label]
            if not isinstance(observed, dict) or not {'iterations', 'seconds'} <= observed.keys():
                raise ValueError('Calibration requires complete raw role batches')
            iterations = observed['iterations']
            if type(iterations) is not int or iterations != pilot['iterations']:
                raise ValueError('Calibration batch iteration counts must match the declared pilot count')
            rate = positive_seconds(positive_seconds(observed['seconds']) / iterations)
            rates[slot][label].append(rate)
    medians = {slot: {label: statistics.median(rates[slot][label]) for label in LABELS} for slot in SLOTS}
    fastest = min(medians[slot][label] for slot in SLOTS for label in LABELS)
    # Exact ratios of the finite observed floats avoid overflowing count
    # arithmetic for an exceptionally small positive calibration duration.
    relative_duration = Fraction(min_seconds) / Fraction(fastest)
    minimum = math.ceil(relative_duration)
    requested = math.ceil(relative_duration * Fraction(CALIBRATION_HEADROOM))
    selected = min(ceiling, requested)
    predictions = (fastest * selected, fastest * ceiling)
    if any(not math.isfinite(value) for value in predictions):
        raise ValueError('Calibration predicts nonfinite batch durations')
    return {'normalized_rates_seconds_per_iteration': rates,
            'pilot_medians_seconds_per_iteration': medians,
            'fastest_pilot_median_seconds_per_iteration': fastest,
            'minimum_batch_seconds': min_seconds, 'minimum_iterations': minimum,
            'requested_iterations': requested, 'selected_iterations': selected,
            'iteration_ceiling': ceiling, 'headroom_clipped': requested > ceiling,
            'minimum_feasible': minimum <= ceiling, 'calibration_headroom': CALIBRATION_HEADROOM,
            'predicted_selected_batch_seconds': predictions[0],
            'predicted_ceiling_batch_seconds': predictions[1]}


def calibrated_iterations(pilots, min_seconds, ceiling):
    details = calibration_details(pilots, min_seconds, ceiling)
    if not details['minimum_feasible']:
        raise CalibrationInfeasible(details)
    return details['selected_iterations']


def analyze_pairs(samples, threshold, min_seconds, case_count=len(CASES), case_index=0):
    """Keep insufficient precision and unstable measurements inconclusive."""
    if not math.isfinite(threshold) or not 0 < threshold <= 1:
        raise ValueError('Practical threshold must be finite and in (0, 1]')
    if not math.isfinite(min_seconds) or min_seconds <= 0 or type(case_count) is not int or case_count < 1:
        raise ValueError('Duration and comparison count must be positive')
    if len(samples) % PAIRS_PER_UNIT or not samples:
        raise ValueError('Paired samples must contain complete eight-batch crossover units')
    values = {label: [] for label in LABELS}
    ratios, orders = [], {label: [] for label in LABELS}
    for index, sample in enumerate(samples):
        expected = crossover_pair(index, case_index)
        if not isinstance(sample, dict) or any(sample.get(key) != value for key, value in expected.items() if key != 'order') \
                or any(type(sample.get(key)) is not int for key in ('pair_index', 'unit_index', 'quad_index')):
            raise ValueError('Sample slot, replica and crossover indices differ from the declared schedule')
        if not isinstance(sample.get('order'), (list, tuple)):
            raise ValueError('Sample requires its declared execution order')
        order = tuple(sample['order'])
        if order != expected['order']:
            raise ValueError('Sample execution order differs from the declared crossover schedule')
        iterations = sample.get('iterations')
        if type(iterations) is not int or not 1 <= iterations <= MAX_ITERATIONS \
                or iterations != samples[0].get('iterations'):
            raise ValueError('Formal samples require one fixed matched iteration count')
        for label in LABELS:
            observed = sample.get(label)
            if not isinstance(observed, dict) or type(observed.get('iterations')) is not int \
                    or observed['iterations'] != iterations:
                raise ValueError('Sample requires both complete matched artifact batches')
            elapsed = observed.get('seconds')
            try:
                valid = type(elapsed) in (int, float) and math.isfinite(elapsed) and elapsed > 0
            except OverflowError:
                valid = False
            if not valid:
                raise ValueError('Elapsed times must be finite and positive')
            values[label].append(elapsed)
        try:
            ratio = values['candidate'][-1] / values['baseline'][-1]
        except OverflowError as error:
            raise ValueError('Paired ratio overflowed; statistics are unusable') from error
        if not math.isfinite(ratio) or ratio <= 0:
            raise ValueError('Paired ratio overflowed or underflowed; statistics are unusable')
        ratios.append(ratio)
        orders[order[0]].append(ratio)
    # Combine complementary quads before inference. Four logical pairs are
    # one unit, not four independent observations or two independent quads.
    try:
        blocks = [math.exp(math.fsum(math.log(samples[index]['candidate']['seconds'])
                                    - math.log(samples[index]['baseline']['seconds'])
                                    for index in range(start, start + PAIRS_PER_UNIT)) / PAIRS_PER_UNIT)
                  for start in range(0, len(samples), PAIRS_PER_UNIT)]
    except OverflowError as error:
        raise ValueError('Crossover ratio overflowed; statistics are unusable') from error
    if any(not math.isfinite(value) or value <= 0 for value in blocks):
        raise ValueError('Crossover ratio overflowed or underflowed; statistics are unusable')
    median_ratio = statistics.median(blocks)
    if not math.isfinite(median_ratio) or median_ratio <= 0:
        raise ValueError('Crossover median ratio is not finite and positive')
    interval = median_interval(blocks, FAMILY_ALPHA / case_count)
    noise = {label: relative_mad(values[label]) for label in LABELS}
    order_medians = {label: statistics.median(orders[label]) for label in LABELS}
    if any(not math.isfinite(value) or value <= 0 for value in order_medians.values()):
        raise ValueError('Order medians are not finite and positive')
    order_bias = max(order_medians.values()) / min(order_medians.values()) - 1
    if not math.isfinite(order_bias):
        raise ValueError('Order-bias ratio overflowed; statistics are unusable')
    issues = []
    if any(min(items) < min_seconds for items in values.values()):
        issues.append('At least one batch is shorter than the predeclared minimum duration')
    if any(value > NOISE_LIMIT for value in noise.values()):
        issues.append('Per-binary relative median absolute deviation exceeds the noise limit')
    if order_bias > ORDER_BIAS_LIMIT:
        issues.append('Baseline-first and candidate-first ratios differ beyond the order-bias limit')
    if interval is None:
        issues.append('Too few complete crossover units for a finite multiplicity-adjusted median interval')
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
    return {'status': status, 'median_ratio': median_ratio,
            'paired_ratios': ratios, 'inference_unit_ratios': blocks, 'inference_unit_count': len(blocks),
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
        self._wait_jobs = queue.Queue(maxsize=1)
        self._waiter = None
        self._execute_lock = threading.Lock()
        self._closed = False

    def _wait(self):
        while True:
            job = self._wait_jobs.get()
            if job is None:
                return
            process, completed, observation = job
            try:
                observation['accepted'] = time.monotonic()
                observation['code'] = process.wait()
                observation['completion'] = time.monotonic()
                observation['seconds'] = observation['completion'] - process.started_at
            except Exception as error:
                observation['error'] = f'{type(error).__name__}: {error}'[:1024]
            finally:
                # Do not retain the previous child or captures while idle.
                del job, process, observation
                completed.set()
                del completed

    def _ensure_waiter(self):
        if self._closed:
            raise BenchmarkError('Supervisor is closed')
        if self._waiter is None:
            self._waiter = threading.Thread(target=self._wait, name='tekai-benchmark-waiter', daemon=True)
            self._waiter.start()
        elif not self._waiter.is_alive():
            raise BenchmarkError('Persistent process waiter stopped unexpectedly')

    def close(self):
        try:
            super().close()
        finally:
            self._closed = True
            if self._waiter is not None and self._waiter.is_alive():
                try:
                    self._wait_jobs.put(None, timeout=5)
                except queue.Full as error:
                    raise BenchmarkError('Persistent process waiter did not accept shutdown') from error
                self._waiter.join(timeout=5)
                if self._waiter.is_alive():
                    raise BenchmarkError('Persistent process waiter did not finish during cleanup')

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
        if not self._execute_lock.acquire(blocking=False):
            raise BenchmarkError('Supervisor accepts only one command at a time')
        try:
            return self._execute(command, cwd, env)
        finally:
            self._execute_lock.release()

    def _execute(self, command, cwd, env):
        self._ensure_waiter()
        allowance = min(self.timeout, self.remaining())
        started = time.monotonic()
        process = self.start(command, cwd, env)
        launch_end = time.monotonic()
        process.observe = False
        process.started_at = started
        completed, observation = threading.Event(), {}

        dispatch = time.monotonic()
        self._wait_jobs.put_nowait((process, completed, observation))
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
            cleanup_start = time.monotonic()
            try:
                self.stop(process)
            finally:
                finished = completed.wait(timeout=5)
                cleanup_end = time.monotonic()
        if not finished:
            raise BenchmarkError('Process waiter did not finish after owned-group cleanup')
        if 'error' in observation:
            raise BenchmarkError('Process waiter failed: ' + observation['error'])
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
        cleanup_end = time.monotonic()
        return {'command_id': process.command_id, 'seconds': observation['seconds'],
                'parent_timing': [started, launch_end, observation['completion']],
                'command_timing': [started, *process.launch_timing, launch_end, dispatch,
                                   observation['accepted'], observation['completion'], cleanup_start, cleanup_end],
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
    env.update(LC_ALL='C', LANG='C', TZ='UTC')
    for kind in ('ENGINE', 'FORMAT', 'AUX', 'BIBTEX'):
        path = root / ('cache-' + kind.lower())
        path.mkdir()
        env['TEKAI_' + kind + '_CACHE'] = str(path)
    return env


def make_fixture(root, binary, case):
    project, out = root / 'project', root / 'out'
    document(project)
    config = project / 'tekai.toml'
    config.write_bytes(EMPTY_CONFIG_BYTES)
    out.mkdir()
    env = isolated_environment(root)
    dependencies = []
    if case == 'warm-build-cache':
        dependencies = dependency_document(project, CACHE_DEPENDENCY_COUNT)
    if case == 'nested-lookup':
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
    command = [binary, 'build', project / 'main.tex', '--config', config,
               '--out-dir', out, '--once', '--quiet', '--report-json'] if case == 'warm-build-cache' else \
        ([binary, '--version'] if case == 'cli-startup' else
         [binary, '__tekai-engine', '-interaction=nonstopmode', '-halt-on-error', '-no-shell-escape',
          f'-output-directory={out}', 'main.tex'])
    return {'root': root, 'project': project, 'out': out, 'env': env,
            'command': command, 'config': config, 'dependencies': dependencies,
            'expected_dependency_paths': frozenset(str(path.resolve()) for path in dependencies),
            'expected_main_path': str((project / 'main.tex').resolve()),
            'input_sha256': fixture_sha256(project)}


def cache_state_snapshot(fixture, establish=False):
    """Bounded state oracle at batch boundaries, outside every command timer."""
    path = fixture['out'] / '.tekai-main.state.toml'
    metadata = path.stat()
    if not 0 < metadata.st_size <= MAX_STATE_BYTES:
        raise BenchmarkError('Build-cache state exceeds its finite size limit or is empty')
    with path.open('rb') as handle:
        data = handle.read(MAX_STATE_BYTES + 1)
    if len(data) != metadata.st_size or len(data) > MAX_STATE_BYTES:
        raise BenchmarkError('Build-cache state changed while the bounded oracle read it')
    try:
        source = data.decode('utf-8')
        blocks = re.findall(r'(?ms)^\[\[inputs\]\]\n(.*?)(?=^\[|\Z)', source)
        paths = []
        for block in blocks:
            fields = re.findall(r'(?m)^path = (.+)$', block)
            if len(fields) != 1:
                raise ValueError('Input table lacks one serialized path')
            value = json.loads(fields[0])
            if not isinstance(value, str):
                raise ValueError('Input path is not a string')
            paths.append(value)
    except (UnicodeError, ValueError) as error:
        raise BenchmarkError('Build-cache state has unsupported recorded-input serialization') from error
    if len(blocks) != len(re.findall(r'(?m)^\[\[inputs\]\]$', source)) or len(paths) != len(set(paths)):
        raise BenchmarkError('Build-cache state has malformed or duplicate recorded inputs')
    expected = fixture.get('expected_dependency_paths')
    if expected is None:
        expected = frozenset(str(path.resolve()) for path in fixture.get('dependencies', []))
    main_path = fixture.get('expected_main_path') or str((fixture['project'] / 'main.tex').resolve())
    if not expected <= set(paths) or main_path not in paths:
        raise BenchmarkError('Build-cache state omits genuinely referenced fixture inputs')
    row = {'sha256': hashlib.sha256(data).hexdigest(), 'bytes': len(data),
           'mtime_ns': metadata.st_mtime_ns, 'ctime_ns': metadata.st_ctime_ns,
           'device': metadata.st_dev, 'inode': metadata.st_ino,
           'recorded_input_count': len(paths), 'referenced_dependency_count': len(expected),
           'input_paths_sha256': hashlib.sha256(json.dumps(sorted(paths), ensure_ascii=False).encode('utf-8')).hexdigest(),
           'all_referenced_inputs_verified': True, 'used_for_gate': False}
    if establish:
        fixture['expected_cache_state'] = row
    elif row != fixture.get('expected_cache_state'):
        raise BenchmarkError('Build-cache state mutated during the verified cache-hit workload')
    return row


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


def reported_build_timing(report, parent_seconds):
    """Keep optional binary-reported timing separate from measured evidence."""
    diagnostic = {'untrusted': True, 'status': 'absent'}
    if 'elapsed_ms' not in report:
        return diagnostic
    value = report['elapsed_ms']
    if type(value) not in (int, float):
        diagnostic['status'] = 'invalid'
        return diagnostic
    try:
        milliseconds = float(value)
    except OverflowError:
        diagnostic['status'] = 'invalid'
        return diagnostic
    if not math.isfinite(milliseconds) or milliseconds < 0:
        diagnostic['status'] = 'invalid'
        return diagnostic
    diagnostic.update(status='reported', reported_elapsed_ms=milliseconds,
                      parent_minus_reported_seconds=parent_seconds - milliseconds / 1000)
    return diagnostic


def execute_fixture(supervisor, fixture, case, initializing=False, validate_output=True):
    if case == 'cli-startup':
        result = supervisor.execute(fixture['command'], fixture['project'], fixture['env'])
        version = fixture.get('expected_version')
        if not isinstance(version, str) or len(version) > 128 or VERSION.fullmatch(version) is None:
            raise BenchmarkError('CLI startup needs an independently recorded package version')
        expected = f'tekai {version}\n'
        if result['stdout'] != expected or result['stderr_tail']:
            raise BenchmarkError('CLI startup output differs from the selected revision package version')
        result['output_validation'] = {'expected_stdout': expected, 'stdout_verified': True,
                                       'stderr_empty': True, 'expected_version': version}
        return result
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
        result['reported_build_timing'] = reported_build_timing(report, result['seconds'])
        if initializing:
            result['cache_state'] = cache_state_snapshot(fixture, establish=True)
    check_pdf(pdf)
    if validate_output:
        result['output_validation'] = check_fixture_output(supervisor, fixture, case)
    return result


def batch(supervisor, fixture, case, iterations):
    observations = []
    cache_before = cache_state_snapshot(fixture) if case == 'warm-build-cache' else None
    for index in range(iterations):
        result = execute_fixture(supervisor, fixture, case, validate_output=index == iterations - 1)
        observation = {'command_id': result['command_id'], 'seconds': result['seconds'],
                       'parent_timing': result['parent_timing'], 'command_timing': result['command_timing']}
        if 'reported_build_timing' in result:
            observation['reported_build_timing'] = result['reported_build_timing']
        observations.append(observation)
    row = {'seconds': sum(item['seconds'] for item in observations),
            'iterations': iterations, 'commands': observations,
            'output_validation': result['output_validation']}
    if cache_before is not None:
        row.update(cache_state_before=cache_before, cache_state_after=cache_state_snapshot(fixture),
                   cache_state_unchanged=True)
    return row


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
        if 'artifact_sha256' in side and (not isinstance(side['artifact_sha256'], str)
                                          or re.fullmatch(r'[0-9a-f]{64}', side['artifact_sha256']) is None):
            raise ValueError('Metadata requires ' + label + ' artifact_sha256 to be an exact lowercase SHA-256')
        if not isinstance(side.get('expected_version'), str) or len(side['expected_version']) > 128 \
                or VERSION.fullmatch(side['expected_version']) is None:
            raise ValueError('Metadata requires ' + label + ' expected_version from the selected revision package manifest')
    return metadata


class ReportTooLarge(BenchmarkError):
    pass


class CommandJournal:
    """Owned append-only raw evidence. Only one active unit remains in memory."""
    def __init__(self, output):
        output.parent.mkdir(parents=True, exist_ok=True)
        self.handle = tempfile.NamedTemporaryFile(mode='w+b', dir=output.parent,
                                                  prefix=output.stem + '.raw.', suffix='.jsonl', delete=False)
        self.path = Path(self.handle.name)
        self.digest = hashlib.sha256()
        self.bytes = 0
        self.records = 0
        self.batches = []
        self.observations = []

    def binding(self):
        return {'path': str(self.path), 'sha256': self.digest.hexdigest(), 'bytes': self.bytes,
                'records': self.records, 'format': 'tekai-command-journal-v1',
                'maximum_bytes': MAX_REPORT_BYTES, 'maximum_record_bytes': MAX_JOURNAL_RECORD_BYTES,
                'scope': 'All completed warmup, pilot and formal batch command records. Initialization remains in report.'}

    def append(self, case_index, phase, rows):
        started = time.monotonic()
        entries = [{'case_index': case_index, 'phase': phase, 'row_index': index, 'label': label,
                    'commands': batch['commands']} for index, label, batch in rows]
        data = (json.dumps({'sequence': self.records, 'batches': entries}, separators=(',', ':'),
                           allow_nan=False) + '\n').encode('utf-8')
        if len(data) > MAX_JOURNAL_RECORD_BYTES or self.bytes + len(data) > MAX_REPORT_BYTES:
            raise ReportTooLarge('Raw command journal exceeds its finite output limit; no samples dropped')
        # Keep active commands attached until the complete append has flushed.
        written = self.handle.write(data)
        self.handle.flush()
        ended = time.monotonic()
        if written != len(data):
            raise BenchmarkError('Raw command journal append was incomplete')
        self.digest.update(data)
        self.bytes += len(data)
        for batch_index, (_index, _label, batch) in enumerate(rows):
            batch['command_journal'] = {'record': self.records, 'batch': batch_index}
            batch['commands'] = []
            self.batches.append(batch)
        self.records += 1
        self.observations.append({'sequence': self.records - 1, 'case_index': case_index, 'phase': phase,
                                  'monotonic_start_seconds': started, 'monotonic_end_seconds': ended,
                                  'seconds': ended - started, 'record_bytes': len(data), 'used_for_gate': False,
                                  'scope': 'Command journal serialization, append and flush outside scored command/unit clocks.'})

    def restore(self):
        """Restore once after scored work. Do not trust a modified journal."""
        self.handle.flush()
        selected, opened = self.path.stat(), os.fstat(self.handle.fileno())
        if self.path.is_symlink() or (selected.st_dev, selected.st_ino) != (opened.st_dev, opened.st_ino):
            raise BenchmarkError('Raw command journal path changed or became an alias')
        self.handle.seek(0)
        digest, size, restored = hashlib.sha256(), 0, 0
        for sequence in range(self.records):
            line = self.handle.readline(MAX_JOURNAL_RECORD_BYTES + 1)
            if not line.endswith(b'\n') or len(line) > MAX_JOURNAL_RECORD_BYTES:
                raise BenchmarkError('Raw command journal contains an incomplete or oversized record')
            digest.update(line)
            size += len(line)
            record = json.loads(line)
            if record.get('sequence') != sequence or not isinstance(record.get('batches'), list):
                raise BenchmarkError('Raw command journal sequence changed')
            for batch_index, entry in enumerate(record['batches']):
                if restored >= len(self.batches):
                    raise BenchmarkError('Raw command journal gained unexpected batches')
                batch = self.batches[restored]
                if batch['command_journal'] != {'record': sequence, 'batch': batch_index} \
                        or not isinstance(entry.get('commands'), list) \
                        or len(entry['commands']) != batch['iterations']:
                    raise BenchmarkError('Raw command journal batch binding changed')
                batch['commands'] = entry['commands']
                restored += 1
        if self.handle.read(1) or size != self.bytes or digest.hexdigest() != self.digest.hexdigest() \
                or restored != len(self.batches):
            raise BenchmarkError('Raw command journal bytes or hash changed')

    def close(self):
        self.handle.close()


def atomic_text(path, value):
    """Replace only the selected report using an owned temporary sibling."""
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = None
    try:
        with tempfile.NamedTemporaryFile(mode='w', encoding='utf-8', dir=path.parent,
                                         prefix=path.name + '.', suffix='.tmp', delete=False) as handle:
            temporary = Path(handle.name)
            handle.write(value)
        os.replace(temporary, path)
    finally:
        if temporary is not None:
            temporary.unlink(missing_ok=True)


def persist(report, path):
    encoded = json.dumps(report, separators=(',', ':'), allow_nan=False) + '\n'
    size = len(encoded.encode('utf-8'))
    if size > MAX_REPORT_BYTES:
        # Never replace the last complete checkpoint with truncated evidence.
        # A separate bounded error manifest explains the failed final/checkpoint write.
        manifest = {'schema_version': SCHEMA_VERSION, 'status': 'error', 'exit_code': 1,
                    'errors': ['Report exceeds the finite output limit; prior checkpoint retained unchanged'],
                    'attempted_bytes': size, 'maximum_bytes': MAX_REPORT_BYTES,
                    'retained_checkpoint': str(path), 'samples_dropped': False}
        atomic_text(path.with_suffix('.overflow.json'), json.dumps(manifest, allow_nan=False) + '\n')
        raise ReportTooLarge('Report exceeds the finite output limit; prior checkpoint retained unchanged')
    atomic_text(path, encoded)
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
                  'Intervals assume independent complete crossover units with a stable population median. '
                  'Shared-runner drift and scheduling correlations can violate that assumption. '
                  'Noise checks detect some violations, not all of them.', '', report['policy']['assumptions'], ''])
    for case in report['results']:
        lines.extend(f"- {case['case']}, {reason}" for reason in case.get('analysis', {}).get('reasons', []))
    lines.extend('- ' + error for error in report['errors'])
    atomic_text(path.with_suffix('.md'), '\n'.join(lines) + '\n')


def host_snapshot(case, unit_index):
    """One bounded, diagnostic-only in-process observation outside command clocks."""
    row = {'case': case, 'unit_index': unit_index, 'monotonic_seconds': time.monotonic(),
           'used_for_gate': False, 'load_average': None,
           'load_average_status': 'unavailable', 'memory_status': 'unavailable',
           'swap_status': 'unavailable', 'cpu_utilization_status': 'unavailable'}
    try:
        values = list(os.getloadavg())
        if len(values) == 3 and all(type(value) in (int, float) and math.isfinite(value) and value >= 0 for value in values):
            row.update(load_average=values, load_average_status='reported')
    except (AttributeError, OSError, ValueError, OverflowError):
        pass
    return row


def parent_snapshot():
    """Cumulative parent resources, sampled outside command and unit clocks."""
    row = {'monotonic_seconds': time.monotonic(), 'used_for_gate': False,
           'source': 'resource.getrusage(RUSAGE_SELF), gc and threading',
           'scope': 'Benchmark parent only. CPU and peak RSS are cumulative, not child costs.',
           'cpu_status': 'unavailable', 'peak_rss_status': 'unavailable',
           'current_rss_status': 'unavailable',
           'current_rss_unavailable_reason': 'No current-RSS observation implemented on this platform',
           'gc_enabled': gc.isenabled(), 'gc_counts': list(gc.get_count()),
           'gc_thresholds': list(gc.get_threshold()), 'gc_generations': gc.get_stats(),
           'active_thread_count': threading.active_count()}
    try:
        usage = resource.getrusage(resource.RUSAGE_SELF)
        if all(math.isfinite(value) and value >= 0 for value in (usage.ru_utime, usage.ru_stime)):
            row.update(cpu_status='reported', user_cpu_seconds=usage.ru_utime, system_cpu_seconds=usage.ru_stime)
        unit = 'bytes' if platform.system() == 'Darwin' else 'KiB'
        divisor = 1048576 if unit == 'bytes' else 1024
        if math.isfinite(usage.ru_maxrss) and usage.ru_maxrss >= 0:
            row.update(peak_rss_status='reported', peak_rss_raw=usage.ru_maxrss,
                       peak_rss_raw_unit=unit, peak_rss_mib=usage.ru_maxrss / divisor)
    except (AttributeError, OSError, ValueError, OverflowError) as error:
        row['resource_unavailable_reason'] = f'{type(error).__name__}: {error}'[:1024]
    if platform.system() == 'Linux':
        try:
            with Path('/proc/self/statm').open('r', encoding='ascii') as handle:
                fields = handle.read(256).split()
            resident = int(fields[1]) * os.sysconf('SC_PAGE_SIZE')
            if resident >= 0:
                row.update(current_rss_status='reported', current_rss_bytes=resident,
                           current_rss_source='/proc/self/statm resident pages times SC_PAGE_SIZE')
                row.pop('current_rss_unavailable_reason')
        except (OSError, ValueError, IndexError, KeyError) as error:
            row['current_rss_unavailable_reason'] = f'{type(error).__name__}: {error}'[:1024]
    elif platform.system() == 'Darwin':
        try:
            observed = darwin_parent_memory()
            row.update(current_rss_status='reported', **observed)
            row.pop('current_rss_unavailable_reason')
        except (AttributeError, OSError, ValueError) as error:
            row['current_rss_unavailable_reason'] = f'{type(error).__name__}: {error}'[:1024]
    return row


class DarwinUsageV0(ctypes.Structure):
    # macOS sys/resource.h, rusage_info_v0. The V0 ABI is available since 10.9.
    _fields_ = [('ri_uuid', ctypes.c_uint8 * 16),
                *[(name, ctypes.c_uint64) for name in
                  ('ri_user_time', 'ri_system_time', 'ri_pkg_idle_wkups', 'ri_interrupt_wkups',
                   'ri_pageins', 'ri_wired_size', 'ri_resident_size', 'ri_phys_footprint',
                   'ri_proc_start_abstime', 'ri_proc_exit_abstime')]]


_darwin_rusage = None


def darwin_parent_memory():
    """Read the current parent without launching a profiling subprocess."""
    global _darwin_rusage
    if _darwin_rusage is None:
        library = ctypes.CDLL('/usr/lib/libproc.dylib', use_errno=True)
        function = library.proc_pid_rusage
        function.argtypes = [ctypes.c_int, ctypes.c_int, ctypes.c_void_p]
        function.restype = ctypes.c_int
        _darwin_rusage = function
    usage = DarwinUsageV0()
    if _darwin_rusage(os.getpid(), 0, ctypes.byref(usage)) != 0:
        raise OSError(ctypes.get_errno(), 'proc_pid_rusage RUSAGE_INFO_V0 failed')
    return {'current_rss_bytes': usage.ri_resident_size,
            'current_rss_source': 'macOS proc_pid_rusage RUSAGE_INFO_V0.ri_resident_size',
            'physical_footprint_bytes': usage.ri_phys_footprint,
            'physical_footprint_source': 'macOS proc_pid_rusage RUSAGE_INFO_V0.ri_phys_footprint',
            'parent_pageins': usage.ri_pageins,
            'parent_interrupt_wakeups': usage.ri_interrupt_wkups,
            'parent_package_idle_wakeups': usage.ri_pkg_idle_wkups}


def checkpoint(report, path, phase='progress', case=None, unit_index=None):
    """Keep checkpoint observer cost without putting it in scored durations.

    A completed observation is included in the next checkpoint or final report.
    The active write is explicitly incomplete in its own checkpoint.
    """
    row = {'phase': phase, 'case': case, 'unit_index': unit_index,
           'monotonic_start_seconds': time.monotonic(), 'complete': False, 'used_for_gate': False}
    report.setdefault('checkpoint_observations', []).append(row)
    report['report_kind'] = 'compact-checkpoint'
    try:
        persist(report, path)
    finally:
        row.update(monotonic_end_seconds=time.monotonic(), complete=True)
        row['seconds'] = row['monotonic_end_seconds'] - row['monotonic_start_seconds']
        row['json_bytes'] = path.stat().st_size if path.exists() else None


def parser():
    result = argparse.ArgumentParser(description=__doc__)
    result.add_argument('--baseline', type=Path, required=True)
    result.add_argument('--candidate', type=Path, required=True)
    result.add_argument('--metadata', type=Path, required=True,
                        help='JSON with runner_label, toolchain and per-side revision/build_command')
    result.add_argument('--output', type=Path, default=REPO / 'target/runtime-performance/report.json')
    result.add_argument('--pairs', type=int, default=DEFAULT_PAIRS,
                        help=f'Logical paired batch count, a multiple of four from 16 to {MAX_PAIRS}')
    result.add_argument('--warmups', type=int, default=2, help='Per artifact/slot single-command warmups before separate calibration pilots, 2 to 10')
    result.add_argument('--timeout', type=float, default=30, help='Per-command deadline, (0, 60] seconds')
    result.add_argument('--budget', type=float, default=DEFAULT_BUDGET,
                        help=f'Whole-run deadline, (0, {MAX_BUDGET:g}] seconds')
    result.add_argument('--threshold', type=float, default=0.10, help='Predeclared relative practical slowdown, (0, 1]')
    result.add_argument('--min-sample-seconds', type=float, default=1.0, help='Minimum aggregate batch duration, [0.05, 2] seconds')
    result.add_argument('--warm-cache-min-sample-seconds', type=float, default=1.0,
                        help='Longer minimum for startup-sensitive cache-hit batches, [0.25, 4] seconds')
    result.add_argument('--cli-startup-min-sample-seconds', type=float, default=0.25,
                        help='Predeclared CLI-startup batch minimum, [0.25, 4] seconds')
    result.add_argument('--max-iterations', type=int, default=MAX_ITERATIONS,
                        help=f'Matched batch iteration ceiling, 1 to {MAX_ITERATIONS}')
    result.add_argument('--gate', action='store_true', help='Exit 0 pass, 1 demonstrated regression/error, 2 inconclusive')
    return result


def main(argv=None):
    argument_parser = parser()
    args = argument_parser.parse_args(argv)
    if os.name != 'posix' or platform.system() not in ('Linux', 'Darwin'):
        argument_parser.error('Requires macOS or Linux owned POSIX process groups')
    for name, upper in (('timeout', 60), ('budget', MAX_BUDGET), ('threshold', 1)):
        value = getattr(args, name)
        if not math.isfinite(value) or not 0 < value <= upper:
            argument_parser.error(f'--{name} must be finite and in (0, {upper}]')
    if not math.isfinite(args.min_sample_seconds) or not 0.05 <= args.min_sample_seconds <= 2:
        argument_parser.error('--min-sample-seconds must be finite and in [0.05, 2]')
    if not math.isfinite(args.warm_cache_min_sample_seconds) or not 0.25 <= args.warm_cache_min_sample_seconds <= 4:
        argument_parser.error('--warm-cache-min-sample-seconds must be finite and in [0.25, 4]')
    if not math.isfinite(args.cli_startup_min_sample_seconds) or not 0.25 <= args.cli_startup_min_sample_seconds <= 4:
        argument_parser.error('--cli-startup-min-sample-seconds must be finite and in [0.25, 4]')
    if not 16 <= args.pairs <= MAX_PAIRS or args.pairs % PAIRS_PER_UNIT or not 2 <= args.warmups <= 10 \
            or not 1 <= args.max_iterations <= MAX_ITERATIONS:
        argument_parser.error(f'Requires --pairs 16..{MAX_PAIRS} divisible by four, --warmups 2..10 and --max-iterations 1..{MAX_ITERATIONS}')
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
    for output_path in (args.output, args.output.with_suffix('.md'), args.output.with_suffix('.overflow.json')):
        for input_path in (*sources.values(), args.metadata.resolve()):
            if output_path == input_path or (output_path.exists() and output_path.samefile(input_path)):
                argument_parser.error('Output paths must not overwrite selected inputs or aliases')
    report = {'schema_version': SCHEMA_VERSION, 'gate': args.gate, 'status': 'incomplete', 'metadata': metadata,
              'timing_metadata': TIMING_METADATA, 'host_observations': [],
              'prospective_sampling_study': prospective_power_study(),
              'machine': {'system': platform.system(), 'release': platform.release(),
                          'architecture': platform.machine(), 'processor': platform.processor(),
                          'logical_cpu_count': os.cpu_count(), 'python': platform.python_version()},
              'diagnostic_telemetry': {'reported_build_timing': {
                  'source': 'Optional build-report JSON elapsed_ms', 'untrusted': True, 'used_for_gate': False,
                  'reported_elapsed_ms_scope': 'Timer inside the build function. Excludes launch, CLI/configuration setup, '
                      'compiler prelude before the timer and local destruction after its final timestamp.',
                  'parent_minus_reported_seconds_scope': 'Diagnostic residual includes parent capture setup, launch, '
                      'CLI/configuration setup, untimed compiler prelude and destruction, serialization, exit and '
                      'completion scheduling. It does not isolate loader time.'}},
              'policy': {'relative_threshold': args.threshold, 'familywise_confidence': 1 - FAMILY_ALPHA,
                         'checkpoint_unit': 'complete-inference-unit', 'maximum_report_bytes': MAX_REPORT_BYTES,
                         'checkpoint_storage': 'compact-checkpoints-with-append-only-command-journal-v1',
                         'raw_command_memory_scope': 'Completed batches leave memory after journal append. '
                             'At most one complete inference unit plus an active batch is retained during measurement. '
                             'Final full-report reconstruction occurs once, after scored work.',
                         'environment_isolation': {'removed_prefixes': ['TEKAI_', 'TEX', 'BIB', 'KPATHSEA'],
                                                   'removed_search_variables': sorted(SEARCH_ENV_VARS),
                                                   'recorded_keys': ISOLATED_ENVIRONMENT_KEYS,
                                                   'locale': 'C', 'timezone': 'UTC'},
                         'pairs_per_case': args.pairs, 'warmups_per_side': args.warmups,
                         'warmup_unit': 'artifact-slot-cell',
                         'crossover_design': 'fixed-slot-complementary-quads-v1',
                         'pairs_per_inference_unit': PAIRS_PER_UNIT, 'batches_per_inference_unit': 2 * PAIRS_PER_UNIT,
                         'inference_units_per_case': args.pairs // PAIRS_PER_UNIT,
                         'slot_ids': SLOTS, 'replica_ids': REPLICAS,
                         'replica_assignment': {slot: {label: replica_for(label, slot) for label in LABELS} for slot in SLOTS},
                         'min_sample_seconds': args.min_sample_seconds, 'max_iterations': args.max_iterations,
                         'warm_cache_min_sample_seconds': args.warm_cache_min_sample_seconds,
                         'cli_startup_min_sample_seconds': args.cli_startup_min_sample_seconds,
                         'calibration_headroom': CALIBRATION_HEADROOM,
                         'calibration_pairs': CALIBRATION_PAIRS,
                         'calibration_pair_unit': 'per-slot',
                         'calibration_iterations': CALIBRATION_ITERATIONS,
                         'calibration_method': 'balanced-batched-slot-pilot-v1',
                         'per_command_deadline_seconds': args.timeout, 'whole_run_deadline_seconds': args.budget,
                         'noise_relative_mad_limit': NOISE_LIMIT, 'order_bias_limit': ORDER_BIAS_LIMIT,
                         'method': 'Four logical within-slot pairs form one complementary eight-batch crossover unit. '
                         'Its artifact ratio is the fourth root of the product of four candidate times divided by '
                         'the product of four baseline times. Exact binomial sign/order-statistic intervals bound '
                         'the population median of complete unit ratios, '
                         'with Bonferroni correction across the four predeclared cases. '
                         'Fail requires the entire interval above the practical boundary and no noise flags. '
                         'Pass requires the entire interval at or below it and no noise flags. '
                         'Otherwise the comparison is inconclusive.',
                         'assumptions': 'Complementary quads balance repeatable slot-by-quad-position factors '
                         'that are additive in log time, equivalently multiplicative elapsed-time factors. '
                         'Crossed neutral slots and replicas balance path factors additive in log time, not arbitrary '
                         'artifact-by-copy/cache interactions. Units must be independent with a stable population median. '
                         'Arbitrary additive wall-clock overhead, changing positional effects, nonlinear drift and '
                         'serial dependence can violate these assumptions.'},
              'executables': [], 'results': [], 'errors': []}
    previous_handler = signal.getsignal(signal.SIGTERM)

    def terminate(_number, _frame):
        raise KeyboardInterrupt

    signal.signal(signal.SIGTERM, terminate)
    started = time.monotonic()
    report['monotonic_start_seconds'] = started
    report['clock_origin'] = {'unix_ns': time.time_ns(), 'monotonic_seconds': time.monotonic(),
                             'alignment': 'Sequential wall then monotonic observations; approximate alignment only.'}
    try:
        with tempfile.TemporaryDirectory(prefix='tekai-paired-benchmark-') as temporary:
            work = Path(temporary).resolve()
            supervisor = Supervisor(work, args.timeout, started + args.budget)
            journal = None
            binaries = {slot: {} for slot in SLOTS}
            try:
                verifiers = {name: shutil.which(name) for name in ('pdftotext', 'pdfinfo', 'pdfimages')}
                if not all(verifiers.values()):
                    raise BenchmarkError('Poppler pdftotext, pdfinfo and pdfimages are required for independent fixture verification')
                report['output_verifiers'] = {name: {'path': path, 'sha256_start': executable_sha256(Path(path))}
                                              for name, path in verifiers.items()}
                for cell in cell_order():
                    label, slot, replica = cell['label'], cell['slot'], cell['replica']
                    source = sources[label]
                    supervisor.remaining()
                    dest = work / slot / replica / 'bin' / 'tekai'
                    dest.parent.mkdir(parents=True)
                    row = {**cell, 'source_path': str(source), 'isolated_path': str(dest),
                           'source_sha256_start': executable_sha256(source)}
                    report['executables'].append(row)
                    shutil.copy2(source, dest)
                    row['sha256_start'] = executable_sha256(dest)
                    if row['sha256_start'] != row['source_sha256_start']:
                        raise BenchmarkError('Selected binary changed during its isolated copy')
                    # Optional build provenance binds the measured bytes to the
                    # recorded build. It never enters timing or calibration.
                    expected = metadata[label].get('artifact_sha256')
                    if expected is not None and expected != row['source_sha256_start']:
                        raise BenchmarkError('Selected ' + label + ' binary differs from its recorded build artifact SHA-256')
                    binaries[slot][label] = dest
                journal = CommandJournal(args.output)
                report['raw_command_journal'] = journal.binding()
                report['journal_observations'] = journal.observations
                checkpoint(report, args.output, 'isolated-binaries')
                for case_index, case in enumerate(CASES):
                    supervisor.remaining()
                    fixtures = {slot: {} for slot in SLOTS}
                    inventory = []
                    row = {'case': case, 'fixture_inventory': inventory,
                           'fixture_sha256': {slot: {} for slot in SLOTS},
                           'fixture_roots': {slot: {} for slot in SLOTS},
                           'slot_schedule': [crossover_pair(index, case_index) for index in range(args.pairs)],
                           'min_sample_seconds': {'warm-build-cache': args.warm_cache_min_sample_seconds,
                                                  'cli-startup': args.cli_startup_min_sample_seconds}.get(case, args.min_sample_seconds),
                           'warmups': [], 'calibration_pilots': [], 'samples': [], 'unit_timing': []}
                    report['results'].append(row)
                    for cell in cell_order(case_index):
                        slot, label, replica = cell['slot'], cell['label'], cell['replica']
                        fixture = make_fixture(work / slot / replica / case, binaries[slot][label], case)
                        fixture.update(verifiers, **cell)
                        fixture['expected_version'] = metadata[label]['expected_version']
                        fixtures[slot][label] = fixture
                        row['fixture_sha256'][slot][label] = fixture['input_sha256']
                        row['fixture_roots'][slot][label] = str(fixture['root'])
                        inventory.append({**cell, 'root': str(fixture['root']), 'project': str(fixture['project']),
                                          'out': str(fixture['out']), 'home': fixture['env']['HOME'],
                                          'config_path': str(fixture['config']), 'config_sha256': executable_sha256(fixture['config']),
                                          'referenced_dependency_count': len(fixture['dependencies']),
                                          'isolated_environment': {key: fixture['env'][key] for key in ISOLATED_ENVIRONMENT_KEYS},
                                          'cache_paths': {kind: fixture['env']['TEKAI_' + kind + '_CACHE']
                                                          for kind in ('ENGINE', 'FORMAT', 'AUX', 'BIBTEX')},
                                          'input_sha256': fixture['input_sha256']})
                    if len({value for hashes in row['fixture_sha256'].values() for value in hashes.values()}) != 1:
                        raise BenchmarkError('Per-binary source fixtures do not have equal content')
                    row['initialization'] = {slot: {} for slot in SLOTS}
                    row['initialization_order'] = cell_order(case_index)
                    for cell in row['initialization_order']:
                        slot, label = cell['slot'], cell['label']
                        initial = execute_fixture(supervisor, fixtures[slot][label], case, initializing=True)
                        row['initialization'][slot][label] = {'command_id': initial['command_id'], 'seconds': initial['seconds'],
                                                             'parent_timing': initial['parent_timing'],
                                                             'command_timing': initial['command_timing'],
                                                             'output_validation': initial['output_validation']}
                        if 'reported_build_timing' in initial:
                            row['initialization'][slot][label]['reported_build_timing'] = initial['reported_build_timing']
                        if 'cache_state' in initial:
                            row['initialization'][slot][label]['cache_state'] = initial['cache_state']
                    for warmup_index in range(args.warmups):
                        for slot in SLOTS:
                            warmup = {'warmup_index': warmup_index, **slot_pair(slot, warmup_index, case_index)}
                            row['warmups'].append(warmup)
                            for label in warmup['order']:
                                warmup[label] = batch(supervisor, fixtures[slot][label], case, 1)
                    journal.append(case_index, 'warmups', [(index, label, warmup[label])
                                   for index, warmup in enumerate(row['warmups']) for label in warmup['order']])
                    report['raw_command_journal'] = journal.binding()
                    for pilot_index in range(CALIBRATION_PAIRS):
                        for slot in SLOTS:
                            pilot = {'pair_index': pilot_index, **slot_pair(slot, pilot_index, case_index),
                                     'iterations': CALIBRATION_ITERATIONS}
                            # Retain completed-side evidence in every phase.
                            # Pilots never enter inference.
                            row['calibration_pilots'].append(pilot)
                            for label in pilot['order']:
                                pilot[label] = batch(supervisor, fixtures[slot][label], case, CALIBRATION_ITERATIONS)
                            journal.append(case_index, 'calibration_pilots',
                                           [(len(row['calibration_pilots']) - 1, label, pilot[label]) for label in pilot['order']])
                            report['raw_command_journal'] = journal.binding()
                            checkpoint(report, args.output, 'calibration', case)
                    row['calibration'] = calibration_details(row['calibration_pilots'], row['min_sample_seconds'], args.max_iterations)
                    try:
                        iterations = calibrated_iterations(row['calibration_pilots'], row['min_sample_seconds'], args.max_iterations)
                    except CalibrationInfeasible as error:
                        row['calibration_infeasible'] = error.details
                        row['analysis'] = {'status': 'inconclusive', 'reasons': [str(error)]}
                    else:
                        row['iterations_per_batch'] = iterations
                        checkpoint(report, args.output, 'calibrated', case)
                        for pair_index in range(args.pairs):
                            if pair_index % PAIRS_PER_UNIT == 0:
                                parent_before = parent_snapshot()
                                unit = {'unit_index': pair_index // PAIRS_PER_UNIT,
                                        'first_pair_index': pair_index, 'pair_count': PAIRS_PER_UNIT,
                                        'parent_before': parent_before,
                                        'monotonic_start_seconds': time.monotonic(), 'complete': False}
                                row['unit_timing'].append(unit)
                            sample = {**crossover_pair(pair_index, case_index), 'iterations': iterations}
                            # Keep partial pairs so a later failure cannot erase the
                            # successful side's observation from the final artifact.
                            row['samples'].append(sample)
                            for label in sample['order']:
                                sample[label] = batch(supervisor, fixtures[sample['slot']][label], case, iterations)
                            ratio = sample['candidate']['seconds'] / sample['baseline']['seconds']
                            if not math.isfinite(ratio) or ratio <= 0:
                                raise ValueError('Paired ratio overflowed or underflowed; statistics are unusable')
                            sample['ratio'] = ratio
                            if (pair_index + 1) % PAIRS_PER_UNIT == 0:
                                unit.update(monotonic_end_seconds=time.monotonic(), complete=True)
                                unit['parent_after'] = parent_snapshot()
                                report['host_observations'].append(host_snapshot(case, unit['unit_index']))
                                journal.append(case_index, 'samples', [(index, label, row['samples'][index][label])
                                               for index in range(pair_index + 1 - PAIRS_PER_UNIT, pair_index + 1)
                                               for label in row['samples'][index]['order']])
                                report['raw_command_journal'] = journal.binding()
                                checkpoint(report, args.output, 'complete-inference-unit', case, unit['unit_index'])
                        row['analysis'] = analyze_pairs(row['samples'], args.threshold, row['min_sample_seconds'], case_index=case_index)
                    row['fixture_sha256_end'] = {slot: {label: fixture_sha256(fixtures[slot][label]['project']) for label in LABELS} for slot in SLOTS}
                    row['fixture_unchanged'] = row['fixture_sha256'] == row['fixture_sha256_end']
                    if not row['fixture_unchanged']:
                        raise BenchmarkError('Compiler changed or removed benchmark fixture inputs')
                    print(case + ' ' + row['analysis']['status'], flush=True)
                    checkpoint(report, args.output, 'case-complete', case)
            except (OSError, ValueError, RuntimeError, subprocess.SubprocessError) as error:
                report['errors'].append(f'{type(error).__name__}: {error}')
                if isinstance(error, ReportTooLarge) and journal is not None:
                    # Raw commands for the unappended unit are still attached.
                    # Preserve those beside all earlier journal descriptors
                    # before the one-time full restoration can exceed its cap.
                    report['raw_command_journal'] = journal.binding()
                    report['status'], report['exit_code'] = 'error', 1
                    try:
                        checkpoint(report, args.output, 'bounded-evidence-error')
                    except (OSError, ValueError, RuntimeError) as checkpoint_error:
                        report['errors'].append('Compact error checkpoint failed: ' + str(checkpoint_error))
            except KeyboardInterrupt:
                report['errors'].append('Run interrupted; terminating owned process groups')
            finally:
                try:
                    supervisor.close()
                except (OSError, RuntimeError, subprocess.SubprocessError) as error:
                    report['errors'].append('Cleanup failed: ' + str(error))
                if journal is not None:
                    try:
                        journal.restore()
                        report['raw_command_journal'] = journal.binding()
                    except (OSError, ValueError, RuntimeError) as error:
                        report['errors'].append('Raw command journal restore failed: ' + str(error))
                    finally:
                        journal.close()
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
    report['monotonic_end_seconds'] = time.monotonic()
    report['elapsed_seconds'] = report['monotonic_end_seconds'] - started
    report['exit_code'] = comparison_exit(report['status'], args.gate)
    report['report_kind'] = 'complete-report'
    try:
        persist(report, args.output)
    except ReportTooLarge as error:
        print(str(error), file=sys.stderr, flush=True)
        return 1
    print('Comparison ' + report['status'] + '; report ' + str(args.output), flush=True)
    return report['exit_code']


if __name__ == '__main__':
    sys.exit(main())
