#!/usr/bin/env python3
"""Separate, bounded runtime diagnostics. Never changes the primary verdict.

System time wrappers and optional binary phase logs change the measured command.
Their observations cannot be used as replacement samples or speedup evidence.
"""

import argparse
import gc
import hashlib
import json
import math
import os
from pathlib import Path
import platform
import re
import resource
import shutil
import signal
import stat
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
IMAGE_ROUTE_PAIRS = 8
MAX_CACHE_ENTRIES = 32768
MAX_CACHE_BYTES = 512 * 1024 * 1024
PUBLIC_ENGINE_ROUTE = '__tekai-engine'
PRIVATE_ENGINE_ROUTE = '__tekai-engine-worker'
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
PHASE_SOURCES_V2 = {**PHASE_SOURCES_V1,
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
PHASE_SOURCES_V3 = {**PHASE_SOURCES_V2,
    'embedded_engine_entry': 'exact engine child, outside parent process instrumentation',
    'format_initialization': 'exact engine child, outside parent process instrumentation',
}
PHASE_SOURCES_V4 = {**PHASE_SOURCES_V3,
    'build_state_parse': 'compiler build-state JSON parse',
}
PHASE_SOURCES = PHASE_SOURCES_V4
COUNTER_UNITS = {'state_bytes_read': 'bytes', 'state_dependencies': 'recorded_inputs',
    'freshness_requests': 'requests', 'freshness_memo_hits': 'requests', 'freshness_memo_misses': 'requests',
    'metadata_checks': 'calls', 'metadata_matches': 'freshness_routes',
    'content_hash_fallbacks': 'freshness_routes', 'effective_hash_fallbacks': 'freshness_routes',
    'virtual_freshness_checks': 'freshness_routes', 'stale_inputs': 'observed_stale_inputs', 'warm_cache_hits': 'builds'}
NATIVE_REASONS_V2 = {
    'embedded_engine_entry': 'native_engine_exits_without_returning_to_Rust',
    'format_initialization': 'not_separately_instrumented_inside_native_engine',
}
CHILD_REASONS = {
    'embedded_engine_entry': 'engine_child_process_outside_parent_instrumentation',
    'format_initialization': 'not_separately_instrumented_inside_engine_child',
}
ENGINE_PHASE_NAMES_V1 = ('owner_construction', 'invocation_setup', 'engine_body', 'cleanup',
                         'owner_drop', 'png_decode', 'pdf_compression')
ENGINE_PHASE_NAMES_V2 = (*ENGINE_PHASE_NAMES_V1, 'format_load', 'png_write')
ENGINE_PHASE_NAMES_BY_VERSION = {1: ENGINE_PHASE_NAMES_V1, 2: ENGINE_PHASE_NAMES_V2}
ENGINE_PHASE_NAMES = ENGINE_PHASE_NAMES_V2
ENGINE_COUNTER_UNITS = {'decode_calls': 'calls', 'cache_hits': 'hits', 'decoded_bytes': 'bytes',
                        'compression_calls': 'calls', 'compression_input_bytes': 'bytes',
                        'compression_output_bytes': 'bytes'}
MAX_ENGINE_PHASE_BYTES = 4096
CASE_ANALYSIS_STATUSES = ('pass', 'fail', 'inconclusive', 'error', 'incomplete')


def record_command_error(row, error):
    row.update(status='error', error=f'{type(error).__name__}: {error}'[-8192:])
    for name in ('failed_command', 'fixture_context'):
        value = getattr(error, name, None)
        if value is not None:
            row[name] = value


def diagnostic_case_plan(primary):
    """Freeze an allowlisted plan before any diagnostic command is launched."""
    evidence = {case: [] for case in benchmark.CASES}
    results = primary.get('results', [])
    if not isinstance(results, list) or len(results) > len(benchmark.CASES):
        raise ValueError('Primary case results are not a bounded allowlisted inventory')
    seen = set()
    for row in results:
        case = row.get('case') if isinstance(row, dict) else None
        if type(case) is not str or case not in evidence or case in seen:
            raise ValueError('Primary case results contain an unknown or duplicate case')
        seen.add(case)
        analysis = row.get('analysis')
        status = analysis.get('status') if isinstance(analysis, dict) else 'incomplete' if analysis is None else None
        if type(status) is not str or status not in CASE_ANALYSIS_STATUSES:
            raise ValueError('Primary case analysis status is missing or unknown')
        evidence[case].append({'source': 'results.analysis' if analysis is None else 'results.analysis.status',
                               'status': status})
    diagnostics = primary.get('failure_diagnostics', [])
    if not isinstance(diagnostics, list) or len(diagnostics) > len(benchmark.CASES):
        raise ValueError('Primary failure diagnostics are not a bounded allowlisted inventory')
    for row in diagnostics:
        context = row.get('fixture_context') if isinstance(row, dict) else None
        if context is None:
            continue
        case = context.get('case') if isinstance(context, dict) else None
        if type(case) is not str or case not in evidence:
            raise ValueError('Primary failure context contains an unknown case')
        if primary['status'] in ('error', 'incomplete'):
            evidence[case].append({'source': 'failure_diagnostics.fixture_context.case',
                                   'status': primary['status']})
    entries = []
    for index, case in enumerate(benchmark.CASES):
        priority = any(row['status'] != 'pass' for row in evidence[case])
        entries.append({'case': case, 'original_case_index': index, 'prioritized': priority,
                        'selection_reason': 'primary_case_requires_diagnosis' if priority else 'remaining_original_order',
                        'primary_evidence': evidence[case]})
    ordered = [row for row in entries if row['prioritized']] + [row for row in entries if not row['prioritized']]
    return {'status': 'fixed', 'diagnostic_only': True, 'used_for_gate': False,
            'selection_reason': 'primary_case_failure_evidence' if any(row['prioritized'] for row in entries)
                                else 'original_order_no_case_failure_evidence',
            'scope': 'Derived once from the hash-bound primary report before diagnostic commands. '
                     'Only known cases and statuses select priority. Remaining cases keep original order. '
                     'Original case indexes determine fixture and crossover assignments. No adaptive reordering or retries.',
            'order': [row['case'] for row in ordered], 'cases': ordered}


def reject_constant(value):
    raise ValueError('Nonfinite JSON constant ' + value)


def unique_object(items):
    result = {}
    for name, value in items:
        if name in result:
            raise ValueError('Duplicate JSON member ' + name)
        result[name] = value
    return result


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
        if version in (2, 3, 4):
            expected_keys.add('counters')
        if set(data) != expected_keys \
                or type(version) is not int or version not in (1, 2, 3, 4) \
                or data.get('producer') != 'tekai-rust' or data.get('scope') != 'cli_process' \
                or data.get('source') != 'opt_in_rust_instrumentation' or data.get('untrusted') is not True \
                or data.get('completed_spans_only') is not True or data.get('phases_overlap') is not True \
                or data.get('status') not in ('success', 'error', 'exit_failure') \
                or not finite_number(data.get('elapsed_ms')):
            raise ValueError('Unsupported opt-in phase protocol')
        if version == 1 and len(lines[0].encode('utf-8')) + len('TEKAI_PROFILE ') + 1 > 8192:
            raise ValueError('Legacy phase record exceeds its 8-KiB bound')
        phases = data.get('phases')
        sources = {1: PHASE_SOURCES_V1, 2: PHASE_SOURCES_V2,
                   3: PHASE_SOURCES_V3, 4: PHASE_SOURCES_V4}[version]
        reasons = CHILD_REASONS if version in (3, 4) else NATIVE_REASONS_V2
        names = tuple(sources)
        if not isinstance(phases, list) or len(phases) != len(names) \
                or any(not isinstance(phase, dict) for phase in phases) \
                or any(type(phase.get('name')) is not str for phase in phases) \
                or {phase.get('name') for phase in phases} != set(names):
            raise ValueError('Missing or duplicated fixed phase inventory')
        for phase in phases:
            name = phase['name']
            scope = ('engine_child_internal' if version in (3, 4) else 'native_engine_internal') if name in reasons else \
                'subprocess_launch_and_wait' if name == 'tex_subprocess' else 'inclusive_wall_time_current_process'
            if set(phase) != {'name', 'status', 'elapsed_ms', 'calls', 'active_calls', 'source', 'scope', 'reason'} \
                    or phase.get('status') not in ('available', 'unavailable') \
                    or not finite_number(phase.get('calls'), integer=True) \
                    or not finite_number(phase.get('active_calls'), integer=True) \
                    or phase.get('source') != sources[name] or phase.get('scope') != scope:
                raise ValueError('Invalid phase scope, source or call count')
            if phase['status'] == 'available':
                if name in reasons or phase['calls'] == 0 or not finite_number(phase.get('elapsed_ms')) \
                        or phase.get('reason') is not None:
                    raise ValueError('Invalid available phase duration')
            elif phase['calls'] != 0 or phase.get('elapsed_ms') is not None \
                    or phase.get('reason') != reasons.get(name, 'no_completed_call_in_this_process'):
                raise ValueError('Unavailable phase requires null duration and a reason')
        if version in (2, 3, 4):
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
                      compatibility={'schema_version': version, 'granular_observations': 'available' if version in (2, 3, 4)
                                     else 'unavailable_legacy_schema', 'missing_phase_names': [] if version in (2, 3, 4)
                                     else sorted(set(PHASE_SOURCES) - set(PHASE_SOURCES_V1)),
                                     'counters_available': version in (2, 3, 4)})
    except (ValueError, RecursionError, OverflowError) as error:
        result.update(status='invalid', reason=str(error))
    return result


def parse_engine_phases(stderr, truncated=False):
    """Parse one invocation record, independently of parent CLI observations."""
    result = {'status': 'unavailable', 'untrusted': True, 'used_for_gate': False, 'data': None}
    prefix = 'TEKAI_ENGINE_PROFILE '
    if truncated:
        result.update(status='invalid', reason='Engine phase capture is truncated')
        return result
    lines = [line[len(prefix):] for line in stderr.splitlines() if line.startswith(prefix)]
    if not lines:
        result['reason'] = 'Selected binary emitted no opt-in engine record'
        return result
    if len(lines) != 1 or len(lines[0].encode('utf-8')) + len(prefix) + 1 > MAX_ENGINE_PHASE_BYTES:
        result.update(status='invalid', reason='Engine record is duplicated or exceeds its finite limit')
        return result
    try:
        data = json.loads(lines[0], parse_constant=reject_constant, object_pairs_hook=unique_object)
        keys = {'schema_version', 'producer', 'scope', 'source', 'untrusted', 'status', 'elapsed_ns',
                'completed_spans_only', 'phases_overlap', 'lifecycle_partition_on_success',
                'worker_preparation', 'phases', 'counters'}
        if not isinstance(data, dict) or set(data) != keys \
                or type(data.get('schema_version')) is not int or data['schema_version'] not in ENGINE_PHASE_NAMES_BY_VERSION \
                or data.get('producer') != 'tekai-engine-rust' \
                or data.get('scope') != 'engine_invocation' \
                or data.get('source') != 'opt_in_rust_instrumentation' \
                or any(data.get(key) is not True for key in ('untrusted', 'completed_spans_only',
                                                           'phases_overlap', 'lifecycle_partition_on_success')) \
                or data.get('status') not in ('success', 'warnings', 'errors', 'setup_error', 'teardown_error') \
                or not finite_number(data.get('elapsed_ns'), integer=True):
            raise ValueError('Unsupported opt-in engine protocol')
        preparation = data['worker_preparation']
        if preparation is not None and (not isinstance(preparation, dict)
                or set(preparation) != {'scope', 'elapsed_ns'}
                or preparation.get('scope') != 'cli_worker_before_invocation'
                or not finite_number(preparation.get('elapsed_ns'), integer=True)):
            raise ValueError('Invalid separate worker preparation observation')
        phase_names = ENGINE_PHASE_NAMES_BY_VERSION[data['schema_version']]
        phases = data['phases']
        if not isinstance(phases, list) or len(phases) != len(phase_names) \
                or any(not isinstance(phase, dict) or type(phase.get('name')) is not str for phase in phases) \
                or {phase['name'] for phase in phases} != set(phase_names):
            raise ValueError('Missing or duplicated fixed engine phase inventory')
        for phase in phases:
            if set(phase) != {'name', 'status', 'calls', 'failed_calls', 'elapsed_ns'} \
                    or phase.get('status') not in ('available', 'unavailable') \
                    or any(not finite_number(phase.get(key), integer=True) for key in ('calls', 'failed_calls')) \
                    or phase['failed_calls'] > phase['calls']:
                raise ValueError('Invalid engine phase status or call count')
            if phase['status'] == 'available':
                if phase['calls'] == 0 or not finite_number(phase['elapsed_ns'], integer=True):
                    raise ValueError('Available engine phase requires completed calls and duration')
            elif phase['calls'] != 0 or phase['failed_calls'] != 0 or phase['elapsed_ns'] is not None:
                raise ValueError('Unavailable engine phase requires zero calls and null duration')
        lifecycle = [phase for phase in phases if phase['name'] in ENGINE_PHASE_NAMES_V1[:5]]
        if sum(phase['elapsed_ns'] or 0 for phase in lifecycle) > data['elapsed_ns']:
            raise ValueError('Engine lifecycle durations exceed invocation time')
        if data['status'] in ('success', 'warnings'):
            if any(phase['calls'] != 1 or phase['failed_calls'] != 0 for phase in lifecycle) \
                    or sum(phase['elapsed_ns'] for phase in lifecycle) != data['elapsed_ns']:
                raise ValueError('Successful engine invocation requires a complete partitioned lifecycle')
        counters = data['counters']
        if not isinstance(counters, list) or len(counters) != len(ENGINE_COUNTER_UNITS) \
                or any(not isinstance(counter, dict) or type(counter.get('name')) is not str for counter in counters) \
                or {counter['name'] for counter in counters} != set(ENGINE_COUNTER_UNITS):
            raise ValueError('Missing or duplicated fixed engine counter inventory')
        for counter in counters:
            if set(counter) != {'name', 'value', 'unit'} \
                    or counter.get('unit') != ENGINE_COUNTER_UNITS[counter['name']] \
                    or not finite_number(counter.get('value'), integer=True):
                raise ValueError('Invalid engine counter value or unit')
        counts = {counter['name']: counter['value'] for counter in counters}
        calls = {phase['name']: phase['calls'] for phase in phases}
        if counts['decode_calls'] != calls['png_decode'] \
                or counts['compression_calls'] != calls['pdf_compression']:
            raise ValueError('Engine codec counters disagree with completed phase calls')
        result.update(status='reported', data=data,
                      compatibility={'schema_version': data['schema_version'],
                                     'missing_phase_names': [name for name in ENGINE_PHASE_NAMES if name not in phase_names]})
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
        raw_control = row.get('unwrapped_route_control') is True
        selected_env = dict(env) if raw_control else dict(
            env, LC_ALL='C', TEKAI_DIAGNOSTIC_PROFILE='1', TEKAI_ENGINE_PROFILE='1')
        wrapped = [] if raw_control else timer_command(self.kind)
        resource_kind = None if raw_control else self.kind
        stderr, truncated = '', False
        try:
            result = super().execute([*wrapped, *command], cwd, selected_env)
            stderr = result['stderr_tail']
            timing = result.get('parent_timing')
            row.update(status='command_completed', parent_seconds=result['seconds'], parent_timing=timing,
                       command_timing=result.get('command_timing'),
                       command_timing_boundaries=benchmark.TIMING_METADATA['command_timing_fields'],
                       parent_timing_scope='Capture setup through blocking WNOWAIT exit observation, before owned-process cleanup. '
                                           + ('No resource wrapper or profiling opt-in.' if raw_control else 'Includes the resource wrapper.'),
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
            record_command_error(row, error)
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
                       resources=parse_resources(stderr, resource_kind, truncated),
                       phases=parse_phases(stderr, truncated),
                       engine_phases=parse_engine_phases(stderr, truncated), stderr_tail=stderr)
            before, after = row['parent_before'], row['parent_after']
            row['parent_resource_delta'] = {name: after[name] - before[name] for name in
                ('user_cpu_seconds', 'system_cpu_seconds', 'minor_faults', 'major_faults',
                 'voluntary_context_switches', 'involuntary_context_switches') if name in before and name in after}
            row['parent_resource_delta_scope'] = 'Observer work plus command supervision between parent snapshots; not child CPU accounting.'
            if raw_control:
                row['resources']['reason'] = 'Unwrapped route control has no system resource timer'
            elif self.kind is None:
                row['resources']['reason'] = self.timer_reason


def image_route_plan():
    """Fixed candidate-only pairs balance route and replica launch order."""
    pairs = []
    for repetition in range(IMAGE_ROUTE_PAIRS):
        slots = benchmark.SLOTS if repetition % 2 == 0 else tuple(reversed(benchmark.SLOTS))
        for ordinal, slot in enumerate(slots):
            routes = ('public', 'worker') if (repetition + benchmark.SLOTS.index(slot)) % 2 == 0 else ('worker', 'public')
            pairs.append({'repetition': repetition, 'slot': slot, 'replica_ordinal': ordinal, 'routes': list(routes)})
    return pairs


def image_cache_snapshot(supervisor, fixture):
    """Complete bounded native metadata inventory, outside command clocks."""
    rows, files, size = [], 0, 0
    root = fixture['root'].resolve()
    for kind in ('ENGINE', 'FORMAT', 'AUX', 'BIBTEX'):
        path = Path(fixture['env']['TEKAI_' + kind + '_CACHE'])
        if not path.is_absolute() or path.is_symlink() or not path.is_dir() \
                or path.resolve() != root / ('cache-' + kind.lower()):
            raise ValueError('Image control cache is outside its owned fixture root')
        pending = [(path, 0)]
        while pending:
            current, depth = pending.pop()
            if len(rows) >= MAX_CACHE_ENTRIES or depth > 64:
                raise ValueError('Image cache inventory exceeds its finite entry/depth bound')
            if len(rows) % 256 == 0:
                supervisor.remaining()
            info = current.lstat()
            directory = stat.S_ISDIR(info.st_mode)
            if not directory and not stat.S_ISREG(info.st_mode):
                raise ValueError('Image cache inventory contains a symlink or special file')
            relative = current.relative_to(root).as_posix()
            if len(relative.encode('utf-8')) > 4096:
                raise ValueError('Image cache inventory path exceeds its finite bound')
            rows.append([relative, stat.S_IMODE(info.st_mode), directory, info.st_size,
                         info.st_dev, info.st_ino, info.st_mtime_ns, info.st_ctime_ns])
            if directory:
                # Bound pending work before allocating a complete listing.
                with os.scandir(current) as children:
                    for child in children:
                        if len(rows) + len(pending) >= MAX_CACHE_ENTRIES:
                            raise ValueError('Image cache inventory exceeds its finite entry bound')
                        pending.append((Path(child.path), depth + 1))
            else:
                files += 1
                size += info.st_size
                if size > MAX_CACHE_BYTES:
                    raise ValueError('Image cache inventory exceeds its finite byte bound')
    rows.sort()
    digest = hashlib.sha256(json.dumps(rows, separators=(',', ':'), allow_nan=False).encode()).hexdigest()
    return {'inventory_sha256': digest, 'entries': len(rows), 'files': files, 'file_bytes': size,
            'used_for_gate': False,
            'scope': 'All four owned cache roots, complete paths/kinds/modes/native inode/change-time/mtime/size metadata. '
                     'Not cache-content hashes. Observed outside command clocks.'}


def image_route_completion(report):
    control = report['image_route_controls']
    units = [unit for unit in report['units'] if unit.get('phase') == 'image_route_control']
    counts = {slot: {route: 0 for route in ('public', 'worker')} for slot in benchmark.SLOTS}
    first = {slot: {route: 0 for route in ('public', 'worker')} for slot in benchmark.SLOTS}
    replicas = {slot: {0: 0, 1: 0} for slot in benchmark.SLOTS}
    for unit in units:
        if not unit.get('complete'):
            continue
        first[unit['slot']][unit['routes'][0]] += 1
        replicas[unit['slot']][unit['replica_ordinal']] += 1
    rows = [row for row in report['commands'] if row.get('unwrapped_route_control')]
    for row in rows:
        if row['status'] == 'verified':
            counts[row['slot']][row['route']] += 1
    complete = len(units) == len(image_route_plan()) and all(unit.get('complete') for unit in units)
    balance = complete and all(set(values.values()) == {IMAGE_ROUTE_PAIRS} for values in counts.values()) \
        and all(set(values.values()) == {IMAGE_ROUTE_PAIRS // 2} for values in (*first.values(), *replicas.values()))
    control['completion'] = {'planned_pairs': len(image_route_plan()), 'started_pairs': len(units),
        'complete_pairs': sum(unit.get('complete', False) for unit in units), 'complete': complete,
        'completed_route_counts': counts, 'first_route_counts': first, 'replica_ordinal_counts': replicas,
        'equal_completed_balance': balance, 'used_for_gate': False}
    control['observations'] = [{
        'slot': slot, 'route': route, 'used_for_gate': False,
        'seconds': describe([row.get('parent_seconds') for row in rows if row['slot'] == slot and row['route'] == route
                             and row['status'] == 'verified'], 'seconds'),
        'cleanup_seconds_unscored': describe([row.get('cleanup_seconds') for row in rows
                                              if row['slot'] == slot and row['route'] == route
                                              and row['status'] == 'verified'], 'seconds')}
        for slot in benchmark.SLOTS for route in ('public', 'worker')]


def run_image_route_controls(report, supervisor, fixtures, metadata, source, source_hash, output):
    control = report['image_route_controls']
    control.update(status='incomplete', candidate_revision=metadata['candidate']['revision'],
                   source_path=str(source), binary_sha256=source_hash, planned_pairs=image_route_plan())
    snapshots = {}
    selected = {}
    try:
        for slot in benchmark.SLOTS:
            fixture = fixtures[(slot, 'candidate')]
            command = list(map(str, fixture['command']))
            if fixture['label'] != 'candidate' or len(command) < 3 or command[1] != PUBLIC_ENGINE_ROUTE \
                    or '-no-shell-escape' not in command:
                raise ValueError('Image route controls require the original trusted public image command')
            selected[slot] = fixture
            snapshots[slot] = image_cache_snapshot(supervisor, fixture)
            if not snapshots[slot]['files']:
                raise ValueError('Image route controls require initialized disk caches')
        control['prepared_cache_snapshots'] = snapshots
        for pair in image_route_plan():
            fixture = selected[pair['slot']]
            unit = {**pair, 'case': 'image-compile', 'phase': 'image_route_control',
                    'unit_id': len(report['units']) + 1, 'command_indexes': [], 'complete': False}
            report['units'].append(unit)
            try:
                for route in pair['routes']:
                    command = list(fixture['command'])
                    command[1] = PUBLIC_ENGINE_ROUTE if route == 'public' else PRIVATE_ENGINE_ROUTE
                    row = {key: fixture[key] for key in ('label', 'slot', 'replica')}
                    row.update(case='image-compile', phase='image_route_' + route + '_control', route=route,
                        repetition=pair['repetition'], unit_id=unit['unit_id'], sequence_index=len(report['commands']),
                        command=list(map(str, command)), input_sha256=fixture['input_sha256'], status='not_started',
                        output_validation=None, unwrapped_route_control=True,
                        candidate_revision=metadata['candidate']['revision'], same_artifact_sha256=source_hash,
                        phase_opt_in_note='Original environment without profiling opt-in. Engine phases are not required.',
                        cleanup_scope=benchmark.TIMING_METADATA['cleanup_scope'], cleanup_used_for_measurement=False)
                    report['commands'].append(row)
                    unit['command_indexes'].append(len(report['commands']) - 1)
                    supervisor.active = row
                    try:
                        row['source_sha256_before'] = benchmark.executable_sha256(source)
                        row['binary_sha256_before'] = benchmark.executable_sha256(Path(command[0]))
                        row['fixture_sha256_before'] = benchmark.fixture_sha256(fixture['project'])
                        row['cache_before'] = image_cache_snapshot(supervisor, fixture)
                        if row['source_sha256_before'] != source_hash or row['binary_sha256_before'] != source_hash \
                                or row['fixture_sha256_before'] != fixture['input_sha256'] \
                                or row['cache_before'] != snapshots[pair['slot']]:
                            raise ValueError('Image route source/binary/fixture/cache changed before command')
                        result = execute_profile_fixture(supervisor, {**fixture, 'command': command}, 'image-compile')
                        row['output_validation'] = result['output_validation']
                        row['source_sha256_after'] = benchmark.executable_sha256(source)
                        row['binary_sha256_after'] = benchmark.executable_sha256(Path(command[0]))
                        row['fixture_sha256_after'] = benchmark.fixture_sha256(fixture['project'])
                        row['cache_after'] = image_cache_snapshot(supervisor, fixture)
                        if row['source_sha256_after'] != source_hash or row['binary_sha256_after'] != source_hash \
                                or row['fixture_sha256_after'] != fixture['input_sha256'] \
                                or row['cache_after'] != snapshots[pair['slot']]:
                            raise ValueError('Image route source/binary/fixture/cache changed after command')
                        row['status'] = 'verified'
                    except (OSError, ValueError, RuntimeError, subprocess.SubprocessError, KeyboardInterrupt) as error:
                        record_command_error(row, error)
                        raise
                    finally:
                        supervisor.active = None
                        image_route_completion(report)
                        persist(report, output)
                unit['complete'] = True
            finally:
                image_route_completion(report)
                persist(report, output)
        if not control['completion']['equal_completed_balance']:
            raise ValueError('Image route controls did not complete their fixed balanced plan')
        control['status'] = 'complete'
    except (OSError, ValueError, RuntimeError, subprocess.SubprocessError, KeyboardInterrupt):
        control['status'] = 'error'
        raise
    finally:
        image_route_completion(report)
        persist(report, output)


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
        engines = [row['engine_phases']['data'] for row in rows
                   if row.get('engine_phases', {}).get('status') == 'reported']
        result['engine_phase_protocol_statuses'] = {
            status: sum(row.get('engine_phases', {}).get('status') == status for row in rows)
            for status in ('reported', 'unavailable', 'invalid')}
        result['engine_phase_schema_versions_observed'] = sorted({engine['schema_version'] for engine in engines})
        result['engine_elapsed'] = describe([engine['elapsed_ns'] for engine in engines], 'nanoseconds')
        result['worker_preparation'] = describe([engine['worker_preparation']['elapsed_ns'] for engine in engines
                                                if engine['worker_preparation'] is not None], 'nanoseconds')
        result['engine_phase_observations'] = {}
        for name in ENGINE_PHASE_NAMES:
            phases = [phase for engine in engines for phase in engine['phases'] if phase['name'] == name]
            missing = len(engines) - len(phases)
            result['engine_phase_observations'][name] = {
                'elapsed': describe([phase['elapsed_ns'] for phase in phases if phase['status'] == 'available'], 'nanoseconds'),
                'calls_observed': sorted({phase['calls'] for phase in phases}),
                'failed_calls_observed': sorted({phase['failed_calls'] for phase in phases}),
                'unavailable_samples': missing + sum(phase['status'] == 'unavailable' for phase in phases),
                'missing_protocol_samples': missing}
        result['engine_counter_observations'] = {}
        for name, unit in ENGINE_COUNTER_UNITS.items():
            values = [counter['value'] for engine in engines for counter in engine['counters'] if counter['name'] == name]
            result['engine_counter_observations'][name] = {'status': 'observed' if values else 'unavailable',
                'samples': len(values), 'unit': unit, 'distinct_values': sorted(set(values))}
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
             'Wrappers and opt-in logs change instrumented timing. Unwrapped controls are identified separately. No ratios, performance verdicts or speedup conclusions are computed.', '',
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
    plan = report.get('case_plan')
    if plan is not None:
        lines.extend(['## Fixed diagnostic case order', '', plan['scope'], '',
                      'Selection reason `' + plan['selection_reason'] + '`.', '',
                      '| Case | Original index | Priority | Selection reason | Primary evidence |',
                      '| --- | ---: | --- | --- | --- |'])
        for row in plan['cases']:
            evidence = ', '.join(item['source'] + '=' + item['status'] for item in row['primary_evidence']) or 'unavailable'
            lines.append(f"| {row['case']} | {row['original_case_index']} | {row['prioritized']} | {row['selection_reason']} | {evidence} |")
        lines.extend(['', 'This order is diagnostic-only and unscored. Repeats and per-case assignments are unchanged.', ''])
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
    lines.extend(['## Separate invocation-owned engine phases', '',
        'Opt-in engine records are untrusted diagnostics. Library timing excludes worker preparation. Format loading, PNG writing, PNG decoding and compression are inclusive nested phases and cannot be added to lifecycle costs.',
        'Absent baseline records stay unavailable. This section cannot change a primary verdict.', '',
        '| Case | Phase | Replica | Artifact | Engine stage | Samples | Median ms | Calls | Failed calls |',
        '| --- | --- | --- | --- | --- | ---: | ---: | --- | --- |'])
    for cohort in report['volatility']['cohorts']:
        if not cohort['engine_phase_protocol_statuses']['reported']:
            continue
        stages = {'library_elapsed': {'elapsed': cohort['engine_elapsed'], 'calls_observed': [], 'failed_calls_observed': []},
                  'worker_preparation_excluded': {'elapsed': cohort['worker_preparation'], 'calls_observed': [], 'failed_calls_observed': []},
                  **cohort['engine_phase_observations']}
        for name, stage in stages.items():
            value = stage['elapsed']
            center = f"{value['median'] / 1_000_000:.4f}" if value['status'] == 'observed' else 'unavailable'
            calls = ','.join(map(str, stage['calls_observed'])) or 'unavailable'
            failed = ','.join(map(str, stage['failed_calls_observed'])) or 'unavailable'
            lines.append(f"| {cohort['case']} | {cohort['phase']} | {cohort['slot']} | {cohort['label']} | {name} | {value['samples']} | {center} | {calls} | {failed} |")
    lines.append('')
    controls = report.get('image_route_controls', {})
    if controls.get('enabled'):
        lines.extend(['## Same-binary image engine routes', '',
            'Diagnostic status `' + controls.get('status', 'incomplete') + '`. Candidate-only, original eight-image fixture, no timing wrapper or profiling opt-in.',
            'The measured clock ends at WNOWAIT exit observation. Owned-process cleanup and all cache/hash/output oracles are outside it.',
            'A route difference includes process launch, scheduling and public CLI supervision. It cannot isolate engine internals or establish a hosted cause.', '',
            'Candidate revision `' + str(controls.get('candidate_revision', 'unavailable')) + '`.',
            'Binary SHA-256 `' + str(controls.get('binary_sha256', 'unavailable')) + '`.',
            'Equal completed balance `' + str(controls.get('completion', {}).get('equal_completed_balance', False)) + '`.', '',
            '| Replica | Route | Verified commands | Median ms | Min/max ms | Unscored cleanup median ms |',
            '| --- | --- | ---: | ---: | --- | ---: |'])
        for row in controls.get('observations', []):
            value, cleanup = row['seconds'], row['cleanup_seconds_unscored']
            center = f"{value['median'] * 1000:.3f}" if value['status'] == 'observed' else 'unavailable'
            bounds = f"{value['minimum'] * 1000:.3f}/{value['maximum'] * 1000:.3f}" if value['status'] == 'observed' else 'unavailable'
            cost = f"{cleanup['median'] * 1000:.3f}" if cleanup['status'] == 'observed' else 'unavailable'
            lines.append(f"| {row['slot']} | {row['route']} | {value['samples']} | {center} | {bounds} | {cost} |")
        lines.append('')
    lines.extend('- ' + error.replace('\n', ' ') for error in report['errors'])
    benchmark.atomic_text(output.with_suffix('.md'), '\n'.join(lines) + '\n')


def parser():
    result = argparse.ArgumentParser(description=__doc__)
    for name in ('baseline', 'candidate', 'metadata', 'primary-report', 'output'):
        result.add_argument('--' + name, type=Path, required=True)
    result.add_argument('--repeats', type=int, default=8, help='Fixed chronological diagnostic repetitions, 1 to 16')
    result.add_argument('--timeout', type=float, default=30, help='Per-command bound, (0, 30] seconds')
    result.add_argument('--budget', type=float, default=600, help='Whole diagnostic bound, (0, 600] seconds')
    result.add_argument('--image-route-controls', action='store_true',
                        help='Candidate-only unwrapped public/private image route controls, eight fixed pairs per replica')
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
    report = {'schema_version': 3, 'status': 'incomplete', 'diagnostic_only': True, 'used_for_gate': False,
              'primary': {'path': str(args.primary_report), 'status': 'unavailable'}, 'executables': [],
              'policy': {'repeats': args.repeats, 'timeout_seconds': args.timeout, 'budget_seconds': args.budget,
                         'phase_opt_in': 'TEKAI_DIAGNOSTIC_PROFILE=1 and TEKAI_ENGINE_PROFILE=1 for instrumented commands only',
                         'sampling': 'Fixed chronological repetitions. No adaptive retries, outlier removal or result selection.',
                         'case_order': 'Fixed before commands from allowlisted primary failure evidence, then remaining original order. Original indexes preserve per-case assignments.',
                         'launch_only_control': 'Same copied artifact --version immediately before each measured warm-cache command.',
                         'same_artifact_cache_control': 'Each artifact has two content-equivalent independently initialized fixture replicas, one per slot.',
                         'comparison': 'None. Diagnostic commands cannot rescore the primary gate.'},
              'image_route_controls': {'enabled': args.image_route_controls, 'used_for_gate': False,
                  'status': 'not_started' if args.image_route_controls else 'not_requested',
                  'pairs_per_replica': IMAGE_ROUTE_PAIRS,
                  'baseline_private_route': 'Not attempted. The baseline may not implement the private worker.',
                  'scope': 'Candidate-only descriptive observations, original image fixture and unchanged command tail. '
                           'No replacement gate samples, ratios or causal engine-internal conclusions.'},
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
        case_plan = diagnostic_case_plan(primary)
        report['case_plan'] = case_plan
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
        persist(report, args.output)
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
                for planned_case in case_plan['cases']:
                    case_index, case = planned_case['original_case_index'], planned_case['case']
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
                        unit = {'case': case, 'original_case_index': case_index, 'repetition': repetition,
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
                                        record_command_error(control_row, error)
                                        raise
                                    finally:
                                        supervisor.active = None
                                        persist(report, args.output)
                                row = {**cell, 'case': case, 'phase': unit['phase'], 'repetition': repetition,
                                       'unit_id': unit['unit_id'], 'sequence_index': len(report['commands']),
                                       'command': list(map(str, fixture['command'])), 'input_sha256': fixture['input_sha256'],
                                       'status': 'not_started', 'output_validation': None}
                                row['phase_opt_in_note'] = 'Clap --version exits before Rust diagnostic emission; absent phases are expected.' \
                                    if case == 'cli-startup' else 'Engine entry bypasses parent TEKAI_PROFILE instrumentation. Separate TEKAI_ENGINE_PROFILE is requested; the baseline may lack that protocol.' \
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
                                    record_command_error(row, error)
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
                    if case == 'image-compile' and args.image_route_controls:
                        run_image_route_controls(report, supervisor, fixtures, metadata, sources['candidate'],
                                                 source_hashes['candidate'], args.output)
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
        if args.image_route_controls and report['image_route_controls']['status'] != 'complete':
            report['errors'].append('Requested image route controls did not complete their fixed balanced plan')
        report['status'] = 'error' if report['errors'] else 'complete'
        report['elapsed_seconds'] = time.monotonic() - started
        report['exit_code'] = int(bool(report['errors']))
        persist(report, args.output)
    return report['exit_code']


if __name__ == '__main__':
    sys.exit(main())
