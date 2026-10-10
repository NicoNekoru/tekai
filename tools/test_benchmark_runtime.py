"""Bounded paired statistics, order, isolation and failure checks without TeX."""

import contextlib
import io
import json
import math
import os
from pathlib import Path
import signal
import sys
import tempfile
import time
import unittest
from unittest.mock import patch

import benchmark_runtime as bench


def samples(ratios, seconds=1):
    return [{'order': bench.paired_order(index), 'iterations': 1,
             'baseline': {'seconds': seconds}, 'candidate': {'seconds': seconds * ratio}}
            for index, ratio in enumerate(ratios)]


class StatisticsTests(unittest.TestCase):
    def analyze(self, ratios, seconds=1):
        return bench.analyze_pairs(samples(ratios, seconds), 0.10, 0.25)

    def test_exact_interval_matches_binomial_tail_and_multiplicity(self):
        values = [1 + index / 100 for index in range(20)]
        interval = bench.median_interval(values, 0.05 / 3)
        self.assertEqual(interval['order_statistic'], 5)
        self.assertEqual((interval['lower'], interval['upper']), (1.04, 1.15))
        self.assertAlmostEqual(interval['coverage_at_least'], 1 - 2 * sum(math.comb(20, k) for k in range(5)) / 2 ** 20)
        self.assertGreaterEqual(interval['coverage_at_least'], 1 - 0.05 / 3)

    def test_equal_binary_and_real_regression_are_distinguished(self):
        for ratio, outcome in ((1, 'pass'), (0.7, 'pass'), (1.09, 'pass'), (1.11, 'fail')):
            with self.subTest(ratio=ratio):
                analysis = self.analyze([ratio] * 20)
                self.assertEqual(analysis['status'], outcome)
                self.assertEqual(len(analysis['balanced_block_ratios']), 10)

    def test_crossing_threshold_is_inconclusive(self):
        analysis = self.analyze([1.08, 1.08, 1.12, 1.12] * 5)
        self.assertEqual(analysis['status'], 'inconclusive')
        self.assertIn('crosses', analysis['reasons'][0])

    def test_high_variability_and_order_bias_cannot_fail_a_gate(self):
        noisy = samples([1.3] * 20)
        for index, sample in enumerate(noisy):
            factor = 0.5 if index % 2 else 1.5
            for label in bench.LABELS:
                sample[label]['seconds'] *= factor
        result = bench.analyze_pairs(noisy, 0.10, 0.25)
        self.assertEqual(result['status'], 'inconclusive')
        self.assertTrue(any('noise limit' in reason for reason in result['reasons']))
        result = self.analyze([1.5, 1.2] * 10)
        self.assertEqual(result['status'], 'inconclusive')
        self.assertTrue(any('order-bias' in reason for reason in result['reasons']))

    def test_small_sample_and_short_batch_are_inconclusive(self):
        self.assertEqual(self.analyze([1.2] * 12)['status'], 'inconclusive')
        self.assertEqual(self.analyze([1.2] * 20, seconds=0.01)['status'], 'inconclusive')
        self.assertIsNone(bench.median_interval([1] * 6, 0.05 / 3))

    def test_balancing_is_per_case_and_rejects_unbalanced_data(self):
        for case_index in range(3):
            order = [bench.paired_order(index, case_index) for index in range(20)]
            self.assertEqual(order.count(('baseline', 'candidate')), 10)
            self.assertEqual(order.count(('candidate', 'baseline')), 10)
            self.assertTrue(all(order[index] == tuple(reversed(order[index + 1])) for index in range(0, 20, 2)))
        bad = samples([1] * 20)
        bad[1]['order'] = bad[0]['order']
        with self.assertRaises(ValueError):
            bench.analyze_pairs(bad, 0.10, 0.25)

    def test_invalid_timings_are_not_statistics(self):
        for value in (0, -1, math.inf, math.nan):
            with self.subTest(value=value), self.assertRaises(ValueError):
                bench.median_interval([value], 0.05)
            bad = samples([1] * 20)
            bad[0]['candidate']['seconds'] = value
            with self.subTest(value=value), self.assertRaises(ValueError):
                bench.analyze_pairs(bad, 0.10, 0.25)

    def test_calibration_has_headroom_for_an_improving_candidate(self):
        warmups = samples([0.8, 0.8], seconds=0.01)
        iterations = bench.calibrated_iterations(warmups, 0.25, 128)
        self.assertEqual(iterations, 63)
        # Even another 40% improvement after warmup retains meaningful batches.
        measured = samples([0.48] * 20, seconds=0.01 * iterations)
        analysis = bench.analyze_pairs(measured, 0.10, 0.25)
        self.assertEqual(analysis['status'], 'pass')
        self.assertEqual(bench.calibrated_iterations(warmups, 0.8, 128), 128)

    def test_infeasible_calibration_is_rejected_before_sampling(self):
        with self.assertRaisesRegex(ValueError, 'ceiling cannot reach'):
            bench.calibrated_iterations(samples([1, 1], seconds=0.003), 1, 256)
        # The ceiling may limit extra headroom, but not the minimum itself.
        self.assertEqual(bench.calibrated_iterations(samples([1, 1], seconds=0.004), 1, 256), 256)

    def test_gate_and_advisory_exits_are_explicit(self):
        for status, exit_code in (('pass', 0), ('fail', 1), ('inconclusive', 2), ('error', 1)):
            self.assertEqual(bench.comparison_exit(status, True), exit_code)
            self.assertEqual(bench.comparison_exit(status, False), 1 if status == 'error' else 0)


@unittest.skipUnless(os.name == 'posix', 'Owned process groups require POSIX')
class SupervisionTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix='test-paired-supervisor-')
        self.work = Path(self.temporary.name).resolve()
        self.runner = bench.Supervisor(self.work, 1, time.monotonic() + 5)

    def tearDown(self):
        self.runner.close()
        self.temporary.cleanup()

    def test_success_keeps_native_completion_time_and_no_owned_process(self):
        result = self.runner.execute([sys.executable, '-c', 'print("ok")'], self.work, os.environ.copy())
        self.assertEqual(result['stdout'].strip(), 'ok')
        self.assertGreater(result['seconds'], 0)
        self.assertFalse(self.runner.owned)

    def test_error_and_output_overflow_are_not_usable_samples(self):
        for code in ('raise SystemExit(7)', f'print("x" * {bench.MAX_CAPTURE_BYTES + 1})'):
            with self.subTest(code=code), self.assertRaises(bench.BenchmarkError):
                self.runner.execute([sys.executable, '-c', code], self.work, os.environ.copy())
            self.assertFalse(self.runner.owned)

    def test_timeout_kills_the_owned_process_group(self):
        self.runner.timeout = 0.05
        stopped = []
        original_stop = self.runner.stop

        def stop(process):
            stopped.append(process.pid)
            original_stop(process)

        with patch.object(self.runner, 'stop', side_effect=stop):
            with self.assertRaisesRegex(bench.BenchmarkError, 'deadline'):
                self.runner.execute([sys.executable, '-c', 'import time; time.sleep(10)'], self.work, os.environ.copy())
        self.assertEqual(len(stopped), 1)
        self.assertFalse(self.runner.owned)
        with self.assertRaises(ProcessLookupError):
            os.killpg(stopped[0], 0)

    def test_completed_parent_cannot_leave_its_child_group_running(self):
        # The child holds file-backed logs open. It must not delay completion,
        # and the inherited cleanup kills it after its parent exits normally.
        stopped = []
        original_stop = self.runner.stop

        def stop(process):
            stopped.append(process)
            original_stop(process)

        code = 'import os,time; pid=os.fork(); time.sleep(10) if pid == 0 else None'
        with patch.object(self.runner, 'stop', side_effect=stop), patch('performance_ci.os.killpg', wraps=os.killpg) as kill:
            result = self.runner.execute([sys.executable, '-c', code], self.work, os.environ.copy())
        self.assertLess(result['seconds'], 1)
        self.assertFalse(self.runner.owned)
        kill.assert_called_with(stopped[0].pid, signal.SIGKILL)

    def test_whole_run_deadline_prevents_launch(self):
        self.runner.deadline = time.monotonic() - 1
        with patch.object(self.runner, 'start') as start, self.assertRaisesRegex(bench.BenchmarkError, 'Whole-run'):
            self.runner.execute([sys.executable, '-c', 'pass'], self.work, os.environ.copy())
        start.assert_not_called()


class FixtureTests(unittest.TestCase):
    def test_sides_have_equal_inputs_and_disjoint_private_caches(self):
        with tempfile.TemporaryDirectory(prefix='test-paired-isolation-') as temporary:
            work = Path(temporary)
            with patch.dict(os.environ, {'TEKAI_EMBEDDED_ENGINE_RUNNER': '/host/engine', 'TEXINPUTS': '/host/files',
                                         'TEKAI_FORMAT_CACHE': '/host/cache', 'HOME': '/host/home',
                                         **{name: '/host/search-root' for name in bench.SEARCH_ENV_VARS}}):
                fixtures = {label: bench.make_fixture(work / label, Path('/binary'), 'nested-lookup') for label in bench.LABELS}
            self.assertEqual(fixtures['baseline']['input_sha256'], fixtures['candidate']['input_sha256'])
            for label in bench.LABELS:
                fixture = fixtures[label]
                env = fixture['env']
                self.assertNotIn('TEKAI_EMBEDDED_ENGINE_RUNNER', env)
                for name in bench.SEARCH_ENV_VARS:
                    self.assertNotIn(name, env)
                self.assertEqual(env['PATH'], '')
                for key in ('HOME', 'TEKAI_ENGINE_CACHE', 'TEKAI_FORMAT_CACHE', 'TEKAI_AUX_CACHE', 'TEKAI_BIBTEX_CACHE'):
                    self.assertTrue(Path(env[key]).is_relative_to(fixture['root']))
                    self.assertNotEqual(env[key], fixtures[next(side for side in bench.LABELS if side != label)]['env'][key])
                self.assertTrue((fixture['project'] / 'content/ordinary/needle.tex').is_file())

    def test_cache_fixture_requires_a_real_warm_hit(self):
        with tempfile.TemporaryDirectory(prefix='test-paired-cache-') as temporary:
            out = Path(temporary)
            (out / 'main.pdf').write_bytes(b'%PDF-1.4\n%%EOF\n')
            fixture = dict(out=out, project=out, command=[], env={})
            for report in ({'skipped': False, 'tex_runs': 1}, {'skipped': True, 'tex_runs': 1},
                           {'skipped': True, 'tex_runs': False}):
                with self.subTest(report=report), patch.object(bench.Supervisor, 'execute', return_value={'stdout': json.dumps(report)}):
                    with self.assertRaises(bench.BenchmarkError):
                        bench.execute_fixture(bench.Supervisor(out, 1, time.monotonic() + 1), fixture, 'warm-build-cache')

    def test_expected_text_rejects_an_empty_successful_pdf(self):
        with tempfile.TemporaryDirectory(prefix='test-paired-output-') as temporary:
            out = Path(temporary)
            fixture = dict(out=out, project=out, env={}, pdftotext='/verifier')
            runner = bench.Supervisor(out, 1, time.monotonic() + 1)
            with patch.object(runner, 'execute', return_value={'stdout': '', 'command_id': 1}):
                with self.assertRaisesRegex(bench.BenchmarkError, 'expected text'):
                    bench.check_fixture_output(runner, fixture, 'nested-lookup')
            with patch.object(runner, 'execute', return_value={'stdout': 'Ordinary nested\nlookup.\n', 'command_id': 1}):
                self.assertTrue(bench.check_fixture_output(runner, fixture, 'nested-lookup')['text_verified'])

    def test_image_dimensions_and_all_pages_are_independently_verified(self):
        with tempfile.TemporaryDirectory(prefix='test-paired-images-') as temporary:
            out = Path(temporary)
            (out / 'main.pdf').write_bytes(b'%PDF-1.4\n%%EOF\n')
            fixture = dict(out=out, project=out, env={}, pdfinfo='/info', pdfimages='/images')
            runner = bench.Supervisor(out, 1, time.monotonic() + 1)
            entries = [(page, kind) for page in range(1, 9) for kind in ('image', 'smask')]
            image_rows = '\n'.join(f'{page} {index} {kind} 1024 1024 {"rgb 3" if kind == "image" else "gray 1"} 8'
                                   for index, (page, kind) in enumerate(entries))
            for pages, image_text, valid in (('8', image_rows, True), ('1', image_rows, False),
                                            ('8', image_rows.replace('1024', '1'), False), ('8', '', False),
                                            ('8', image_rows.replace('gray 1', 'rgb 3'), False)):
                def verify(command, _project, _env):
                    if command[0] == '/info':
                        return {'stdout': 'Pages: ' + pages + '\n', 'command_id': 1}
                    if '-list' in command:
                        return {'stdout': image_text, 'command_id': 2}
                    prefix = command[-1]
                    for index, (_, kind) in enumerate(entries):
                        prefix.with_name(f'{prefix.name}-{index:03d}.ppm').touch()
                    return {'stdout': '', 'command_id': 3}

                with self.subTest(pages=pages, valid=valid), patch.object(runner, 'execute', side_effect=verify), \
                        patch.object(bench, 'check_uniform_pnm') as pixels:
                    if valid:
                        output = bench.check_fixture_output(runner, fixture, 'image-compile')
                        self.assertEqual(output['pages'], 8)
                        self.assertTrue(output['every_decoded_pixel_verified'])
                        self.assertEqual([call.args[1] for index, call in enumerate(pixels.call_args_list) if index % 2 == 0],
                                         [bytes((n * 7, n * 13, 90)) for n in range(8)])
                        self.assertEqual([call.args[1] for index, call in enumerate(pixels.call_args_list) if index % 2 == 1],
                                         [bytes((128, 128, 128))] * 8)
                    else:
                        with self.assertRaises(bench.BenchmarkError):
                            bench.check_fixture_output(runner, fixture, 'image-compile')

    def test_decoded_pixels_reject_wrong_colors_masks_and_duplicate_streams(self):
        with tempfile.TemporaryDirectory(prefix='test-paired-pixels-') as temporary:
            path = Path(temporary) / 'decoded.ppm'
            for expected in (bytes((7, 13, 90)), bytes((128,))):
                magic = b'P6' if len(expected) == 3 else b'P5'
                header = magic + b'\n2 2\n255\n'
                path.write_bytes(header + expected * 4)
                bench.check_uniform_pnm(path, expected, dimensions=(2, 2))
                # A duplicated earlier RGB image or a wrong mask with otherwise
                # correct dimensions/sample depth must not become timing evidence.
                wrong = bytes(len(expected))
                path.write_bytes(header + expected * 3 + wrong)
                with self.assertRaisesRegex(bench.BenchmarkError, 'pixels differ'):
                    bench.check_uniform_pnm(path, expected, dimensions=(2, 2))

    def test_markdown_output_collision_is_rejected_before_launch(self):
        for suffix in ('.md', '.MD', '.txt'):
            with self.subTest(suffix=suffix), patch.object(bench.Supervisor, 'start') as start, \
                    contextlib.redirect_stderr(io.StringIO()):
                with self.assertRaises(SystemExit) as failure:
                    bench.main(['--baseline', '/binary', '--candidate', '/binary', '--metadata', '/metadata',
                                '--output', '/private/tmp/report' + suffix])
            self.assertEqual(failure.exception.code, 2)
            start.assert_not_called()

    def test_existing_output_alias_cannot_overwrite_a_selected_binary(self):
        with tempfile.TemporaryDirectory(prefix='test-paired-alias-') as temporary:
            work = Path(temporary)
            source = work / 'binary'
            source.write_bytes(b'executable fixture')
            source.chmod(0o700)
            alias = work / 'alias.json'
            os.link(source, alias)
            metadata = work / 'metadata.json'
            metadata.write_text(json.dumps({'runner_label': 'test', 'toolchain': 'exact',
                               **{label: {'revision': 'a' * 40, 'build_command': 'test'} for label in bench.LABELS}}), encoding='utf-8')
            with patch.object(bench.Supervisor, 'start') as start, contextlib.redirect_stderr(io.StringIO()):
                with self.assertRaises(SystemExit) as failure:
                    bench.main(['--baseline', str(source), '--candidate', str(source), '--metadata', str(metadata),
                                '--output', str(alias)])
            self.assertEqual(failure.exception.code, 2)
            start.assert_not_called()
            self.assertEqual(source.read_bytes(), b'executable fixture')

    def test_metadata_requires_provenance_and_retains_optional_fields(self):
        with tempfile.TemporaryDirectory(prefix='test-paired-metadata-') as temporary:
            path = Path(temporary) / 'metadata.json'
            metadata = {'runner_label': 'test', 'toolchain': 'rustc exact version', 'optional': {'lto': 'fat'},
                        **{label: {'revision': 'a' * 40, 'build_command': 'cargo build --release'} for label in bench.LABELS}}
            path.write_text(json.dumps(metadata), encoding='utf-8')
            self.assertEqual(bench.load_metadata(path), metadata)
            del metadata['toolchain']
            path.write_text(json.dumps(metadata), encoding='utf-8')
            with self.assertRaises(ValueError):
                bench.load_metadata(path)

    def test_error_report_is_written_and_binary_hashes_are_rechecked(self):
        with tempfile.TemporaryDirectory(prefix='test-paired-failure-') as temporary:
            work = Path(temporary)
            source = work / 'binary'
            source.write_bytes(b'executable fixture')
            source.chmod(0o700)
            metadata = work / 'metadata.json'
            metadata.write_text(json.dumps({'runner_label': 'test', 'toolchain': 'exact',
                               **{label: {'revision': 'a' * 40, 'build_command': 'test'} for label in bench.LABELS}}), encoding='utf-8')
            output = work / 'report.json'
            argv = ['--baseline', str(source), '--candidate', str(source), '--metadata', str(metadata),
                    '--output', str(output), '--gate']
            with patch('benchmark_runtime.shutil.which', return_value=sys.executable), \
                    patch.object(bench, 'make_fixture', side_effect=bench.BenchmarkError('controlled fixture failure')), \
                    contextlib.redirect_stdout(io.StringIO()):
                result = bench.main(argv)
            report = json.loads(output.read_text())
            self.assertEqual((result, report['status'], report['exit_code']), (1, 'error', 1))
            self.assertEqual(len(report['executables']), 2)
            self.assertTrue(all(row['unchanged'] for row in report['executables']))
            self.assertTrue(all(row['sha256_start'] == bench.executable_sha256(source) for row in report['executables']))
            self.assertTrue(output.with_suffix('.md').is_file())

    def test_driver_initializes_before_warmups_and_checks_fixture_mutation(self):
        with tempfile.TemporaryDirectory(prefix='test-paired-driver-') as temporary:
            work = Path(temporary)
            source = work / 'binary'
            source.write_bytes(b'executable fixture')
            source.chmod(0o700)
            metadata = work / 'metadata.json'
            metadata.write_text(json.dumps({'runner_label': 'test', 'toolchain': 'exact',
                               **{label: {'revision': 'a' * 40, 'build_command': 'test'} for label in bench.LABELS}}), encoding='utf-8')
            output = work / 'report.json'
            argv = ['--baseline', str(source), '--candidate', str(source), '--metadata', str(metadata),
                    '--output', str(output), '--pairs', '16', '--min-sample-seconds', '0.25', '--gate']
            events = []
            warm_seconds = 0.01

            def fixture(root, binary, case):
                project = root / 'project'
                bench.document(project)
                return dict(root=root, project=project, label=root.parent.name, input_sha256=bench.fixture_sha256(project))

            def initialize(_runner, fixture, case, initializing=False):
                self.assertTrue(initializing)
                events.append((case, 'initialize', fixture['label']))
                return {'command_id': 0, 'seconds': 100, 'output_validation': {'verified': True}}

            def batch(_runner, fixture, case, iterations):
                events.append((case, 'batch', fixture['label'], iterations))
                elapsed = warm_seconds if fixture['label'] == 'baseline' else warm_seconds * 0.8
                return dict(seconds=elapsed * iterations, iterations=iterations, commands=[], output_validation={'verified': True})

            for mutate, warm_seconds, expected in ((False, 0.01, 0), (True, 0.01, 1), (False, 0.001, 2)):
                events.clear()

                def measured(*args):
                    result = batch(*args)
                    if mutate and args[-1] > 1:
                        (args[1]['project'] / 'main.tex').write_text('changed during measurement', encoding='utf-8')
                    return result

                with patch('benchmark_runtime.shutil.which', return_value=sys.executable), \
                        patch.object(bench, 'make_fixture', side_effect=fixture), \
                        patch.object(bench, 'execute_fixture', side_effect=initialize), \
                        patch.object(bench, 'batch', side_effect=measured), contextlib.redirect_stdout(io.StringIO()):
                    self.assertEqual(bench.main(argv), expected)
                report = json.loads(output.read_text())
                for row in report['results']:
                    case_events = [event for event in events if event[0] == row['case']]
                    self.assertEqual([event[1] for event in case_events[:2]], ['initialize', 'initialize'])
                    self.assertEqual([event[3] for event in case_events[2:6]], [1] * 4)
                    self.assertEqual(row['min_sample_seconds'], 1 if row['case'] == 'warm-build-cache' else 0.25)
                    self.assertEqual(row['fixture_unchanged'], not mutate)
                    if expected == 2:
                        self.assertEqual(len(case_events), 6)
                        self.assertEqual(row['samples'], [])
                        self.assertEqual(row['analysis']['status'], 'inconclusive')
                        self.assertEqual(row['calibration_infeasible']['iteration_ceiling'], 256)
                        self.assertGreater(row['calibration_infeasible']['minimum_iterations'], 256)
                        self.assertLess(row['calibration_infeasible']['predicted_ceiling_batch_seconds'], row['min_sample_seconds'])
                    else:
                        self.assertEqual(row['iterations_per_batch'], 250 if row['case'] == 'warm-build-cache' else 63)
                if mutate:
                    self.assertEqual(report['status'], 'error')
                else:
                    self.assertEqual(report['status'], 'inconclusive' if expected == 2 else 'pass')
                    self.assertEqual(report['errors'], [])
                    self.assertEqual(len(report['results']), 3)


if __name__ == '__main__':
    unittest.main()
