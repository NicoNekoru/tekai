#!/usr/bin/env python3
"""Failure-only, non-gating controls using the original paired executables."""

import argparse
import contextlib
import copy
import hashlib
import json
import math
import os
from pathlib import Path
import re
import shutil
import signal
import sys

import benchmark_runtime as benchmark
from benchmark_sampling import DEFAULT_PAIRS, DEFAULT_BUDGET, SCHEMA_VERSION, MAX_REPORT_BYTES
from performance_build import VERSION

POLICY = {
    'pairs_per_case': DEFAULT_PAIRS, 'warmups_per_side': 2, 'min_sample_seconds': 1.0,
    'checkpoint_unit': 'complete-inference-unit', 'maximum_report_bytes': MAX_REPORT_BYTES,
    'checkpoint_storage': 'compact-checkpoints-with-append-only-command-journal-v1',
    'raw_command_memory_scope': 'Completed batches leave memory after journal append. '
        'At most one complete inference unit plus an active batch is retained during measurement. '
        'Final full-report reconstruction occurs once, after scored work.',
    'environment_isolation': {'removed_prefixes': ['TEKAI_', 'TEX', 'BIB', 'KPATHSEA'],
                              'removed_search_variables': sorted(benchmark.SEARCH_ENV_VARS),
                              'recorded_keys': list(benchmark.ISOLATED_ENVIRONMENT_KEYS),
                              'locale': 'C', 'timezone': 'UTC'},
    'warm_cache_min_sample_seconds': 1.0, 'max_iterations': 512,
    'cli_startup_min_sample_seconds': 0.25,
    'calibration_pairs': 2, 'calibration_iterations': 32,
    'calibration_pair_unit': 'per-slot',
    'calibration_method': 'balanced-batched-slot-pilot-v1',
    'warmup_unit': 'artifact-slot-cell',
    'crossover_design': 'fixed-slot-complementary-quads-v1',
    'pairs_per_inference_unit': 4, 'batches_per_inference_unit': 8,
    'inference_units_per_case': DEFAULT_PAIRS // 4,
    'slot_ids': ['s0', 's1'], 'replica_ids': ['r0', 'r1'],
    'replica_assignment': {'s0': {'baseline': 'r0', 'candidate': 'r1'},
                           's1': {'baseline': 'r1', 'candidate': 'r0'}},
    'per_command_deadline_seconds': 30.0, 'whole_run_deadline_seconds': DEFAULT_BUDGET,
    'relative_threshold': 0.10, 'familywise_confidence': 0.95,
    'calibration_headroom': 2.0, 'noise_relative_mad_limit': 0.10,
    'order_bias_limit': 0.05,
}
BUILD_COMMAND = 'cargo build --release --locked --bin tekai --no-default-features'
SHA256 = re.compile(r'[0-9a-f]{64}\Z')
REVISION = re.compile(r'[0-9a-f]{40}\Z')


def read_json(path):
    if path.stat().st_size > MAX_REPORT_BYTES:
        raise ValueError('Report exceeds the finite input limit')
    with path.open(encoding='utf-8') as handle:
        report = json.load(handle)
    require(isinstance(report, dict), 'Report must be a JSON object')
    return report


def digest(value):
    return isinstance(value, str) and SHA256.fullmatch(value) is not None


def matching_build_hash(side, measured):
    return isinstance(side, dict) and ('artifact_sha256' not in side
        or (digest(side['artifact_sha256']) and side['artifact_sha256'] == measured))


def positive(value):
    try:
        return type(value) in (int, float) and math.isfinite(value) and value > 0
    except OverflowError:
        return False


def records(value):
    return isinstance(value, list) and all(isinstance(row, dict) for row in value)


def verified_output(case, value, expected_version=None):
    if not isinstance(value, dict):
        return False
    if case == 'cli-startup':
        version = value.get('expected_version')
        return set(value) == {'expected_version', 'expected_stdout', 'stdout_verified', 'stderr_empty'} \
            and isinstance(version, str) and VERSION.fullmatch(version) is not None \
            and version == expected_version \
            and value.get('expected_stdout') == 'tekai ' + version + '\n' \
            and value.get('stdout_verified') is True and value.get('stderr_empty') is True
    if case == 'image-compile':
        return value.get('every_decoded_pixel_verified') is True and value.get('pages') == 8 \
            and value.get('image_objects') == 16 and value.get('dimensions') == [1024, 1024] \
            and value.get('alpha') == 128 and value.get('rgb_by_page') == [[i * 7 % 256, i * 13 % 256, 90] for i in range(8)]
    expected = 'Ordinary nested lookup.' if case == 'nested-lookup' else 'Probe.'
    return value.get('text_verified') is True and value.get('expected_text') == expected


def valid_batch(batch, iterations, case, expected_version=None):
    if not isinstance(batch, dict):
        return False
    commands = batch.get('commands')
    return records(commands) and len(commands) == iterations and type(batch.get('iterations')) is int \
        and batch['iterations'] == iterations \
        and all(valid_command_timing(command) and type(command.get('command_id')) is int
                and command['command_id'] > 0 for command in commands) and positive(batch.get('seconds')) \
        and verified_output(case, batch.get('output_validation'), expected_version) \
        and math.isclose(batch['seconds'], sum(command['seconds'] for command in commands), rel_tol=1e-12)


def valid_cache_state(state):
    if not isinstance(state, dict) or set(state) != {
        'sha256', 'bytes', 'mtime_ns', 'ctime_ns', 'device', 'inode', 'recorded_input_count',
        'referenced_dependency_count', 'input_paths_sha256', 'all_referenced_inputs_verified', 'used_for_gate'}:
        return False
    return all(digest(state.get(key)) for key in ('sha256', 'input_paths_sha256')) \
        and type(state['bytes']) is int and 0 < state['bytes'] <= benchmark.MAX_STATE_BYTES \
        and all(type(state[key]) is int and state[key] >= 0 for key in ('mtime_ns', 'ctime_ns', 'device', 'inode')) \
        and type(state['recorded_input_count']) is int and state['recorded_input_count'] >= 1025 \
        and type(state['referenced_dependency_count']) is int and state['referenced_dependency_count'] == 1024 \
        and state['all_referenced_inputs_verified'] is True and state['used_for_gate'] is False


def nonnegative(value):
    try:
        return type(value) in (int, float) and math.isfinite(value) and value >= 0
    except OverflowError:
        return False


def valid_command_timing(command):
    if not isinstance(command, dict) or not positive(command.get('seconds')):
        return False
    timing, boundaries = command.get('parent_timing'), command.get('command_timing')
    return isinstance(timing, list) and len(timing) == 3 and all(nonnegative(value) for value in timing) \
        and timing[0] <= timing[1] <= timing[2] \
        and isinstance(boundaries, list) and len(boundaries) == 12 and all(nonnegative(value) for value in boundaries) \
        and all(left <= right for left, right in zip(boundaries, boundaries[1:])) \
        and timing == [boundaries[index] for index in (0, 6, 9)] \
        and math.isclose(command['seconds'], timing[2] - timing[0], rel_tol=1e-12, abs_tol=1e-9)


def require(condition, reason):
    if not condition:
        raise ValueError(reason)


def validate_policy(report):
    require(isinstance(report.get('policy'), dict) and all(
                (type(report['policy'].get(key)) in (int, float) and report['policy'][key] == value
                 if type(value) is float else matching_details(report['policy'].get(key), value))
                for key, value in POLICY.items()),
            'Reported policy differs from the fixed control policy')


def matching_details(recorded, expected):
    """Compare recorded evidence without accepting booleans as numeric fields."""
    if type(recorded) is not type(expected):
        return False
    if isinstance(expected, dict):
        return recorded.keys() == expected.keys() and all(
            matching_details(recorded[key], value) for key, value in expected.items())
    if isinstance(expected, list):
        return len(recorded) == len(expected) and all(
            matching_details(value, reference) for value, reference in zip(recorded, expected))
    if isinstance(expected, float) and not math.isfinite(recorded):
        return False
    return recorded == expected


def json_schedule(value):
    return json.loads(json.dumps(value))


def copy_inventory(report, complete=True):
    rows = report.get('executables', [])
    require(records(rows), 'Executable cell inventory is malformed')
    cells, roots, paths = {}, set(), set()
    for row in rows:
        label, slot, replica = row.get('label'), row.get('slot'), row.get('replica')
        require(type(label) is str and label in benchmark.LABELS
                and type(slot) is str and slot in benchmark.SLOTS
                and replica == benchmark.replica_for(label, slot), 'Executable artifact-slot assignment differs')
        key = (slot, label)
        path = row.get('isolated_path')
        require(isinstance(path, str) and Path(path).is_absolute()
                and str(Path(path)) == path and Path(path).resolve() == Path(path), 'Executable isolated path is missing')
        path = Path(path)
        require(key not in cells and path not in paths
                and path.parts[-4:] == (slot, replica, 'bin', 'tekai'), 'Executable cell paths are aliased or misplaced')
        cells[key] = row
        paths.add(path)
        roots.add(path.parents[3])
    require(len(roots) <= 1 and (not complete or set(cells) == {
        (slot, label) for slot in benchmark.SLOTS for label in benchmark.LABELS}),
        'Executable artifact-slot inventory is incomplete or disjoint')
    require([{key: row.get(key) for key in ('slot', 'label', 'replica')} for row in rows]
            == benchmark.cell_order()[:len(rows)], 'Executable cell creation order differs')
    return cells, next(iter(roots), None)


def fixture_inventory(row, work, complete=True):
    start, end, roots = row.get('fixture_sha256'), row.get('fixture_sha256_end'), row.get('fixture_roots')
    require(isinstance(start, dict) and set(start) == set(benchmark.SLOTS)
            and isinstance(roots, dict) and set(roots) == set(benchmark.SLOTS)
            and all(isinstance(start[slot], dict) and set(start[slot]) <= set(benchmark.LABELS)
                    and (not complete or set(start[slot]) == set(benchmark.LABELS))
                    and isinstance(roots[slot], dict) and set(roots[slot]) == set(start[slot])
                    for slot in benchmark.SLOTS), 'Fixture artifact-slot maps are incomplete')
    hashes = [value for slot in benchmark.SLOTS for value in start[slot].values()]
    require(all(digest(value) for value in hashes) and len(set(hashes)) <= 1,
            'Fixture artifact-slot inputs differ')
    require(row.get('fixture_unchanged') is not False
            and ('fixture_sha256_end' not in row or (start == end and row.get('fixture_unchanged') is True))
            and (not complete or (start == end and row.get('fixture_unchanged') is True)),
            'Fixture input integrity is invalid')
    inventory = row.get('fixture_inventory')
    require(records(inventory) and len(inventory) == len(hashes) <= 4
            and (not complete or len(inventory) == 4) and work is not None,
            'Fixture cell inventory is incomplete')
    expected_order = benchmark.cell_order(benchmark.CASES.index(row.get('case')))
    require({(slot, label) for slot in benchmark.SLOTS for label in start[slot]}
            == {(cell['slot'], cell['label']) for cell in expected_order[:len(inventory)]},
            'Partial fixture maps differ from completed cell inventory')
    owned = set()
    for entry, cell in zip(inventory, expected_order):
        require(all(matching_details(entry.get(key), value) for key, value in cell.items()),
                'Fixture cell creation order or assignment differs')
        slot, label, replica = cell['slot'], cell['label'], cell['replica']
        root = work / slot / replica / row['case']
        expected = {'root': str(root), 'project': str(root / 'project'), 'out': str(root / 'out'),
                    'home': str(root / 'home'), 'input_sha256': start[slot][label],
                    'config_path': str(root / 'project' / 'tekai.toml'),
                    'config_sha256': benchmark.EMPTY_CONFIG_SHA256,
                    'referenced_dependency_count': 1024 if row['case'] == 'warm-build-cache' else 0,
                    'isolated_environment': {'HOME': str(root / 'home'), 'USERPROFILE': str(root / 'home'),
                        'TMPDIR': str(root / 'tmp'), 'TMP': str(root / 'tmp'), 'TEMP': str(root / 'tmp'),
                        'XDG_CACHE_HOME': str(root / 'xdg-cache'), 'XDG_CONFIG_HOME': str(root / 'xdg-config'),
                        'XDG_DATA_HOME': str(root / 'xdg-data'), 'APPDATA': str(root / 'xdg-data'),
                        'LOCALAPPDATA': str(root / 'xdg-cache'), 'PATH': '', 'LC_ALL': 'C', 'LANG': 'C',
                        'TZ': 'UTC', 'TEKAI_TEXMF_MODE': 'bundled',
                        'TEXINPUTS': str(root / 'project') + '//:' + str(root / 'out') + '//:'},
                    'cache_paths': {kind: str(root / ('cache-' + kind.lower()))
                                    for kind in ('ENGINE', 'FORMAT', 'AUX', 'BIBTEX')}}
        require(all(matching_details(entry.get(key), value) for key, value in expected.items())
                and roots[slot][label] == str(root), 'Fixture owned paths or input hash differ')
        paths = [entry[key] for key in ('root', 'project', 'out', 'home')] + list(entry['cache_paths'].values())
        require(len(set(paths)) == len(paths) and owned.isdisjoint(paths), 'Fixture cells share owned paths')
        owned.update(paths)


def validate_journal(report, report_path=None):
    """Bind final command evidence to every bounded, ordered journal record."""
    require(report.get('report_kind') == 'complete-report', 'Requires a completed restored command report')
    binding = report.get('raw_command_journal')
    require(isinstance(binding, dict) and set(binding) == {
        'path', 'sha256', 'bytes', 'records', 'format', 'maximum_bytes', 'maximum_record_bytes', 'scope'}
        and binding.get('format') == 'tekai-command-journal-v1'
        and binding.get('maximum_bytes') == MAX_REPORT_BYTES and type(binding.get('maximum_bytes')) is int
        and binding.get('maximum_record_bytes') == benchmark.MAX_JOURNAL_RECORD_BYTES
        and type(binding.get('maximum_record_bytes')) is int and digest(binding.get('sha256'))
        and type(binding.get('bytes')) is int and 0 < binding['bytes'] <= MAX_REPORT_BYTES
        and type(binding.get('records')) is int and 0 < binding['records'] <= DEFAULT_PAIRS * len(benchmark.CASES)
        and binding.get('scope') == 'All completed warmup, pilot and formal batch command records. Initialization remains in report.',
        'Raw command journal binding is malformed')
    recorded = binding.get('path')
    require(isinstance(recorded, str) and Path(recorded).is_absolute()
            and Path(recorded).resolve() == Path(recorded) and str(Path(recorded)) == recorded,
            'Raw command journal path is not canonical')
    path = Path(recorded)
    require(path.is_file() and not path.is_symlink() and path.stat().st_size == binding['bytes'],
            'Raw command journal is missing, aliased or has a different size')
    if report_path is not None:
        require(path.parent == report_path.parent and path.name.startswith(report_path.stem + '.raw.')
                and path.suffix == '.jsonl', 'Raw command journal is outside the report-owned sibling path')
    batches, expected_records = {}, []
    rows = report.get('results')
    require(records(rows), 'Raw command journal case inventory is malformed')
    for case_index, row in enumerate(rows):
        for phase in ('warmups', 'calibration_pilots', 'samples'):
            items = row.get(phase)
            require(records(items), 'Raw command journal phase inventory is malformed')
            for row_index, item in enumerate(items):
                for label in benchmark.LABELS:
                    require(isinstance(item.get(label), dict), 'Raw command journal batch is missing')
                    batches[(case_index, phase, row_index, label)] = item[label]
            width = 1 if phase == 'calibration_pilots' else 4
            for first in range(0, len(items), width):
                expected = []
                for row_index in range(first, min(first + width, len(items))):
                    order = items[row_index].get('order')
                    require(isinstance(order, list) and len(order) == 2 and set(order) == set(benchmark.LABELS),
                            'Raw command journal batch order is malformed')
                    expected.extend({'case_index': case_index, 'phase': phase, 'row_index': row_index, 'label': label}
                                    for label in order)
                expected_records.append(expected)
    require(binding['records'] == len(expected_records), 'Raw command journal record grouping differs')
    observed, content_hash, byte_count = set(), hashlib.sha256(), 0
    with path.open('rb') as handle:
        for sequence in range(binding['records']):
            line = handle.readline(benchmark.MAX_JOURNAL_RECORD_BYTES + 1)
            require(line.endswith(b'\n') and len(line) <= benchmark.MAX_JOURNAL_RECORD_BYTES,
                    'Raw command journal record is incomplete or oversized')
            byte_count += len(line)
            content_hash.update(line)
            record = json.loads(line)
            require(isinstance(record, dict) and set(record) == {'sequence', 'batches'}
                    and type(record.get('sequence')) is int and record['sequence'] == sequence
                    and records(record.get('batches')) and 0 < len(record['batches']) <= 8,
                    'Raw command journal record order or shape differs')
            require([{key: entry.get(key) for key in ('case_index', 'phase', 'row_index', 'label')}
                     for entry in record['batches']] == expected_records[sequence],
                    'Raw command journal append order differs from its predeclared phase and unit grouping')
            for batch_index, entry in enumerate(record['batches']):
                require(set(entry) == {'case_index', 'phase', 'row_index', 'label', 'commands'}
                        and type(entry.get('case_index')) is int and type(entry.get('row_index')) is int
                        and isinstance(entry.get('phase'), str) and isinstance(entry.get('label'), str),
                        'Raw command journal batch descriptor is malformed')
                key = (entry['case_index'], entry['phase'], entry['row_index'], entry['label'])
                require(key in batches and key not in observed
                        and matching_details(batches[key].get('command_journal'), {'record': sequence, 'batch': batch_index})
                        and matching_details(entry.get('commands'), batches[key].get('commands')),
                        'Raw command journal does not match restored command evidence')
                observed.add(key)
        require(not handle.read(1), 'Raw command journal has trailing records')
    require(observed == set(batches) and byte_count == binding['bytes']
            and content_hash.hexdigest() == binding['sha256'], 'Raw command journal content or inventory differs')
    return path


def valid_parent_snapshot(row):
    if not isinstance(row, dict) or row.get('used_for_gate') is not False \
            or not nonnegative(row.get('monotonic_seconds')) \
            or row.get('source') != 'resource.getrusage(RUSAGE_SELF), gc and threading' \
            or row.get('scope') != 'Benchmark parent only. CPU and peak RSS are cumulative, not child costs.' \
            or type(row.get('gc_enabled')) is not bool \
            or type(row.get('active_thread_count')) is not int or row['active_thread_count'] < 1:
        return False
    for key in ('gc_counts', 'gc_thresholds'):
        value = row.get(key)
        if not isinstance(value, list) or len(value) != 3 or not all(type(count) is int and count >= 0 for count in value):
            return False
    generations = row.get('gc_generations')
    if not records(generations) or len(generations) != 3 or not all(
        set(generation) == {'collections', 'collected', 'uncollectable'}
        and all(type(value) is int and value >= 0 for value in generation.values()) for generation in generations):
        return False
    if row.get('cpu_status') == 'reported':
        if not all(nonnegative(row.get(key)) for key in ('user_cpu_seconds', 'system_cpu_seconds')):
            return False
    elif row.get('cpu_status') != 'unavailable' or any(key in row for key in ('user_cpu_seconds', 'system_cpu_seconds')):
        return False
    if row.get('peak_rss_status') == 'reported':
        unit = row.get('peak_rss_raw_unit')
        if unit not in ('bytes', 'KiB') or not nonnegative(row.get('peak_rss_raw')) \
                or not nonnegative(row.get('peak_rss_mib')) \
                or not math.isclose(row['peak_rss_mib'], row['peak_rss_raw'] / (1048576 if unit == 'bytes' else 1024), rel_tol=1e-12):
            return False
    elif row.get('peak_rss_status') != 'unavailable' or any(key in row for key in ('peak_rss_raw', 'peak_rss_raw_unit', 'peak_rss_mib')):
        return False
    if row.get('current_rss_status') == 'reported':
        if type(row.get('current_rss_bytes')) is not int or not 0 <= row['current_rss_bytes'] < 2 ** 64 \
                or 'current_rss_unavailable_reason' in row:
            return False
        native_counters = ('physical_footprint_bytes', 'parent_pageins', 'parent_interrupt_wakeups',
                           'parent_package_idle_wakeups')
        if row.get('current_rss_source') == '/proc/self/statm resident pages times SC_PAGE_SIZE':
            return all(key not in row for key in (*native_counters, 'physical_footprint_source'))
        return row.get('current_rss_source') == 'macOS proc_pid_rusage RUSAGE_INFO_V0.ri_resident_size' \
            and row.get('physical_footprint_source') == 'macOS proc_pid_rusage RUSAGE_INFO_V0.ri_phys_footprint' \
            and all(type(row.get(key)) is int and 0 <= row[key] < 2 ** 64 for key in native_counters)
    return row.get('current_rss_status') == 'unavailable' \
        and isinstance(row.get('current_rss_unavailable_reason'), str) \
        and len(row['current_rss_unavailable_reason']) <= 1024 \
        and all(key not in row for key in ('current_rss_bytes', 'current_rss_source', 'physical_footprint_bytes',
                'physical_footprint_source', 'parent_pageins', 'parent_interrupt_wakeups', 'parent_package_idle_wakeups'))


def validate_cases(report, allow_infeasible=False):
    rows = report.get('results', [])
    require(records(rows) and [row.get('case') for row in rows] == list(benchmark.CASES),
            'Completed case inventory is incomplete')
    validate_journal(report)
    outcomes = []
    observed_commands = set()
    _, work = copy_inventory(report)
    require(matching_details(report.get('timing_metadata'), benchmark.TIMING_METADATA)
            and matching_details(report.get('prospective_sampling_study'), benchmark.prospective_power_study())
            and nonnegative(report.get('monotonic_start_seconds'))
            and positive(report.get('monotonic_end_seconds'))
            and math.isclose(report['elapsed_seconds'], report['monotonic_end_seconds'] - report['monotonic_start_seconds'],
                             rel_tol=1e-12, abs_tol=1e-9)
            and isinstance(report.get('clock_origin'), dict)
            and type(report['clock_origin'].get('unix_ns')) is int and report['clock_origin']['unix_ns'] > 0
            and nonnegative(report['clock_origin'].get('monotonic_seconds'))
            and report['monotonic_start_seconds'] <= report['clock_origin']['monotonic_seconds'] <= report['monotonic_end_seconds']
            and report['clock_origin'].get('alignment') == 'Sequential wall then monotonic observations; approximate alignment only.',
            'Parent clock metadata is incomplete or inconsistent')
    previous_completion = report['monotonic_start_seconds']
    previous_command_id = 0
    expected_hosts = []
    expected_checkpoints = [('isolated-binaries', None, None)]
    command_windows = []

    def claim_timing(commands):
        nonlocal previous_completion, previous_command_id
        for command in commands:
            require(valid_command_timing(command)
                    and previous_completion <= command['parent_timing'][0]
                    and command['command_id'] > previous_command_id
                    and command['command_timing'][-1] <= report['monotonic_end_seconds'],
                    'Recorded command timestamps differ from actual declared launch order')
            previous_completion = command['command_timing'][-1]
            previous_command_id = command['command_id']
            command_windows.append((command['parent_timing'][0], command['command_timing'][-1]))

    def claim_commands(command_ids):
        require(len(set(command_ids)) == len(command_ids) and observed_commands.isdisjoint(command_ids),
                'Completed measurement phases reuse command evidence')
        observed_commands.update(command_ids)

    for case_index, row in enumerate(rows):
        fixture_inventory(row, work)
        schedule = [json_schedule(benchmark.crossover_pair(index, case_index)) for index in range(DEFAULT_PAIRS)]
        require(matching_details(row.get('slot_schedule'), schedule), 'Predeclared crossover schedule differs')
        samples, iterations = row.get('samples'), row.get('iterations_per_batch')
        warmups, initial = row.get('warmups'), row.get('initialization')
        minimum = 0.25 if row['case'] == 'cli-startup' else 1.0
        require(records(samples) and records(warmups) and len(warmups) == 4
                and type(row.get('min_sample_seconds')) in (int, float) and row['min_sample_seconds'] == minimum,
                'Completed case sampling configuration differs')
        require(matching_details(row.get('initialization_order'), benchmark.cell_order(case_index))
                and isinstance(initial, dict) and set(initial) == set(benchmark.SLOTS)
                and all(isinstance(initial[slot], dict) and set(initial[slot]) == set(benchmark.LABELS)
                        for slot in benchmark.SLOTS)
                and all(valid_command_timing(value)
                        and type(value.get('command_id')) is int and value['command_id'] > 0
                        and verified_output(row['case'], initial[slot][label].get('output_validation'),
                                            report['metadata'][label].get('expected_version'))
                        for slot in benchmark.SLOTS for label in benchmark.LABELS
                        for value in (initial[slot][label],)),
                'Completed initialization is incomplete')
        claim_timing([initial[cell['slot']][cell['label']] for cell in row['initialization_order']])
        if row['case'] == 'warm-build-cache':
            require(all(valid_cache_state(initial[slot][label].get('cache_state'))
                        for slot in benchmark.SLOTS for label in benchmark.LABELS),
                    'Dependency-heavy cache initialization is incomplete')
        for index, warmup in enumerate(warmups):
            expected = {'warmup_index': index // 2,
                        **json_schedule(benchmark.slot_pair(benchmark.SLOTS[index % 2], index // 2, case_index))}
            require(all(matching_details(warmup.get(key), value) for key, value in expected.items())
                    and all(valid_batch(warmup.get(label), 1, row['case'],
                                        report['metadata'][label].get('expected_version')) for label in benchmark.LABELS),
                    'Completed warmups are incomplete')
            claim_timing([command for label in warmup['order'] for command in warmup[label]['commands']])
        pilots = row.get('calibration_pilots')
        require(records(pilots) and len(pilots) == 4, 'Completed calibration pilots are incomplete')
        for index, pilot in enumerate(pilots):
            expected = {'pair_index': index // 2, 'iterations': 32,
                        **json_schedule(benchmark.slot_pair(benchmark.SLOTS[index % 2], index // 2, case_index))}
            require(all(matching_details(pilot.get(key), value) for key, value in expected.items())
                    and all(valid_batch(pilot.get(label), 32, row['case'],
                                        report['metadata'][label].get('expected_version')) for label in benchmark.LABELS),
                    'Completed calibration pilot batches are incomplete')
            claim_timing([command for label in pilot['order'] for command in pilot[label]['commands']])
            expected_checkpoints.append(('calibration', row['case'], None))
        claim_commands([value['command_id'] for slot in benchmark.SLOTS for value in initial[slot].values()] + [
            command['command_id'] for item in warmups + pilots for label in benchmark.LABELS
            for command in item[label]['commands']])
        if row['case'] == 'warm-build-cache':
            for item in warmups + pilots + samples:
                slot = item['slot']
                require(all(valid_cache_state(item[label].get('cache_state_before'))
                            and matching_details(item[label].get('cache_state_before'), initial[slot][label]['cache_state'])
                            and matching_details(item[label].get('cache_state_after'), initial[slot][label]['cache_state'])
                            and item[label].get('cache_state_unchanged') is True
                            for label in benchmark.LABELS),
                        'Dependency-heavy cache batch state differs from its initialized identity')
        details = benchmark.calibration_details(pilots, minimum, 512)
        require(matching_details(row.get('calibration'), details),
                'Completed calibration details differ from raw pilots')
        try:
            calibrated = benchmark.calibrated_iterations(pilots, minimum, 512)
        except benchmark.CalibrationInfeasible as error:
            require(allow_infeasible and samples == [] and row.get('unit_timing') == [] and iterations is None
                    and matching_details(row.get('calibration_infeasible'), error.details)
                    and matching_details(row.get('analysis'), {'status': 'inconclusive', 'reasons': [str(error)]}),
                    'Completed calibration is infeasible or inconsistent')
            outcomes.append('inconclusive')
            expected_checkpoints.append(('case-complete', row['case'], None))
            continue
        require(len(samples) == DEFAULT_PAIRS and type(iterations) is int and iterations == calibrated
                and 'calibration_infeasible' not in row, 'Completed case batches differ from calibration')
        phase_completion = previous_completion
        expected_checkpoints.append(('calibrated', row['case'], None))
        for index, sample in enumerate(samples):
            require(all(matching_details(sample.get(key), value) for key, value in schedule[index].items())
                    and type(sample.get('iterations')) is int and sample['iterations'] == iterations,
                    'Completed sample inventory is incomplete')
            require(all(valid_batch(sample.get(label), iterations, row['case'],
                                    report['metadata'][label].get('expected_version')) for label in benchmark.LABELS),
                    'Completed paired batches are incomplete')
            require(matching_details(sample.get('ratio'), sample['candidate']['seconds'] / sample['baseline']['seconds']),
                    'Completed pair ratio differs from raw batches')
            claim_commands([command['command_id'] for label in benchmark.LABELS
                            for command in sample[label]['commands']])
            claim_timing([command for label in sample['order'] for command in sample[label]['commands']])
        units = row.get('unit_timing')
        require(records(units) and len(units) == DEFAULT_PAIRS // 4, 'Completed unit timestamps are incomplete')
        for unit_index, unit in enumerate(units):
            within = samples[unit_index * 4:unit_index * 4 + 4]
            commands = [command for sample in within for label in sample['order'] for command in sample[label]['commands']]
            require(type(unit.get('unit_index')) is int and unit['unit_index'] == unit_index
                    and type(unit.get('first_pair_index')) is int and unit['first_pair_index'] == unit_index * 4
                    and type(unit.get('pair_count')) is int and unit['pair_count'] == 4
                    and unit.get('complete') is True and nonnegative(unit.get('monotonic_start_seconds'))
                    and positive(unit.get('monotonic_end_seconds'))
                    and unit['monotonic_start_seconds'] <= commands[0]['parent_timing'][0]
                    and (unit_index != 0 or phase_completion <= unit['monotonic_start_seconds'])
                    and commands[-1]['command_timing'][-1] <= unit['monotonic_end_seconds']
                    and (unit_index == 0 or units[unit_index - 1]['monotonic_end_seconds'] <= unit['monotonic_start_seconds'])
                    and unit['monotonic_end_seconds'] <= report['monotonic_end_seconds'],
                    'Completed unit timestamp boundaries differ')
            before, after = unit.get('parent_before'), unit.get('parent_after')
            require(valid_parent_snapshot(before) and valid_parent_snapshot(after)
                    and report['monotonic_start_seconds'] <= before['monotonic_seconds'] <= unit['monotonic_start_seconds']
                    and unit['monotonic_end_seconds'] <= after['monotonic_seconds'] <= report['monotonic_end_seconds']
                    and (unit_index != 0 or phase_completion <= before['monotonic_seconds'])
                    and (unit_index == 0 or units[unit_index - 1]['parent_after']['monotonic_seconds'] <= before['monotonic_seconds'])
                    and (unit_index == 0 or before['monotonic_seconds'] >= units[unit_index - 1]['monotonic_end_seconds']),
                    'Unit-boundary parent resource diagnostics are missing, gating or out of order')
            expected_checkpoints.append(('complete-inference-unit', row['case'], unit_index))
            expected_hosts.append((row['case'], unit_index, unit['monotonic_end_seconds'],
                                   units[unit_index + 1]['monotonic_start_seconds'] if unit_index + 1 < len(units)
                                   else (rows[case_index + 1]['initialization'][benchmark.cell_order(case_index + 1)[0]['slot']]
                                         [benchmark.cell_order(case_index + 1)[0]['label']]['parent_timing'][0]
                                         if case_index + 1 < len(rows) else report['monotonic_end_seconds'])))
            require(after['monotonic_seconds'] <= expected_hosts[-1][3],
                    'Unit parent observation overlaps subsequent measured work')
        previous_completion = max(previous_completion, units[-1]['monotonic_end_seconds'])
        analysis = benchmark.analyze_pairs(samples, 0.10, minimum, case_index=case_index)
        require(matching_details(row.get('analysis'), analysis),
                'Completed sample analysis differs')
        outcomes.append(analysis['status'])
        expected_checkpoints.append(('case-complete', row['case'], None))
    overall = 'fail' if 'fail' in outcomes else ('inconclusive' if 'inconclusive' in outcomes else 'pass')
    require(report.get('status') == overall, 'Completed overall outcome differs')
    checkpoints = report.get('checkpoint_observations')
    require(records(checkpoints) and len(checkpoints) == len(expected_checkpoints),
            'Compact checkpoint observations are incomplete')
    prior_checkpoint = report['monotonic_start_seconds']
    for checkpoint, expected in zip(checkpoints, expected_checkpoints):
        require((checkpoint.get('phase'), checkpoint.get('case'), checkpoint.get('unit_index')) == expected
                and checkpoint.get('complete') is True and checkpoint.get('used_for_gate') is False
                and nonnegative(checkpoint.get('monotonic_start_seconds'))
                and nonnegative(checkpoint.get('monotonic_end_seconds'))
                and prior_checkpoint <= checkpoint['monotonic_start_seconds'] <= checkpoint['monotonic_end_seconds']
                and checkpoint['monotonic_end_seconds'] <= report['monotonic_end_seconds']
                and nonnegative(checkpoint.get('seconds'))
                and math.isclose(checkpoint['seconds'], checkpoint['monotonic_end_seconds'] - checkpoint['monotonic_start_seconds'],
                                 rel_tol=1e-12, abs_tol=1e-9)
                and type(checkpoint.get('json_bytes')) is int and 0 < checkpoint['json_bytes'] <= MAX_REPORT_BYTES
                and all(checkpoint['monotonic_end_seconds'] <= first or checkpoint['monotonic_start_seconds'] >= last
                        for first, last in command_windows),
                'Compact checkpoint chronology or diagnostic scope differs')
        prior_checkpoint = checkpoint['monotonic_end_seconds']
    hosts = report.get('host_observations')
    require(records(hosts) and len(hosts) == len(expected_hosts), 'Unit-boundary host observations are incomplete')
    for host, (case, unit_index, earliest, latest) in zip(hosts, expected_hosts):
        require(host.get('case') == case and type(host.get('unit_index')) is int and host['unit_index'] == unit_index
                and host.get('used_for_gate') is False and nonnegative(host.get('monotonic_seconds'))
                and earliest <= host['monotonic_seconds'] <= latest
                and all(host.get(field) == 'unavailable' for field in ('memory_status', 'swap_status', 'cpu_utilization_status'))
                and ((host.get('load_average_status') == 'unavailable' and host.get('load_average') is None)
                     or (host.get('load_average_status') == 'reported' and isinstance(host.get('load_average'), list)
                         and len(host['load_average']) == 3 and all(nonnegative(value) for value in host['load_average']))),
                'Unit-boundary host diagnostic is malformed')


def eligible(primary, metadata, sources, revisions, profile):
    """Validate completed evidence, without changing its verdict or statistics."""
    require(isinstance(primary, dict), 'Report must be a JSON object')
    require(profile == 'full', 'Attribution is limited to the full profile')
    require(type(primary.get('schema_version')) is int and primary['schema_version'] == SCHEMA_VERSION
            and primary.get('gate') is True,
            'Requires a primary statistical gate report')
    status = primary.get('status')
    require(status in ('fail', 'inconclusive') and type(primary.get('exit_code')) is int
            and primary['exit_code'] == {'fail': 1, 'inconclusive': 2}.get(status),
            'Requires a completed non-green primary gate')
    require(primary.get('errors') == [] and positive(primary.get('elapsed_seconds')),
            'Primary execution errors or incomplete timing prevent attribution')
    require(primary.get('metadata') == metadata, 'Primary provenance does not match its metadata file')
    require(metadata.get('release_settings') == {'codegen_units': 1, 'lto': 'fat', 'panic': 'abort'}
            and type(metadata['release_settings']['codegen_units']) is int,
            'Primary release settings differ')
    validate_policy(primary)
    for label in benchmark.LABELS:
        side = metadata.get(label, {})
        require(isinstance(revisions[label], str) and REVISION.fullmatch(revisions[label]) is not None
                and side.get('revision') == revisions[label]
                and side.get('build_command') == BUILD_COMMAND, 'Primary revision/build provenance differs')
    require(revisions['baseline'] != revisions['candidate'], 'The primary must compare distinct revisions')
    protected = {}
    executables, _ = copy_inventory(primary)
    for row in executables.values():
        label = row['label']
        hashes = [row.get(key) for key in ('source_sha256_start', 'source_sha256_end', 'sha256_start', 'sha256_end')]
        require(row.get('unchanged') is True and all(digest(value) for value in hashes) and len(set(hashes)) == 1,
                'Primary executable integrity is invalid')
        require(matching_build_hash(metadata[label], hashes[0]),
                'Primary build artifact hash differs from measured executable')
        require(Path(row.get('source_path', '')).resolve() == sources[label]
                and sources[label].is_file() and os.access(sources[label], os.X_OK),
                'Primary executable source path differs')
        require(sources[label] not in protected or protected[sources[label]] == hashes[0],
                'Primary artifact identity differs across slots')
        protected[sources[label]] = hashes[0]
    verifiers = primary.get('output_verifiers', {})
    require(isinstance(verifiers, dict) and set(verifiers) == {'pdftotext', 'pdfinfo', 'pdfimages'}
            and all(isinstance(row, dict) for row in verifiers.values()), 'Primary Poppler inventory is incomplete')
    for name, row in verifiers.items():
        start, end = row.get('sha256_start'), row.get('sha256_end')
        current = shutil.which(name)
        require(row.get('unchanged') is True and digest(start) and start == end and current is not None
                and Path(current).resolve() == Path(row.get('path', '')).resolve(),
                'Primary Poppler integrity/path differs')
        protected[Path(current).resolve()] = start
    validate_cases(primary)
    return protected


def verify_current(protected, verifiers):
    for path, expected in protected.items():
        require(benchmark.executable_sha256(path) == expected, 'Protected source/report/verifier changed: ' + str(path))
    for name, row in verifiers.items():
        current = shutil.which(name)
        require(current is not None and Path(current).resolve() == Path(row['path']).resolve(),
                'Poppler selection changed: ' + name)


def control_metadata(metadata, kind, primary_path):
    result = copy.deepcopy(metadata)
    if kind == 'reversed':
        result['baseline'], result['candidate'] = copy.deepcopy(metadata['candidate']), copy.deepcopy(metadata['baseline'])
    else:
        result['baseline'] = copy.deepcopy(metadata['candidate'])
        result['candidate'] = copy.deepcopy(metadata['candidate'])
    result.update(baseline_policy='diagnostic-' + kind, diagnostic_only=True, used_for_primary_gate=False,
                  diagnostic_kind=kind, primary_report=str(primary_path), original_provenance=copy.deepcopy(metadata))
    return result


def stop_reason(report, selected, expected):
    errors = report.get('errors', [])
    require(isinstance(errors, list) and all(isinstance(error, str) for error in errors), 'Control errors are malformed')
    for error in errors:
        if any(text in error for text in ('Run interrupted', 'Cleanup failed:', 'owned-group cleanup', 'TimeoutExpired',
                                         'changed', 'disappeared', 'removed benchmark fixture',
                                         'fixtures do not have equal content', 'journal')):
            return error
    executables, verifiers, fixtures = report.get('executables', []), report.get('output_verifiers', {}), report.get('results', [])
    require(records(executables) and isinstance(verifiers, dict)
            and all(isinstance(row, dict) for row in verifiers.values()) and records(fixtures),
            'Control integrity inventories are malformed')
    _, work = copy_inventory(report, complete=report.get('status') != 'error')
    if report.get('status') != 'error':
        require(set(verifiers) == {'pdftotext', 'pdfinfo', 'pdfimages'}, 'Control integrity inventories are incomplete')
        require(errors == [] and positive(report.get('elapsed_seconds')), 'Completed control has errors or no elapsed time')
        validate_cases(report, allow_infeasible=True)
    for row in executables:
        hashes = [row.get(key) for key in ('source_sha256_start', 'source_sha256_end', 'sha256_start', 'sha256_end')]
        label = row.get('label')
        if label not in selected or row.get('unchanged') is not True or not all(digest(value) for value in hashes) \
                or len(set(hashes)) != 1 or hashes[0] != expected[selected[label]] \
                or Path(row.get('source_path', '')).resolve() != selected[label]:
            return 'Control executable integrity failed'
        metadata = report.get('metadata')
        if not matching_build_hash(metadata.get(label) if isinstance(metadata, dict) else None, hashes[0]):
            return 'Control build artifact hash differs from measured executable'
    for name, row in verifiers.items():
        path = Path(row.get('path', '')).resolve()
        if name not in ('pdftotext', 'pdfinfo', 'pdfimages') or row.get('unchanged') is not True \
                or not digest(row.get('sha256_start')) or row['sha256_start'] != row.get('sha256_end') \
                or row['sha256_start'] != expected.get(path) or shutil.which(name) is None \
                or Path(shutil.which(name)).resolve() != path:
            return 'Control Poppler integrity failed'
    require([row.get('case') for row in fixtures] == list(benchmark.CASES)[:len(fixtures)],
            'Control fixture case prefix is invalid')
    for index, row in enumerate(fixtures):
        fixture_inventory(row, work, complete=report.get('status') != 'error' or index < len(fixtures) - 1)
    return None


def persist(summary, directory):
    (directory / 'attribution.json').write_text(json.dumps(summary, indent=2, allow_nan=False) + '\n', encoding='utf-8')
    lines = ['# Non-gating attribution controls', '',
             'Primary failure remains authoritative. These controls cannot replace or pass the primary gate.', '',
             'Attribution status `' + summary['status'] + '`.', '', summary.get('reason', ''), '',
             '| Control | Semantic outcome | Exit code |', '| --- | --- | ---: |']
    for row in summary['controls']:
        lines.append('| ' + row['kind'] + ' | ' + row['status'] + ' | ' + str(row['exit_code']) + ' |')
    (directory / 'attribution.md').write_text('\n'.join(lines) + '\n', encoding='utf-8')


def run_attribution(primary_path, metadata_path, sources, revisions, directory, profile='full', runner=None):
    """Run two advisory controls. The benchmark owns all native processes."""
    directory = Path(directory)
    if directory.exists() or directory.is_symlink():
        raise FileExistsError('Attribution output directory already exists or is a symlink')
    primary_path, metadata_path, directory = (Path(path).resolve() for path in (primary_path, metadata_path, directory))
    sources = {label: Path(path).resolve() for label, path in sources.items()}
    # Claim a new directory exclusively. Existing files, directories and aliases
    # are never reused, including any primary report or selected executable.
    directory.mkdir(exist_ok=False)
    summary = {'schema_version': 1, 'diagnostic_only': True, 'used_for_primary_gate': False,
               'status': 'skipped', 'controls': [], 'primary_report': str(primary_path)}
    try:
        primary, metadata = read_json(primary_path), benchmark.load_metadata(metadata_path)
        summary['primary_outcome'] = {
            'status': primary.get('status') if isinstance(primary.get('status'), str) else 'invalid',
            'exit_code': primary.get('exit_code') if type(primary.get('exit_code')) is int else None,
        }
        protected = eligible(primary, metadata, sources, revisions, profile)
        journal_path = validate_journal(primary, primary_path)
        protected[journal_path] = primary['raw_command_journal']['sha256']
        input_paths = [primary_path, metadata_path]
        if primary_path.with_suffix('.md').exists():
            input_paths.append(primary_path.with_suffix('.md'))
        protected.update({path: benchmark.executable_sha256(path) for path in input_paths})
        verify_current(protected, primary['output_verifiers'])
    except (OSError, ValueError, KeyError, TypeError, OverflowError) as error:
        summary['reason'] = str(error)
        persist(summary, directory)
        return 0
    summary['status'] = 'running'
    persist(summary, directory)
    previous_handler = signal.getsignal(signal.SIGTERM)

    def terminate(_number, _frame):
        raise KeyboardInterrupt

    signal.signal(signal.SIGTERM, terminate)
    try:
        for kind in ('reversed', 'candidate-self'):
            try:
                verify_current(protected, primary['output_verifiers'])
                metadata_file = directory / (kind + '-metadata.json')
                provenance = control_metadata(metadata, kind, primary_path)
                selected = {'baseline': sources['candidate'], 'candidate': sources['baseline']} if kind == 'reversed' \
                    else {'baseline': sources['candidate'], 'candidate': sources['candidate']}
                output = directory / (kind + '.json')
                log_file = directory / (kind + '.log')
                require(all(not path.exists() and not path.is_symlink()
                            for path in (metadata_file, output, output.with_suffix('.md'), log_file)),
                        'Control destinations already exist or alias inputs')
                with metadata_file.open('x', encoding='utf-8') as handle:
                    handle.write(json.dumps(provenance, indent=2) + '\n')
                protected[metadata_file] = benchmark.executable_sha256(metadata_file)
                argv = ['--baseline', str(selected['baseline']), '--candidate', str(selected['candidate']),
                        '--metadata', str(metadata_file), '--output', str(output), '--pairs', str(DEFAULT_PAIRS), '--warmups', '2',
                        '--timeout', '30', '--budget', str(DEFAULT_BUDGET), '--threshold', '0.10', '--min-sample-seconds', '1.0',
                        '--cli-startup-min-sample-seconds', '0.25',
                        '--warm-cache-min-sample-seconds', '1.0', '--max-iterations', '512']
                row = {'kind': kind, 'status': 'error', 'exit_code': None}
                summary['controls'].append(row)
                with log_file.open('x', encoding='utf-8') as log, \
                        contextlib.redirect_stdout(log), contextlib.redirect_stderr(log):
                    try:
                        row['exit_code'] = (runner or benchmark.main)(argv)
                    except SystemExit as error:
                        row['exit_code'] = error.code if type(error.code) is int else 1
                        print('Benchmark invocation exited: ' + str(error))
                    except Exception as error:
                        row['exit_code'] = 1
                        row['error'] = type(error).__name__ + ': ' + str(error)
                        print(row['error'])
                if type(row['exit_code']) is not int:
                    row['invalid_exit_code'] = repr(row['exit_code'])
                    row['exit_code'] = None
                    raise ValueError('Control runner returned a non-integer exit code')
                control = read_json(output) if output.exists() else {'status': 'error', 'errors': [row.get('error', 'Control report missing')]}
                require(control.get('status') in ('pass', 'fail', 'inconclusive', 'error')
                        and isinstance(control.get('errors', []), list)
                        and all(isinstance(error, str) for error in control.get('errors', [])),
                        'Control outcome is malformed')
                row.update(status=control['status'], reported_status=control['status'], errors=list(control.get('errors', [])))
                if row.get('error') and row['error'] not in row['errors']:
                    row['errors'].append(row['error'])
                    row['status'] = 'error'
                persist(summary, directory)
                verify_current(protected, primary['output_verifiers'])
                if output.exists():
                    control_journal = None
                    require(type(control.get('schema_version')) is int and control['schema_version'] == SCHEMA_VERSION
                            and control.get('gate') is False
                            and control.get('metadata') == provenance, 'Control report provenance/gate differs')
                    validate_policy(control)
                    if control['status'] != 'error':
                        control_journal = validate_journal(control, output)
                    require(positive(control.get('elapsed_seconds'))
                            and type(control.get('exit_code')) is int and type(row['exit_code']) is int
                            and control['exit_code'] == row['exit_code']
                            and row['exit_code'] == benchmark.comparison_exit(control['status'], False)
                            and bool(control.get('errors')) == (control['status'] == 'error'),
                            'Control completion or exit evidence differs')
                reason = stop_reason(dict(control, errors=row['errors']), selected, protected)
                if reason:
                    raise ValueError(reason)
                if output.exists() and control_journal is not None:
                    protected[control_journal] = control['raw_command_journal']['sha256']
                for path in (metadata_file, output, output.with_suffix('.md'), directory / (kind + '.log')):
                    if path.exists():
                        protected[path] = benchmark.executable_sha256(path)
            except (OSError, ValueError, KeyError, TypeError, OverflowError) as error:
                summary.update(status='stopped', reason=str(error))
                if summary['controls'] and summary['controls'][-1]['kind'] == kind:
                    summary['controls'][-1].update(status='invalid', reason=str(error))
                break
        else:
            summary['status'] = 'complete'
    except KeyboardInterrupt:
        summary.update(status='stopped', reason='Run interrupted; no further control launched')
        if summary['controls'] and summary['controls'][-1]['exit_code'] is None:
            summary['controls'][-1].update(status='interrupted', exit_code=130)
    finally:
        signal.signal(signal.SIGTERM, previous_handler)
        persist(summary, directory)
    return int(summary['status'] == 'stopped' or any(row['exit_code'] != 0 or row['status'] == 'error' for row in summary['controls']))


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ('primary-report', 'primary-metadata', 'baseline', 'candidate', 'output-dir'):
        parser.add_argument('--' + name, type=Path, required=True)
    for name in ('baseline-revision', 'candidate-revision', 'profile'):
        parser.add_argument('--' + name, required=True)
    args = parser.parse_args(argv)
    try:
        return run_attribution(args.primary_report, args.primary_metadata,
                               {label: getattr(args, label) for label in benchmark.LABELS},
                               {label: getattr(args, label + '_revision') for label in benchmark.LABELS},
                               args.output_dir, args.profile)
    except (OSError, ValueError) as error:
        print('Attribution setup failed: ' + str(error), file=sys.stderr)
        return 1


if __name__ == '__main__':
    sys.exit(main())
