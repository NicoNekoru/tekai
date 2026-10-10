"""Mocked failure-only control orchestration, without launching a benchmark."""

import copy
import hashlib
import json
import os
from pathlib import Path
import signal
import tempfile
import unittest
from unittest.mock import Mock, patch

import benchmark_attribution as attribution
import benchmark_runtime as benchmark


class AttributionTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix='test-benchmark-attribution-')
        self.addCleanup(self.temporary.cleanup)
        self.work = Path(self.temporary.name).resolve()
        self.sources = {}
        for label in benchmark.LABELS:
            path = self.work / (label + '-executable')
            path.write_bytes(('Never execute this ' + label + ' fixture.\n').encode('ascii'))
            path.chmod(0o700)
            self.sources[label] = path
        self.verifiers = {}
        for name in ('pdftotext', 'pdfinfo', 'pdfimages'):
            path = self.work / name
            path.write_bytes(('Never execute this ' + name + ' verifier.\n').encode('ascii'))
            path.chmod(0o700)
            self.verifiers[name] = path
        self.revisions = {'baseline': 'a' * 40, 'candidate': 'b' * 40}
        self.metadata = {
            'runner_label': 'macos-15-ARM64',
            'toolchain': 'rustc fixture version and exact target',
            'baseline_policy': 'explicit-sha',
            'release_settings': {'codegen_units': 1, 'lto': 'fat', 'panic': 'abort'},
            'optional': {'preserve': ['whole', 'objects']},
            **{label: {'revision': self.revisions[label], 'build_command': attribution.BUILD_COMMAND,
                       'artifact_sha256': benchmark.executable_sha256(self.sources[label]),
                       'optional_side': {'role': label, 'flags': ['locked', 'release']}}
               for label in benchmark.LABELS},
        }
        self.primary_path = self.work / 'primary.json'
        self.metadata_path = self.work / 'primary-metadata.json'
        self.primary_markdown = self.primary_path.with_suffix('.md')
        self.primary_markdown.write_text('Authoritative primary report.\n', encoding='utf-8')
        self.primary = self.make_report(self.metadata, self.sources, 'fail', gate=True,
                                        fixture_prefix=self.work / 'primary-fixtures')
        self.save_inputs()
        self.output_index = 0
        self.which = self.enter_patch('benchmark_attribution.shutil.which',
                                     side_effect=lambda name: str(self.verifiers[name]))
        self.native_main = self.enter_patch('benchmark_attribution.benchmark.main',
                                            side_effect=AssertionError('A test attempted a real benchmark'))
        self.previous_handler = object()
        self.getsignal = self.enter_patch('benchmark_attribution.signal.getsignal',
                                          return_value=self.previous_handler)
        self.signal = self.enter_patch('benchmark_attribution.signal.signal')

    def enter_patch(self, target, **kwargs):
        patcher = patch(target, **kwargs)
        result = patcher.start()
        self.addCleanup(patcher.stop)
        return result

    def tearDown(self):
        self.native_main.assert_not_called()

    @staticmethod
    def output_validation(case, command_id):
        if case == 'image-compile':
            return {
                'pages': 8, 'image_objects': 16, 'dimensions': [1024, 1024],
                'rgb_by_page': [[page * 7 % 256, page * 13 % 256, 90] for page in range(8)],
                'alpha': 128, 'every_decoded_pixel_verified': True,
                'verifier_command_ids': [command_id + 1, command_id + 2, command_id + 3],
            }
        return {'expected_text': 'Ordinary nested lookup.' if case == 'nested-lookup' else 'Probe.',
                'text_verified': True, 'verifier_command_id': command_id + 1}

    def make_report(self, metadata, sources, status, gate=False, fixture_prefix=None):
        prefix = fixture_prefix or self.work / 'control-fixtures'
        sample_status = 'pass' if status == 'error' else status
        pairs = attribution.POLICY['pairs_per_case']
        ratios = [1.2] * pairs if sample_status == 'fail' else (
            ([1.08] * 4 + [1.12] * 4) * (pairs // 8) if sample_status == 'inconclusive' else [1.0] * pairs)
        command_id = 0
        clock = 10.0
        hosts = []

        def batch(case, seconds, iterations=1):
            nonlocal command_id, clock
            commands = []
            for _ in range(iterations):
                command_id += 4
                commands.append({'command_id': command_id, 'seconds': seconds,
                                 'parent_timing': [clock, clock + seconds / 10, clock + seconds]})
                clock += seconds + 0.1
            return {
                'seconds': sum(command['seconds'] for command in commands), 'iterations': iterations,
                'commands': commands,
                'output_validation': self.output_validation(case, command_id),
            }

        rows = []
        for case_index, case in enumerate(benchmark.CASES):
            fixture_hash = hashlib.sha256(('stable generated fixture ' + case).encode('ascii')).hexdigest()
            row = {
                'case': case,
                'fixture_sha256': {slot: {label: fixture_hash for label in benchmark.LABELS} for slot in benchmark.SLOTS},
                'fixture_sha256_end': {slot: {label: fixture_hash for label in benchmark.LABELS} for slot in benchmark.SLOTS},
                'fixture_unchanged': True,
                'fixture_roots': {slot: {label: str(prefix / slot / benchmark.replica_for(label, slot) / case)
                                        for label in benchmark.LABELS} for slot in benchmark.SLOTS},
                'fixture_inventory': [],
                'slot_schedule': [attribution.json_schedule(benchmark.crossover_pair(index, case_index)) for index in range(pairs)],
                'initialization_order': benchmark.cell_order(case_index),
                'min_sample_seconds': 1.0,
                'initialization': {slot: {} for slot in benchmark.SLOTS},
                'warmups': [], 'calibration_pilots': [], 'samples': [], 'unit_timing': [],
            }
            for cell in benchmark.cell_order(case_index):
                slot, label, replica = cell['slot'], cell['label'], cell['replica']
                root = prefix / slot / replica / case
                row['fixture_inventory'].append({**cell, 'root': str(root), 'project': str(root / 'project'),
                    'out': str(root / 'out'), 'home': str(root / 'home'), 'input_sha256': fixture_hash,
                    'cache_paths': {kind: str(root / ('cache-' + kind.lower()))
                                    for kind in ('ENGINE', 'FORMAT', 'AUX', 'BIBTEX')}})
                observed = batch(case, 1.0)
                row['initialization'][slot][label] = {
                    'command_id': observed['commands'][0]['command_id'],
                    'parent_timing': observed['commands'][0]['parent_timing'],
                    'seconds': observed['seconds'], 'output_validation': observed['output_validation'],
                }
            for index in range(2):
                for slot in benchmark.SLOTS:
                    warmup = {'warmup_index': index,
                              **attribution.json_schedule(benchmark.slot_pair(slot, index, case_index))}
                    for label in warmup['order']:
                        warmup[label] = batch(case, 1.0)
                    row['warmups'].append(warmup)
            for index in range(2):
                for slot in benchmark.SLOTS:
                    pilot = {'pair_index': index, 'iterations': 32,
                             **attribution.json_schedule(benchmark.slot_pair(slot, index, case_index))}
                    for label in pilot['order']:
                        pilot[label] = batch(case, 1.0, 32)
                    row['calibration_pilots'].append(pilot)
            row['calibration'] = benchmark.calibration_details(row['calibration_pilots'], 1.0, 512)
            iterations = benchmark.calibrated_iterations(row['calibration_pilots'], 1.0, 512)
            row['iterations_per_batch'] = iterations
            for index, ratio in enumerate(ratios):
                if index % 4 == 0:
                    unit = {'unit_index': index // 4, 'first_pair_index': index, 'pair_count': 4,
                            'monotonic_start_seconds': clock, 'complete': False}
                    row['unit_timing'].append(unit)
                sample = {**attribution.json_schedule(benchmark.crossover_pair(index, case_index)),
                          'iterations': iterations}
                for label in sample['order']:
                    sample[label] = batch(case, ratio if label == 'candidate' else 1.0, iterations)
                sample['ratio'] = sample['candidate']['seconds'] / sample['baseline']['seconds']
                row['samples'].append(sample)
                if (index + 1) % 4 == 0:
                    unit.update(monotonic_end_seconds=clock, complete=True)
                    hosts.append({'case': case, 'unit_index': unit['unit_index'], 'monotonic_seconds': clock,
                                  'used_for_gate': False, 'load_average': None, 'load_average_status': 'unavailable',
                                  'memory_status': 'unavailable', 'swap_status': 'unavailable',
                                  'cpu_utilization_status': 'unavailable'})
            row['analysis'] = benchmark.analyze_pairs(row['samples'], 0.10, 1.0, case_index=case_index)
            rows.append(row)
        report = {
            'schema_version': 5, 'gate': gate, 'status': status,
            'prospective_sampling_study': benchmark.prospective_power_study(),
            'timing_metadata': copy.deepcopy(benchmark.TIMING_METADATA), 'host_observations': hosts,
            'monotonic_start_seconds': 10.0, 'monotonic_end_seconds': clock,
            'clock_origin': {'unix_ns': 1800000000000000000, 'monotonic_seconds': 10.0,
                             'alignment': 'Sequential wall then monotonic observations; approximate alignment only.'},
            'metadata': copy.deepcopy(metadata), 'policy': copy.deepcopy(attribution.POLICY),
            'machine': {'system': 'Darwin', 'architecture': 'arm64'},
            'elapsed_seconds': clock - 10.0, 'exit_code': benchmark.comparison_exit(status, gate),
            'executables': [], 'output_verifiers': {}, 'results': rows,
            'errors': [] if status != 'error' else ['BenchmarkError: Command failed; return code 7'],
        }
        for cell in benchmark.cell_order():
            label, slot, replica = cell['label'], cell['slot'], cell['replica']
            source = Path(sources[label]).resolve()
            content_hash = benchmark.executable_sha256(source)
            report['executables'].append({
                **cell, 'source_path': str(source),
                'isolated_path': str(prefix / slot / replica / 'bin' / 'tekai'),
                'source_sha256_start': content_hash, 'source_sha256_end': content_hash,
                'sha256_start': content_hash, 'sha256_end': content_hash, 'unchanged': True,
            })
        for name, path in self.verifiers.items():
            content_hash = benchmark.executable_sha256(path)
            report['output_verifiers'][name] = {
                'path': str(path), 'sha256_start': content_hash,
                'sha256_end': content_hash, 'unchanged': True,
            }
        return report

    def save_inputs(self, primary=None, metadata=None):
        self.primary_path.write_text(json.dumps(self.primary if primary is None else primary), encoding='utf-8')
        self.metadata_path.write_text(json.dumps(self.metadata if metadata is None else metadata), encoding='utf-8')

    @staticmethod
    def retime_report(report):
        """Build a fresh literal diagnostic clock after deliberate workload-model changes."""
        clock, command_id = 10.0, 0
        report['host_observations'] = []

        def command(value):
            nonlocal clock, command_id
            command_id += 4
            seconds = value['seconds']
            value.update(command_id=command_id, parent_timing=[clock, clock + seconds / 10, clock + seconds])
            clock += seconds + 0.1

        for row in report['results']:
            for cell in row['initialization_order']:
                command(row['initialization'][cell['slot']][cell['label']])
            for item in row['warmups'] + row['calibration_pilots']:
                for label in item['order']:
                    for value in item[label]['commands']:
                        command(value)
            row['unit_timing'] = []
            for index, sample in enumerate(row['samples']):
                if index % 4 == 0:
                    unit = {'unit_index': index // 4, 'first_pair_index': index, 'pair_count': 4,
                            'monotonic_start_seconds': clock, 'complete': False}
                    row['unit_timing'].append(unit)
                for label in sample['order']:
                    for value in sample[label]['commands']:
                        command(value)
                if (index + 1) % 4 == 0:
                    unit.update(monotonic_end_seconds=clock, complete=True)
                    report['host_observations'].append({'case': row['case'], 'unit_index': index // 4,
                        'monotonic_seconds': clock, 'used_for_gate': False, 'load_average': None,
                        'load_average_status': 'unavailable', 'memory_status': 'unavailable',
                        'swap_status': 'unavailable', 'cpu_utilization_status': 'unavailable'})
        report.update(monotonic_end_seconds=clock, elapsed_seconds=clock - 10.0)

    def make_runner(self, mutations=None, statuses=None):
        mutations, statuses = mutations or {}, statuses or {}

        def run(argv):
            args = benchmark.parser().parse_args(argv)
            kind = args.output.stem
            metadata = json.loads(args.metadata.read_text(encoding='utf-8'))
            selected = {'baseline': args.baseline.resolve(), 'candidate': args.candidate.resolve()}
            report = self.make_report(metadata, selected, statuses.get(kind, 'pass'), gate=args.gate,
                                      fixture_prefix=args.output.parent / (kind + '-fixtures'))
            exit_code = report['exit_code']
            if report['status'] == 'error':
                report['results'] = []
            mutation = mutations.get(kind)
            if mutation:
                mutation(report, args)
            args.output.write_text(json.dumps(report), encoding='utf-8')
            args.output.with_suffix('.md').write_text('Advisory control report.\n', encoding='utf-8')
            print('Mock control ' + kind + ' ' + report['status'])
            return exit_code

        return Mock(side_effect=run)

    def invoke(self, primary=None, metadata=None, sources=None, revisions=None, profile='full', runner=None):
        self.save_inputs(primary, metadata)
        self.output_index += 1
        directory = self.work / ('controls-' + str(self.output_index))
        runner = runner if runner is not None else self.make_runner()
        result = attribution.run_attribution(
            self.primary_path, self.metadata_path, sources or self.sources,
            revisions or self.revisions, directory, profile=profile, runner=runner)
        summary = json.loads((directory / 'attribution.json').read_text(encoding='utf-8'))
        return result, summary, runner, directory

    def assert_skipped(self, **kwargs):
        result, summary, runner, _ = self.invoke(**kwargs)
        self.assertEqual(result, 0)
        self.assertEqual(summary['status'], 'skipped')
        self.assertEqual(summary['controls'], [])
        self.assertTrue(summary['reason'])
        runner.assert_not_called()

    def assert_stopped(self, runner):
        result, summary, runner, directory = self.invoke(runner=runner)
        self.assertEqual(result, 1)
        self.assertEqual(summary['status'], 'stopped')
        self.assertTrue(summary['reason'])
        self.assertEqual(runner.call_count, 1)
        self.assertFalse((directory / 'candidate-self.json').exists())
        return summary

    def test_full_failure_runs_two_controls_and_preserves_authoritative_inputs(self):
        protected = {path: path.read_bytes() for path in (
            self.primary_path, self.primary_markdown, self.metadata_path, *self.sources.values(), *self.verifiers.values())}
        result, summary, runner, directory = self.invoke()
        self.assertEqual(result, 0)
        self.assertEqual(summary['status'], 'complete')
        self.assertEqual(summary['primary_outcome'], {'status': 'fail', 'exit_code': 1})
        self.assertTrue(summary['diagnostic_only'])
        self.assertIs(summary['used_for_primary_gate'], False)
        self.assertEqual([row['kind'] for row in summary['controls']], ['reversed', 'candidate-self'])
        self.assertEqual(runner.call_count, 2)
        for path, content in protected.items():
            self.assertEqual(path.read_bytes(), content, str(path))
        for kind in ('reversed', 'candidate-self'):
            for suffix in ('.json', '.md', '.log', '-metadata.json'):
                self.assertTrue((directory / (kind + suffix)).is_file())
        self.assertTrue((directory / 'attribution.md').is_file())

    def test_fully_sampled_inconclusive_primary_is_eligible(self):
        primary = self.make_report(self.metadata, self.sources, 'inconclusive', gate=True)
        result, summary, runner, _ = self.invoke(primary=primary)
        self.assertEqual(result, 0)
        self.assertEqual(summary['status'], 'complete')
        self.assertEqual(summary['primary_outcome'], {'status': 'inconclusive', 'exit_code': 2})
        self.assertEqual(runner.call_count, 2)

    def test_pass_quick_error_incomplete_and_non_gate_are_skipped(self):
        for status in ('pass', 'error', 'incomplete'):
            with self.subTest(status=status):
                primary = copy.deepcopy(self.primary)
                primary['status'] = status
                primary['exit_code'] = 0 if status == 'pass' else 1
                self.assert_skipped(primary=primary)
        self.assert_skipped(profile='quick')
        primary = copy.deepcopy(self.primary)
        primary['gate'] = False
        self.assert_skipped(primary=primary)

    def test_calibration_infeasible_case_is_skipped(self):
        primary = self.make_report(self.metadata, self.sources, 'inconclusive', gate=True)
        self.make_infeasible_case(primary['results'][0])
        self.assert_skipped(primary=primary)

    def test_synthetic_batches_match_calibrated_iterations_and_command_totals(self):
        for row in self.primary['results']:
            iterations = benchmark.calibrated_iterations(row['calibration_pilots'], 1.0, 512)
            self.assertEqual(iterations, 2)
            self.assertEqual(row['iterations_per_batch'], iterations)
            for sample in row['samples']:
                self.assertEqual(sample['iterations'], iterations)
                for label in benchmark.LABELS:
                    batch = sample[label]
                    self.assertEqual(batch['iterations'], iterations)
                    self.assertEqual(len(batch['commands']), iterations)
                    self.assertEqual(batch['seconds'], sum(command['seconds'] for command in batch['commands']))

    def test_pilot_rate_model_ignores_single_launch_warmups_and_command_variation(self):
        primary = copy.deepcopy(self.primary)
        for row in primary['results']:
            for warmup in row['warmups']:
                for label in benchmark.LABELS:
                    warmup[label]['seconds'] = 1 / 1024
                    warmup[label]['commands'][0]['seconds'] = 1 / 1024
            for pilot in row['calibration_pilots']:
                for label in benchmark.LABELS:
                    for index, command in enumerate(pilot[label]['commands']):
                        command['seconds'] = 0.5 if index % 2 else 1.5
            # Both 32-command totals stay at 32 seconds. Their normalized
            # rate is one second, so two seconds of headroom selects two.
            self.assertEqual(row['iterations_per_batch'], 2)
        self.retime_report(primary)
        result, summary, runner, _ = self.invoke(primary=primary)
        self.assertEqual(result, 0)
        self.assertEqual(summary['status'], 'complete')
        self.assertEqual(runner.call_count, 2)

    def test_recomputed_pilot_size_disagreement_rejects_primary_and_control(self):
        def change(report, _args=None):
            row = report['results'][0]
            for pilot in row['calibration_pilots']:
                if pilot['slot'] == 's1':
                    for command in pilot['candidate']['commands']:
                        command['seconds'] = 0.25
                    pilot['candidate']['seconds'] = 8.0
            # One faster artifact-slot cell needs eight matched iterations.
            # The other cells and formal batches still claim the old two.
            row['calibration'] = benchmark.calibration_details(row['calibration_pilots'], 1.0, 512)
            self.assertEqual(row['calibration']['selected_iterations'], 8)
            self.assertEqual(row['iterations_per_batch'], 2)
        primary = copy.deepcopy(self.primary)
        change(primary)
        self.assert_skipped(primary=primary)
        self.assert_stopped(self.make_runner(mutations={'reversed': change}))

    def test_full_raw_pilot_contract_rejects_primary_and_control_corruption(self):
        modes = ('missing', 'partial', 'duplicate', 'order', 'pair-index', 'slot', 'replicas', 'iterations',
                 'side', 'batch-count', 'command-count', 'command-type', 'command-id',
                 'command-time', 'batch-total', 'oracle', 'formal-contamination')
        for mode in modes:
            with self.subTest(mode=mode):
                def change(report, _args=None):
                    row = report['results'][0]
                    pilot = row['calibration_pilots'][0]
                    batch = pilot['baseline']
                    if mode == 'missing':
                        row.pop('calibration_pilots')
                    elif mode == 'partial':
                        row['calibration_pilots'].pop()
                    elif mode == 'duplicate':
                        row['calibration_pilots'][1] = copy.deepcopy(pilot)
                    elif mode == 'order':
                        pilot['order'].reverse()
                    elif mode == 'pair-index':
                        pilot['pair_index'] = False
                    elif mode == 'slot':
                        pilot['slot'] = 's1'
                    elif mode == 'replicas':
                        pilot['replicas']['candidate'] = pilot['replicas']['baseline']
                    elif mode == 'iterations':
                        pilot['iterations'] = 32.0
                    elif mode == 'side':
                        pilot.pop('candidate')
                    elif mode == 'batch-count':
                        batch['iterations'] = 1
                    elif mode == 'command-count':
                        batch['commands'].pop()
                    elif mode == 'command-type':
                        batch['commands'][0] = None
                    elif mode == 'command-id':
                        batch['commands'][0]['command_id'] = True
                    elif mode == 'command-time':
                        batch['commands'][0]['seconds'] = 0
                    elif mode == 'batch-total':
                        batch['seconds'] = 31.0
                    elif mode == 'oracle':
                        batch['output_validation'] = {}
                    else:
                        row['samples'][0] = copy.deepcopy(pilot)
                primary = copy.deepcopy(self.primary)
                change(primary)
                self.assert_skipped(primary=primary)
                self.assert_stopped(self.make_runner(mutations={'reversed': change}))

    def test_recorded_calibration_details_must_match_recomputed_values_and_types(self):
        modes = ('missing', 'extra', 'rate', 'nonfinite', 'boolean-count', 'float-count', 'numeric-flag')
        for mode in modes:
            with self.subTest(mode=mode):
                def change(report, _args=None):
                    row = report['results'][0]
                    details = row['calibration']
                    if mode == 'missing':
                        row.pop('calibration')
                    elif mode == 'extra':
                        details['unverified'] = 'ignored field'
                    elif mode == 'rate':
                        details['normalized_rates_seconds_per_iteration']['s0']['baseline'][0] = 2.0
                    elif mode == 'nonfinite':
                        details['fastest_pilot_median_seconds_per_iteration'] = float('inf')
                    elif mode == 'boolean-count':
                        details['minimum_iterations'] = True
                    elif mode == 'float-count':
                        details['selected_iterations'] = 2.0
                    else:
                        details['minimum_feasible'] = 1
                primary = copy.deepcopy(self.primary)
                change(primary)
                self.assert_skipped(primary=primary)
                self.assert_stopped(self.make_runner(mutations={'reversed': change}))

    def test_pilots_cannot_be_reused_as_samples_when_selected_count_is_32(self):
        def select_32(report):
            used_ids = []
            for row in report['results']:
                used_ids.extend(value['command_id'] for slot in benchmark.SLOTS
                                for value in row['initialization'][slot].values())
                used_ids.extend(command['command_id'] for field in ('warmups', 'calibration_pilots', 'samples')
                                for item in row[field] for label in benchmark.LABELS
                                for command in item[label]['commands'])
            next_id = max(used_ids)
            row = report['results'][0]
            for pilot in row['calibration_pilots']:
                for label in benchmark.LABELS:
                    for command in pilot[label]['commands']:
                        command['seconds'] = 1 / 16
                    pilot[label]['seconds'] = 2.0
            row['calibration'] = benchmark.calibration_details(row['calibration_pilots'], 1.0, 512)
            self.assertEqual(row['calibration']['selected_iterations'], 32)
            row['iterations_per_batch'] = 32
            for sample in row['samples']:
                sample['iterations'] = 32
                for label in benchmark.LABELS:
                    batch = sample[label]
                    seconds = batch['commands'][0]['seconds']
                    batch['commands'] = []
                    for _ in range(32):
                        next_id += 4
                        batch['commands'].append({'command_id': next_id, 'seconds': seconds})
                    batch.update(iterations=32, seconds=sum(command['seconds'] for command in batch['commands']))
                sample['ratio'] = sample['candidate']['seconds'] / sample['baseline']['seconds']
            row['analysis'] = benchmark.analyze_pairs(row['samples'], 0.10, 1.0)
            self.retime_report(report)
            return row

        primary = copy.deepcopy(self.primary)
        select_32(primary)
        result, summary, _, _ = self.invoke(primary=primary)
        self.assertEqual(result, 0)
        self.assertEqual(summary['status'], 'complete')

        def contaminate(report, _args=None):
            row = select_32(report)
            sample = row['samples'][0]
            sample['baseline'] = copy.deepcopy(row['calibration_pilots'][0]['baseline'])
            sample['ratio'] = sample['candidate']['seconds'] / sample['baseline']['seconds']
            row['analysis'] = benchmark.analyze_pairs(row['samples'], 0.10, 1.0)
        contaminate(primary)
        self.assert_skipped(primary=primary)
        self.assert_stopped(self.make_runner(mutations={'reversed': contaminate}))

    def test_previous_report_schema_has_no_silent_calibration_fallback(self):
        for schema in (2, 3, 4):
            with self.subTest(schema=schema):
                def change(report, _args=None):
                    report['schema_version'] = schema
                primary = copy.deepcopy(self.primary)
                change(primary)
                self.assert_skipped(primary=primary)
                self.assert_stopped(self.make_runner(mutations={'reversed': change}))

    @staticmethod
    def make_infeasible_case(row):
        for pilot in row['calibration_pilots']:
            for label in benchmark.LABELS:
                for command in pilot[label]['commands']:
                    command['seconds'] = 1 / 1024
                pilot[label]['seconds'] = 32 / 1024
        row['calibration'] = benchmark.calibration_details(row['calibration_pilots'], 1.0, 512)
        try:
            benchmark.calibrated_iterations(row['calibration_pilots'], 1.0, 512)
        except benchmark.CalibrationInfeasible as error:
            row['calibration_infeasible'] = copy.deepcopy(error.details)
            row['analysis'] = {'status': 'inconclusive', 'reasons': [str(error)]}
        else:
            raise AssertionError('Synthetic calibration must be infeasible')
        row.pop('iterations_per_batch', None)
        row['samples'] = []
        row['unit_timing'] = []

    def test_genuine_calibration_infeasible_control_continues_to_self(self):
        def change(report, _args):
            self.make_infeasible_case(report['results'][0])
            self.retime_report(report)
            report['status'] = 'inconclusive'
            report['exit_code'] = 0
        result, summary, runner, directory = self.invoke(
            runner=self.make_runner(mutations={'reversed': change}))
        self.assertEqual(result, 0)
        self.assertEqual(summary['status'], 'complete')
        self.assertEqual([row['status'] for row in summary['controls']], ['inconclusive', 'pass'])
        self.assertEqual(runner.call_count, 2)
        reversed_report = json.loads((directory / 'reversed.json').read_text(encoding='utf-8'))
        row = reversed_report['results'][0]
        self.assertEqual(row['samples'], [])
        self.assertNotIn('iterations_per_batch', row)
        self.assertEqual(row['calibration_infeasible']['minimum_iterations'], 1024)
        self.assertEqual(row['calibration_infeasible']['iteration_ceiling'], 512)

    def test_inconsistent_calibration_infeasible_control_stops(self):
        for mode in ('details', 'sample', 'iterations'):
            with self.subTest(mode=mode):
                def change(report, _args):
                    row = report['results'][0]
                    old_sample = copy.deepcopy(row['samples'][0])
                    self.make_infeasible_case(row)
                    report['status'] = 'inconclusive'
                    if mode == 'details':
                        row['calibration_infeasible']['minimum_iterations'] = 1023
                    elif mode == 'sample':
                        row['samples'] = [old_sample]
                    else:
                        row['iterations_per_batch'] = 512
                self.assert_stopped(self.make_runner(mutations={'reversed': change}))

    def test_completed_non_error_control_requires_all_sampled_cases(self):
        for mode in ('empty-results', 'missing-case', 'partial-samples', 'partial-side',
                     'calibration', 'oracle', 'commands', 'command-id'):
            with self.subTest(mode=mode):
                def change(report, _args):
                    if mode == 'empty-results':
                        report['results'] = []
                    elif mode == 'missing-case':
                        report['results'].pop()
                    else:
                        row = report['results'][0]
                        if mode == 'partial-samples':
                            row['samples'].pop()
                        elif mode == 'partial-side':
                            row['samples'][0].pop('candidate')
                        elif mode == 'calibration':
                            row['iterations_per_batch'] = 1
                        elif mode == 'oracle':
                            row['samples'][0]['candidate']['output_validation'] = {}
                        elif mode == 'commands':
                            row['samples'][0]['candidate']['commands'] = [None, None]
                        else:
                            row['samples'][0]['candidate']['commands'][0]['command_id'] = True
                self.assert_stopped(self.make_runner(mutations={'reversed': change}))

    def test_completed_control_errors_status_and_elapsed_must_agree(self):
        cases = (
            ('pass', 'errors'), ('pass', 'no-elapsed'), ('pass', 'infinite-elapsed'),
            ('pass', 'huge-elapsed'), ('error', 'no-errors'), ('error', 'no-elapsed'),
            ('error', 'infinite-elapsed'), ('error', 'huge-elapsed'),
        )
        for status, mode in cases:
            with self.subTest(status=status, mode=mode):
                def change(report, _args):
                    if mode == 'errors':
                        report['errors'] = ['BenchmarkError: ordinary command failure']
                    elif mode == 'no-errors':
                        report['errors'] = []
                    elif mode == 'no-elapsed':
                        report.pop('elapsed_seconds')
                    elif mode == 'infinite-elapsed':
                        report['elapsed_seconds'] = float('inf')
                    else:
                        report['elapsed_seconds'] = 10 ** 400
                self.assert_stopped(self.make_runner(mutations={'reversed': change}, statuses={'reversed': status}))

    def test_ordinary_phase_failures_keep_partial_cell_evidence_and_allow_self_control(self):
        for phase in ('warmups', 'calibration_pilots', 'samples'):
            with self.subTest(phase=phase):
                def change(report, args):
                    selected = {label: getattr(args, label) for label in benchmark.LABELS}
                    prefix = Path(report['executables'][0]['isolated_path']).parents[3]
                    row = self.make_report(report['metadata'], selected, 'pass', fixture_prefix=prefix)['results'][0]
                    row[phase] = row[phase][:1]
                    row[phase][0].pop('candidate')
                    row.pop('fixture_sha256_end')
                    row.pop('fixture_unchanged')
                    row.pop('analysis')
                    if phase != 'samples':
                        row['samples'] = []
                        row.pop('iterations_per_batch')
                        row.pop('calibration')
                    if phase == 'warmups':
                        row['calibration_pilots'] = []
                    report['results'] = [row]
                result, summary, runner, directory = self.invoke(
                    runner=self.make_runner(mutations={'reversed': change}, statuses={'reversed': 'error'}))
                self.assertEqual(result, 1)
                self.assertEqual(summary['status'], 'complete')
                self.assertEqual(runner.call_count, 2)
                failed = json.loads((directory / 'reversed.json').read_text(encoding='utf-8'))['results'][0]
                self.assertIn('baseline', failed[phase][0])
                self.assertNotIn('candidate', failed[phase][0])

    def test_ordinary_fixture_creation_failure_keeps_only_the_completed_cell_prefix(self):
        for completed in range(4):
            with self.subTest(completed=completed):
                def change(report, args):
                    selected = {label: getattr(args, label) for label in benchmark.LABELS}
                    prefix = Path(report['executables'][0]['isolated_path']).parents[3]
                    row = self.make_report(report['metadata'], selected, 'pass', fixture_prefix=prefix)['results'][0]
                    row['fixture_inventory'] = row['fixture_inventory'][:completed]
                    cells = {(entry['slot'], entry['label']) for entry in row['fixture_inventory']}
                    for field in ('fixture_sha256', 'fixture_roots'):
                        row[field] = {slot: {label: value for label, value in row[field][slot].items()
                                           if (slot, label) in cells} for slot in benchmark.SLOTS}
                    for field in ('initialization', 'initialization_order', 'fixture_sha256_end',
                                  'fixture_unchanged', 'analysis', 'calibration', 'iterations_per_batch'):
                        row.pop(field)
                    for field in ('warmups', 'calibration_pilots', 'samples'):
                        row[field] = []
                    report['results'] = [row]
                result, summary, runner, directory = self.invoke(
                    runner=self.make_runner(mutations={'reversed': change}, statuses={'reversed': 'error'}))
                self.assertEqual(result, 1)
                self.assertEqual(summary['status'], 'complete')
                self.assertEqual(runner.call_count, 2)
                failed = json.loads((directory / 'reversed.json').read_text(encoding='utf-8'))['results'][0]
                self.assertEqual(len(failed['fixture_inventory']), completed)
                if completed:
                    def malformed(report, args):
                        change(report, args)
                        entry = report['results'][0]['fixture_inventory'][0]
                        entry['cache_paths']['ENGINE'] = entry['home']
                    self.assert_stopped(self.make_runner(mutations={'reversed': malformed}, statuses={'reversed': 'error'}))

    def test_control_report_exit_and_runner_return_must_agree_with_status(self):
        for mode in ('error-zero', 'pass-one', 'missing-exit', 'boolean-exit'):
            with self.subTest(mode=mode):
                def change(report, _args):
                    if mode == 'missing-exit':
                        report.pop('exit_code')
                    else:
                        report['exit_code'] = False if mode == 'boolean-exit' else (0 if mode == 'error-zero' else 1)
                runner = self.make_runner(mutations={'reversed': change},
                                          statuses={'reversed': 'error' if mode == 'error-zero' else 'pass'})
                self.assert_stopped(runner)

    def test_malformed_or_mismatched_runner_exit_stops(self):
        for value in (None, False, '0', 1, float('nan'), float('inf')):
            with self.subTest(value=value):
                ordinary = self.make_runner()
                def run(argv):
                    ordinary(argv)
                    return value
                self.assert_stopped(Mock(side_effect=run))

    def test_invalid_completed_control_has_no_misleading_pass_label(self):
        def change(report, _args):
            report['results'] = []
        summary = self.assert_stopped(self.make_runner(mutations={'reversed': change}))
        self.assertEqual(summary['controls'][0]['status'], 'invalid')
        self.assertEqual(summary['controls'][0]['reported_status'], 'pass')
        self.assertTrue(summary['controls'][0]['reason'])

    def test_generated_control_metadata_change_is_detected(self):
        def change(_report, args):
            provenance = json.loads(args.metadata.read_text(encoding='utf-8'))
            provenance['baseline']['revision'] = 'c' * 40
            args.metadata.write_text(json.dumps(provenance), encoding='utf-8')
        summary = self.assert_stopped(self.make_runner(mutations={'reversed': change}))
        self.assertIn('changed', summary['reason'])

    def test_malformed_control_top_level_json_stops(self):
        for value in (None, [], ['unexpected'], 'report', 12, True):
            with self.subTest(value=value):
                ordinary = self.make_runner()
                def run(argv):
                    result = ordinary(argv)
                    args = benchmark.parser().parse_args(argv)
                    args.output.write_text(json.dumps(value), encoding='utf-8')
                    return result
                self.assert_stopped(Mock(side_effect=run))

    def test_malformed_control_collections_stop(self):
        cases = (
            ('policy', None), ('executables', {}), ('executables', [None, None]),
            ('output_verifiers', []), ('results', {}), ('results', [None]),
            ('errors', None), ('errors', [12]), ('metadata', []), ('status', []),
        )
        for field, value in cases:
            with self.subTest(field=field, value=value):
                def change(report, _args):
                    report[field] = value
                self.assert_stopped(self.make_runner(mutations={'reversed': change}))
        for field, value in (('samples', {}), ('samples', [None] * 40), ('warmups', [None, None]),
                             ('initialization', []), ('fixture_sha256', []), ('analysis', [])):
            with self.subTest(case_field=field, value=value):
                def change(report, _args):
                    report['results'][0][field] = value
                self.assert_stopped(self.make_runner(mutations={'reversed': change}))

    def test_overflowing_batch_aggregate_is_safe_for_primary_and_control(self):
        for command_seconds in (10 ** 308, 1e308):
            with self.subTest(command_seconds=type(command_seconds).__name__):
                def overflow(report):
                    batch = report['results'][0]['samples'][0]['candidate']
                    self.assertEqual(len(batch['commands']), 2)
                    for command in batch['commands']:
                        command['seconds'] = command_seconds
                    batch['seconds'] = 1e308
                primary = copy.deepcopy(self.primary)
                overflow(primary)
                self.assert_skipped(primary=primary)
                self.assert_stopped(self.make_runner(mutations={'reversed': lambda report, _args: overflow(report)}))

    def test_reversed_and_self_metadata_preserve_complete_side_provenance(self):
        _, _, runner, directory = self.invoke()
        reversed_metadata = json.loads((directory / 'reversed-metadata.json').read_text(encoding='utf-8'))
        self_metadata = json.loads((directory / 'candidate-self-metadata.json').read_text(encoding='utf-8'))
        self.assertEqual(reversed_metadata['baseline'], self.metadata['candidate'])
        self.assertEqual(reversed_metadata['candidate'], self.metadata['baseline'])
        for label in benchmark.LABELS:
            self.assertEqual(self_metadata[label], self.metadata['candidate'])
        for metadata in (reversed_metadata, self_metadata):
            for key in ('runner_label', 'toolchain', 'release_settings', 'optional'):
                self.assertEqual(metadata[key], self.metadata[key])
            self.assertEqual(metadata['original_provenance'], self.metadata)
            self.assertTrue(metadata['diagnostic_only'])
            self.assertIs(metadata['used_for_primary_gate'], False)
        reverse_args, self_args = [benchmark.parser().parse_args(call.args[0]) for call in runner.call_args_list]
        self.assertEqual((reverse_args.baseline, reverse_args.candidate),
                         (self.sources['candidate'], self.sources['baseline']))
        self.assertEqual((self_args.baseline, self_args.candidate), (self.sources['candidate'],) * 2)

    def test_controls_have_fixed_bounds_and_never_gate(self):
        _, _, runner, _ = self.invoke()
        for call in runner.call_args_list:
            argv = call.args[0]
            self.assertNotIn('--gate', argv)
            args = benchmark.parser().parse_args(argv)
            self.assertFalse(args.gate)
            self.assertEqual((args.pairs, args.warmups, args.max_iterations), (128, 2, 512))
            self.assertEqual((args.timeout, args.budget, args.threshold), (30.0, 2700.0, 0.10))
            self.assertEqual((args.min_sample_seconds, args.warm_cache_min_sample_seconds), (1.0, 1.0))

    def test_parent_clock_unit_boundaries_and_host_records_are_bound_diagnostics(self):
        for mode in ('missing-timing', 'clock-total', 'launch-before-start', 'wrong-duration', 'wrong-launch-order',
                     'unit-before-pilot', 'unit-incomplete', 'host-before-end', 'host-is-gating', 'study-is-gating'):
            with self.subTest(mode=mode):
                primary = copy.deepcopy(self.primary)
                row = primary['results'][0]
                sample = row['samples'][0]
                command = sample[sample['order'][0]]['commands'][0]
                if mode == 'missing-timing':
                    command.pop('parent_timing')
                elif mode == 'clock-total':
                    primary['monotonic_end_seconds'] += 1
                elif mode == 'launch-before-start':
                    command['parent_timing'][1] = command['parent_timing'][0] - 1
                elif mode == 'wrong-duration':
                    command['parent_timing'][2] += 0.1
                elif mode == 'wrong-launch-order':
                    sample['baseline']['commands'][0]['parent_timing'], sample['candidate']['commands'][0]['parent_timing'] = \
                        sample['candidate']['commands'][0]['parent_timing'], sample['baseline']['commands'][0]['parent_timing']
                elif mode == 'unit-before-pilot':
                    row['unit_timing'][0]['monotonic_start_seconds'] = primary['monotonic_start_seconds']
                elif mode == 'unit-incomplete':
                    row['unit_timing'][0]['complete'] = False
                elif mode == 'host-before-end':
                    primary['host_observations'][0]['monotonic_seconds'] = row['unit_timing'][0]['monotonic_start_seconds']
                elif mode == 'host-is-gating':
                    primary['host_observations'][0]['used_for_gate'] = True
                else:
                    primary['prospective_sampling_study']['used_for_gate'] = True
                self.assert_skipped(primary=primary)

    def test_valid_extreme_host_load_does_not_reinterpret_primary_or_controls(self):
        def change(report, _args=None):
            for host in report['host_observations']:
                host.update(load_average_status='reported', load_average=[1000000.0] * 3)
        primary = copy.deepcopy(self.primary)
        original_analysis = copy.deepcopy([row['analysis'] for row in primary['results']])
        change(primary)
        result, summary, runner, _ = self.invoke(primary=primary, runner=self.make_runner(mutations={'reversed': change}))
        self.assertEqual((result, summary['status']), (0, 'complete'))
        self.assertEqual(runner.call_count, 2)
        self.assertEqual([row['analysis'] for row in primary['results']], original_analysis)

    def test_control_semantic_fail_and_inconclusive_do_not_pass_or_replace_primary(self):
        runner = self.make_runner(statuses={'reversed': 'fail', 'candidate-self': 'inconclusive'})
        result, summary, _, _ = self.invoke(runner=runner)
        self.assertEqual(result, 0)
        self.assertEqual(summary['status'], 'complete')
        self.assertEqual(summary['primary_outcome'], {'status': 'fail', 'exit_code': 1})
        self.assertEqual([row['status'] for row in summary['controls']], ['fail', 'inconclusive'])
        self.assertEqual([row['exit_code'] for row in summary['controls']], [0, 0])
        self.assertEqual(json.loads(self.primary_path.read_text())['status'], 'fail')

    def test_ordinary_reversed_errors_continue_to_self(self):
        for message in ('BenchmarkError: Command failed; return code 7',
                        'BenchmarkError: Command deadline exceeded; return code -9',
                        'BenchmarkError: Command capture exceeded the finite output limit; return code 0'):
            with self.subTest(message=message):
                def change(report, _args):
                    report['errors'] = [message]
                runner = self.make_runner(mutations={'reversed': change}, statuses={'reversed': 'error'})
                result, summary, runner, directory = self.invoke(runner=runner)
                self.assertEqual(result, 1)
                self.assertEqual(summary['status'], 'complete')
                self.assertEqual(runner.call_count, 2)
                self.assertEqual([row['status'] for row in summary['controls']], ['error', 'pass'])
                self.assertTrue((directory / 'candidate-self.json').is_file())

    def test_runner_exception_and_missing_report_continue_to_self(self):
        regular = self.make_runner()
        calls = 0

        def run(argv):
            nonlocal calls
            calls += 1
            if calls == 1:
                raise RuntimeError('ordinary mock command failure')
            return regular(argv)

        result, summary, runner, _ = self.invoke(runner=Mock(side_effect=run))
        self.assertEqual(result, 1)
        self.assertEqual(summary['status'], 'complete')
        self.assertEqual(runner.call_count, 2)
        self.assertEqual(summary['controls'][0]['status'], 'error')
        self.assertIn('ordinary mock command failure', summary['controls'][0]['errors'][0])

    def test_reported_cancellation_cleanup_and_integrity_failures_stop(self):
        fatal_errors = (
            'Run interrupted; terminating owned process groups',
            'Cleanup failed: Owned process cleanup failed: mocked failure',
            'BenchmarkError: Process waiter did not finish after owned-group cleanup',
            'TimeoutExpired: mock command timed out after 5 seconds',
            'Executable content changed or disappeared: baseline',
            'Output verifier changed or disappeared: pdfimages',
            'BenchmarkError: Selected binary changed during its isolated copy',
            'BenchmarkError: Compiler changed or removed benchmark fixture inputs',
            'BenchmarkError: Per-binary source fixtures do not have equal content',
        )
        for message in fatal_errors:
            with self.subTest(message=message):
                def change(report, _args):
                    report['errors'] = [message]
                self.assert_stopped(self.make_runner(mutations={'reversed': change}, statuses={'reversed': 'error'}))

    def test_keyboard_interrupt_stops_and_restores_sigterm_handler(self):
        runner = Mock(side_effect=KeyboardInterrupt)
        summary = self.assert_stopped(runner)
        self.assertEqual(summary['controls'][0]['status'], 'interrupted')
        self.assertEqual(summary['controls'][0]['exit_code'], 130)
        self.assertEqual(self.signal.call_args_list[-1].args, (signal.SIGTERM, self.previous_handler))

    def test_sigterm_handler_stops_and_restores_without_sending_a_signal(self):
        def interrupt(_argv):
            installed_handler = self.signal.call_args_list[-1].args[1]
            installed_handler(signal.SIGTERM, None)
        self.assert_stopped(Mock(side_effect=interrupt))
        self.assertEqual(self.signal.call_args_list[-1].args, (signal.SIGTERM, self.previous_handler))

    def test_sigterm_handler_restored_after_completed_controls(self):
        self.invoke()
        self.getsignal.assert_called_once_with(signal.SIGTERM)
        self.assertEqual(self.signal.call_count, 2)
        self.assertTrue(callable(self.signal.call_args_list[0].args[1]))
        self.assertEqual(self.signal.call_args_list[-1].args, (signal.SIGTERM, self.previous_handler))

    def test_skipped_primary_does_not_replace_signal_handlers(self):
        self.assert_skipped(profile='quick')
        self.getsignal.assert_not_called()
        self.signal.assert_not_called()

    def test_current_source_hash_change_prevents_all_controls(self):
        self.sources['candidate'].write_bytes(b'Changed source after primary completed.\n')
        self.assert_skipped()

    def test_source_or_verifier_hash_change_between_controls_stops(self):
        for target in (self.sources['candidate'], self.verifiers['pdfimages']):
            with self.subTest(target=target.name):
                before = target.read_bytes()
                def change(_report, _args):
                    target.write_bytes(before + b' changed')
                self.assert_stopped(self.make_runner(mutations={'reversed': change}))
                target.write_bytes(before)

    def test_poppler_selection_change_between_controls_stops(self):
        alternate = self.work / 'alternate-pdfimages'
        alternate.write_bytes(self.verifiers['pdfimages'].read_bytes())
        def change(_report, _args):
            self.which.side_effect = lambda name: str(alternate if name == 'pdfimages' else self.verifiers[name])
        self.assert_stopped(self.make_runner(mutations={'reversed': change}))

    def test_primary_source_copy_hash_disagreement_is_ineligible(self):
        primary = copy.deepcopy(self.primary)
        primary['executables'][0]['sha256_start'] = 'c' * 64
        primary['executables'][0]['sha256_end'] = 'c' * 64
        self.assert_skipped(primary=primary)

    def test_crossover_copy_inventory_rejects_missing_aliased_and_misassigned_cells(self):
        modes = ('missing', 'duplicate', 'slot', 'replica', 'copy-path', 'root', 'creation-order', 'other-slot-hash')
        for mode in modes:
            with self.subTest(mode=mode):
                def change(report, _args=None):
                    copies = report['executables']
                    if mode == 'missing':
                        copies.pop()
                    elif mode == 'duplicate':
                        copies[3] = copy.deepcopy(copies[0])
                    elif mode == 'slot':
                        copies[0]['slot'] = 's1'
                    elif mode == 'replica':
                        copies[0]['replica'] = 'r1'
                    elif mode == 'copy-path':
                        copies[3]['isolated_path'] = copies[0]['isolated_path']
                    elif mode == 'root':
                        path = Path(copies[3]['isolated_path'])
                        copies[3]['isolated_path'] = str(path.parents[3] / 'other' / Path(*path.parts[-4:]))
                    elif mode == 'creation-order':
                        copies.reverse()
                    else:
                        for key in ('source_sha256_start', 'source_sha256_end', 'sha256_start', 'sha256_end'):
                            copies[3][key] = 'c' * 64
                primary = copy.deepcopy(self.primary)
                change(primary)
                self.assert_skipped(primary=primary)
                self.assert_stopped(self.make_runner(mutations={'reversed': change}))

    def test_crossover_fixture_inventory_rejects_shared_paths_and_broken_cells(self):
        modes = ('missing', 'duplicate', 'order', 'replica', 'home', 'cache', 'project', 'root-map', 'hash-map', 'inventory-hash')
        for mode in modes:
            with self.subTest(mode=mode):
                def change(report, _args=None):
                    row = report['results'][0]
                    inventory = row['fixture_inventory']
                    if mode == 'missing':
                        inventory.pop()
                    elif mode == 'duplicate':
                        inventory[3] = copy.deepcopy(inventory[0])
                    elif mode == 'order':
                        inventory.reverse()
                    elif mode == 'replica':
                        inventory[0]['replica'] = 'r1'
                    elif mode in ('home', 'project'):
                        inventory[3][mode] = inventory[0][mode]
                    elif mode == 'cache':
                        inventory[3]['cache_paths']['ENGINE'] = inventory[0]['cache_paths']['ENGINE']
                    elif mode == 'root-map':
                        row['fixture_roots']['s1']['baseline'] = row['fixture_roots']['s0']['baseline']
                    elif mode == 'hash-map':
                        row['fixture_sha256']['s1'].pop('baseline')
                    else:
                        inventory[0]['input_sha256'] = 'c' * 64
                primary = copy.deepcopy(self.primary)
                change(primary)
                self.assert_skipped(primary=primary)
                self.assert_stopped(self.make_runner(mutations={'reversed': change}))

    def test_crossover_schedule_rejects_incomplete_or_unbalanced_eight_batch_units(self):
        modes = ('truncated', 'slot', 'replicas', 'order', 'unit-index', 'quad-index', 'declared-only', 'initialization-order')
        for mode in modes:
            with self.subTest(mode=mode):
                def change(report, _args=None):
                    row = report['results'][0]
                    if mode == 'truncated':
                        row['samples'].pop()
                    elif mode == 'initialization-order':
                        row['initialization_order'].reverse()
                    elif mode == 'declared-only':
                        row['slot_schedule'][1]['slot'] = 's0'
                    else:
                        sample, schedule = row['samples'][1], row['slot_schedule'][1]
                        if mode == 'slot':
                            sample['slot'] = schedule['slot'] = 's0'
                        elif mode == 'replicas':
                            sample['replicas'] = schedule['replicas'] = {'baseline': 'r0', 'candidate': 'r1'}
                        elif mode == 'order':
                            sample['order'].reverse()
                            schedule['order'].reverse()
                        elif mode == 'unit-index':
                            sample['unit_index'] = schedule['unit_index'] = False
                        else:
                            sample['quad_index'] = schedule['quad_index'] = 0.0
                primary = copy.deepcopy(self.primary)
                change(primary)
                self.assert_skipped(primary=primary)
                self.assert_stopped(self.make_runner(mutations={'reversed': change}))

    def test_crossover_analysis_cannot_claim_more_units_or_different_raw_statistics(self):
        modes = ('unit-count', 'unit-ratio', 'paired-ratio', 'interval', 'noise', 'pair-ratio')
        for mode in modes:
            with self.subTest(mode=mode):
                def change(report, _args=None):
                    row = report['results'][0]
                    analysis = row['analysis']
                    if mode == 'unit-count':
                        analysis['inference_unit_count'] = 20
                    elif mode == 'unit-ratio':
                        analysis['inference_unit_ratios'][0] = 0.95
                    elif mode == 'paired-ratio':
                        analysis['paired_ratios'][0] = 0.95
                    elif mode == 'interval':
                        analysis['median_ratio_interval']['lower'] = 0.95
                    elif mode == 'noise':
                        analysis['relative_mad']['baseline'] = 0.01
                    else:
                        row['samples'][0]['ratio'] = 0.95
                primary = copy.deepcopy(self.primary)
                change(primary)
                self.assert_skipped(primary=primary)
                self.assert_stopped(self.make_runner(mutations={'reversed': change}))

    def test_stable_primary_replacement_cannot_retain_previous_build_hash(self):
        for label in benchmark.LABELS:
            source = self.sources[label]
            original = source.read_bytes()
            with self.subTest(label=label):
                source.write_bytes(original + b' Stable replacement before primary launch.\n')
                try:
                    primary = self.make_report(self.metadata, self.sources, 'fail', gate=True)
                    observed = next(row for row in primary['executables'] if row['label'] == label)
                    self.assertTrue(observed['unchanged'])
                    self.assertEqual(observed['source_sha256_start'], benchmark.executable_sha256(source))
                    self.assertNotEqual(observed['source_sha256_start'], self.metadata[label]['artifact_sha256'])
                    self.assert_skipped(primary=primary)
                finally:
                    source.write_bytes(original)

    def test_control_build_hash_binds_stable_copies_to_transformed_role_provenance(self):
        expected = {path: benchmark.executable_sha256(path)
                    for path in (*self.sources.values(), *self.verifiers.values())}
        for kind in ('reversed', 'candidate-self'):
            selected = {'baseline': self.sources['candidate'],
                        'candidate': self.sources['baseline'] if kind == 'reversed' else self.sources['candidate']}
            metadata = attribution.control_metadata(self.metadata, kind, self.primary_path)
            report = self.make_report(metadata, selected, 'pass')
            self.assertIsNone(attribution.stop_reason(report, selected, expected))
            for label in benchmark.LABELS:
                with self.subTest(kind=kind, label=label):
                    stale = copy.deepcopy(report)
                    stale['metadata'][label]['artifact_sha256'] = 'c' * 64
                    self.assertEqual(attribution.stop_reason(stale, selected, expected),
                                     'Control build artifact hash differs from measured executable')

    def test_supplied_build_hashes_must_be_valid_but_absent_legacy_hashes_are_allowed(self):
        for value in (None, False, 123, 'c' * 63, 'g' * 64):
            with self.subTest(value=value):
                metadata = copy.deepcopy(self.metadata)
                metadata['candidate']['artifact_sha256'] = value
                primary = self.make_report(metadata, self.sources, 'fail', gate=True)
                self.assert_skipped(primary=primary, metadata=metadata)
                selected = dict(self.sources)
                expected = {path: benchmark.executable_sha256(path)
                            for path in (*selected.values(), *self.verifiers.values())}
                control = self.make_report(metadata, selected, 'pass')
                self.assertEqual(attribution.stop_reason(control, selected, expected),
                                 'Control build artifact hash differs from measured executable')
        metadata = copy.deepcopy(self.metadata)
        for label in benchmark.LABELS:
            metadata[label].pop('artifact_sha256')
        primary = self.make_report(metadata, self.sources, 'fail', gate=True)
        result, summary, runner, _ = self.invoke(primary=primary, metadata=metadata)
        self.assertEqual(result, 0)
        self.assertEqual(summary['status'], 'complete')
        self.assertEqual(runner.call_count, 2)

    def test_control_source_copy_hash_disagreement_stops(self):
        def change(report, _args):
            report['executables'][0]['sha256_start'] = 'c' * 64
            report['executables'][0]['sha256_end'] = 'c' * 64
        self.assert_stopped(self.make_runner(mutations={'reversed': change}))

    def test_control_hash_cannot_change_to_an_unrelated_stable_binary(self):
        def change(report, _args):
            row = report['executables'][0]
            for key in ('source_sha256_start', 'source_sha256_end', 'sha256_start', 'sha256_end'):
                row[key] = 'c' * 64
        self.assert_stopped(self.make_runner(mutations={'reversed': change}))

    def test_control_source_path_must_match_selected_role(self):
        def change(report, _args):
            report['executables'][0]['source_path'] = str(self.sources['baseline'])
        self.assert_stopped(self.make_runner(mutations={'reversed': change}))

    def test_control_duplicate_or_missing_executable_role_stops(self):
        for mode in ('duplicate', 'missing'):
            with self.subTest(mode=mode):
                def change(report, _args):
                    if mode == 'duplicate':
                        report['executables'][3] = copy.deepcopy(report['executables'][0])
                    else:
                        report['executables'].pop()
                self.assert_stopped(self.make_runner(mutations={'reversed': change}))

    def test_control_provenance_or_gate_mismatch_stops(self):
        for mode in ('metadata', 'gate'):
            with self.subTest(mode=mode):
                def change(report, _args):
                    if mode == 'metadata':
                        report['metadata']['baseline'] = copy.deepcopy(self.metadata['baseline'])
                    else:
                        report['gate'] = True
                self.assert_stopped(self.make_runner(mutations={'reversed': change}))

    def test_control_policy_mismatch_stops(self):
        def change(report, _args):
            report['policy']['pairs_per_case'] = 16
        self.assert_stopped(self.make_runner(mutations={'reversed': change}))

    def test_control_poppler_path_or_hash_mismatch_stops(self):
        for mode in ('path', 'hash', 'missing'):
            with self.subTest(mode=mode):
                def change(report, _args):
                    if mode == 'missing':
                        del report['output_verifiers']['pdfimages']
                    elif mode == 'path':
                        report['output_verifiers']['pdfimages']['path'] = str(self.sources['baseline'])
                    else:
                        report['output_verifiers']['pdfimages']['sha256_end'] = 'c' * 64
                self.assert_stopped(self.make_runner(mutations={'reversed': change}))

    def test_control_fixture_mutation_or_hash_disagreement_stops(self):
        for mode in ('flag', 'hash'):
            with self.subTest(mode=mode):
                def change(report, _args):
                    row = report['results'][0]
                    if mode == 'flag':
                        row['fixture_unchanged'] = False
                    else:
                        row['fixture_sha256_end']['s0']['candidate'] = 'c' * 64
                self.assert_stopped(self.make_runner(mutations={'reversed': change}))

    def test_primary_metadata_revision_settings_and_policy_mismatches_skip(self):
        for mode in ('metadata', 'revision', 'build', 'settings', 'policy', 'bool-policy',
                     'replica-policy', 'float-unit-policy', 'bool-unit-policy', 'slot-policy'):
            with self.subTest(mode=mode):
                primary, metadata = copy.deepcopy(self.primary), copy.deepcopy(self.metadata)
                if mode == 'metadata':
                    metadata['optional']['preserve'] = ['changed']
                elif mode == 'revision':
                    metadata['candidate']['revision'] = 'c' * 40
                    primary['metadata'] = copy.deepcopy(metadata)
                elif mode == 'build':
                    metadata['candidate']['build_command'] = 'unmatched command'
                    primary['metadata'] = copy.deepcopy(metadata)
                elif mode == 'settings':
                    metadata['release_settings']['lto'] = 'thin'
                    primary['metadata'] = copy.deepcopy(metadata)
                elif mode == 'policy':
                    primary['policy']['relative_threshold'] = 0.20
                elif mode == 'bool-policy':
                    primary['policy']['min_sample_seconds'] = True
                elif mode == 'replica-policy':
                    primary['policy']['replica_assignment']['s1']['baseline'] = 'r0'
                elif mode == 'float-unit-policy':
                    primary['policy']['inference_units_per_case'] = 10.0
                elif mode == 'bool-unit-policy':
                    primary['policy']['pairs_per_inference_unit'] = True
                else:
                    primary['policy']['slot_ids'] = ['s0', 's0']
                self.assert_skipped(primary=primary, metadata=metadata)

    def test_equal_invalid_or_unmatched_primary_revisions_skip(self):
        for revisions in ({'baseline': 'a' * 40, 'candidate': 'a' * 40},
                          {'baseline': 'not-a-sha', 'candidate': 'b' * 40},
                          {'baseline': 'a' * 40, 'candidate': 'c' * 40}):
            with self.subTest(revisions=revisions):
                self.assert_skipped(revisions=revisions)

    def test_primary_executable_role_source_and_verifier_policy_failures_skip(self):
        for mode in ('duplicate-role', 'source-path', 'missing-verifier', 'verifier-path', 'verifier-hash'):
            with self.subTest(mode=mode):
                primary = copy.deepcopy(self.primary)
                if mode == 'duplicate-role':
                    primary['executables'][1]['label'] = 'baseline'
                elif mode == 'source-path':
                    primary['executables'][0]['source_path'] = str(self.sources['candidate'])
                elif mode == 'missing-verifier':
                    del primary['output_verifiers']['pdfinfo']
                elif mode == 'verifier-path':
                    primary['output_verifiers']['pdfinfo']['path'] = str(self.sources['baseline'])
                else:
                    primary['output_verifiers']['pdfinfo']['sha256_end'] = 'c' * 64
                self.assert_skipped(primary=primary)

    def test_primary_fixture_case_sample_warmup_and_oracle_failures_skip(self):
        modes = ('duplicate-case', 'fixture-hash', 'fixture-flag', 'partial-pair', 'missing-command',
                 'batch-total', 'analysis', 'warmup-order', 'sample-order', 'oracle', 'initialization')
        for mode in modes:
            with self.subTest(mode=mode):
                primary = copy.deepcopy(self.primary)
                row = primary['results'][0]
                if mode == 'duplicate-case':
                    primary['results'][1]['case'] = row['case']
                elif mode == 'fixture-hash':
                    row['fixture_sha256_end']['s0']['candidate'] = 'c' * 64
                elif mode == 'fixture-flag':
                    row['fixture_unchanged'] = False
                elif mode == 'partial-pair':
                    del row['samples'][0]['candidate']
                elif mode == 'missing-command':
                    row['samples'][0]['candidate']['commands'] = []
                elif mode == 'batch-total':
                    row['samples'][0]['candidate']['seconds'] = 8.0
                elif mode == 'analysis':
                    row['analysis']['status'] = 'pass'
                elif mode == 'warmup-order':
                    row['warmups'][1]['order'] = row['warmups'][0]['order'][:]
                elif mode == 'sample-order':
                    row['samples'][1]['order'] = row['samples'][0]['order'][:]
                elif mode == 'oracle':
                    row['samples'][0]['candidate']['output_validation']['expected_text'] = 'Wrong text.'
                else:
                    del row['initialization']['s0']['candidate']
                self.assert_skipped(primary=primary)

    def test_non_object_primary_json_is_skipped(self):
        for value in (None, [], ['unexpected'], 'report', 12, True):
            with self.subTest(value=value):
                self.output_index += 1
                directory = self.work / ('invalid-json-' + str(self.output_index))
                self.primary_path.write_text(json.dumps(value), encoding='utf-8')
                runner = self.make_runner()
                result = attribution.run_attribution(self.primary_path, self.metadata_path, self.sources,
                                                     self.revisions, directory, runner=runner)
                self.assertEqual(result, 0)
                summary = json.loads((directory / 'attribution.json').read_text(encoding='utf-8'))
                self.assertEqual(summary['status'], 'skipped')
                runner.assert_not_called()

    def test_malformed_primary_exit_does_not_break_skipped_diagnostic(self):
        for value in (None, False, [], float('nan'), float('inf')):
            with self.subTest(value=value):
                primary = copy.deepcopy(self.primary)
                primary['exit_code'] = value
                self.assert_skipped(primary=primary)

    def test_malformed_primary_collections_and_rows_are_skipped(self):
        cases = (
            ('policy', None), ('executables', {}), ('executables', [None, None]),
            ('output_verifiers', []), ('results', {}), ('results', [None]),
            ('metadata', []), ('errors', None),
        )
        for field, value in cases:
            with self.subTest(field=field, value=value):
                primary = copy.deepcopy(self.primary)
                primary[field] = value
                self.assert_skipped(primary=primary)
        for field, value in (('samples', {}), ('samples', [None] * 40), ('warmups', [None, None]),
                             ('initialization', []), ('fixture_sha256', []), ('analysis', [])):
            with self.subTest(case_field=field, value=value):
                primary = copy.deepcopy(self.primary)
                primary['results'][0][field] = value
                self.assert_skipped(primary=primary)

    def test_huge_or_invalid_numbers_do_not_escape_as_exceptions(self):
        for value in (10 ** 400, True, 0, -1, float('nan'), float('inf')):
            for place in ('elapsed', 'batch', 'command', 'initialization'):
                with self.subTest(value=str(value)[:20], place=place):
                    primary = copy.deepcopy(self.primary)
                    row = primary['results'][0]
                    if place == 'elapsed':
                        primary['elapsed_seconds'] = value
                    elif place == 'batch':
                        row['samples'][0]['candidate']['seconds'] = value
                    elif place == 'command':
                        row['samples'][0]['candidate']['commands'][0]['seconds'] = value
                    else:
                        row['initialization']['s0']['candidate']['seconds'] = value
                    self.assert_skipped(primary=primary)

    def test_existing_output_directory_is_not_reused_or_overwritten(self):
        directory = self.work / 'existing-controls'
        directory.mkdir()
        sentinel = directory / 'attribution.json'
        sentinel.write_bytes(b'Existing report must survive.\n')
        runner = self.make_runner()
        with self.assertRaises(OSError):
            attribution.run_attribution(self.primary_path, self.metadata_path, self.sources,
                                        self.revisions, directory, runner=runner)
        self.assertEqual(sentinel.read_bytes(), b'Existing report must survive.\n')
        self.assertEqual(list(directory.iterdir()), [sentinel])
        runner.assert_not_called()

    def test_output_symlink_is_not_followed_or_overwritten(self):
        target = self.work / 'existing-target'
        target.mkdir()
        sentinel = target / 'sentinel'
        sentinel.write_bytes(b'Preserve this symlink target.\n')
        directory = self.work / 'controls-symlink'
        directory.symlink_to(target, target_is_directory=True)
        runner = self.make_runner()
        with self.assertRaises(OSError):
            attribution.run_attribution(self.primary_path, self.metadata_path, self.sources,
                                        self.revisions, directory, runner=runner)
        self.assertTrue(directory.is_symlink())
        self.assertEqual(sentinel.read_bytes(), b'Preserve this symlink target.\n')
        self.assertEqual(list(target.iterdir()), [sentinel])
        runner.assert_not_called()

    def test_dangling_output_symlink_is_not_followed(self):
        target = self.work / 'absent-symlink-target'
        directory = self.work / 'dangling-controls'
        directory.symlink_to(target, target_is_directory=True)
        runner = self.make_runner()
        with self.assertRaises(OSError):
            attribution.run_attribution(self.primary_path, self.metadata_path, self.sources,
                                        self.revisions, directory, runner=runner)
        self.assertTrue(directory.is_symlink())
        self.assertFalse(target.exists())
        runner.assert_not_called()

    def test_dangling_output_ancestor_is_not_created(self):
        target = self.work / 'absent-ancestor-target'
        ancestor = self.work / 'dangling-parent'
        ancestor.symlink_to(target, target_is_directory=True)
        runner = self.make_runner()
        with self.assertRaises(OSError):
            attribution.run_attribution(self.primary_path, self.metadata_path, self.sources,
                                        self.revisions, ancestor / 'controls', runner=runner)
        self.assertTrue(ancestor.is_symlink())
        self.assertFalse(target.exists())
        runner.assert_not_called()

    def test_output_input_aliases_are_not_overwritten(self):
        for index, target in enumerate((self.primary_path, self.metadata_path, self.sources['candidate'])):
            with self.subTest(target=target.name):
                before = target.read_bytes()
                alias = self.work / ('input-alias-' + str(index))
                os.link(target, alias)
                runner = self.make_runner()
                with self.assertRaises(OSError):
                    attribution.run_attribution(self.primary_path, self.metadata_path, self.sources,
                                                self.revisions, alias, runner=runner)
                self.assertEqual(target.read_bytes(), before)
                self.assertEqual(alias.read_bytes(), before)
                runner.assert_not_called()

    def test_previous_control_artifact_change_is_detected(self):
        def change(_report, args):
            previous = args.output.parent / 'reversed.json'
            previous.write_bytes(previous.read_bytes() + b' changed')
        runner = self.make_runner(mutations={'candidate-self': change})
        result, summary, runner, _ = self.invoke(runner=runner)
        self.assertEqual(result, 1)
        self.assertEqual(summary['status'], 'stopped')
        self.assertEqual(runner.call_count, 2)
        self.assertIn('changed', summary['reason'])

    def test_existing_next_control_destinations_are_never_overwritten(self):
        for suffix in ('.json', '.md', '.log', '-metadata.json'):
            with self.subTest(suffix=suffix):
                def change(_report, args):
                    destination = args.output.parent / ('candidate-self' + suffix)
                    destination.write_bytes(b'Existing next-control output must survive.\n')
                result, summary, runner, directory = self.invoke(
                    runner=self.make_runner(mutations={'reversed': change}))
                self.assertEqual(result, 1)
                self.assertEqual(summary['status'], 'stopped')
                self.assertEqual(runner.call_count, 1)
                self.assertEqual((directory / ('candidate-self' + suffix)).read_bytes(),
                                 b'Existing next-control output must survive.\n')


if __name__ == '__main__':
    unittest.main()
