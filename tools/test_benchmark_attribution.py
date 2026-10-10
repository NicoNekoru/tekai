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
        ratios = [1.2] * 40 if sample_status == 'fail' else (
            [1.08, 1.08, 1.12, 1.12] * 10 if sample_status == 'inconclusive' else [1.0] * 40)
        command_id = 0

        def batch(case, seconds, iterations=1):
            nonlocal command_id
            commands = []
            for _ in range(iterations):
                command_id += 4
                commands.append({'command_id': command_id, 'seconds': seconds})
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
                'fixture_sha256': {label: fixture_hash for label in benchmark.LABELS},
                'fixture_sha256_end': {label: fixture_hash for label in benchmark.LABELS},
                'fixture_unchanged': True,
                'fixture_roots': {label: str(prefix / label / case) for label in benchmark.LABELS},
                'min_sample_seconds': 1.0,
                'initialization': {}, 'warmups': [], 'calibration_pilots': [], 'samples': [],
            }
            for label in benchmark.paired_order(0, case_index):
                observed = batch(case, 1.0)
                row['initialization'][label] = {
                    'command_id': observed['commands'][0]['command_id'],
                    'seconds': observed['seconds'], 'output_validation': observed['output_validation'],
                }
            for index in range(2):
                warmup = {'order': list(benchmark.paired_order(index, case_index))}
                for label in warmup['order']:
                    warmup[label] = batch(case, 1.0)
                row['warmups'].append(warmup)
            for index in range(2):
                pilot = {'pair_index': index, 'order': list(benchmark.paired_order(index, case_index)),
                         'iterations': 32}
                for label in pilot['order']:
                    pilot[label] = batch(case, 1.0, 32)
                row['calibration_pilots'].append(pilot)
            row['calibration'] = benchmark.calibration_details(row['calibration_pilots'], 1.0, 512)
            iterations = benchmark.calibrated_iterations(row['calibration_pilots'], 1.0, 512)
            row['iterations_per_batch'] = iterations
            for index, ratio in enumerate(ratios):
                sample = {'pair_index': index, 'order': list(benchmark.paired_order(index, case_index)),
                          'iterations': iterations}
                for label in sample['order']:
                    sample[label] = batch(case, ratio if label == 'candidate' else 1.0, iterations)
                sample['ratio'] = ratio
                row['samples'].append(sample)
            row['analysis'] = benchmark.analyze_pairs(row['samples'], 0.10, 1.0)
            rows.append(row)
        report = {
            'schema_version': 3, 'gate': gate, 'status': status,
            'metadata': copy.deepcopy(metadata), 'policy': copy.deepcopy(attribution.POLICY),
            'machine': {'system': 'Darwin', 'architecture': 'arm64'},
            'elapsed_seconds': 3.0, 'exit_code': benchmark.comparison_exit(status, gate),
            'executables': [], 'output_verifiers': {}, 'results': rows,
            'errors': [] if status != 'error' else ['BenchmarkError: Command failed; return code 7'],
        }
        for label in benchmark.LABELS:
            source = Path(sources[label]).resolve()
            content_hash = benchmark.executable_sha256(source)
            report['executables'].append({
                'label': label, 'source_path': str(source),
                'isolated_path': str(prefix.parent / (prefix.name + '-copies') / label / 'tekai'),
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
        result, summary, runner, _ = self.invoke(primary=primary)
        self.assertEqual(result, 0)
        self.assertEqual(summary['status'], 'complete')
        self.assertEqual(runner.call_count, 2)

    def test_recomputed_pilot_size_disagreement_rejects_primary_and_control(self):
        def change(report, _args=None):
            row = report['results'][0]
            for pilot in row['calibration_pilots']:
                for label in benchmark.LABELS:
                    for command in pilot[label]['commands']:
                        command['seconds'] = 2.0
                    pilot[label]['seconds'] = 64.0
            # The independently known rate of two seconds needs one
            # iteration, but formal batches still claim the old count of two.
            row['calibration'] = benchmark.calibration_details(row['calibration_pilots'], 1.0, 512)
            self.assertEqual(row['calibration']['selected_iterations'], 1)
            self.assertEqual(row['iterations_per_batch'], 2)
        primary = copy.deepcopy(self.primary)
        change(primary)
        self.assert_skipped(primary=primary)
        self.assert_stopped(self.make_runner(mutations={'reversed': change}))

    def test_full_raw_pilot_contract_rejects_primary_and_control_corruption(self):
        modes = ('missing', 'partial', 'duplicate', 'order', 'pair-index', 'iterations',
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
                        details['normalized_rates_seconds_per_iteration']['baseline'][0] = 2.0
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
                used_ids.extend(value['command_id'] for value in row['initialization'].values())
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
        def change(report, _args=None):
            report['schema_version'] = 2
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

    def test_genuine_calibration_infeasible_control_continues_to_self(self):
        def change(report, _args):
            self.make_infeasible_case(report['results'][0])
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
            self.assertEqual((args.pairs, args.warmups, args.max_iterations), (40, 2, 512))
            self.assertEqual((args.timeout, args.budget, args.threshold), (30.0, 900.0, 0.10))
            self.assertEqual((args.min_sample_seconds, args.warm_cache_min_sample_seconds), (1.0, 1.0))

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
                    report['executables'] = [copy.deepcopy(report['executables'][0])]
                    if mode == 'duplicate':
                        report['executables'].append(copy.deepcopy(report['executables'][0]))
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
                        row['fixture_sha256_end']['candidate'] = 'c' * 64
                self.assert_stopped(self.make_runner(mutations={'reversed': change}))

    def test_primary_metadata_revision_settings_and_policy_mismatches_skip(self):
        for mode in ('metadata', 'revision', 'build', 'settings', 'policy', 'bool-policy'):
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
                else:
                    primary['policy']['min_sample_seconds'] = True
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
                    row['fixture_sha256_end']['candidate'] = 'c' * 64
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
                    del row['initialization']['candidate']
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
                        row['initialization']['candidate']['seconds'] = value
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
