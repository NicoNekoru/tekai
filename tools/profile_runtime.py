#!/usr/bin/env python3
"""Separate, bounded runtime diagnostics. Never changes the primary verdict.

System time wrappers and optional binary phase logs change the measured command.
Their observations cannot be used as replacement samples or speedup evidence.
"""

import argparse
import gc
import json
import math
import os
from pathlib import Path
import platform
import re
import resource
import shutil
import signal
import statistics
import subprocess
import sys
import tempfile
import time

import benchmark_runtime as benchmark

MAX_REPORT_BYTES = 128 * 1024 * 1024
MAX_PROFILE_BYTES = 16 * 1024 * 1024
MAX_TEXT_BYTES = 32768
MAX_PHASE_BYTES = 16384
MAX_COUNTER = (1 << 63) - 1
RESOURCE_MARKER = '__TEKAI_PROFILE_RESOURCE__'
RESOURCE_FIELDS = ('user_cpu_seconds', 'system_cpu_seconds', 'peak_rss_bytes', 'major_faults',
                   'minor_faults', 'voluntary_context_switches', 'involuntary_context_switches',
                   'filesystem_inputs', 'filesystem_outputs', 'swaps')
HASH = re.compile(r'[0-9a-f]{64}\Z')
PHASE_SOURCES_V1 = {
    'cli_parse': 'main::run_cli argument parsing',
    'cli_config_setup': 'main CLI configuration and option setup',
    'build_total': 'compiler::build',
    'build_setup': 'compiler direct build path, directory and mode-key setup',
    'build_state_load': 'compiler::read_build_state_if_exists',
    'build_cache_checks': 'compiler direct build initial cache decision',
    'input_freshness': 'compiler::build_state_inputs_are_fresh',
    'settled_cache_restore': 'compiler::restore_settled_aux_cache_if_fresh',
    'tex_subprocess': 'compiler TeX command.status calls, including preamble compilation',
    'embedded_engine_entry': 'native exact engine, outside parent process instrumentation',
    'format_initialization': 'native exact engine, outside parent process instrumentation',
}
PHASE_SOURCES = {**PHASE_SOURCES_V1,
    'build_state_read': 'compiler build-state file read',
    'build_state_parse': 'compiler build-state TOML parse',
    'input_metadata': 'compiler::file_metadata_fingerprint',
    'input_content_hash': 'compiler::file_content_hash_hex read and hash',
    'input_effective_hash': 'compiler freshness fallback effective-source read and hash',
    'input_freshness_memo': 'compiler ordered freshness memo lookup and update',
    'lookup_session_reset': 'compiler::clear_kpathsea_resolution_cache',
    'cli_report_serialization': 'main build-report JSON conversion and serialization',
    'build_mode_key': 'compiler::direct_mode_key',
}
COUNTER_UNITS = {'state_bytes_read': 'bytes', 'state_dependencies': 'recorded_inputs',
    'freshness_requests': 'requests', 'freshness_memo_hits': 'requests', 'freshness_memo_misses': 'requests',
    'metadata_checks': 'calls', 'metadata_matches': 'freshness_routes',
    'content_hash_fallbacks': 'freshness_routes', 'effective_hash_fallbacks': 'freshness_routes',
    'virtual_freshness_checks': 'freshness_routes', 'stale_inputs': 'observed_stale_inputs', 'warm_cache_hits': 'builds'}
NATIVE_REASONS = {
    'embedded_engine_entry': 'native_engine_exits_without_returning_to_Rust',
    'format_initialization': 'not_separately_instrumented_inside_native_engine',
}


def reject_constant(value):
    raise ValueError('Nonfinite JSON constant ' + value)


def read_json(path):
    with path.open('rb') as handle:
        content = handle.read(MAX_REPORT_BYTES + 1)
    if len(content) > MAX_REPORT_BYTES:
        raise ValueError('Report exceeds the finite input limit')
    value = json.loads(content, parse_constant=reject_constant)
    if not isinstance(value, dict):
        raise ValueError('Report must be a JSON object')
    return value


def finite_number(value, integer=False):
    try:
        return type(value) is int and 0 <= value <= MAX_COUNTER if integer else \
            type(value) in (int, float) and math.isfinite(value) and 0 <= value <= MAX_COUNTER
    except OverflowError:
        return False


def parse_resources(stderr, kind, truncated=False):
    result = {'status': 'unavailable', 'source': kind, 'untrusted': True, 'used_for_gate': False,
              'capture_note': 'Wrapper and selected binary share stderr; this text channel is not authenticated.',
              'scope': 'System timer accounting for the wrapped process and its waited-for children. '
                       'Peak RSS conventions are platform-specific; this is not a process-tree RSS sum or whole-job memory.',
              'cpu_timer_resolution_seconds': 0.01,
              'precision_note': 'Short-command CPU may round to 0.00 seconds; zero does not establish zero CPU work.',
              'io_counter_units': 'Operations, not bytes',
              'rss_source_units': 'Bytes' if kind == 'darwin' else 'KiB' if kind == 'gnu' else None,
              'values': {field: None for field in RESOURCE_FIELDS}}
    if truncated or len(stderr.encode('utf-8')) > MAX_TEXT_BYTES:
        result['reason'] = 'Resource capture is truncated or exceeds its parser limit'
        return result
    if kind not in ('darwin', 'gnu'):
        result['reason'] = 'No supported system timer is available'
        return result
    try:
        if kind == 'gnu':
            lines = [line for line in stderr.splitlines() if line.startswith(RESOURCE_MARKER)]
            if len(lines) != 1:
                raise ValueError('Expected exactly one GNU time record')
            parts = lines[0].split()
            if len(parts) != len(RESOURCE_FIELDS) + 1 or parts[0] != RESOURCE_MARKER:
                raise ValueError('Malformed GNU time record')
            values = {}
            for index, field in enumerate(RESOURCE_FIELDS):
                token = parts[index + 1]
                value = float(token) if index < 2 else int(token)
                if not finite_number(value, integer=index >= 2):
                    raise ValueError('Invalid resource value')
                values[field] = value * 1024 if field == 'peak_rss_bytes' else value
                if not finite_number(values[field], integer=index >= 2):
                    raise ValueError('Resource conversion overflow')
        else:
            cpu = re.findall(r'(?m)^\s*([\S]+) real\s+([\S]+) user\s+([\S]+) sys\s*$', stderr)
            if len(cpu) != 1 or any(not finite_number(float(value)) for value in cpu[0]):
                raise ValueError('Expected exactly one finite BSD time CPU record')
            values = {'user_cpu_seconds': float(cpu[0][1]), 'system_cpu_seconds': float(cpu[0][2])}
            labels = {'peak_rss_bytes': 'maximum resident set size', 'major_faults': 'page faults',
                      'minor_faults': 'page reclaims', 'voluntary_context_switches': 'voluntary context switches',
                      'involuntary_context_switches': 'involuntary context switches',
                      'filesystem_inputs': 'block input operations', 'filesystem_outputs': 'block output operations',
                      'swaps': 'swaps'}
            for field, label in labels.items():
                matches = re.findall(r'(?m)^\s*(\S+)\s+' + re.escape(label) + r'\s*$', stderr)
                if len(matches) != 1:
                    raise ValueError('Missing or duplicate BSD time field ' + label)
                value = int(matches[0])
                if not finite_number(value, integer=True):
                    raise ValueError('Invalid BSD time counter')
                values[field] = value
        result.update(status='reported', values=values)
    except (ValueError, OverflowError) as error:
        result['reason'] = str(error)
    return result


def parse_phases(stderr, truncated=False):
    result = {'status': 'unavailable', 'untrusted': True, 'used_for_gate': False, 'data': None}
    if truncated:
        result.update(status='invalid', reason='Phase capture is truncated')
        return result
    lines = [line[len('TEKAI_PROFILE '):] for line in stderr.splitlines() if line.startswith('TEKAI_PROFILE ')]
    if not lines:
        result['reason'] = 'Selected binary emitted no opt-in phase record'
        return result
    if len(lines) != 1 or len(lines[0].encode('utf-8')) + len('TEKAI_PROFILE ') + 1 > MAX_PHASE_BYTES:
        result.update(status='invalid', reason='Phase record is duplicated, truncated or exceeds its finite limit')
        return result
    try:
        data = json.loads(lines[0], parse_constant=reject_constant)
        nodes = [0]

        def bounded(value, depth=0):
            nodes[0] += 1
            if depth > 4 or nodes[0] > 512:
                return False
            if isinstance(value, dict):
                return len(value) <= 64 and all(type(key) is str and len(key) <= 128
                    and bounded(item, depth + 1) for key, item in value.items())
            if isinstance(value, list):
                return len(value) <= 32 and all(bounded(item, depth + 1) for item in value)
            if type(value) in (int, float):
                return finite_number(value)
            return value is None or type(value) is bool or type(value) is str and len(value) <= 256

        if not isinstance(data, dict) or not bounded(data):
            raise ValueError('Phase JSON exceeds its finite structural or value limits')
        version = data.get('schema_version')
        expected_keys = {'schema_version', 'producer', 'scope', 'source', 'untrusted', 'status', 'elapsed_ms',
                         'completed_spans_only', 'phases_overlap', 'phases'}
        if version == 2:
            expected_keys.add('counters')
        if set(data) != expected_keys \
                or type(version) is not int or version not in (1, 2) \
                or data.get('producer') != 'tekai-rust' or data.get('scope') != 'cli_process' \
                or data.get('source') != 'opt_in_rust_instrumentation' or data.get('untrusted') is not True \
                or data.get('completed_spans_only') is not True or data.get('phases_overlap') is not True \
                or data.get('status') not in ('success', 'error', 'exit_failure') \
                or not finite_number(data.get('elapsed_ms')):
            raise ValueError('Unsupported opt-in phase protocol')
        if version == 1 and len(lines[0].encode('utf-8')) + len('TEKAI_PROFILE ') + 1 > 8192:
            raise ValueError('Legacy phase record exceeds its 8-KiB bound')
        phases = data.get('phases')
        sources = PHASE_SOURCES if version == 2 else PHASE_SOURCES_V1
        names = tuple(sources)
        if not isinstance(phases, list) or len(phases) != len(names) \
                or any(not isinstance(phase, dict) for phase in phases) \
                or any(type(phase.get('name')) is not str for phase in phases) \
                or {phase.get('name') for phase in phases} != set(names):
            raise ValueError('Missing or duplicated fixed phase inventory')
        for phase in phases:
            name = phase['name']
            scope = 'native_engine_internal' if name in NATIVE_REASONS else \
                'subprocess_launch_and_wait' if name == 'tex_subprocess' else 'inclusive_wall_time_current_process'
            if set(phase) != {'name', 'status', 'elapsed_ms', 'calls', 'active_calls', 'source', 'scope', 'reason'} \
                    or phase.get('status') not in ('available', 'unavailable') \
                    or not finite_number(phase.get('calls'), integer=True) \
                    or not finite_number(phase.get('active_calls'), integer=True) \
                    or phase.get('source') != sources[name] or phase.get('scope') != scope:
                raise ValueError('Invalid phase scope, source or call count')
            if phase['status'] == 'available':
                if name in NATIVE_REASONS or phase['calls'] == 0 or not finite_number(phase.get('elapsed_ms')) \
                        or phase.get('reason') is not None:
                    raise ValueError('Invalid available phase duration')
            elif phase['calls'] != 0 or phase.get('elapsed_ms') is not None \
                    or phase.get('reason') != NATIVE_REASONS.get(name, 'no_completed_call_in_this_process'):
                raise ValueError('Unavailable phase requires null duration and a reason')
        if version == 2:
            counters = data.get('counters')
            if not isinstance(counters, list) or len(counters) != len(COUNTER_UNITS) \
                    or any(not isinstance(counter, dict) or type(counter.get('name')) is not str for counter in counters) \
                    or {counter['name'] for counter in counters} != set(COUNTER_UNITS):
                raise ValueError('Missing or duplicated fixed counter inventory')
            for counter in counters:
                if set(counter) != {'name', 'value', 'unit', 'scope'} \
                        or not finite_number(counter.get('value'), integer=True) \
                        or counter.get('unit') != COUNTER_UNITS[counter['name']] \
                        or counter.get('scope') != 'aggregate_observations_current_process':
                    raise ValueError('Invalid counter unit, scope or count')
        result.update(status='reported', data=data,
                      compatibility={'schema_version': version, 'granular_observations': 'available' if version == 2
                                     else 'unavailable_legacy_schema', 'missing_phase_names': [] if version == 2
                                     else sorted(set(PHASE_SOURCES) - set(PHASE_SOURCES_V1)),
                                     'counters_available': version == 2})
    except (ValueError, RecursionError, OverflowError) as error:
        result.update(status='invalid', reason=str(error))
    return result


def parse_vm_stat(text):
    match = re.search(r'page size of (\d+) bytes', text)
    if not match or not finite_number(int(match[1]), integer=True) or int(match[1]) == 0:
        raise ValueError('Missing vm_stat page size')
    page_size = int(match[1])
    fields = {}
    for line in text.splitlines()[1:]:
        match = re.fullmatch(r'([^:]+):\s*(\d+)\.?\s*', line)
        if match:
            key = match[1].strip().strip('"')
            if key in fields or not finite_number(int(match[2]), integer=True):
                raise ValueError('Invalid or duplicate vm_stat counter')
            fields[key] = int(match[2])
    if not fields:
        raise ValueError('vm_stat has no finite counters')
    memory = {key: fields[key] * page_size for key in ('Pages free', 'Pages active', 'Pages inactive',
              'Pages wired down', 'Pages occupied by compressor') if key in fields}
    if not memory or any(not finite_number(value, integer=True) for value in memory.values()):
        raise ValueError('vm_stat memory conversion overflow')
    return {'page_size_bytes': page_size, 'memory_bytes': memory,
            'vm_counters': {key: fields.get(key) for key in ('Pageins', 'Pageouts', 'Swapins', 'Swapouts')}}


def parse_swap(text):
    values = {}
    for name in ('total', 'used', 'free'):
        matches = re.findall(r'\b' + name + r'\s*=\s*([\d.]+)([KMGT]?)\b', text)
        if len(matches) != 1:
            raise ValueError('Missing or duplicate swap counter')
        number, unit = matches[0]
        value = float(number) * 1024 ** ('KMGT'.index(unit) + 1 if unit else 0)
        if not finite_number(value):
            raise ValueError('Invalid swap extent')
        values[name] = int(value)
    if values['free'] > values['total'] or values['used'] > values['total'] \
            or abs(values['free'] + values['used'] - values['total']) > 1024 * 1024:
        raise ValueError('Inconsistent swap extents')
    return values


def parse_host_cpu(text):
    matches = re.findall(r'(?m)^CPU usage:\s*([\d.]+)% user,\s*([\d.]+)% sys,\s*([\d.]+)% idle\s*$', text)
    if len(matches) != 2:
        raise ValueError('Expected two bounded host CPU samples')
    values = dict(zip(('user', 'system', 'idle'), map(float, matches[-1])))
    if any(not finite_number(value) or value > 100 for value in values.values()) \
            or not 99 <= sum(values.values()) <= 101:
        raise ValueError('Invalid host CPU percentages')
    return values


def parse_linux_host(stat, memory):
    first = stat.splitlines()[0].split() if stat else []
    if len(first) < 5 or first[0] != 'cpu':
        raise ValueError('Missing Linux aggregate CPU counters')
    counters = list(map(int, first[1:11]))
    if any(not finite_number(value, integer=True) for value in counters):
        raise ValueError('Invalid Linux CPU counter')
    fields = {}
    for line in memory.splitlines():
        match = re.fullmatch(r'(MemTotal|MemAvailable|MemFree|SwapTotal|SwapFree):\s*(\d+) kB\s*', line)
        if match:
            if match[1] in fields:
                raise ValueError('Duplicate Linux memory counter')
            value = int(match[2]) * 1024
            if not finite_number(value, integer=True):
                raise ValueError('Linux memory conversion overflow')
            fields[match[1]] = value
    if not all(key in fields for key in ('MemTotal', 'MemFree', 'SwapTotal', 'SwapFree')) \
            or fields['SwapFree'] > fields['SwapTotal'] or fields['MemFree'] > fields['MemTotal'] \
            or fields.get('MemAvailable', 0) > fields['MemTotal']:
        raise ValueError('Missing or inconsistent Linux memory counters')
    return {'cpu_ticks': dict(zip(('user', 'nice', 'system', 'idle', 'iowait', 'irq', 'softirq',
                                  'steal', 'guest', 'guest_nice'), counters)),
            'memory_bytes': {key: value for key, value in fields.items() if key.startswith('Mem')},
            'swap_bytes': {'total': fields['SwapTotal'], 'free': fields['SwapFree'],
                           'used': fields['SwapTotal'] - fields['SwapFree']}}


def timer_command(kind):
    if kind == 'darwin':
        return ['/usr/bin/time', '-l']
    if kind == 'gnu':
        return ['/usr/bin/time', '-f', RESOURCE_MARKER + ' %U %S %M %F %R %w %c %I %O %W']
    return []


class ProfileSupervisor(benchmark.Supervisor):
    def __init__(self, work, timeout, deadline):
        super().__init__(work, timeout, deadline)
        self.kind = None
        self.timer_reason = 'System timer has not been probed'
        self.active = None

    @staticmethod
    def captured(process):
        # The primary supervisor intentionally keeps only an 8-KiB stderr tail.
        # Diagnostics need the full bounded channel to reject hidden duplicate
        # protocol lines and preserve a maximum-size phase line plus timer text.
        stdout, _tail, stdout_truncated = benchmark.Supervisor.captured(process)
        with process.capture_paths[1].open('rb') as handle:
            raw = handle.read(MAX_TEXT_BYTES + 1)
        return stdout, raw[:MAX_TEXT_BYTES].decode('utf-8', errors='replace'), \
            stdout_truncated or len(raw) > MAX_TEXT_BYTES

    def probe_timer(self, env):
        kind = 'darwin' if platform.system() == 'Darwin' else 'gnu' if platform.system() == 'Linux' else None
        if kind is None or not Path('/usr/bin/time').is_file():
            self.timer_reason = 'Supported system timer is unavailable'
            return
        try:
            result = super().execute([*timer_command(kind), sys.executable, '-B', '-c', 'pass'], self.work, env)
            resource = parse_resources(result['stderr_tail'], kind)
            if resource['status'] != 'reported':
                self.timer_reason = resource['reason']
                return
            self.kind, self.timer_reason = kind, None
        except (OSError, ValueError, RuntimeError, subprocess.SubprocessError) as error:
            self.timer_reason = str(error)[-8192:]

    def execute(self, command, cwd, env):
        if self.active is None or list(map(str, command)) != self.active['command']:
            return super().execute(command, cwd, env)
        row = self.active
        row['parent_before'] = parent_snapshot()
        row.update(status='running', launch_unix_ns=time.time_ns(), launch_monotonic_ns=time.monotonic_ns(),
                   popen_latency_seconds=None,
                   popen_latency_unavailable_reason='Pure Popen call latency is not separately timed',
                   popen_latency_scope=benchmark.TIMING_METADATA['popen_scope'],
                   launch_setup_seconds=None, command_id=self.counter + 1)
        selected_env = dict(env, LC_ALL='C', TEKAI_DIAGNOSTIC_PROFILE='1')
        stderr, truncated = '', False
        try:
            result = super().execute([*timer_command(self.kind), *command], cwd, selected_env)
            stderr = result['stderr_tail']
            timing = result.get('parent_timing')
            row.update(status='command_completed', parent_seconds=result['seconds'], parent_timing=timing,
                       command_timing=result.get('command_timing'),
                       command_timing_boundaries=benchmark.TIMING_METADATA['command_timing_fields'],
                       parent_timing_scope='Start before capture setup; launch end after Popen returns; blocking-wait completion. '
                                           'All monotonic seconds. Includes the resource wrapper.',
                       launch_setup_scope='Capture-file setup plus Popen, not pure Popen or loader time')
            if isinstance(timing, list) and len(timing) == 3 and all(finite_number(value) for value in timing) \
                    and timing[0] <= timing[1] <= timing[2]:
                row['launch_setup_seconds'] = timing[1] - timing[0]
            command_timing = result.get('command_timing')
            boundaries = benchmark.TIMING_METADATA['command_timing_fields']
            if isinstance(command_timing, list) and len(command_timing) == len(boundaries) \
                    and all(finite_number(value) for value in command_timing) \
                    and all(left <= right for left, right in zip(command_timing, command_timing[1:])):
                by_name = dict(zip(boundaries, command_timing))
                if all(name in by_name for name in ('capture_start', 'capture_files_open', 'popen_start',
                                                   'popen_return', 'waiter_dispatch', 'waiter_accepted',
                                                   'cleanup_start', 'cleanup_end')):
                    row.update(popen_latency_seconds=by_name['popen_return'] - by_name['popen_start'],
                        popen_latency_unavailable_reason=None,
                        capture_setup_seconds=by_name['capture_files_open'] - by_name['capture_start'],
                        waiter_dispatch_seconds=by_name['waiter_accepted'] - by_name['waiter_dispatch'],
                        cleanup_seconds=by_name['cleanup_end'] - by_name['cleanup_start'])
            else:
                row['command_timing_unavailable_reason'] = 'Fine command timing is absent or violates its fixed monotonic inventory'
            return result
        except (OSError, ValueError, RuntimeError, subprocess.SubprocessError, KeyboardInterrupt) as error:
            row.update(status='error', error=f'{type(error).__name__}: {error}'[-8192:])
            path = self.work / f'command-{row["command_id"]:04d}.stderr'
            if path.is_file():
                with path.open('rb') as handle:
                    raw = handle.read(MAX_TEXT_BYTES + 1)
                stderr = raw[:MAX_TEXT_BYTES].decode('utf-8', errors='replace')
                truncated = len(raw) > MAX_TEXT_BYTES
            raise
        finally:
            row.update(finish_unix_ns=time.time_ns(), finish_monotonic_ns=time.monotonic_ns(),
                       parent_after=parent_snapshot(),
                       resources=parse_resources(stderr, self.kind, truncated),
                       phases=parse_phases(stderr, truncated), stderr_tail=stderr)
            before, after = row['parent_before'], row['parent_after']
            row['parent_resource_delta'] = {name: after[name] - before[name] for name in
                ('user_cpu_seconds', 'system_cpu_seconds', 'minor_faults', 'major_faults',
                 'voluntary_context_switches', 'involuntary_context_switches') if name in before and name in after}
            row['parent_resource_delta_scope'] = 'Observer work plus command supervision between parent snapshots; not child CPU accounting.'
            if self.kind is None:
                row['resources']['reason'] = self.timer_reason


def parent_snapshot():
    """Current observer evidence outside every fixture-command clock."""
    row = {'unix_ns': time.time_ns(), 'monotonic_ns': time.monotonic_ns(),
           'scope': 'Diagnostic Python parent only, sampled outside the fixture command clock.',
           'rss_scope': 'Process-lifetime high-water RSS, not current RSS or proof of a memory leak.',
           'gc_counts': list(gc.get_count()), 'gc_stats': gc.get_stats(),
           'allocated_blocks': sys.getallocatedblocks() if hasattr(sys, 'getallocatedblocks') else None,
           'unavailable': {}}
    try:
        usage = resource.getrusage(resource.RUSAGE_SELF)
        row.update(user_cpu_seconds=usage.ru_utime, system_cpu_seconds=usage.ru_stime,
                   peak_rss_bytes=int(usage.ru_maxrss * (1 if platform.system() == 'Darwin' else 1024)),
                   minor_faults=usage.ru_minflt, major_faults=usage.ru_majflt,
                   voluntary_context_switches=usage.ru_nvcsw, involuntary_context_switches=usage.ru_nivcsw)
    except (OSError, ValueError) as error:
        row['unavailable']['resources'] = str(error)[-1024:]
    try:
        if platform.system() == 'Darwin':
            row.update(benchmark.darwin_parent_memory())
        elif platform.system() == 'Linux':
            fields = bounded_text(Path('/proc/self/statm')).split()
            row.update(current_rss_bytes=int(fields[1]) * os.sysconf('SC_PAGE_SIZE'),
                       current_rss_source='/proc/self/statm resident pages times SC_PAGE_SIZE')
        else:
            row['unavailable']['current_rss'] = 'Current RSS is unavailable on this platform'
    except (AttributeError, OSError, ValueError, IndexError) as error:
        row['unavailable']['current_rss'] = str(error)[-1024:]
    return row


def execute_profile_fixture(supervisor, fixture, case, initializing=False):
    if case != 'cli-startup':
        return benchmark.execute_fixture(supervisor, fixture, case, initializing=initializing)
    result = supervisor.execute(fixture['command'], fixture['project'], fixture['env'])
    version = fixture.get('expected_version')
    if not isinstance(version, str) or re.fullmatch(r'[0-9]+\.[0-9]+\.[0-9]+(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?', version) is None:
        raise benchmark.BenchmarkError('Diagnostic startup lacks the independently recorded package version')
    expected = f'tekai {version}\n'
    if result['stdout'] != expected or supervisor.kind is None and result['stderr_tail']:
        raise benchmark.BenchmarkError('Diagnostic startup output differs from the selected package version')
    result['output_validation'] = {'expected_stdout': expected, 'stdout_verified': True,
        'expected_version': version, 'used_for_gate': False,
        'stderr_verification': 'unavailable_shared_wrapper_channel' if supervisor.kind else 'empty_verified'}
    return result


def bounded_text(path):
    with path.open('rb') as handle:
        content = handle.read(MAX_TEXT_BYTES + 1)
    if len(content) > MAX_TEXT_BYTES:
        raise ValueError('Host snapshot exceeds its finite parser limit')
    return content.decode('ascii')


def host_snapshot(supervisor, env):
    row = {'unix_ns': time.time_ns(), 'monotonic_ns': time.monotonic_ns(), 'load_average': None,
           'cpu_percent': None, 'cpu_ticks': None, 'memory_bytes': None, 'swap_bytes': None,
           'vm_counters': None, 'unavailable': {}, 'scope': 'Whole-host observations outside fixture commands.'}
    try:
        load = list(os.getloadavg())
        if len(load) != 3 or any(not finite_number(value) for value in load):
            raise ValueError('Invalid load averages')
        row['load_average'] = load
    except (OSError, ValueError) as error:
        row['unavailable']['load_average'] = str(error)
    if platform.system() == 'Linux':
        row['unavailable']['cpu_percent'] = 'Linux snapshot retains cumulative CPU ticks, not an instantaneous CPU percentage'
        row['unavailable']['vm_counters'] = 'Darwin VM counter inventory is not collected on Linux'
        row['cpu_ticks_scope'] = 'Aggregate cumulative /proc/stat CPU ticks; guest counters overlap user/nice counters'
        try:
            row.update(parse_linux_host(bounded_text(Path('/proc/stat')), bounded_text(Path('/proc/meminfo'))))
        except (OSError, ValueError, UnicodeError) as error:
            row['unavailable']['linux_cpu_memory_swap'] = str(error)
    elif platform.system() == 'Darwin':
        row['unavailable']['cpu_ticks'] = 'Linux cumulative CPU tick inventory is not collected on Darwin'
        for name, command, parse in (
                ('vm', ['/usr/bin/vm_stat'], parse_vm_stat),
                ('swap_bytes', ['/usr/sbin/sysctl', '-n', 'vm.swapusage'], parse_swap),
                ('cpu_percent', ['/usr/bin/top', '-l', '2', '-s', '1', '-n', '0'], parse_host_cpu)):
            try:
                if not Path(command[0]).is_file():
                    raise ValueError('Host observation tool is unavailable')
                previous_timeout = supervisor.timeout
                supervisor.timeout = min(previous_timeout, 3)
                try:
                    result = supervisor.execute(command, supervisor.work, dict(env, LC_ALL='C'))
                finally:
                    supervisor.timeout = previous_timeout
                if len(result['stdout'].encode('utf-8')) > MAX_TEXT_BYTES:
                    raise ValueError('Host tool output exceeds its parser limit')
                parsed = parse(result['stdout'])
                if name == 'vm':
                    row.update(parsed)
                else:
                    row[name] = parsed
                    if name == 'cpu_percent':
                        row['cpu_percent_scope'] = 'Last of two top samples, requested one-second interval; observer work perturbs the host'
            except (OSError, ValueError, RuntimeError, subprocess.SubprocessError) as error:
                row['unavailable'][name] = str(error)[-1024:]
    else:
        row['unavailable']['host'] = 'Unsupported host observation platform'
    row.update(finish_unix_ns=time.time_ns(), finish_monotonic_ns=time.monotonic_ns())
    return row


def bind_primary(primary, metadata, sources):
    if type(primary.get('schema_version')) is not int or primary['schema_version'] not in (4, 5, 6) or primary.get('gate') is not True \
            or primary.get('metadata') != metadata:
        raise ValueError('Requires primary gate provenance matching the supplied metadata')
    if primary['schema_version'] == 6 and primary.get('report_kind') != 'complete-report':
        raise ValueError('Schema 6 requires the final complete report, not a compact checkpoint')
    if primary['schema_version'] == 6 and not isinstance(primary.get('raw_command_journal'), dict):
        raise ValueError('Schema 6 requires its final raw command journal binding')
    if primary.get('status') not in ('pass', 'fail', 'inconclusive', 'error', 'incomplete'):
        raise ValueError('Primary status is missing or unknown')
    hashes = {}
    if not isinstance(primary.get('executables'), list):
        raise ValueError('Primary executable inventory is missing')
    for label in benchmark.LABELS:
        rows = [row for row in primary.get('executables', []) if isinstance(row, dict) and row.get('label') == label]
        if not rows:
            raise ValueError('Primary has no executable binding for ' + label)
        starts = [row.get('source_sha256_start') for row in rows]
        if any(not isinstance(value, str) or HASH.fullmatch(value) is None for value in starts) or len(set(starts)) != 1:
            raise ValueError('Primary source hashes are missing or inconsistent')
        expected = starts[0]
        if any(type(row.get('source_path')) is not str or not Path(row['source_path']).is_absolute()
               or Path(row['source_path']).resolve() != sources[label]
               or row.get('unchanged') is not True
               or any(row.get(key) != expected for key in ('source_sha256_end', 'sha256_start', 'sha256_end')) for row in rows):
            raise ValueError('Primary source/copy integrity is not established for ' + label)
        recorded = metadata[label].get('artifact_sha256')
        if recorded is not None and recorded != expected:
            raise ValueError('Primary source differs from its recorded build hash')
        if benchmark.executable_sha256(sources[label]) != expected:
            raise ValueError('Current selected executable differs from primary ' + label)
        hashes[label] = expected
    return hashes


def primary_journal_path(primary, primary_path):
    journal = primary.get('raw_command_journal')
    if journal is None:
        if primary.get('schema_version') == 6:
            raise ValueError('Schema 6 requires its final raw command journal binding')
        return None
    if not isinstance(journal, dict) or type(journal.get('path')) is not str \
            or not Path(journal['path']).is_absolute() or journal.get('format') != 'tekai-command-journal-v1' \
            or not finite_number(journal.get('bytes'), integer=True) or journal['bytes'] > MAX_REPORT_BYTES \
            or type(journal.get('sha256')) is not str or HASH.fullmatch(journal['sha256']) is None:
        raise ValueError('Primary raw command journal binding is malformed')
    path = Path(journal['path']).resolve()
    if path.parent != primary_path.parent or not path.name.startswith(primary_path.stem + '.raw.') \
            or path.suffix != '.jsonl' or not path.is_file() or path.stat().st_size != journal['bytes']:
        raise ValueError('Primary raw command journal path, file kind or byte count is invalid')
    return path


def describe(values, unit):
    values = [value for value in values if finite_number(value)]
    if not values:
        return {'status': 'unavailable', 'samples': 0, 'unit': unit, 'reason': 'No finite observations'}
    center = statistics.median(values)
    mad = statistics.median(abs(value - center) for value in values)
    return {'status': 'observed', 'samples': len(values), 'unit': unit, 'median': center,
            'minimum': min(values), 'maximum': max(values), 'median_absolute_deviation': mad,
            'relative_median_absolute_deviation': mad / center if center else None}


def volatility_summary(commands):
    """Within-cell descriptive evidence, with no cross-artifact comparisons."""
    groups = {}
    for row in commands:
        key = tuple(row.get(field) for field in ('case', 'phase', 'slot', 'label'))
        groups.setdefault(key, []).append(row)
    summary = []
    for key, rows in groups.items():
        result = dict(zip(('case', 'phase', 'slot', 'label'), key))
        result.update(diagnostic_only=True, used_for_gate=False,
                      commands=len(rows), verified_commands=sum(row.get('status') == 'verified' for row in rows),
                      command_ids=[row.get('command_id') for row in rows],
                      repetitions=[row.get('repetition') for row in rows],
                      parent_stages={field: describe([row.get(field) for row in rows], 'seconds') for field in
                          ('parent_seconds', 'capture_setup_seconds', 'popen_latency_seconds', 'launch_setup_seconds',
                           'waiter_dispatch_seconds', 'cleanup_seconds')},
                      child_peak_rss=describe([row.get('resources', {}).get('values', {}).get('peak_rss_bytes')
                                               for row in rows], 'bytes'),
                      reported_build_elapsed=describe([row.get('reported_build_timing', {}).get('reported_elapsed_ms')
                                                        for row in rows], 'milliseconds'),
                      phase_protocol_statuses={status: sum(row.get('phases', {}).get('status') == status for row in rows)
                                               for status in ('reported', 'unavailable', 'invalid')},
                      phase_observations={}, counter_observations={})
        for name in PHASE_SOURCES:
            observed = [phase for row in rows if isinstance(row.get('phases', {}).get('data'), dict)
                        for phase in row['phases']['data'].get('phases', []) if phase.get('name') == name]
            if observed:
                result['phase_observations'][name] = {
                    'elapsed': describe([phase.get('elapsed_ms') for phase in observed
                                         if phase.get('status') == 'available'], 'milliseconds'),
                    'calls_observed': sorted({phase['calls'] for phase in observed}),
                    'active_calls_observed': sorted({phase['active_calls'] for phase in observed}),
                    'unavailable_samples': sum(phase['status'] == 'unavailable' for phase in observed)}
        for name, unit in COUNTER_UNITS.items():
            values = [counter['value'] for row in rows if isinstance(row.get('phases', {}).get('data'), dict)
                      for counter in row['phases']['data'].get('counters', []) if counter['name'] == name]
            result['counter_observations'][name] = {'status': 'observed' if values else 'unavailable',
                'samples': len(values), 'unit': unit, 'distinct_values': sorted(set(values)),
                'constant_observed': len(set(values)) == 1 if values else None}
        result['parent_resource_chronology'] = [
            {'command_id': row.get('command_id'), 'repetition': row.get('repetition'),
             'before_unix_ns': row.get('parent_before', {}).get('unix_ns'),
             'after_unix_ns': row.get('parent_after', {}).get('unix_ns'),
             'before_peak_rss_bytes': row.get('parent_before', {}).get('peak_rss_bytes'),
             'after_peak_rss_bytes': row.get('parent_after', {}).get('peak_rss_bytes'),
             'before_current_rss_bytes': row.get('parent_before', {}).get('current_rss_bytes'),
             'after_current_rss_bytes': row.get('parent_after', {}).get('current_rss_bytes'),
             'before_physical_footprint_bytes': row.get('parent_before', {}).get('physical_footprint_bytes'),
             'after_physical_footprint_bytes': row.get('parent_after', {}).get('physical_footprint_bytes'),
             'before_allocated_blocks': row.get('parent_before', {}).get('allocated_blocks'),
             'after_allocated_blocks': row.get('parent_after', {}).get('allocated_blocks'),
             'gc_before': row.get('parent_before', {}).get('gc_counts'),
             'gc_after': row.get('parent_after', {}).get('gc_counts')} for row in rows]
        summary.append(result)
    return {'scope': 'Descriptive within-artifact, within-slot fixed chronology. No cross-artifact ratios or performance verdicts.',
            'phase_scope': 'Opt-in inclusive phases overlap. State/host/parent snapshots and output verification are outside fixture clocks.',
            'memory_scope': 'Fresh CLI process per command. Child timer peak RSS and Python-parent lifetime peak RSS cannot establish a leak. '
                            'The diagnostic parent intentionally retains this bounded report; its allocation/RSS growth includes observer records.',
            'cohorts': summary}


def persist(report, output):
    output.parent.mkdir(parents=True, exist_ok=True)
    report['volatility'] = volatility_summary(report['commands'])
    payload = json.dumps(report, indent=2, allow_nan=False) + '\n'
    if len(payload.encode('utf-8')) > MAX_PROFILE_BYTES:
        raise ValueError('Diagnostic report exceeds the finite 16-MiB output limit')
    benchmark.atomic_text(output, payload)
    lines = ['# Separate runtime diagnostics', '', 'Diagnostic status `' + report['status'] + '`.', '',
             'Primary outcome `' + str(report.get('primary', {}).get('status', 'unavailable')) + '` remains unchanged.', '',
             'Wrappers and opt-in logs change command timing. No ratios, performance verdicts or speedup conclusions are computed.', '',
             '| Case | Phase | Slot | Artifact | Command status | Parent seconds | Resource status | Phase status |',
             '| --- | --- | --- | --- | --- | ---: | --- | --- |']
    for row in report['commands']:
        seconds = row.get('parent_seconds')
        elapsed = f'{seconds:.6f}' if seconds is not None else 'unavailable'
        lines.append(f"| {row['case']} | {row['phase']} | {row['slot']} | {row['label']} | {row['status']} | {elapsed} | "
                     f"{row.get('resources', {}).get('status', 'unavailable')} | {row.get('phases', {}).get('status', 'unavailable')} |")
    lines.extend(['', 'System timer accounting is platform-specific and is not a whole-job memory claim.',
                  'Host snapshots occur outside commands. Phase JSON is optional and untrusted.',
                  'Phases overlap and are inclusive completed spans, not a partition to sum.', ''])
    lines.extend(['## Within-cell volatility', '', report['volatility']['scope'], '',
                  '| Case | Phase | Slot | Artifact | Samples | Parent median ms | Parent min/max ms | Parent MAD % | Child RSS min/max MiB |',
                  '| --- | --- | --- | --- | ---: | ---: | --- | ---: | --- |'])
    for row in report['volatility']['cohorts']:
        timing, rss = row['parent_stages']['parent_seconds'], row['child_peak_rss']
        center = f"{timing['median'] * 1000:.3f}" if timing['status'] == 'observed' else 'unavailable'
        bounds = f"{timing['minimum'] * 1000:.3f}/{timing['maximum'] * 1000:.3f}" if timing['status'] == 'observed' else 'unavailable'
        mad = timing.get('relative_median_absolute_deviation')
        noise = f'{mad * 100:.2f}' if mad is not None else 'unavailable'
        memory = f"{rss['minimum'] / 1024 ** 2:.3f}/{rss['maximum'] / 1024 ** 2:.3f}" if rss['status'] == 'observed' else 'unavailable'
        lines.append(f"| {row['case']} | {row['phase']} | {row['slot']} | {row['label']} | {timing['samples']} | {center} | {bounds} | {noise} | {memory} |")
    lines.extend(['', '## Cache instruction routes', '',
                  'Fixed aggregate counters show observed route variants. Missing counters remain unavailable for legacy or absent protocols.', '',
                  '| Phase | Slot | Artifact | Dependencies | Metadata checks | Metadata matches | Content/effective fallbacks | Memo hits/misses |',
                  '| --- | --- | --- | --- | --- | --- | --- | --- |'])
    for row in report['volatility']['cohorts']:
        if row['case'] != 'warm-build-cache' or row['phase'] == 'launch_only_control':
            continue
        def observed(name):
            values = row['counter_observations'][name]['distinct_values']
            return ','.join(map(str, values)) if values else 'unavailable'
        lines.append(f"| {row['phase']} | {row['slot']} | {row['label']} | {observed('state_dependencies')} | {observed('metadata_checks')} | "
                     f"{observed('metadata_matches')} | {observed('content_hash_fallbacks')}/{observed('effective_hash_fallbacks')} | "
                     f"{observed('freshness_memo_hits')}/{observed('freshness_memo_misses')} |")
    lines.extend(['', '## Cache and aligned startup stages', '',
                  'Each row describes one artifact and slot. Resource wrappers change these commands. No subtraction or comparison verdict is computed.', '',
                  '| Phase | Slot | Artifact | Stage | Samples | Median ms | Min/max ms | MAD % |',
                  '| --- | --- | --- | --- | ---: | ---: | --- | ---: |'])
    for row in report['volatility']['cohorts']:
        if row['case'] != 'warm-build-cache' or row['phase'] == 'initialization':
            continue
        stages = dict(row['parent_stages'])
        if row['phase'] == 'measurement':
            stages.update({name: observed['elapsed'] for name, observed in row['phase_observations'].items() if name in
                ('build_state_read', 'build_state_parse', 'input_metadata', 'input_content_hash', 'input_effective_hash',
                 'input_freshness_memo', 'lookup_session_reset', 'cli_report_serialization', 'build_mode_key',
                 'cli_parse', 'cli_config_setup')})
        for name, value in stages.items():
            factor = 1000 if value['unit'] == 'seconds' else 1
            center = f"{value['median'] * factor:.4f}" if value['status'] == 'observed' else 'unavailable'
            bounds = f"{value['minimum'] * factor:.4f}/{value['maximum'] * factor:.4f}" if value['status'] == 'observed' else 'unavailable'
            mad = value.get('relative_median_absolute_deviation')
            noise = f'{mad * 100:.2f}' if mad is not None else 'unavailable'
            lines.append(f"| {row['phase']} | {row['slot']} | {row['label']} | {name} | {value['samples']} | {center} | {bounds} | {noise} |")
    lines.extend(['', '## Parent resource chronology', '',
                  '| Case | Phase | Slot | Artifact | First/last peak RSS MiB | First/last current RSS MiB | First/last allocated blocks |',
                  '| --- | --- | --- | --- | --- | --- | --- |'])
    for row in report['volatility']['cohorts']:
        chronology = row['parent_resource_chronology']
        first, last = chronology[0], chronology[-1]
        rss = [item['after_peak_rss_bytes'] for item in (first, last)]
        memory = '/'.join(f'{value / 1024 ** 2:.3f}' for value in rss) if all(finite_number(value) for value in rss) else 'unavailable'
        current_rss = [item['after_current_rss_bytes'] for item in (first, last)]
        current = '/'.join(f'{value / 1024 ** 2:.3f}' for value in current_rss) if all(finite_number(value) for value in current_rss) else 'unavailable'
        blocks = '/'.join(str(item['after_allocated_blocks']) for item in (first, last))
        lines.append(f"| {row['case']} | {row['phase']} | {row['slot']} | {row['label']} | {memory} | {current} | {blocks} |")
    lines.extend(['', report['volatility']['phase_scope'], '', report['volatility']['memory_scope'], ''])
    lines.extend('- ' + error.replace('\n', ' ') for error in report['errors'])
    benchmark.atomic_text(output.with_suffix('.md'), '\n'.join(lines) + '\n')


def parser():
    result = argparse.ArgumentParser(description=__doc__)
    for name in ('baseline', 'candidate', 'metadata', 'primary-report', 'output'):
        result.add_argument('--' + name, type=Path, required=True)
    result.add_argument('--repeats', type=int, default=8, help='Fixed chronological diagnostic repetitions, 1 to 16')
    result.add_argument('--timeout', type=float, default=30, help='Per-command bound, (0, 30] seconds')
    result.add_argument('--budget', type=float, default=600, help='Whole diagnostic bound, (0, 600] seconds')
    return result


def main(argv=None):
    argument_parser = parser()
    args = argument_parser.parse_args(argv)
    if os.name != 'posix' or platform.system() not in ('Darwin', 'Linux'):
        argument_parser.error('Requires owned POSIX process groups on macOS or Linux')
    if not 1 <= args.repeats <= 16 or any(not math.isfinite(getattr(args, name))
            or not 0 < getattr(args, name) <= limit for name, limit in (('timeout', 30), ('budget', 600))):
        argument_parser.error('Requires repeats 1..16 and finite positive bounded timeout/budget')
    sources = {label: getattr(args, label).resolve() for label in benchmark.LABELS}
    args.metadata, args.primary_report, args.output = args.metadata.resolve(), args.primary_report.resolve(), args.output.resolve()
    verifiers = {name: shutil.which(name) for name in ('pdftotext', 'pdfinfo', 'pdfimages')}
    observation_tools = [Path(value) for value in ('/usr/bin/time', '/usr/bin/vm_stat', '/usr/sbin/sysctl', '/usr/bin/top')
                         if Path(value).is_file()]
    primary_paths = [args.primary_report]
    if args.primary_report.with_suffix('.md').exists():
        primary_paths.append(args.primary_report.with_suffix('.md').resolve())
    try:
        primary_hint = read_json(args.primary_report)
    except (OSError, ValueError, RecursionError):
        primary_hint = {}
    journal_hint_error = None
    try:
        journal_hint_path = primary_journal_path(primary_hint, args.primary_report)
        if journal_hint_path is not None:
            primary_paths.append(journal_hint_path)
    except (OSError, ValueError) as error:
        journal_hint_error = str(error)
    protected_paths = [*sources.values(), args.metadata, *primary_paths, *observation_tools,
                       *(Path(value).resolve() for value in verifiers.values() if value)]
    if args.output.suffix.lower() != '.json':
        argument_parser.error('Output must have a .json extension')
    for output in (args.output, args.output.with_suffix('.md')):
        for path in protected_paths:
            if output == path or output.exists() and path.exists() and output.samefile(path):
                argument_parser.error('Diagnostic output cannot overwrite a source, primary or metadata alias')
    started = time.monotonic()
    report = {'schema_version': 2, 'status': 'incomplete', 'diagnostic_only': True, 'used_for_gate': False,
              'primary': {'path': str(args.primary_report), 'status': 'unavailable'}, 'executables': [],
              'policy': {'repeats': args.repeats, 'timeout_seconds': args.timeout, 'budget_seconds': args.budget,
                         'phase_opt_in': 'TEKAI_DIAGNOSTIC_PROFILE=1',
                         'sampling': 'Fixed chronological repetitions. No adaptive retries, outlier removal or result selection.',
                         'launch_only_control': 'Same copied artifact --version immediately before each measured warm-cache command.',
                         'same_artifact_cache_control': 'Each artifact has two content-equivalent independently initialized fixture replicas, one per slot.',
                         'comparison': 'None. Instrumented commands cannot rescore the primary gate.'},
              'machine': {'system': platform.system(), 'architecture': platform.machine(),
                          'logical_cpu_count': os.cpu_count(), 'python': platform.python_version()},
              'units': [], 'commands': [], 'errors': []}
    persist(report, args.output)
    protected, supervisor = {}, None
    previous_handler = signal.getsignal(signal.SIGTERM)

    def terminate(_number, _frame):
        raise KeyboardInterrupt

    signal.signal(signal.SIGTERM, terminate)
    try:
        for path in (*primary_paths, args.metadata):
            protected[path] = benchmark.executable_sha256(path)
        primary = read_json(args.primary_report)
        primary_status = primary.get('status')
        report['primary'].update(status=primary_status if primary_status in ('pass', 'fail', 'inconclusive', 'error', 'incomplete') else 'unknown',
                                 exit_code=primary.get('exit_code') if type(primary.get('exit_code')) is int
                                 and 0 <= primary['exit_code'] <= 130 else None,
                                 sha256_start=protected[args.primary_report])
        if journal_hint_error is not None:
            raise ValueError(journal_hint_error)
        journal = primary.get('raw_command_journal')
        if journal is not None:
            journal_path = primary_journal_path(primary, args.primary_report)
            if protected.get(journal_path) != journal['sha256'] or journal_path.stat().st_size != journal['bytes']:
                raise ValueError('Primary raw command journal differs from its exact byte/hash binding')
            report['primary']['raw_command_journal'] = dict(journal)
        # The primary loader validates required build fields; also reject
        # nonfinite optional metadata before it can poison the diagnostic JSON.
        read_json(args.metadata)
        metadata = benchmark.load_metadata(args.metadata)
        report['metadata'] = metadata
        source_hashes = bind_primary(primary, metadata, sources)
        protected.update({sources[label]: source_hashes[label] for label in benchmark.LABELS})
        if any(not path.is_file() or not os.access(path, os.X_OK) for path in sources.values()):
            raise ValueError('Selected source executables are unavailable')
        if not all(verifiers.values()):
            raise ValueError('Poppler output verification tools are required')
        report['output_verifiers'] = {name: {'path': value, 'sha256_start': benchmark.executable_sha256(Path(value))}
                                      for name, value in verifiers.items()}
        for row in report['output_verifiers'].values():
            protected[Path(row['path']).resolve()] = row['sha256_start']
        report['observation_tools'] = []
        for path in observation_tools:
            protected[path] = benchmark.executable_sha256(path)
            report['observation_tools'].append({'path': str(path), 'sha256_start': protected[path]})
        for path in (args.output, args.output.with_suffix('.md')):
            if path in protected or any(path.exists() and source.exists() and path.samefile(source) for source in protected):
                raise ValueError('Diagnostic output aliases a selected output verifier')
        with tempfile.TemporaryDirectory(prefix='tekai-runtime-profile-') as temporary:
            work = Path(temporary).resolve()
            supervisor = ProfileSupervisor(work, args.timeout, started + args.budget)
            host_env = benchmark.isolated_environment(work / 'host-observers')
            supervisor.probe_timer(dict(host_env, LC_ALL='C'))
            report['timer'] = {'kind': supervisor.kind, 'unavailable_reason': supervisor.timer_reason}
            binaries = {slot: {} for slot in benchmark.SLOTS}
            try:
                for cell in benchmark.cell_order():
                    supervisor.remaining()
                    label, slot, replica = cell['label'], cell['slot'], cell['replica']
                    destination = work / slot / replica / 'bin/tekai'
                    destination.parent.mkdir(parents=True)
                    row = {**cell, 'source_path': str(sources[label]), 'isolated_path': str(destination),
                           'source_sha256_start': source_hashes[label], 'sha256_start': None}
                    report['executables'].append(row)
                    shutil.copy2(sources[label], destination)
                    row['sha256_start'] = benchmark.executable_sha256(destination)
                    if row['sha256_start'] != source_hashes[label]:
                        raise ValueError('Diagnostic isolated copy differs from the primary source')
                    binaries[slot][label] = destination
                for case_index, case in enumerate(benchmark.CASES):
                    fixtures = {}
                    for cell in benchmark.cell_order(case_index):
                        label, slot, replica = cell['label'], cell['slot'], cell['replica']
                        fixture = benchmark.make_fixture(work / slot / replica / case, binaries[slot][label], case)
                        fixture.update(verifiers, **cell)
                        fixture['expected_version'] = metadata[label]['expected_version']
                        fixtures[(slot, label)] = fixture
                    hashes = {fixture['input_sha256'] for fixture in fixtures.values()}
                    if len(hashes) != 1:
                        raise ValueError('Diagnostic cell input content differs')
                    for repetition in range(-1, args.repeats):
                        unit = {'case': case, 'repetition': repetition,
                                'phase': 'initialization' if repetition < 0 else 'measurement',
                                'unit_id': len(report['units']) + 1,
                                'before': host_snapshot(supervisor, host_env), 'command_indexes': []}
                        unit['before'].update(unit_id=unit['unit_id'], position='before')
                        report['units'].append(unit)
                        completed_unit = False
                        try:
                            for cell in benchmark.cell_order((case_index + repetition + 1) % 2):
                                fixture = fixtures[(cell['slot'], cell['label'])]
                                if case == 'warm-build-cache' and repetition >= 0:
                                    control = {**fixture, 'command': [binaries[cell['slot']][cell['label']], '--version']}
                                    control_row = {**cell, 'case': case, 'phase': 'launch_only_control',
                                        'repetition': repetition, 'unit_id': unit['unit_id'],
                                        'sequence_index': len(report['commands']),
                                        'same_artifact_sha256': source_hashes[cell['label']],
                                        'phase_opt_in_note': 'Clap --version exits before Rust diagnostic emission; absent phases are expected.',
                                        'command': list(map(str, control['command'])),
                                        'input_sha256': fixture['input_sha256'], 'status': 'not_started',
                                        'output_validation': None}
                                    report['commands'].append(control_row)
                                    unit['command_indexes'].append(len(report['commands']) - 1)
                                    supervisor.active = control_row
                                    try:
                                        control_result = execute_profile_fixture(supervisor, control, 'cli-startup')
                                        control_row.update(status='verified', output_validation=control_result['output_validation'])
                                    except (OSError, ValueError, RuntimeError, subprocess.SubprocessError, KeyboardInterrupt) as error:
                                        control_row.update(status='error', error=f'{type(error).__name__}: {error}'[-8192:])
                                        raise
                                    finally:
                                        supervisor.active = None
                                        persist(report, args.output)
                                row = {**cell, 'case': case, 'phase': unit['phase'], 'repetition': repetition,
                                       'unit_id': unit['unit_id'], 'sequence_index': len(report['commands']),
                                       'command': list(map(str, fixture['command'])), 'input_sha256': fixture['input_sha256'],
                                       'status': 'not_started', 'output_validation': None}
                                row['phase_opt_in_note'] = 'Clap --version exits before Rust diagnostic emission; absent phases are expected.' \
                                    if case == 'cli-startup' else 'Native engine entry bypasses Rust CLI instrumentation; absent phases are expected.' \
                                    if case in ('nested-lookup', 'image-compile') else \
                                    'Warm-build instrumentation is opt-in; selected baseline may have an older or absent protocol.'
                                if case == 'warm-build-cache' and repetition >= 0:
                                    row['cache_state_before'] = benchmark.cache_state_snapshot(fixture)
                                report['commands'].append(row)
                                unit['command_indexes'].append(len(report['commands']) - 1)
                                supervisor.active = row
                                try:
                                    result = execute_profile_fixture(supervisor, fixture, case, initializing=repetition < 0)
                                    row['output_validation'] = result['output_validation']
                                    row['status'] = 'verified'
                                    if 'reported_build_timing' in result:
                                        row['reported_build_timing'] = result['reported_build_timing']
                                    if case == 'warm-build-cache':
                                        row['cache_state_after'] = benchmark.cache_state_snapshot(fixture)
                                        row['cache_state_unchanged'] = None if repetition < 0 else row['cache_state_before'] == row['cache_state_after']
                                        row['cache_state_scope'] = 'Bounded state bytes/hash/input inventory and file identity, outside fixture command clocks.'
                                except (OSError, ValueError, RuntimeError, subprocess.SubprocessError, KeyboardInterrupt) as error:
                                    row.update(status='error', error=f'{type(error).__name__}: {error}'[-8192:])
                                    raise
                                finally:
                                    supervisor.active = None
                                    persist(report, args.output)
                            completed_unit = True
                        finally:
                            unit['after'] = host_snapshot(supervisor, host_env) if completed_unit else {
                                'unix_ns': time.time_ns(), 'monotonic_ns': time.monotonic_ns(),
                                'unavailable': {'host': 'Unit incomplete; no further observer commands launched'}}
                            unit['after'].update(unit_id=unit['unit_id'], position='after')
                            persist(report, args.output)
                    for fixture in fixtures.values():
                        if benchmark.fixture_sha256(fixture['project']) != fixture['input_sha256']:
                            raise ValueError('Diagnostic compiler changed fixture inputs')
            finally:
                try:
                    supervisor.close()
                except (OSError, ValueError, RuntimeError, subprocess.SubprocessError) as error:
                    report['errors'].append('Diagnostic cleanup failed: ' + str(error)[-8192:])
                for row in report['executables']:
                    try:
                        row['sha256_end'] = benchmark.executable_sha256(Path(row['isolated_path']))
                    except OSError:
                        row['sha256_end'] = None
                    row['unchanged'] = row['sha256_start'] is not None and row['sha256_start'] == row['sha256_end']
                    if not row['unchanged']:
                        report['errors'].append('Diagnostic executable copy changed or disappeared')
    except (OSError, ValueError, RuntimeError, subprocess.SubprocessError) as error:
        report['errors'].append(f'{type(error).__name__}: {error}'[-8192:])
    except KeyboardInterrupt:
        report['errors'].append('Diagnostic run interrupted; primary outcome remains unchanged')
    finally:
        if supervisor is not None:
            try:
                supervisor.close()
            except (OSError, ValueError, RuntimeError, subprocess.SubprocessError) as error:
                report['errors'].append('Diagnostic cleanup failed: ' + str(error)[-8192:])
        for path, expected in protected.items():
            try:
                current = benchmark.executable_sha256(path)
            except OSError:
                current = None
            if path == args.primary_report:
                report['primary'].update(sha256_end=current, unchanged=expected == current)
            report.setdefault('protected_inputs', []).append(
                {'path': str(path), 'sha256_start': expected, 'sha256_end': current, 'unchanged': expected == current})
            if current != expected:
                report['errors'].append('Protected primary/metadata/source/verifier changed or disappeared: ' + str(path))
        signal.signal(signal.SIGTERM, previous_handler)
        report['status'] = 'error' if report['errors'] else 'complete'
        report['elapsed_seconds'] = time.monotonic() - started
        report['exit_code'] = int(bool(report['errors']))
        persist(report, args.output)
    return report['exit_code']


if __name__ == '__main__':
    sys.exit(main())
