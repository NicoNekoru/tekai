"""Pure/parser, modeled driver and Python-only supervision tests; no TeX."""

import contextlib
import copy
import io
import json
import os
from pathlib import Path
import signal
import sys
import tempfile
import time
import unittest
from unittest import mock

import benchmark_runtime as benchmark
import profile_runtime as profile


GNU = profile.RESOURCE_MARKER + ' 0.02 0.01 123 2 7 8 9 10 11 12\n'
BSD = '''0.03 real 0.02 user 0.01 sys
123 maximum resident set size
2 page faults
7 page reclaims
8 voluntary context switches
9 involuntary context switches
10 block input operations
11 block output operations
12 swaps
'''
VM = '''Mach Virtual Memory Statistics: (page size of 16384 bytes)
Pages free: 10.
Pages active: 20.
Pages inactive: 30.
Pages wired down: 40.
Pages occupied by compressor: 50.
Pageins: 60.
Pageouts: 70.
Swapins: 80.
Swapouts: 90.
'''
MEM = 'MemTotal: 1000 kB\nMemAvailable: 500 kB\nMemFree: 100 kB\nSwapTotal: 200 kB\nSwapFree: 150 kB\n'
TOP = 'CPU usage: 1.00% user, 2.00% sys, 97.00% idle\nCPU usage: 3.00% user, 4.00% sys, 93.00% idle\n'


def phase_record(version=4):
    phases = []
    sources = {1: profile.PHASE_SOURCES_V1, 2: profile.PHASE_SOURCES_V2,
               3: profile.PHASE_SOURCES_V3, 4: profile.PHASE_SOURCES_V4}[version]
    reasons = profile.CHILD_REASONS if version in (3, 4) else profile.NATIVE_REASONS_V2
    for name, source in sources.items():
        native = name in reasons
        phases.append({'name': name, 'status': 'unavailable' if native else 'available',
                       'elapsed_ms': None if native else 4.0, 'calls': 0 if native else 1,
                       'active_calls': 0, 'source': source,
                       'scope': ('engine_child_internal' if version in (3, 4) else 'native_engine_internal') if native else 'subprocess_launch_and_wait'
                                if name == 'tex_subprocess' else 'inclusive_wall_time_current_process',
                       'reason': reasons[name] if native else None})
    record = {'schema_version': version, 'producer': 'tekai-rust', 'scope': 'cli_process',
            'source': 'opt_in_rust_instrumentation', 'untrusted': True, 'status': 'success',
            'elapsed_ms': 10.0, 'completed_spans_only': True, 'phases_overlap': True, 'phases': phases}
    if version in (2, 3, 4):
        record['counters'] = [{'name': name, 'value': 1, 'unit': unit,
                              'scope': 'aggregate_observations_current_process'}
                             for name, unit in profile.COUNTER_UNITS.items()]
    return record


def phase_line(data=None):
    return 'TEKAI_PROFILE ' + json.dumps(phase_record() if data is None else data) + '\n'


def engine_record():
    return {'schema_version': 1, 'producer': 'tekai-engine-rust', 'scope': 'engine_invocation',
            'source': 'opt_in_rust_instrumentation', 'untrusted': True, 'status': 'success',
            'elapsed_ns': 50, 'completed_spans_only': True, 'phases_overlap': True,
            'lifecycle_partition_on_success': True,
            'worker_preparation': {'scope': 'cli_worker_before_invocation', 'elapsed_ns': 200},
            'phases': [{'name': name, 'status': 'available', 'calls': 1, 'failed_calls': 0,
                        'elapsed_ns': (5, 5, 35, 3, 2, 20, 10)[index]}
                       for index, name in enumerate(profile.ENGINE_PHASE_NAMES)],
            'counters': [{'name': name, 'unit': unit, 'value': 1}
                         for name, unit in profile.ENGINE_COUNTER_UNITS.items()]}


def engine_line(data=None):
    return 'TEKAI_ENGINE_PROFILE ' + json.dumps(engine_record() if data is None else data) + '\n'


class EnginePhaseTests(unittest.TestCase):
    def test_nested_phases_and_worker_preparation_stay_separate(self):
        result = profile.parse_engine_phases(phase_line() + engine_line())
        self.assertEqual(result['status'], 'reported')
        self.assertTrue(result['untrusted'])
        self.assertFalse(result['used_for_gate'])
        data = result['data']
        self.assertGreater(sum(phase['elapsed_ns'] for phase in data['phases']), data['elapsed_ns'])
        self.assertGreater(data['worker_preparation']['elapsed_ns'], data['elapsed_ns'])
        self.assertEqual(profile.parse_phases(engine_line())['status'], 'unavailable')
        self.assertEqual(profile.parse_engine_phases(phase_line())['status'], 'unavailable')

    def test_setup_failures_and_unobserved_codec_phases_are_not_invented(self):
        data = engine_record()
        data.update(status='setup_error', worker_preparation=None)
        for phase in data['phases']:
            phase.update(status='unavailable', calls=0, failed_calls=0, elapsed_ns=None)
        data['phases'][0].update(status='available', calls=1, failed_calls=1, elapsed_ns=10)
        for counter in data['counters']:
            counter['value'] = 0
        self.assertEqual(profile.parse_engine_phases(engine_line(data))['status'], 'reported')
        data['status'] = 'success'
        self.assertEqual(profile.parse_engine_phases(engine_line(data))['status'], 'invalid')

    def test_exact_protocol_and_fixed_inventories(self):
        mutations = (
            lambda data: data.update(schema_version=True),
            lambda data: data.update(producer='tekai-rust'),
            lambda data: data.update(scope='cli_process'),
            lambda data: data.update(untrusted=False),
            lambda data: data.update(completed_spans_only=False),
            lambda data: data.update(phases_overlap=False),
            lambda data: data.update(lifecycle_partition_on_success=False),
            lambda data: data.update(status='pass'),
            lambda data: data.update(unexpected=1),
            lambda data: data['phases'].pop(),
            lambda data: data['phases'].__setitem__(0, data['phases'][1]),
            lambda data: data['phases'][0].update(name=[]),
            lambda data: data['counters'].pop(),
            lambda data: data['counters'].__setitem__(0, data['counters'][1]),
            lambda data: data['counters'][0].update(name=[]),
            lambda data: data['counters'][0].update(unit='bytes'),
            lambda data: data['worker_preparation'].update(scope='engine_invocation'),
        )
        for index, mutate in enumerate(mutations):
            with self.subTest(mutation=index):
                data = engine_record()
                mutate(data)
                self.assertEqual(profile.parse_engine_phases(engine_line(data))['status'], 'invalid')

    def test_numeric_bounds_nulls_and_call_consistency(self):
        mutations = (
            lambda data, value: data.update(elapsed_ns=value),
            lambda data, value: data['phases'][0].update(elapsed_ns=value),
            lambda data, value: data['phases'][0].update(calls=value),
            lambda data, value: data['phases'][0].update(failed_calls=value),
            lambda data, value: data['counters'][0].update(value=value),
            lambda data, value: data['worker_preparation'].update(elapsed_ns=value),
        )
        for index, mutate in enumerate(mutations):
            for value in (True, None, -1, 1.5, float('nan'), float('inf'), profile.MAX_COUNTER + 1):
                with self.subTest(mutation=index, value=value):
                    data = engine_record()
                    mutate(data, value)
                    self.assertEqual(profile.parse_engine_phases(engine_line(data))['status'], 'invalid')
        for mutate in (
                lambda data: data['phases'][0].update(failed_calls=2),
                lambda data: data['phases'][0].update(elapsed_ns=101),
                lambda data: data['phases'][0].update(elapsed_ns=0),
                lambda data: data['phases'][0].update(status='unavailable'),
                lambda data: data['phases'][0].update(calls=0),
                lambda data: data['counters'][0].update(value=2)):
            data = engine_record()
            mutate(data)
            self.assertEqual(profile.parse_engine_phases(engine_line(data))['status'], 'invalid')

    def test_duplicate_truncated_and_oversized_captures_are_invalid(self):
        for text, truncated in ((engine_line() + engine_line(), False), (engine_line(), True),
                                (engine_line().replace('"schema_version": 1', '"schema_version": 1, "schema_version": 1'), False),
                                ('TEKAI_ENGINE_PROFILE {}\n', False),
                                ('TEKAI_ENGINE_PROFILE ' + '[' * 5000, False),
                                ('TEKAI_ENGINE_PROFILE ' + '[]' * 2500, False)):
            with self.subTest(text=text[:100], truncated=truncated):
                self.assertEqual(profile.parse_engine_phases(text, truncated)['status'], 'invalid')


class ResourceTests(unittest.TestCase):
    def test_units_and_scope(self):
        gnu = profile.parse_resources(GNU, 'gnu')
        bsd = profile.parse_resources(BSD, 'darwin')
        self.assertEqual(gnu['status'], 'reported')
        self.assertEqual(bsd['status'], 'reported')
        self.assertEqual(gnu['values']['peak_rss_bytes'], 123 * 1024)
        self.assertEqual(bsd['values']['peak_rss_bytes'], 123)
        self.assertEqual(gnu['values']['filesystem_inputs'], 10)
        self.assertEqual(gnu['values']['user_cpu_seconds'], 0.02)
        self.assertIn('not bytes', gnu['io_counter_units'])
        self.assertIn('not a process-tree RSS sum', gnu['scope'])
        self.assertTrue(gnu['untrusted'])
        self.assertFalse(gnu['used_for_gate'])
        self.assertEqual(gnu['cpu_timer_resolution_seconds'], 0.01)
        self.assertIn('zero does not establish zero CPU', gnu['precision_note'])

    def test_rounded_zero_is_reported_not_invented(self):
        result = profile.parse_resources(GNU.replace('0.02 0.01', '0.00 0.00'), 'gnu')
        self.assertEqual(result['status'], 'reported')
        self.assertEqual(result['values']['user_cpu_seconds'], 0)
        self.assertIs(profile.parse_resources('', None)['values']['user_cpu_seconds'], None)

    def test_invalid_gnu_models(self):
        for text in ('', GNU + GNU, GNU.replace('0.02', 'nan'), GNU.replace('0.02', 'inf'),
                     GNU.replace('0.02', '-1'), GNU.replace(' 123 ', ' -1 '),
                     GNU.replace(' 123 ', ' ' + str(profile.MAX_COUNTER) + ' '),
                     GNU.replace(' 123 ', ' 1.5 '), GNU.rstrip() + ' extra\n',
                     profile.RESOURCE_MARKER + ' ' + '9' * 10000):
            with self.subTest(text=text[:100]):
                result = profile.parse_resources(text, 'gnu')
                self.assertEqual(result['status'], 'unavailable')
                self.assertTrue(all(value is None for value in result['values'].values()))

    def test_invalid_bsd_models(self):
        for text in (BSD + BSD, BSD.replace('0.03', 'nan'), BSD.replace('0.01', '-0.1'),
                     BSD.replace('123 maximum', '-1 maximum'), BSD.replace('2 page faults\n', ''),
                     BSD + '2 page faults\n', BSD.replace('123 maximum', '1.5 maximum'),
                     BSD.replace('123 maximum', str(profile.MAX_COUNTER + 1) + ' maximum')):
            with self.subTest(text=text[:100]):
                self.assertEqual(profile.parse_resources(text, 'darwin')['status'], 'unavailable')

    def test_capture_limits_and_unavailable_timer(self):
        self.assertEqual(profile.parse_resources(GNU, 'gnu', True)['status'], 'unavailable')
        self.assertEqual(profile.parse_resources('a' * (profile.MAX_TEXT_BYTES + 1), 'gnu')['status'], 'unavailable')
        self.assertIsNone(profile.parse_resources(GNU, None)['rss_source_units'])
        self.assertEqual(profile.timer_command(None), [])


class ParentAndStartupTests(unittest.TestCase):
    def test_parent_inventory_has_gc_and_lifetime_rss_scope(self):
        row = profile.parent_snapshot()
        self.assertEqual(len(row['gc_counts']), 3)
        self.assertEqual(len(row['gc_stats']), 3)
        self.assertGreaterEqual(row['peak_rss_bytes'], 0)
        self.assertIn('not current RSS', row['rss_scope'])
        self.assertIn('outside', row['scope'])

    def test_startup_wrapper_does_not_claim_binary_stderr_is_empty(self):
        supervisor = mock.Mock(kind='gnu')
        supervisor.execute.return_value = {'stdout': 'tekai 0.5.0\n', 'stderr_tail': GNU, 'seconds': .1}
        fixture = {'command': ['tekai', '--version'], 'project': Path('/fixture'),
                   'env': {}, 'expected_version': '0.5.0'}
        result = profile.execute_profile_fixture(supervisor, fixture, 'cli-startup')
        self.assertEqual(result['output_validation']['stderr_verification'], 'unavailable_shared_wrapper_channel')
        self.assertFalse(result['output_validation']['used_for_gate'])
        supervisor.kind = None
        with self.assertRaises(benchmark.BenchmarkError):
            profile.execute_profile_fixture(supervisor, fixture, 'cli-startup')
        supervisor.execute.return_value['stderr_tail'] = ''
        self.assertEqual(profile.execute_profile_fixture(supervisor, fixture, 'cli-startup')
                         ['output_validation']['stderr_verification'], 'empty_verified')
        supervisor.execute.return_value['stdout'] = 'tekai 0.0.0\n'
        with self.assertRaises(benchmark.BenchmarkError):
            profile.execute_profile_fixture(supervisor, fixture, 'cli-startup')


class VolatilitySummaryTests(unittest.TestCase):
    def row(self, seconds, count, label='candidate'):
        data = phase_record()
        data['counters'][0]['value'] = count
        return {'case': 'warm-build-cache', 'phase': 'measurement', 'slot': 's0', 'label': label,
                'status': 'verified', 'command_id': count, 'repetition': count,
                'parent_seconds': seconds, 'phases': profile.parse_phases(phase_line(data)),
                'parent_before': {'peak_rss_bytes': 100, 'allocated_blocks': 50, 'gc_counts': [1, 2, 3]},
                'parent_after': {'peak_rss_bytes': 120, 'allocated_blocks': 55, 'gc_counts': [2, 2, 3]}}

    def test_descriptive_within_cell_stages_and_route_variants(self):
        summary = profile.volatility_summary([self.row(.002, 1), self.row(.004, 2)])
        cohort = summary['cohorts'][0]
        elapsed = cohort['parent_stages']['parent_seconds']
        self.assertAlmostEqual(elapsed['median'], .003)
        self.assertAlmostEqual(elapsed['median_absolute_deviation'], .001)
        self.assertAlmostEqual(elapsed['relative_median_absolute_deviation'], 1 / 3)
        self.assertEqual(elapsed['samples'], 2)
        self.assertEqual(cohort['counter_observations']['state_bytes_read']['distinct_values'], [1, 2])
        self.assertFalse(cohort['counter_observations']['state_bytes_read']['constant_observed'])
        self.assertEqual(cohort['child_peak_rss']['status'], 'unavailable')
        self.assertEqual(cohort['parent_resource_chronology'][1]['after_allocated_blocks'], 55)
        self.assertFalse(cohort['used_for_gate'])
        self.assertIn('No cross-artifact ratios', summary['scope'])

    def test_legacy_counters_remain_unavailable_and_cells_do_not_pool(self):
        row = self.row(.1, 1)
        row['phases'] = profile.parse_phases(phase_line(phase_record(1)))
        summary = profile.volatility_summary([row, self.row(.01, 1, label='baseline')])
        self.assertEqual(len(summary['cohorts']), 2)
        self.assertEqual(summary['cohorts'][0]['counter_observations']['metadata_checks']['status'], 'unavailable')
        self.assertIsNone(summary['cohorts'][0]['counter_observations']['metadata_checks']['constant_observed'])
        self.assertEqual(profile.describe([None, float('nan')], 'seconds')['status'], 'unavailable')

    def test_engine_library_worker_preparation_and_nested_codec_durations_stay_separate(self):
        candidate = self.row(.01, 1)
        candidate['engine_phases'] = profile.parse_engine_phases(engine_line())
        baseline = self.row(.01, 1, label='baseline')
        baseline['engine_phases'] = profile.parse_engine_phases('')
        cohorts = profile.volatility_summary([candidate, baseline])['cohorts']
        observed = next(row for row in cohorts if row['label'] == 'candidate')
        absent = next(row for row in cohorts if row['label'] == 'baseline')
        self.assertEqual(observed['engine_elapsed']['median'], 50)
        self.assertEqual(observed['worker_preparation']['median'], 200)
        self.assertEqual(observed['engine_phase_observations']['png_decode']['elapsed']['median'], 20)
        self.assertEqual(observed['engine_counter_observations']['decode_calls']['distinct_values'], [1])
        self.assertEqual(absent['engine_elapsed']['status'], 'unavailable')
        self.assertEqual(absent['engine_phase_observations']['png_decode']['elapsed']['samples'], 0)
        self.assertEqual(absent['engine_counter_observations']['decode_calls']['status'], 'unavailable')


class PhaseTests(unittest.TestCase):
    def test_valid_overlap_is_not_a_partition(self):
        result = profile.parse_phases(phase_line())
        self.assertEqual(result['status'], 'reported')
        self.assertTrue(result['untrusted'])
        self.assertFalse(result['used_for_gate'])
        self.assertGreater(sum(p['elapsed_ms'] or 0 for p in result['data']['phases']), result['data']['elapsed_ms'])

    def test_absent_baseline_and_native_protocol_unavailable(self):
        self.assertEqual(profile.parse_phases('other stderr\n')['status'], 'unavailable')

    def test_legacy_protocol_explicitly_lacks_granular_inventory(self):
        result = profile.parse_phases(phase_line(phase_record(1)))
        self.assertEqual(result['status'], 'reported')
        self.assertEqual(result['compatibility']['granular_observations'], 'unavailable_legacy_schema')
        self.assertFalse(result['compatibility']['counters_available'])
        self.assertIn('input_metadata', result['compatibility']['missing_phase_names'])

    def test_schema2_keeps_legacy_engine_labels_and_granular_observations(self):
        result = profile.parse_phases(phase_line(phase_record(2)))
        self.assertEqual(result['status'], 'reported')
        self.assertEqual(result['compatibility']['granular_observations'], 'available')
        self.assertTrue(result['compatibility']['counters_available'])
        self.assertEqual(result['compatibility']['missing_phase_names'], [])

    def test_build_state_parser_source_is_versioned_without_relabeling_old_records(self):
        for version in (2, 3, 4):
            data = phase_record(version)
            entry = next(phase for phase in data['phases'] if phase['name'] == 'build_state_parse')
            expected = 'compiler build-state JSON parse' if version == 4 else 'compiler build-state TOML parse'
            with self.subTest(version=version, mutation='original source'):
                result = profile.parse_phases(phase_line(data))
                self.assertEqual(result['status'], 'reported')
                observed = next(phase for phase in result['data']['phases'] if phase['name'] == 'build_state_parse')
                self.assertEqual(observed['source'], expected)
                self.assertTrue(result['compatibility']['counters_available'])
                self.assertEqual(result['compatibility']['missing_phase_names'], [])
            entry['source'] = 'compiler build-state TOML parse' if version == 4 else 'compiler build-state JSON parse'
            with self.subTest(version=version, mutation='wrong version source'):
                self.assertEqual(profile.parse_phases(phase_line(data))['status'], 'invalid')

    def test_engine_unavailable_reasons_are_versioned_without_invented_measurements(self):
        for version in (1, 2, 3, 4):
            data = phase_record(version)
            entry = next(phase for phase in data['phases'] if phase['name'] == 'embedded_engine_entry')
            incompatible = profile.NATIVE_REASONS_V2 if version in (3, 4) else profile.CHILD_REASONS
            entry['reason'] = incompatible['embedded_engine_entry']
            with self.subTest(version=version, mutation='wrong version reason'):
                self.assertEqual(profile.parse_phases(phase_line(data))['status'], 'invalid')
            data = phase_record(version)
            entry = next(phase for phase in data['phases'] if phase['name'] == 'embedded_engine_entry')
            entry.update(status='available', elapsed_ms=1, calls=1, reason=None)
            with self.subTest(version=version, mutation='invented child measurement'):
                self.assertEqual(profile.parse_phases(phase_line(data))['status'], 'invalid')

    def test_unfinished_calls_are_not_invented(self):
        data = phase_record()
        data['phases'][0].update(status='unavailable', elapsed_ms=None, calls=0, active_calls=1,
                                 reason='no_completed_call_in_this_process')
        data['phases'][1]['active_calls'] = 2
        self.assertEqual(profile.parse_phases(phase_line(data))['status'], 'reported')

    def test_required_top_level_contract(self):
        mutations = ({}, [], {'bad': 1})
        for data in mutations:
            self.assertEqual(profile.parse_phases(phase_line(data))['status'], 'invalid')
        for key, value in (('schema_version', True), ('schema_version', 5), ('producer', 'other'),
                           ('scope', 'tree'), ('source', 'guessed'), ('untrusted', False),
                           ('completed_spans_only', False), ('phases_overlap', False),
                           ('status', 'pass'), ('status', []), ('elapsed_ms', -1), ('elapsed_ms', True),
                           ('elapsed_ms', float('nan')), ('elapsed_ms', float('inf'))):
            data = phase_record()
            data[key] = value
            with self.subTest(key=key, value=value):
                self.assertEqual(profile.parse_phases(phase_line(data))['status'], 'invalid')
        data = phase_record()
        data['extra'] = True
        self.assertEqual(profile.parse_phases(phase_line(data))['status'], 'invalid')

    def test_exact_fixed_inventory(self):
        for edit in (lambda d: d['phases'].pop(),
                     lambda d: d['phases'].__setitem__(0, d['phases'][1]),
                     lambda d: d['phases'][0].update(name=[]),
                     lambda d: d['phases'][0].update(name='unknown'),
                     lambda d: d['phases'].__setitem__(0, None)):
            data = phase_record()
            edit(data)
            self.assertEqual(profile.parse_phases(phase_line(data))['status'], 'invalid')

    def test_exact_phase_values(self):
        for key, value in (('status', 'success'), ('status', []), ('calls', True), ('calls', -1),
                           ('calls', 0), ('calls', 1.5), ('active_calls', True), ('active_calls', -1),
                           ('source', 'other'), ('scope', 'exclusive'), ('elapsed_ms', None),
                           ('elapsed_ms', -1), ('reason', 'other'), ('extra', 0)):
            data = phase_record()
            data['phases'][0][key] = value
            with self.subTest(key=key, value=value):
                self.assertEqual(profile.parse_phases(phase_line(data))['status'], 'invalid')

    def test_native_internals_must_remain_unavailable(self):
        for update in ({'status': 'available', 'elapsed_ms': 1, 'calls': 1, 'reason': None},
                       {'reason': 'other'}, {'calls': 1}, {'elapsed_ms': 0}):
            data = phase_record()
            data['phases'][list(profile.PHASE_SOURCES).index('format_initialization')].update(update)
            self.assertEqual(profile.parse_phases(phase_line(data))['status'], 'invalid')

    def test_finite_capture_and_json_limits(self):
        for text, truncated in ((phase_line() + phase_line(), False), (phase_line(), True),
                                ('TEKAI_PROFILE {\n', False), ('TEKAI_PROFILE ' + 'x' * profile.MAX_PHASE_BYTES, False),
                                ('TEKAI_PROFILE ' + '[' * 1100 + '0' + ']' * 1100, False)):
            self.assertEqual(profile.parse_phases(text, truncated)['status'], 'invalid')
        data = phase_record()
        data['phases'][0]['source'] = 'x' * 257
        self.assertEqual(profile.parse_phases(phase_line(data))['status'], 'invalid')

    def test_fixed_counter_contract_rejects_wrong_routes_and_types(self):
        for edit in (lambda d: d['counters'].pop(), lambda d: d['counters'].__setitem__(0, d['counters'][1]),
                     lambda d: d['counters'][0].update(value=-1), lambda d: d['counters'][0].update(value=True),
                     lambda d: d['counters'][0].update(value=1.5), lambda d: d['counters'][0].update(unit='seconds'),
                     lambda d: d['counters'][0].update(scope='whole_job'), lambda d: d['counters'][0].update(extra=1)):
            data = phase_record()
            edit(data)
            self.assertEqual(profile.parse_phases(phase_line(data))['status'], 'invalid')


class HostParserTests(unittest.TestCase):
    def test_vm_stat(self):
        value = profile.parse_vm_stat(VM)
        self.assertEqual(value['page_size_bytes'], 16384)
        self.assertEqual(value['memory_bytes']['Pages free'], 10 * 16384)
        self.assertEqual(value['vm_counters']['Swapouts'], 90)
        self.assertIsNone(profile.parse_vm_stat(VM.replace('Swapouts: 90.\n', ''))['vm_counters']['Swapouts'])
        for text in ('', VM.replace('16384', '0'), VM + 'Pages free: 10.\n',
                     VM.replace('Pages free: 10.', 'Pages free: ' + str(profile.MAX_COUNTER) + '.')):
            with self.assertRaises(ValueError):
                profile.parse_vm_stat(text)

    def test_swap_units_and_consistency(self):
        self.assertEqual(profile.parse_swap('total = 1.00G used = 256.00M free = 768.00M'),
                         {'total': 1024 ** 3, 'used': 256 * 1024 ** 2, 'free': 768 * 1024 ** 2})
        for text in ('', 'total=1G used=2G free=0G', 'total=1G used=0G free=0G',
                     'total=1G total=1G used=0G free=1G', 'total=nanG used=0G free=1G',
                     'total=99999999999999999999999T used=0G free=1G'):
            with self.assertRaises(ValueError):
                profile.parse_swap(text)

    def test_two_cpu_samples_use_last_interval(self):
        self.assertEqual(profile.parse_host_cpu(TOP), {'user': 3, 'system': 4, 'idle': 93})
        for text in ('', TOP.splitlines()[0], TOP + TOP, TOP.replace('93.00', '193.00'),
                     TOP.replace('93.00', '80.00'), TOP.replace('93.00', 'nan')):
            with self.assertRaises(ValueError):
                profile.parse_host_cpu(text)

    def test_linux_counters_and_units(self):
        value = profile.parse_linux_host('cpu 1 2 3 4 5 6 7 8 9 10\n', MEM)
        self.assertEqual(value['cpu_ticks']['guest_nice'], 10)
        self.assertEqual(value['memory_bytes']['MemTotal'], 1000 * 1024)
        self.assertEqual(value['swap_bytes']['used'], 50 * 1024)
        for stat, memory in (('', MEM), ('cpu 1 2 3\n', MEM), ('cpu -1 2 3 4\n', MEM),
                             ('cpu 1 x 3 4\n', MEM), ('cpu 1 2 3 4\n', MEM + 'MemFree: 1 kB\n'),
                             ('cpu 1 2 3 4\n', MEM.replace('SwapFree: 150', 'SwapFree: 250')),
                             ('cpu 1 2 3 4\n', MEM.replace('MemFree: 100', 'MemFree: 2000'))):
            with self.assertRaises(ValueError):
                profile.parse_linux_host(stat, memory)

    def test_linux_available_memory_cannot_exceed_total(self):
        with self.assertRaises(ValueError):
            profile.parse_linux_host('cpu 1 2 3 4\n', MEM.replace('MemAvailable: 500', 'MemAvailable: 2000'))

    def test_bounded_parent_read(self):
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / 'text'
            path.write_bytes(b'x' * (profile.MAX_TEXT_BYTES + 1))
            with self.assertRaises(ValueError):
                profile.bounded_text(path)

    def test_darwin_observers_are_bounded_and_restore_timeout(self):
        supervisor = mock.Mock(timeout=30, work=Path('/fixture'))
        supervisor.execute.side_effect = [{'stdout': VM}, {'stdout': 'total=1G used=0G free=1G'}, {'stdout': TOP}]
        with mock.patch.object(profile.platform, 'system', return_value='Darwin'), \
                mock.patch.object(profile.Path, 'is_file', return_value=True), \
                mock.patch.object(profile.os, 'getloadavg', return_value=(1, 2, 3)):
            row = profile.host_snapshot(supervisor, {})
        self.assertEqual(supervisor.timeout, 30)
        self.assertEqual(row['cpu_percent']['idle'], 93)
        self.assertEqual(supervisor.execute.call_args_list[-1].args[0], ['/usr/bin/top', '-l', '2', '-s', '1', '-n', '0'])
        self.assertNotIn('TEKAI_DIAGNOSTIC_PROFILE', supervisor.execute.call_args_list[-1].args[2])

    def test_denied_observers_are_explicit(self):
        supervisor = mock.Mock(timeout=30, work=Path('/fixture'))
        supervisor.execute.side_effect = benchmark.BenchmarkError('denied')
        with mock.patch.object(profile.platform, 'system', return_value='Darwin'), \
                mock.patch.object(profile.Path, 'is_file', return_value=True), \
                mock.patch.object(profile.os, 'getloadavg', side_effect=OSError('denied')):
            row = profile.host_snapshot(supervisor, {})
        self.assertEqual(supervisor.timeout, 30)
        self.assertIsNone(row['cpu_percent'])
        self.assertEqual(set(row['unavailable']), {'load_average', 'vm', 'swap_bytes', 'cpu_percent', 'cpu_ticks'})


class FixtureFiles(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name).resolve()
        self.sources = {}
        for label in benchmark.LABELS:
            path = self.root / label
            path.write_bytes(('fixture ' + label).encode())
            path.chmod(0o700)
            self.sources[label] = path
        self.metadata = {'runner_label': 'fixture', 'toolchain': 'fixture',
                         **{label: {'revision': 'fixture-' + label, 'build_command': 'fixture-only',
                                    'expected_version': '0.5.0' if label == 'baseline' else '0.6.0-rc.1+test',
                                    'artifact_sha256': benchmark.executable_sha256(path)}
                            for label, path in self.sources.items()}}
        self.primary = {'schema_version': 5, 'gate': True, 'status': 'inconclusive', 'exit_code': 1,
                        'metadata': self.metadata, 'executables': []}
        for label, path in self.sources.items():
            digest = benchmark.executable_sha256(path)
            for slot in benchmark.SLOTS:
                self.primary['executables'].append({'label': label, 'slot': slot, 'source_path': str(path),
                    'source_sha256_start': digest, 'source_sha256_end': digest,
                    'sha256_start': digest, 'sha256_end': digest, 'unchanged': True})
        self.metadata_path = self.root / 'metadata.json'
        self.primary_path = self.root / 'primary.json'
        self.metadata_path.write_text(json.dumps(self.metadata))
        self.primary_path.write_text(json.dumps(self.primary))
        self.output = self.root / 'diagnostic.json'
        self.verifiers = {}
        for name in ('pdftotext', 'pdfinfo', 'pdfimages'):
            path = self.root / name
            path.write_bytes(b'fixture verifier')
            self.verifiers[name] = str(path)

    def argv(self, *extra):
        return ['--baseline', str(self.sources['baseline']), '--candidate', str(self.sources['candidate']),
                '--metadata', str(self.metadata_path), '--primary-report', str(self.primary_path),
                '--output', str(self.output), *extra]


class BindingTests(FixtureFiles):
    def test_primary_status_is_preserved_not_rescored(self):
        for schema in (4, 5, 6):
            for status in ('pass', 'fail', 'inconclusive', 'error', 'incomplete'):
                data = copy.deepcopy(self.primary)
                data.update(schema_version=schema, status=status)
                if schema == 6:
                    data['report_kind'] = 'complete-report'
                    data['raw_command_journal'] = {'path': str(self.root / 'primary.raw.fixture.jsonl'),
                                                  'format': 'tekai-command-journal-v1'}
                self.assertEqual(profile.bind_primary(data, self.metadata, self.sources),
                                 {label: self.metadata[label]['artifact_sha256'] for label in benchmark.LABELS})

    def test_compact_schema_6_checkpoint_cannot_bind_as_final_primary(self):
        data = {**self.primary, 'schema_version': 6, 'report_kind': 'compact-checkpoint'}
        with self.assertRaisesRegex(ValueError, 'final complete report'):
            profile.bind_primary(data, self.metadata, self.sources)

    def test_schema_6_requires_journal_but_legacy_reports_do_not(self):
        data = {**self.primary, 'schema_version': 6, 'report_kind': 'complete-report'}
        with self.assertRaisesRegex(ValueError, 'raw command journal binding'):
            profile.bind_primary(data, self.metadata, self.sources)
        with self.assertRaisesRegex(ValueError, 'raw command journal binding'):
            profile.primary_journal_path(data, self.primary_path)
        for schema in (4, 5):
            legacy = {**self.primary, 'schema_version': schema}
            self.assertEqual(profile.bind_primary(legacy, self.metadata, self.sources),
                             {label: self.metadata[label]['artifact_sha256'] for label in benchmark.LABELS})
            self.assertIsNone(profile.primary_journal_path(legacy, self.primary_path))

    def test_rejects_unbound_primary(self):
        for edit in (lambda d: d.update(gate=False), lambda d: d.update(schema_version=True),
                     lambda d: d.update(status='unknown'), lambda d: d.update(metadata={}),
                     lambda d: d.update(executables=[]), lambda d: d['executables'][0].update(unchanged=False),
                     lambda d: d['executables'][0].update(sha256_end='0' * 64),
                     lambda d: d['executables'][0].update(source_path='relative'),
                     lambda d: d['executables'][0].update(source_path=str(self.sources['candidate'])),
                     lambda d: d['executables'][0].update(source_sha256_start='not-a-hash')):
            data = copy.deepcopy(self.primary)
            edit(data)
            with self.assertRaises(ValueError):
                profile.bind_primary(data, self.metadata, self.sources)

    def test_current_changed_or_missing_source_rejected(self):
        self.sources['candidate'].write_bytes(b'changed')
        with self.assertRaises(ValueError):
            profile.bind_primary(self.primary, self.metadata, self.sources)
        self.sources['candidate'].unlink()
        with self.assertRaises(OSError):
            profile.bind_primary(self.primary, self.metadata, self.sources)

    def test_input_json_shape_nonfinite_and_bound(self):
        for content in ('[]', '{"a":NaN}', '{"a":Infinity}', '{'):
            self.primary_path.write_text(content)
            with self.assertRaises(ValueError):
                profile.read_json(self.primary_path)
        self.primary_path.write_bytes(b'x' * 101)
        with mock.patch.object(profile, 'MAX_REPORT_BYTES', 100), self.assertRaises(ValueError):
            profile.read_json(self.primary_path)

    def test_finite_cli_bounds(self):
        for arguments in (('--repeats', '0'), ('--repeats', '17'), ('--timeout', 'nan'),
                          ('--timeout', '31'), ('--budget', 'inf'), ('--budget', '0'), ('--budget', '601')):
            with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as error:
                profile.main(self.argv(*arguments))
            self.assertEqual(error.exception.code, 2)
            self.assertFalse(self.output.exists())

    def test_primary_journal_is_bound_and_cannot_be_overwritten(self):
        journal = self.root / 'primary.raw.fixture.jsonl'
        journal.write_bytes(b'{"sequence":0}\n')
        self.primary['raw_command_journal'] = {'path': str(journal), 'bytes': journal.stat().st_size,
            'sha256': benchmark.executable_sha256(journal), 'format': 'tekai-command-journal-v1'}
        self.primary_path.write_text(json.dumps(self.primary))
        self.assertEqual(profile.primary_journal_path(self.primary, self.primary_path), journal)
        os.link(journal, self.output)
        with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
            profile.main(self.argv())
        self.assertEqual(journal.read_bytes(), b'{"sequence":0}\n')

    def test_unbounded_or_external_journal_rejected_before_reading(self):
        for journal in ({'path': '/dev/zero', 'bytes': 0, 'sha256': '0' * 64,
                         'format': 'tekai-command-journal-v1'},
                        {'path': str(self.primary_path), 'bytes': profile.MAX_REPORT_BYTES + 1,
                         'sha256': '0' * 64, 'format': 'tekai-command-journal-v1'}):
            data = {**self.primary, 'raw_command_journal': journal}
            with self.assertRaises(ValueError):
                profile.primary_journal_path(data, self.primary_path)

    def test_outputs_cannot_alias_protected_inputs(self):
        original = self.primary_path.read_bytes()
        with mock.patch.object(profile.shutil, 'which', side_effect=self.verifiers.get):
            for source in (*self.sources.values(), self.metadata_path, self.primary_path,
                           *(Path(value) for value in self.verifiers.values())):
                if self.output.exists():
                    self.output.unlink()
                os.link(source, self.output)
                with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
                    profile.main(self.argv())
        self.assertEqual(self.primary_path.read_bytes(), original)

    def test_companion_markdown_and_symlink_alias_are_protected(self):
        companion = self.output.with_suffix('.md')
        companion.symlink_to(self.sources['candidate'])
        original = self.sources['candidate'].read_bytes()
        with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
            profile.main(self.argv())
        self.assertEqual(self.sources['candidate'].read_bytes(), original)
        self.assertFalse(self.output.exists())

    def test_primary_markdown_alias_is_protected_before_initial_persist(self):
        primary_markdown = self.primary_path.with_suffix('.md')
        primary_markdown.write_text('Primary outcome must remain unchanged')
        for symlink in (False, True):
            companion = self.output.with_suffix('.md')
            if companion.exists():
                companion.unlink()
            if symlink:
                companion.symlink_to(primary_markdown)
            else:
                os.link(primary_markdown, companion)
            with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
                profile.main(self.argv())
            self.assertFalse(self.output.exists())
            self.assertEqual(primary_markdown.read_text(), 'Primary outcome must remain unchanged')

    def test_invalid_primary_produces_error_without_rewriting_it(self):
        self.primary_path.write_text('{')
        original = self.primary_path.read_bytes()
        self.assertEqual(profile.main(self.argv()), 1)
        report = profile.read_json(self.output)
        self.assertEqual(report['status'], 'error')
        self.assertTrue(report['primary']['unchanged'])
        self.assertEqual(self.primary_path.read_bytes(), original)

    def test_nonfinite_optional_metadata_is_an_honest_error(self):
        self.metadata_path.write_text('{"extra":NaN}')
        self.assertEqual(profile.main(self.argv()), 1)
        self.assertEqual(profile.read_json(self.output)['status'], 'error')

    def test_output_and_nan_limits(self):
        report = {'status': 'complete', 'commands': [], 'errors': []}
        with mock.patch.object(profile, 'MAX_PROFILE_BYTES', 10), self.assertRaises(ValueError):
            profile.persist(report, self.output)
        report['bad'] = float('nan')
        with self.assertRaises(ValueError):
            profile.persist(report, self.output)


class DriverTests(FixtureFiles):
    def run_model(self, *, fail_at=None, interrupt=False, mutation=None, copy_mismatch=False,
                  timer_available=True, missing_verifier=False, image_route_controls=False,
                  route_error=False, route_mutation=None):
        commands, hosts = [], []
        original_primary = self.primary_path.read_bytes()
        real_copy = profile.shutil.copy2

        def make_fixture(root, binary, case):
            project = root / 'project'
            project.mkdir(parents=True)
            (project / 'main.tex').write_text('fixture ' + case)
            fixture = {'root': root, 'project': project, 'env': {'FIXTURE': '1'},
                       'command': [binary, case], 'input_sha256': benchmark.fixture_sha256(project)}
            if image_route_controls and case == 'image-compile':
                out = root / 'out'
                out.mkdir()
                fixture['out'] = out
                fixture['command'] = [binary, profile.PUBLIC_ENGINE_ROUTE, '-interaction=nonstopmode',
                                      '-halt-on-error', '-no-shell-escape', f'-output-directory={out}', 'main.tex']
                for kind in ('ENGINE', 'FORMAT', 'AUX', 'BIBTEX'):
                    path = root / ('cache-' + kind.lower())
                    path.mkdir()
                    fixture['env']['TEKAI_' + kind + '_CACHE'] = str(path)
                (root / 'cache-engine/complete-fixture').write_bytes(b'immutable model cache')
            return fixture

        def timer(supervisor, env):
            supervisor.kind = 'gnu' if timer_available else None
            supervisor.timer_reason = None if timer_available else 'fixture timer unavailable'

        def execute(supervisor, command, cwd, env):
            supervisor.counter += 1
            commands.append((list(map(str, command)), dict(env)))
            active = supervisor.active is not None and str(command[0]) != self.verifiers['pdftotext']
            raw = active and supervisor.active.get('unwrapped_route_control') is True
            if profile.PRIVATE_ENGINE_ROUTE in command and supervisor.active['label'] == 'baseline':
                raise AssertionError('The baseline has no private worker route')
            return {'command_id': supervisor.counter, 'seconds': .01, 'parent_timing': [10, 10.001, 10.01],
                    'command_timing': [10, 10, 10.0001, 10.0002, 10.0008, 10.0009,
                                       10.001, 10.0011, 10.0012, 10.01, 10.0101, 10.0102],
                    'stderr_tail': (GNU if timer_available and active and not raw else '') +
                                   (phase_line() if active and supervisor.active['label'] == 'candidate'
                                    and supervisor.active['case'] == 'warm-build-cache'
                                    and supervisor.active['phase'] != 'launch_only_control' else '') +
                                   (engine_line() if active and not raw and supervisor.active['label'] == 'candidate'
                                    and supervisor.active['case'] == 'image-compile' else ''), 'stdout': '{}'}

        def fixture_run(supervisor, fixture, case, initializing=False):
            if case == 'cli-startup':
                self.assertEqual(fixture['expected_version'], self.metadata[fixture['label']]['expected_version'])
            result = supervisor.execute(fixture['command'], fixture['project'], fixture['env'])
            if supervisor.active and supervisor.active.get('unwrapped_route_control'):
                if route_mutation:
                    route_mutation(supervisor, fixture)
                if route_error and supervisor.active['route'] == 'worker':
                    raise benchmark.BenchmarkError('Candidate private worker route is unsupported')
            if mutation:
                mutation(supervisor, fixture)
            if fail_at is not None and len(commands) == fail_at:
                raise KeyboardInterrupt if interrupt else benchmark.BenchmarkError('fixture oracle failed')
            oracle = supervisor.execute([fixture['pdftotext'], 'oracle'], fixture['project'], fixture['env'])
            result['output_validation'] = {'text_verified': True, 'verifier_command_id': oracle['command_id']}
            if case == 'cli-startup':
                result['output_validation']['expected_version'] = fixture['expected_version']
            if case == 'warm-build-cache':
                result['reported_build_timing'] = {'untrusted': True, 'status': 'reported', 'reported_elapsed_ms': 1}
            return result

        def host(supervisor, env):
            hosts.append(True)
            return {'unix_ns': 100, 'monotonic_ns': 100, 'load_average': [1, 2, 3], 'scope': 'fixture'}

        def copied(source, destination):
            result = real_copy(source, destination)
            if copy_mismatch:
                destination.write_bytes(b'wrong copy')
            return result

        selected = dict(self.verifiers)
        if missing_verifier:
            selected['pdfimages'] = None
        with mock.patch.object(profile.shutil, 'which', side_effect=selected.get), \
                mock.patch.object(profile.shutil, 'copy2', side_effect=copied), \
                mock.patch.object(profile.ProfileSupervisor, 'probe_timer', timer), \
                mock.patch.object(benchmark.Supervisor, 'execute', execute), \
                mock.patch.object(benchmark, 'make_fixture', side_effect=make_fixture), \
                mock.patch.object(benchmark, 'cache_state_snapshot', return_value={'sha256': 'a' * 64,
                                       'recorded_input_count': 1025, 'used_for_gate': False}), \
                mock.patch.object(benchmark, 'execute_fixture', side_effect=fixture_run), \
                mock.patch.object(profile, 'execute_profile_fixture', side_effect=fixture_run), \
                mock.patch.object(profile, 'host_snapshot', side_effect=host):
            code = profile.main(self.argv(*(['--image-route-controls'] if image_route_controls else [])))
        self.assertEqual(self.primary_path.read_bytes(), original_primary)
        return code, profile.read_json(self.output), commands, hosts

    def test_fixed_isolated_verified_diagnostics_do_not_rescore_primary(self):
        self.primary_path.with_suffix('.md').write_text('Original primary summary')
        code, report, commands, hosts = self.run_model()
        self.assertEqual(code, 0)
        self.assertEqual(report['status'], 'complete')
        self.assertEqual(report['primary']['status'], 'inconclusive')
        self.assertEqual(report['primary']['exit_code'], 1)
        self.assertTrue(report['diagnostic_only'])
        self.assertFalse(report['used_for_gate'])
        self.assertEqual(len(report['commands']), 176)
        self.assertEqual(len(report['units']), 36)
        self.assertEqual(len(hosts), 72)
        self.assertTrue(all(row['status'] == 'verified' and row['output_validation']['text_verified'] for row in report['commands']))
        self.assertTrue(all(row['unchanged'] for row in report['executables'] + report['protected_inputs']))
        self.assertTrue(report['primary']['unchanged'])
        self.assertEqual({row['slot'] for row in report['executables']}, {'s0', 's1'})
        for row in report['commands']:
            self.assertEqual(row['parent_timing'], [10, 10.001, 10.01])
            self.assertAlmostEqual(row['launch_setup_seconds'], .001)
            self.assertAlmostEqual(row['popen_latency_seconds'], .0006)
            self.assertAlmostEqual(row['capture_setup_seconds'], .0001)
            self.assertAlmostEqual(row['waiter_dispatch_seconds'], .0001)
            self.assertAlmostEqual(row['cleanup_seconds'], .0001)
            self.assertIn('gc_counts', row['parent_before'])
            self.assertIn('peak_rss_bytes', row['parent_after'])
            self.assertNotIn('speedup', row)
            self.assertEqual(Path(row['command'][0]).name, 'tekai')
            self.assertTrue(row['finish_unix_ns'] >= row['launch_unix_ns'])
            self.assertEqual(row['resources']['status'], 'reported')
            expected = 'reported' if row['label'] == 'candidate' and row['case'] == 'warm-build-cache' \
                                    and row['phase'] != 'launch_only_control' else 'unavailable'
            self.assertEqual(row['phases']['status'], expected)
        for command, env in commands:
            if command[0] == self.verifiers['pdftotext']:
                self.assertNotIn('TEKAI_DIAGNOSTIC_PROFILE', env)
            else:
                self.assertEqual(command[:2], ['/usr/bin/time', '-f'])
                self.assertEqual(env['TEKAI_DIAGNOSTIC_PROFILE'], '1')
        self.assertTrue(self.output.with_suffix('.md').is_file())
        self.assertEqual(self.primary_path.with_suffix('.md').read_text(), 'Original primary summary')
        self.assertTrue(any(row['path'] == str(self.primary_path.with_suffix('.md')) and row['unchanged']
                            for row in report['protected_inputs']))
        controls = [row for row in report['commands'] if row['phase'] == 'launch_only_control']
        self.assertEqual(len(controls), 32)
        self.assertEqual({row['repetition'] for row in controls}, set(range(8)))
        self.assertTrue(all(row['same_artifact_sha256'] == self.metadata[row['label']]['artifact_sha256']
                            for row in controls))
        for row in controls:
            self.assertEqual(row['output_validation']['expected_version'], self.metadata[row['label']]['expected_version'])
        self.assertNotEqual(self.metadata['baseline']['expected_version'], self.metadata['candidate']['expected_version'])
        cache_rows = [row for row in report['commands'] if row['case'] == 'warm-build-cache'
                      and row['phase'] == 'measurement']
        self.assertTrue(all(row['cache_state_unchanged'] for row in cache_rows))
        self.assertTrue(all(row['unit_id'] in {unit['unit_id'] for unit in report['units']} for row in report['commands']))

    def test_partial_oracle_failure_preserves_command_evidence_and_stops(self):
        code, report, commands, hosts = self.run_model(fail_at=1)
        self.assertEqual(code, 1)
        self.assertEqual(report['status'], 'error')
        self.assertEqual(len(report['commands']), 1)
        self.assertEqual(len(commands), 1)
        self.assertEqual(len(hosts), 1)
        row = report['commands'][0]
        self.assertEqual(row['status'], 'error')
        self.assertEqual(row['resources']['status'], 'reported')
        self.assertIsNone(row['output_validation'])
        self.assertIn('Unit incomplete', report['units'][0]['after']['unavailable']['host'])

    def test_interrupt_is_not_followed_by_observer_commands(self):
        code, report, commands, hosts = self.run_model(fail_at=1, interrupt=True)
        self.assertEqual(code, 1)
        self.assertEqual(len(hosts), 1)
        self.assertTrue(any('interrupted' in error for error in report['errors']))
        self.assertTrue(all(row['unchanged'] for row in report['executables']))

    def test_unavailable_timer_is_explicit_not_zero(self):
        code, report, commands, _ = self.run_model(timer_available=False)
        self.assertEqual(code, 0)
        self.assertIsNone(report['timer']['kind'])
        self.assertTrue(all(row['resources']['status'] == 'unavailable' and
                            row['resources']['values']['peak_rss_bytes'] is None for row in report['commands']))
        self.assertFalse(any(command[0] == '/usr/bin/time' for command, _ in commands))

    def test_missing_oracle_is_diagnostic_error(self):
        code, report, commands, hosts = self.run_model(missing_verifier=True)
        self.assertEqual(code, 1)
        self.assertEqual(commands, [])
        self.assertEqual(hosts, [])
        self.assertTrue(any('Poppler' in error for error in report['errors']))

    def test_copy_mismatch_stops_before_commands(self):
        code, report, commands, hosts = self.run_model(copy_mismatch=True)
        self.assertEqual(code, 1)
        self.assertEqual(commands, [])
        self.assertEqual(hosts, [])
        self.assertTrue(any('isolated copy differs' in error for error in report['errors']))
        self.assertTrue(all('sha256_end' in row for row in report['executables']))

    def test_source_or_verifier_change_cannot_be_complete(self):
        for target in (self.sources['candidate'], Path(self.verifiers['pdftotext'])):
            with self.subTest(target=target):
                original = target.read_bytes()
                code, report, _, _ = self.run_model(mutation=lambda _s, _f: target.write_bytes(b'changed'))
                self.assertEqual(code, 1)
                self.assertTrue(any('Protected' in error for error in report['errors']))
                target.write_bytes(original)

    def test_copied_executable_mutation_and_disappearance(self):
        for remove in (False, True):
            with self.subTest(remove=remove):
                def mutate(_supervisor, fixture):
                    path = Path(fixture['command'][0])
                    if remove:
                        path.unlink(missing_ok=True)
                    else:
                        path.write_bytes(b'changed')
                code, report, _, _ = self.run_model(mutation=mutate)
                self.assertEqual(code, 1)
                self.assertTrue(any(not row['unchanged'] for row in report['executables']))
                self.assertTrue(any('copy changed or disappeared' in error for error in report['errors']))

    def test_image_controls_are_opt_in_candidate_only_unwrapped_and_balanced(self):
        code, report, commands, hosts = self.run_model(image_route_controls=True)
        self.assertEqual(code, 0)
        control = report['image_route_controls']
        self.assertEqual(control['status'], 'complete')
        self.assertFalse(control['used_for_gate'])
        self.assertEqual(control['candidate_revision'], self.metadata['candidate']['revision'])
        self.assertEqual(control['binary_sha256'], self.metadata['candidate']['artifact_sha256'])
        self.assertEqual(control['completion']['complete_pairs'], 16)
        self.assertTrue(control['completion']['equal_completed_balance'])
        self.assertEqual(control['completion']['completed_route_counts'],
                         {'s0': {'public': 8, 'worker': 8}, 's1': {'public': 8, 'worker': 8}})
        self.assertEqual(control['completion']['first_route_counts'],
                         {'s0': {'public': 4, 'worker': 4}, 's1': {'public': 4, 'worker': 4}})
        self.assertEqual(control['completion']['replica_ordinal_counts'],
                         {'s0': {'0': 4, '1': 4}, 's1': {'0': 4, '1': 4}})
        rows = [row for row in report['commands'] if row.get('unwrapped_route_control')]
        self.assertEqual(len(rows), 32)
        self.assertEqual(len(hosts), 72)
        self.assertEqual(report['primary']['status'], 'inconclusive')
        self.assertTrue(report['primary']['unchanged'])
        for row in rows:
            self.assertEqual(row['label'], 'candidate')
            self.assertEqual(row['source_sha256_before'], control['binary_sha256'])
            self.assertEqual(row['source_sha256_after'], control['binary_sha256'])
            self.assertEqual(row['binary_sha256_before'], row['binary_sha256_after'])
            self.assertEqual(row['fixture_sha256_before'], row['fixture_sha256_after'])
            self.assertEqual(row['cache_before'], row['cache_after'])
            self.assertFalse(row['cleanup_used_for_measurement'])
            self.assertIn('before owned-process cleanup', row['parent_timing_scope'])
            self.assertEqual(row['resources']['status'], 'unavailable')
            self.assertEqual(row['engine_phases']['status'], 'unavailable')
            self.assertTrue(row['output_validation']['text_verified'])
            self.assertNotIn('ratio', row)
        raw_commands = [(command, env) for command, env in commands
                        if len(command) > 1 and command[1] in (profile.PUBLIC_ENGINE_ROUTE, profile.PRIVATE_ENGINE_ROUTE)]
        self.assertEqual(len(raw_commands), 32)
        for command, env in raw_commands:
            self.assertEqual(Path(command[0]).name, 'tekai')
            self.assertNotIn('TEKAI_DIAGNOSTIC_PROFILE', env)
            self.assertNotIn('TEKAI_ENGINE_PROFILE', env)
            self.assertIn('-no-shell-escape', command)
        ordinary = [row for row in report['commands'] if row['case'] == 'image-compile'
                    and not row.get('unwrapped_route_control')]
        self.assertTrue(all(row['engine_phases']['status'] == ('reported' if row['label'] == 'candidate' else 'unavailable')
                            for row in ordinary))

    def test_unsupported_candidate_worker_is_partial_without_baseline_worker_or_fallback(self):
        code, report, commands, _ = self.run_model(image_route_controls=True, route_error=True)
        self.assertEqual(code, 1)
        self.assertEqual(report['image_route_controls']['status'], 'error')
        completion = report['image_route_controls']['completion']
        self.assertFalse(completion['complete'])
        self.assertFalse(completion['equal_completed_balance'])
        self.assertEqual(completion['complete_pairs'], 0)
        self.assertEqual(completion['completed_route_counts']['s0'], {'public': 1, 'worker': 0})
        rows = [row for row in report['commands'] if row.get('unwrapped_route_control')]
        self.assertEqual([row['route'] for row in rows], ['public', 'worker'])
        self.assertEqual([row['status'] for row in rows], ['verified', 'error'])
        self.assertIsNone(rows[1]['output_validation'])
        self.assertIn('unsupported', rows[1]['error'])
        self.assertEqual(report['primary']['status'], 'inconclusive')
        self.assertTrue(report['primary']['unchanged'])

    def test_image_controls_fail_closed_on_cache_fixture_binary_or_source_corruption(self):
        def change_cache(_supervisor, fixture):
            (Path(fixture['env']['TEKAI_ENGINE_CACHE']) / 'complete-fixture').write_bytes(b'changed')

        def change_fixture(_supervisor, fixture):
            (fixture['project'] / 'main.tex').write_text('changed')

        def change_binary(_supervisor, fixture):
            Path(fixture['command'][0]).write_bytes(b'changed')

        def change_source(_supervisor, _fixture):
            self.sources['candidate'].write_bytes(b'changed')

        for mutate in (change_cache, change_fixture, change_binary, change_source):
            with self.subTest(mutation=mutate.__name__):
                original = self.sources['candidate'].read_bytes()
                code, report, _, _ = self.run_model(image_route_controls=True, route_mutation=mutate)
                self.sources['candidate'].write_bytes(original)
                self.assertEqual(code, 1)
                self.assertFalse(report['image_route_controls']['completion']['equal_completed_balance'])
                rows = [row for row in report['commands'] if row.get('unwrapped_route_control')]
                self.assertEqual(len(rows), 1)
                self.assertEqual(rows[0]['status'], 'error')
                self.assertIn('changed after command', rows[0]['error'])
                self.assertEqual(report['primary']['status'], 'inconclusive')


class ImageRouteInventoryTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name).resolve()
        self.fixture = {'root': self.root, 'env': {}}
        for kind in ('ENGINE', 'FORMAT', 'AUX', 'BIBTEX'):
            path = self.root / ('cache-' + kind.lower())
            path.mkdir()
            self.fixture['env']['TEKAI_' + kind + '_CACHE'] = str(path)
        self.file = self.root / 'cache-engine/immutable'
        self.file.write_bytes(b'cache data')
        self.supervisor = mock.Mock()

    def test_native_inventory_is_stable_for_reads_and_detects_preserved_mtime_replacement(self):
        first = profile.image_cache_snapshot(self.supervisor, self.fixture)
        self.assertEqual(first, profile.image_cache_snapshot(self.supervisor, self.fixture))
        self.assertEqual(first['entries'], 5)
        self.assertFalse(first['used_for_gate'])
        self.assertIn('Not cache-content hashes', first['scope'])
        info = self.file.stat()
        replacement = self.root / 'replacement'
        replacement.write_bytes(self.file.read_bytes())
        os.utime(replacement, ns=(info.st_atime_ns, info.st_mtime_ns))
        replacement.replace(self.file)
        self.assertNotEqual(first, profile.image_cache_snapshot(self.supervisor, self.fixture))

    def test_symlinks_external_roots_special_files_and_finite_limits_fail_closed(self):
        link = self.root / 'cache-engine/link'
        link.symlink_to(self.file)
        with self.assertRaises(ValueError):
            profile.image_cache_snapshot(self.supervisor, self.fixture)
        link.unlink()
        self.fixture['env']['TEKAI_ENGINE_CACHE'] = str(self.root)
        with self.assertRaises(ValueError):
            profile.image_cache_snapshot(self.supervisor, self.fixture)
        self.fixture['env']['TEKAI_ENGINE_CACHE'] = str(self.root / 'cache-engine')
        for name, limit in (('MAX_CACHE_ENTRIES', 2), ('MAX_CACHE_BYTES', 1)):
            with mock.patch.object(profile, name, limit), self.assertRaises(ValueError):
                profile.image_cache_snapshot(self.supervisor, self.fixture)
        fifo = self.root / 'cache-engine/fifo'
        os.mkfifo(fifo)
        with self.assertRaises(ValueError):
            profile.image_cache_snapshot(self.supervisor, self.fixture)

    def test_bounded_inventory_obeys_existing_whole_run_deadline(self):
        self.supervisor.remaining.side_effect = benchmark.BenchmarkError('Whole-run deadline exceeded')
        with self.assertRaisesRegex(benchmark.BenchmarkError, 'deadline'):
            profile.image_cache_snapshot(self.supervisor, self.fixture)

    def test_workflow_opt_in_and_rust_route_constants_match(self):
        root = Path(__file__).resolve().parent.parent
        source = (root / 'src/compiler.rs').read_text()
        self.assertIn(f'EMBEDDED_ENGINE_SUBCOMMAND: &str = "{profile.PUBLIC_ENGINE_ROUTE}"', source)
        self.assertIn(f'EMBEDDED_ENGINE_WORKER_SUBCOMMAND: &str = "{profile.PRIVATE_ENGINE_ROUTE}"', source)
        workflow = (root / '.github/workflows/performance.yml').read_text()
        diagnostic = workflow.split('id: profiling', 1)[1].split('- name:', 1)[0]
        self.assertIn('tools/profile_runtime.py', diagnostic)
        self.assertIn('--image-route-controls', diagnostic)
        primary = workflow.split('id: comparison', 1)[1].split('- name:', 1)[0]
        self.assertNotIn('--image-route-controls', primary)


@unittest.skipUnless(os.name == 'posix', 'Owned POSIX process groups')
class SupervisorTests(unittest.TestCase):
    def test_unwrapped_control_uses_owned_waiter_without_timer_or_profile_opt_in(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            supervisor = profile.ProfileSupervisor(root, 2, time.monotonic() + 10)
            supervisor.kind = 'darwin' if sys.platform == 'darwin' else 'gnu'
            command = [sys.executable, '-B', '-c',
                       'import os; assert "TEKAI_ENGINE_PROFILE" not in os.environ; '
                       'assert "TEKAI_DIAGNOSTIC_PROFILE" not in os.environ; print("owned raw control")']
            row = {'command': command, 'unwrapped_route_control': True}
            supervisor.active = row
            try:
                result = supervisor.execute(command, root, {})
                self.assertEqual(result['stdout'], 'owned raw control\n')
                self.assertEqual(result['stderr_tail'], '')
                self.assertEqual(len(result['command_timing']), len(benchmark.TIMING_METADATA['command_timing_fields']))
                self.assertEqual(row['resources']['status'], 'unavailable')
                self.assertEqual(row['engine_phases']['status'], 'unavailable')
                self.assertIn('No resource wrapper or profiling opt-in', row['parent_timing_scope'])
                self.assertGreaterEqual(row['cleanup_seconds'], 0)
                self.assertEqual(supervisor.owned, {})
            finally:
                supervisor.close()

    def test_real_system_timer_probe_with_tiny_python_child(self):
        with tempfile.TemporaryDirectory() as temporary:
            supervisor = profile.ProfileSupervisor(Path(temporary), 2, time.monotonic() + 5)
            try:
                supervisor.probe_timer(dict(os.environ, LC_ALL='C'))
                if supervisor.kind is None:
                    self.assertIsInstance(supervisor.timer_reason, str)
                    self.assertTrue(supervisor.timer_reason)
                else:
                    self.assertIn(supervisor.kind, ('gnu', 'darwin'))
                    command = [sys.executable, '-B', '-c', 'print("timer fixture")']
                    supervisor.active = {'command': command}
                    result = supervisor.execute(command, Path(temporary), dict(os.environ))
                    self.assertEqual(result['stdout'], 'timer fixture\n')
                    self.assertEqual(supervisor.active['resources']['status'], 'reported')
                    self.assertTrue(supervisor.active['resources']['untrusted'])
                    self.assertGreaterEqual(supervisor.active['resources']['values']['peak_rss_bytes'], 0)
            finally:
                supervisor.close()
            self.assertFalse(supervisor.owned)

    def test_timer_probe_unavailable_is_explicit(self):
        with tempfile.TemporaryDirectory() as temporary:
            supervisor = profile.ProfileSupervisor(Path(temporary), 2, time.monotonic() + 5)
            with mock.patch.object(profile.Path, 'is_file', return_value=False):
                supervisor.probe_timer({})
            self.assertIsNone(supervisor.kind)
            self.assertEqual(supervisor.timer_reason, 'Supported system timer is unavailable')
            self.assertFalse(supervisor.owned)

    def test_full_bounded_capture_preserves_duplicate_protocol_lines(self):
        with tempfile.TemporaryDirectory() as temporary:
            supervisor = profile.ProfileSupervisor(Path(temporary), 2, time.monotonic() + 5)
            text = phase_line() + 'padding\n' * 1150 + phase_line()
            self.assertGreater(len(text), 8192)
            self.assertLess(len(text), profile.MAX_TEXT_BYTES)
            command = [sys.executable, '-B', '-c', 'import sys; sys.stderr.write(' + repr(text) + ')']
            supervisor.active = {'command': command}
            try:
                supervisor.execute(command, Path(temporary), dict(os.environ))
                self.assertEqual(supervisor.active['phases']['status'], 'invalid')
                self.assertEqual(supervisor.active['stderr_tail'], text)
            finally:
                supervisor.close()

    def test_resource_duplicate_outside_old_tail_cannot_pass(self):
        with tempfile.TemporaryDirectory() as temporary:
            supervisor = profile.ProfileSupervisor(Path(temporary), 2, time.monotonic() + 5)
            supervisor.kind = 'gnu'
            text = GNU + 'padding\n' * 1100 + GNU
            command = [sys.executable, '-B', '-c', 'import sys; sys.stderr.write(' + repr(text) + ')']
            supervisor.active = {'command': command}
            # Exercise full capture without relying on a platform's GNU timer.
            with mock.patch.object(profile, 'timer_command', return_value=[]):
                try:
                    supervisor.execute(command, Path(temporary), dict(os.environ))
                    self.assertEqual(supervisor.active['resources']['status'], 'unavailable')
                    self.assertIn('exactly one', supervisor.active['resources']['reason'])
                finally:
                    supervisor.close()

    def test_oversized_stderr_fails_and_marks_parsers_incomplete(self):
        with tempfile.TemporaryDirectory() as temporary:
            supervisor = profile.ProfileSupervisor(Path(temporary), 2, time.monotonic() + 5)
            command = [sys.executable, '-B', '-c', 'import sys; sys.stderr.write("x" * ' + str(profile.MAX_TEXT_BYTES + 100) + ')']
            supervisor.active = {'command': command}
            try:
                with self.assertRaises(benchmark.BenchmarkError):
                    supervisor.execute(command, Path(temporary), dict(os.environ))
                self.assertEqual(supervisor.active['status'], 'error')
                self.assertEqual(supervisor.active['phases']['status'], 'invalid')
                self.assertEqual(supervisor.active['resources']['status'], 'unavailable')
            finally:
                supervisor.close()
            self.assertFalse(supervisor.owned)

    def test_python_only_success_preserves_parent_times(self):
        with tempfile.TemporaryDirectory() as temporary:
            supervisor = profile.ProfileSupervisor(Path(temporary), 2, time.monotonic() + 5)
            command = [sys.executable, '-B', '-c', 'print("fixture")']
            supervisor.active = {'command': command}
            result = supervisor.execute(command, Path(temporary), dict(os.environ))
            row = supervisor.active
            supervisor.close()
            self.assertEqual(result['stdout'], 'fixture\n')
            self.assertEqual(len(row['parent_timing']), 3)
            self.assertGreaterEqual(row['launch_setup_seconds'], 0)
            self.assertGreaterEqual(row['popen_latency_seconds'], 0)
            self.assertGreaterEqual(row['capture_setup_seconds'], 0)
            self.assertGreaterEqual(row['waiter_dispatch_seconds'], 0)
            self.assertEqual(len(row['command_timing']), 12)
            self.assertIn('allocated_blocks', row['parent_after'])
            self.assertEqual(row['resources']['status'], 'unavailable')
            self.assertEqual(row['phases']['status'], 'unavailable')
            self.assertEqual(supervisor.owned, {})

    def test_python_only_timeout_retains_error_and_reaps_group(self):
        with tempfile.TemporaryDirectory() as temporary:
            supervisor = profile.ProfileSupervisor(Path(temporary), .04, time.monotonic() + 2)
            command = [sys.executable, '-B', '-c', 'import time; time.sleep(2)']
            supervisor.active = {'command': command}
            with self.assertRaises(benchmark.BenchmarkError):
                supervisor.execute(command, Path(temporary), dict(os.environ))
            row = supervisor.active
            supervisor.close()
            self.assertEqual(row['status'], 'error')
            self.assertGreaterEqual(row['finish_monotonic_ns'], row['launch_monotonic_ns'])
            self.assertEqual(supervisor.owned, {})
            self.assertIsNone(row['resources']['values']['peak_rss_bytes'])


if __name__ == '__main__':
    unittest.main()
