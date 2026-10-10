"""Bounded paired statistics, order, isolation and failure checks without TeX."""

import contextlib
import io
import json
import math
import os
from pathlib import Path
import random
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


def calibration_pilots(baseline_rates, candidate_rates=None):
    rates = {'baseline': baseline_rates, 'candidate': candidate_rates or baseline_rates}
    return [{'pair_index': index, 'order': bench.paired_order(index), 'iterations': 32,
             **{label: {'seconds': rates[label][index] * 32, 'iterations': 32} for label in bench.LABELS}}
            for index in range(2)]


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
        pilots = calibration_pilots([0.01, 0.01], [0.008, 0.008])
        iterations = bench.calibrated_iterations(pilots, 0.25, 128)
        self.assertEqual(iterations, 63)
        # Even another 40% improvement after warmup retains meaningful batches.
        measured = samples([0.48] * 20, seconds=0.01 * iterations)
        analysis = bench.analyze_pairs(measured, 0.10, 0.25)
        self.assertEqual(analysis['status'], 'pass')
        self.assertEqual(bench.calibrated_iterations(pilots, 0.8, 128), 128)

    def test_batched_calibration_normalizes_parent_times_and_records_headroom(self):
        pilots = calibration_pilots([0.00657, 0.00657])
        details = bench.calibration_details(pilots, 1, 512)
        self.assertEqual(details['normalized_rates_seconds_per_iteration'],
                         {label: [0.00657, 0.00657] for label in bench.LABELS})
        self.assertEqual(details['minimum_iterations'], 153)
        self.assertEqual(details['requested_iterations'], 305)
        self.assertEqual(details['selected_iterations'], 305)
        self.assertFalse(details['headroom_clipped'])
        self.assertTrue(details['minimum_feasible'])
        self.assertAlmostEqual(details['predicted_selected_batch_seconds'], 305 * 0.00657)
        self.assertEqual(bench.calibrated_iterations(pilots, 1, 512), 305)
        # A further 40% speedup still leaves batches above the declared floor.
        self.assertEqual(bench.analyze_pairs(samples([1] * 20, seconds=305 * 0.00657 * 0.6), 0.10, 1)['status'], 'pass')

    def test_infeasible_calibration_is_rejected_before_sampling(self):
        pilots = calibration_pilots([0.003, 0.003])
        details = bench.calibration_details(pilots, 1, 256)
        self.assertFalse(details['minimum_feasible'])
        self.assertEqual(details['selected_iterations'], 256)
        with self.assertRaisesRegex(bench.CalibrationInfeasible, 'ceiling cannot reach') as failure:
            bench.calibrated_iterations(pilots, 1, 256)
        self.assertEqual(failure.exception.details, details)
        # The ceiling may limit extra headroom, but not the minimum itself.
        feasible = bench.calibration_details(calibration_pilots([0.004, 0.004]), 1, 256)
        self.assertTrue(feasible['minimum_feasible'])
        self.assertTrue(feasible['headroom_clipped'])
        self.assertEqual(feasible['selected_iterations'], 256)
        with self.assertRaises(bench.CalibrationInfeasible) as tiny:
            bench.calibrated_iterations(calibration_pilots([1e-308, 1e-308]), 1, 512)
        json.dumps(tiny.exception.details, allow_nan=False)

    def test_calibration_rejects_invalid_durations_counts_and_unbalanced_pilots(self):
        for value in (0, math.nan, True, '1'):
            pilots = calibration_pilots([0.01, 0.01])
            pilots[0]['baseline']['seconds'] = value
            with self.subTest(seconds=value), self.assertRaises(ValueError):
                bench.calibration_details(pilots, 1, 512)
        for value in (0, True, 32.0, 31):
            pilots = calibration_pilots([0.01, 0.01])
            pilots[0]['baseline']['iterations'] = value
            with self.subTest(iterations=value), self.assertRaises(ValueError):
                bench.calibration_details(pilots, 1, 512)
        pilots = calibration_pilots([0.01, 0.01])
        pilots[1]['order'] = pilots[0]['order']
        with self.assertRaises(ValueError):
            bench.calibration_details(pilots, 1, 512)
        del pilots[0]['candidate']
        with self.assertRaises(ValueError):
            bench.calibration_details(pilots, 1, 512)
        with self.assertRaises(ValueError):
            bench.calibration_details(calibration_pilots([0.01, 0.01]), 1, 513)

    def test_generated_calibration_matches_exact_integer_rate_model(self):
        # Power-of-two rates are exactly representable. This integer model
        # independently checks normalization, fastest-role choice and clipping.
        generated = random.Random(20261009)
        for case_index in range(128):
            numerators = [[generated.randint(1, 1024) for _ in range(2)] for _ in bench.LABELS]
            quarters = generated.choice((1, 2, 4, 8))
            ceiling = generated.choice((1, 8, 32, 64, 256, 512))
            rates = [[value / 4096 for value in role] for role in numerators]
            pilots = calibration_pilots(*rates)
            fastest_numerator = min(sum(role) for role in numerators)
            minimum = (quarters * 2048 + fastest_numerator - 1) // fastest_numerator
            requested = (quarters * 4096 + fastest_numerator - 1) // fastest_numerator
            with self.subTest(case=case_index, numerators=numerators, quarters=quarters, ceiling=ceiling):
                details = bench.calibration_details(pilots, quarters / 4, ceiling)
                self.assertEqual(details['minimum_iterations'], minimum)
                self.assertEqual(details['requested_iterations'], requested)
                self.assertEqual(details['selected_iterations'], min(requested, ceiling))
                self.assertEqual(details['minimum_feasible'], minimum <= ceiling)
                self.assertEqual(details['headroom_clipped'], requested > ceiling)
                swapped = bench.calibration_details(calibration_pilots(*reversed(rates)), quarters / 4, ceiling)
                self.assertEqual(swapped['selected_iterations'], details['selected_iterations'])
                self.assertEqual(swapped['fastest_pilot_median_seconds_per_iteration'],
                                 details['fastest_pilot_median_seconds_per_iteration'])

    def test_gate_and_advisory_exits_are_explicit(self):
        for status, exit_code in (('pass', 0), ('fail', 1), ('inconclusive', 2), ('error', 1)):
            self.assertEqual(bench.comparison_exit(status, True), exit_code)
            self.assertEqual(bench.comparison_exit(status, False), 1 if status == 'error' else 0)

    def test_diagnostic_telemetry_cannot_change_calibration_or_inference(self):
        for ratios in ([1] * 20, [1.2] * 20, [1.08, 1.08, 1.12, 1.12] * 5):
            measured = samples(ratios)
            pilots = calibration_pilots([0.0065, 0.0065])
            analysis = bench.analyze_pairs(measured, 0.10, 0.25)
            iterations = bench.calibrated_iterations(pilots, 1, 512)
            for index, sample in enumerate(measured):
                for label in bench.LABELS:
                    diagnostic = {'untrusted': True, 'status': ('absent', 'invalid', 'reported')[index % 3]}
                    if diagnostic['status'] == 'reported':
                        diagnostic.update(reported_elapsed_ms=1e300, parent_minus_reported_seconds=-1e297)
                    sample[label]['commands'] = [{'reported_build_timing': diagnostic}]
            for pilot in pilots:
                for label in bench.LABELS:
                    pilot[label]['commands'] = [{'reported_build_timing': diagnostic}] * 32
            self.assertEqual(bench.analyze_pairs(measured, 0.10, 0.25), analysis)
            self.assertEqual(bench.calibrated_iterations(pilots, 1, 512), iterations)


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
                with self.subTest(report=report), patch.object(bench.Supervisor, 'execute',
                        return_value={'stdout': json.dumps({**report, 'elapsed_ms': 5})}):
                    with self.assertRaises(bench.BenchmarkError):
                        bench.execute_fixture(bench.Supervisor(out, 1, time.monotonic() + 1), fixture, 'warm-build-cache')

    def test_optional_cache_timing_is_untrusted_finite_and_does_not_replace_parent_time(self):
        telemetry = [({}, 'absent'), ({'elapsed_ms': 0}, 'reported'), ({'elapsed_ms': 30.5}, 'reported')]
        telemetry.extend(({'elapsed_ms': value}, 'invalid') for value in
                         (None, True, '3', [], {}, -1, math.nan, math.inf, 10 ** 400))
        with tempfile.TemporaryDirectory(prefix='test-paired-telemetry-') as temporary:
            out = Path(temporary)
            (out / 'main.pdf').write_bytes(b'%PDF-1.4\n%%EOF\n')
            fixture = dict(out=out, project=out, command=[], env={})
            runner = bench.Supervisor(out, 1, time.monotonic() + 1)
            for initializing in (False, True):
                for optional, status in telemetry:
                    report = {'skipped': not initializing, 'tex_runs': 1 if initializing else 0, **optional}
                    response = {'stdout': json.dumps(report), 'command_id': 1, 'seconds': 0.02}
                    with self.subTest(initializing=initializing, optional=optional), \
                            patch.object(runner, 'execute', return_value=response), \
                            patch.object(bench, 'check_fixture_output', return_value={'verified': True}):
                        result = bench.execute_fixture(runner, fixture, 'warm-build-cache', initializing=initializing)
                    diagnostic = result['reported_build_timing']
                    self.assertEqual(result['seconds'], 0.02)
                    self.assertTrue(result['output_validation']['verified'])
                    self.assertEqual(diagnostic['status'], status)
                    self.assertTrue(diagnostic['untrusted'])
                    if status == 'reported':
                        self.assertEqual(diagnostic['reported_elapsed_ms'], optional['elapsed_ms'])
                        self.assertAlmostEqual(diagnostic['parent_minus_reported_seconds'], 0.02 - optional['elapsed_ms'] / 1000)
                    else:
                        self.assertNotIn('reported_elapsed_ms', diagnostic)
                        self.assertNotIn('parent_minus_reported_seconds', diagnostic)
                    # NaN, infinity and huge integers cannot leak into persisted
                    # diagnostics and turn an optional value into a run error.
                    json.dumps(diagnostic, allow_nan=False)

    def test_batch_retains_per_invocation_diagnostics_and_measured_seconds(self):
        diagnostics = [bench.reported_build_timing({'elapsed_ms': 30}, 0.01),
                       bench.reported_build_timing({'elapsed_ms': None}, 0.02)]
        responses = [{'command_id': index + 1, 'seconds': seconds,
                      'reported_build_timing': diagnostic, 'output_validation': {'verified': True}}
                     for index, (seconds, diagnostic) in enumerate(zip((0.01, 0.02), diagnostics))]
        with patch.object(bench, 'execute_fixture', side_effect=responses):
            result = bench.batch(None, {}, 'warm-build-cache', 2)
        self.assertEqual(result['seconds'], 0.03)
        self.assertEqual(result['iterations'], 2)
        self.assertEqual([command['reported_build_timing'] for command in result['commands']], diagnostics)
        self.assertEqual([command['seconds'] for command in result['commands']], [0.01, 0.02])
        self.assertLess(result['commands'][0]['reported_build_timing']['parent_minus_reported_seconds'], 0)

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

    def test_cli_iteration_ceiling_is_bounded_before_launch(self):
        self.assertEqual(bench.parser().parse_args(['--baseline', '/binary', '--candidate', '/binary',
                                                   '--metadata', '/metadata']).max_iterations, 512)
        for ceiling in (0, 513):
            with self.subTest(ceiling=ceiling), patch.object(bench.Supervisor, 'start') as start, \
                    contextlib.redirect_stderr(io.StringIO()):
                with self.assertRaises(SystemExit) as failure:
                    bench.main(['--baseline', '/binary', '--candidate', '/binary', '--metadata', '/metadata',
                                '--max-iterations', str(ceiling)])
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

    def test_supplied_artifact_hashes_require_exact_lowercase_digests(self):
        with tempfile.TemporaryDirectory(prefix='test-paired-metadata-hashes-') as temporary:
            path = Path(temporary) / 'metadata.json'
            metadata = {'runner_label': 'test', 'toolchain': 'exact',
                        **{label: {'revision': 'a' * 40, 'build_command': 'test'} for label in bench.LABELS}}
            for label in bench.LABELS:
                for value in ('0' * 64, '0123456789abcdef' * 4):
                    metadata[label]['artifact_sha256'] = value
                    path.write_text(json.dumps(metadata), encoding='utf-8')
                    self.assertEqual(bench.load_metadata(path), metadata)
                for value in (None, True, 0, [], {}, '', 'a' * 63, 'a' * 65, 'A' * 64, 'g' * 64,
                              'a' * 64 + '\n', ' ' + 'a' * 64):
                    metadata[label]['artifact_sha256'] = value
                    path.write_text(json.dumps(metadata), encoding='utf-8')
                    with self.subTest(label=label, value=value), self.assertRaisesRegex(ValueError, 'artifact_sha256'):
                        bench.load_metadata(path)
                del metadata[label]['artifact_sha256']

    def test_error_report_is_written_and_binary_hashes_are_rechecked(self):
        with tempfile.TemporaryDirectory(prefix='test-paired-failure-') as temporary:
            work = Path(temporary)
            source = work / 'binary'
            source.write_bytes(b'executable fixture')
            source.chmod(0o700)
            metadata = work / 'metadata.json'
            output = work / 'report.json'
            argv = ['--baseline', str(source), '--candidate', str(source), '--metadata', str(metadata),
                    '--output', str(output), '--gate']
            for supplied in (False, True):
                provenance = {'runner_label': 'test', 'toolchain': 'exact',
                              **{label: {'revision': 'a' * 40, 'build_command': 'test'} for label in bench.LABELS}}
                if supplied:
                    for label in bench.LABELS:
                        provenance[label]['artifact_sha256'] = bench.executable_sha256(source)
                metadata.write_text(json.dumps(provenance), encoding='utf-8')
                with self.subTest(supplied=supplied), patch('benchmark_runtime.shutil.which', return_value=sys.executable), \
                        patch.object(bench.Supervisor, 'start') as start, \
                        patch.object(bench, 'make_fixture', side_effect=bench.BenchmarkError('controlled fixture failure')) as fixture, \
                        contextlib.redirect_stdout(io.StringIO()):
                    result = bench.main(argv)
                fixture.assert_called_once()
                start.assert_not_called()
                report = json.loads(output.read_text())
                self.assertEqual((result, report['status'], report['exit_code']), (1, 'error', 1))
                self.assertIn('controlled fixture failure', report['errors'][0])
                self.assertEqual(report['metadata'], provenance)
                self.assertEqual(len(report['executables']), 2)
                self.assertTrue(all(row['unchanged'] for row in report['executables']))
                self.assertTrue(all(row['sha256_start'] == bench.executable_sha256(source) for row in report['executables']))
                self.assertTrue(output.with_suffix('.md').is_file())

    def test_artifact_hash_mismatch_stops_before_fixtures_and_native_commands(self):
        with tempfile.TemporaryDirectory(prefix='test-paired-build-provenance-') as temporary:
            work = Path(temporary)
            source = work / 'binary'
            source.write_bytes(b'executable fixture')
            source.chmod(0o700)
            metadata = work / 'metadata.json'
            output = work / 'report.json'
            actual = bench.executable_sha256(source)
            for mismatch in bench.LABELS:
                provenance = {'runner_label': 'test', 'toolchain': 'exact',
                              **{label: {'revision': 'a' * 40, 'build_command': 'test', 'artifact_sha256': actual}
                                 for label in bench.LABELS}}
                provenance[mismatch]['artifact_sha256'] = '0' * 64
                metadata.write_text(json.dumps(provenance), encoding='utf-8')
                with self.subTest(mismatch=mismatch), patch('benchmark_runtime.shutil.which', return_value=sys.executable), \
                        patch.object(bench.Supervisor, 'start') as start, patch.object(bench, 'make_fixture') as fixture, \
                        contextlib.redirect_stdout(io.StringIO()):
                    result = bench.main(['--baseline', str(source), '--candidate', str(source),
                                         '--metadata', str(metadata), '--output', str(output), '--gate'])
                start.assert_not_called()
                fixture.assert_not_called()
                report = json.loads(output.read_text())
                self.assertEqual((result, report['status'], report['exit_code']), (1, 'error', 1))
                self.assertIn('Selected ' + mismatch + ' binary differs from its recorded build artifact SHA-256', report['errors'][0])
                self.assertEqual(report['metadata'], provenance)
                self.assertEqual(report['results'], [])
                self.assertTrue(all(row['unchanged'] for row in report['executables']))

    def test_driver_initializes_before_warmups_and_checks_fixture_mutation(self):
        with tempfile.TemporaryDirectory(prefix='test-paired-driver-') as temporary:
            work = Path(temporary)
            source = work / 'binary'
            source.write_bytes(b'executable fixture')
            source.chmod(0o700)
            metadata = work / 'metadata.json'
            metadata.write_text(json.dumps({'runner_label': 'test', 'toolchain': 'exact',
                               **{label: {'revision': 'a' * 40, 'build_command': 'test',
                                          'artifact_sha256': bench.executable_sha256(source)} for label in bench.LABELS}}), encoding='utf-8')
            output = work / 'report.json'
            argv = ['--baseline', str(source), '--candidate', str(source), '--metadata', str(metadata),
                    '--output', str(output), '--pairs', '16', '--min-sample-seconds', '0.25', '--gate']
            events = []
            warm_seconds = 0.01
            pilot_seconds = 0.01

            def fixture(root, binary, case):
                project = root / 'project'
                bench.document(project)
                return dict(root=root, project=project, label=root.parent.name, input_sha256=bench.fixture_sha256(project))

            def initialize(_runner, fixture, case, initializing=False):
                self.assertTrue(initializing)
                events.append((case, 'initialize', fixture['label']))
                result = {'command_id': 0, 'seconds': 100, 'output_validation': {'verified': True}}
                if case == 'warm-build-cache':
                    result['reported_build_timing'] = bench.reported_build_timing({'elapsed_ms': 125}, 100)
                return result

            def batch(_runner, fixture, case, iterations):
                events.append((case, 'batch', fixture['label'], iterations))
                rate = warm_seconds if iterations == 1 else pilot_seconds
                elapsed = rate if fixture['label'] == 'baseline' else rate * 0.8
                return dict(seconds=elapsed * iterations, iterations=iterations, commands=[], output_validation={'verified': True})

            for mutate, warm_seconds, pilot_seconds, expected in ((False, 0.01, 0.01, 0), (True, 0.01, 0.01, 1),
                                                                  (False, 0.014, 0.0065, 0), (False, 0.001, 0.0001, 2)):
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
                self.assertEqual(report['schema_version'], 3)
                self.assertEqual(report['policy']['calibration_pairs'], 2)
                self.assertEqual(report['policy']['calibration_iterations'], 32)
                self.assertEqual(report['policy']['calibration_method'], 'balanced-batched-pilot-v1')
                self.assertEqual(report['policy']['max_iterations'], 512)
                self.assertFalse(report['diagnostic_telemetry']['reported_build_timing']['used_for_gate'])
                for row in report['results']:
                    case_events = [event for event in events if event[0] == row['case']]
                    self.assertEqual([event[1] for event in case_events[:2]], ['initialize', 'initialize'])
                    self.assertEqual([event[3] for event in case_events[2:6]], [1] * 4)
                    self.assertEqual([event[3] for event in case_events[6:10]], [32] * 4)
                    self.assertEqual(row['min_sample_seconds'], 1 if row['case'] == 'warm-build-cache' else 0.25)
                    self.assertEqual(row['fixture_unchanged'], not mutate)
                    self.assertEqual(len(row['warmups']), 2)
                    self.assertEqual(len(row['calibration_pilots']), 2)
                    case_index = bench.CASES.index(row['case'])
                    for index, pilot in enumerate(row['calibration_pilots']):
                        self.assertEqual(pilot['pair_index'], index)
                        self.assertEqual(pilot['order'], list(bench.paired_order(index, case_index)))
                        self.assertEqual(pilot['iterations'], 32)
                        self.assertEqual([pilot[label]['iterations'] for label in bench.LABELS], [32, 32])
                    self.assertEqual(row['calibration'], bench.calibration_details(row['calibration_pilots'], row['min_sample_seconds'], 512))
                    for initialization in row['initialization'].values():
                        if row['case'] == 'warm-build-cache':
                            self.assertEqual(initialization['reported_build_timing'],
                                             bench.reported_build_timing({'elapsed_ms': 125}, 100))
                        else:
                            self.assertNotIn('reported_build_timing', initialization)
                    if expected == 2:
                        self.assertEqual(len(case_events), 10)
                        self.assertEqual(row['samples'], [])
                        self.assertEqual(row['analysis']['status'], 'inconclusive')
                        self.assertEqual(row['calibration_infeasible'], row['calibration'])
                        self.assertEqual(row['calibration_infeasible']['iteration_ceiling'], 512)
                        self.assertGreater(row['calibration_infeasible']['minimum_iterations'], 512)
                        self.assertLess(row['calibration_infeasible']['predicted_ceiling_batch_seconds'], row['min_sample_seconds'])
                    else:
                        expected_counts = (97, 385) if pilot_seconds == 0.0065 else (63, 250)
                        self.assertEqual(row['iterations_per_batch'], expected_counts[row['case'] == 'warm-build-cache'])
                        self.assertEqual(len(row['samples']), 16)
                if mutate:
                    self.assertEqual(report['status'], 'error')
                else:
                    self.assertEqual(report['status'], 'inconclusive' if expected == 2 else 'pass')
                    self.assertEqual(report['errors'], [])
                    self.assertEqual(len(report['results']), 3)

    def test_later_side_failures_preserve_partial_warmups_and_pilots(self):
        with tempfile.TemporaryDirectory(prefix='test-paired-partial-calibration-') as temporary:
            work = Path(temporary)
            source = work / 'binary'
            source.write_bytes(b'executable fixture')
            source.chmod(0o700)
            metadata = work / 'metadata.json'
            metadata.write_text(json.dumps({'runner_label': 'test', 'toolchain': 'exact',
                               **{label: {'revision': 'a' * 40, 'build_command': 'test'} for label in bench.LABELS}}), encoding='utf-8')
            output = work / 'report.json'

            def fixture(root, _binary, _case):
                project = root / 'project'
                bench.document(project)
                return dict(root=root, project=project, label=root.parent.name, input_sha256=bench.fixture_sha256(project))

            for phase, failure_iterations in (('warmups', 1), ('calibration_pilots', 32)):
                def measured(_runner, fixture, _case, iterations):
                    if iterations == failure_iterations and fixture['label'] == 'candidate':
                        raise bench.BenchmarkError('controlled later-side ' + phase + ' failure')
                    return {'seconds': iterations * 0.01, 'iterations': iterations, 'commands': [],
                            'output_validation': {'verified': True}}

                with self.subTest(phase=phase), patch('benchmark_runtime.shutil.which', return_value=sys.executable), \
                        patch.object(bench, 'make_fixture', side_effect=fixture), \
                        patch.object(bench, 'execute_fixture', return_value={'command_id': 0, 'seconds': 1,
                                     'output_validation': {'verified': True}}), \
                        patch.object(bench, 'batch', side_effect=measured), contextlib.redirect_stdout(io.StringIO()):
                    self.assertEqual(bench.main(['--baseline', str(source), '--candidate', str(source),
                                                '--metadata', str(metadata), '--output', str(output), '--gate']), 1)
                report = json.loads(output.read_text())
                row = report['results'][0]
                self.assertEqual(report['status'], 'error')
                self.assertIn('controlled later-side ' + phase + ' failure', report['errors'][0])
                self.assertEqual(len(row[phase]), 1)
                self.assertIn('baseline', row[phase][0])
                self.assertNotIn('candidate', row[phase][0])
                self.assertEqual(row[phase][0]['baseline']['seconds'], failure_iterations * 0.01)
                self.assertEqual(row['samples'], [])
                self.assertTrue(all(binary['unchanged'] for binary in report['executables']))


if __name__ == '__main__':
    unittest.main()
