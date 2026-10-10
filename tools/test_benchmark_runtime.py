"""Bounded paired statistics, order, isolation and failure checks without TeX."""

import contextlib
import io
import itertools
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
import weakref
from unittest.mock import Mock, patch

import benchmark_runtime as bench


def model_schedule(index, case_index=0):
    # Literal complementary quads are independent of the production schedule.
    quads = [('s0', ('baseline', 'candidate')), ('s1', ('candidate', 'baseline')),
             ('s0', ('candidate', 'baseline')), ('s1', ('baseline', 'candidate'))]
    unit, within = divmod(index, 4)
    if (unit + case_index) % 2:
        quads = quads[2:] + quads[:2]
    slot, order = quads[within]
    assignments = {'s0': {'baseline': 'r0', 'candidate': 'r1'},
                   's1': {'baseline': 'r1', 'candidate': 'r0'}}
    return {'pair_index': index, 'unit_index': unit, 'quad_index': within // 2,
            'slot': slot, 'order': order, 'replicas': assignments[slot]}


def samples(ratios, seconds=1, case_index=0):
    return [{**model_schedule(index, case_index), 'iterations': 1,
             'baseline': {'seconds': seconds, 'iterations': 1},
             'candidate': {'seconds': seconds * ratio, 'iterations': 1}}
            for index, ratio in enumerate(ratios)]


def calibration_pilots(baseline_rates, candidate_rates=None, slot_rates=None):
    rates = slot_rates or {slot: {'baseline': baseline_rates, 'candidate': candidate_rates or baseline_rates}
                          for slot in bench.SLOTS}
    return [{'pair_index': index, 'slot': slot,
             'order': ('baseline', 'candidate') if (index + slot_index) % 2 == 0 else ('candidate', 'baseline'),
             'replicas': model_schedule(slot_index)['replicas'], 'iterations': 32,
             **{label: {'seconds': rates[slot][label][index] * 32, 'iterations': 32} for label in bench.LABELS}}
            for index in range(2) for slot_index, slot in enumerate(bench.SLOTS)]


def mocked_fixture(root, _binary, _case):
    project, out = root / 'project', root / 'out'
    bench.document(project)
    config = project / 'tekai.toml'
    config.write_bytes(bench.EMPTY_CONFIG_BYTES)
    out.mkdir()
    env = bench.isolated_environment(root)
    env['TEXINPUTS'] = f'{project}//:{out}//:'
    return dict(root=root, project=project, out=out, config=config, dependencies=[], env=env,
                input_sha256=bench.fixture_sha256(project))


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
        ten = bench.median_interval(values[:10], 0.05 / 3)
        self.assertEqual(ten['order_statistic'], 1)
        self.assertEqual((ten['lower'], ten['upper']), (values[0], values[9]))

    def test_equal_binary_and_real_regression_are_distinguished(self):
        for ratio, outcome in ((1, 'pass'), (0.7, 'pass'), (1.09, 'pass'), (1.11, 'fail')):
            with self.subTest(ratio=ratio):
                analysis = self.analyze([ratio] * 40)
                self.assertEqual(analysis['status'], outcome)
                self.assertEqual(analysis['inference_unit_count'], 10)
                self.assertEqual(len(analysis['inference_unit_ratios']), 10)
                self.assertEqual(analysis['median_ratio_interval']['order_statistic'], 1)

    def test_future_fixed_plan_has_32_units_and_exact_rank_9_without_sample_selection(self):
        for ratio, outcome in ((1, 'pass'), (1.11, 'fail')):
            rows = samples([ratio] * 128, seconds=2)
            result = bench.analyze_pairs(rows, 0.10, 1)
            self.assertEqual(result['status'], outcome)
            self.assertEqual(result['inference_unit_count'], 32)
            self.assertEqual(result['median_ratio_interval']['order_statistic'], 9)
            self.assertEqual(len(result['paired_ratios']), 128)
        # Fixed ranks tolerate eight hypothetical upper-tail units, retaining
        # every observation. Nine upper-tail units make the interval cross.
        for count, status in ((8, 'pass'), (9, 'inconclusive')):
            result = bench.analyze_pairs(samples([1.2] * (4 * count) + [1] * (128 - 4 * count), seconds=2), 0.10, 1)
            self.assertEqual(result['status'], status)
            self.assertEqual(len(result['inference_unit_ratios']), 32)

    def test_crossing_threshold_is_inconclusive(self):
        analysis = self.analyze(([1.08] * 4 + [1.12] * 4) * 5)
        self.assertEqual(analysis['status'], 'inconclusive')
        self.assertIn('crosses', analysis['reasons'][0])

    def test_high_variability_and_order_bias_cannot_fail_a_gate(self):
        noisy = samples([1.3] * 40)
        for index, sample in enumerate(noisy):
            factor = 0.5 if index % 2 else 1.5
            for label in bench.LABELS:
                sample[label]['seconds'] *= factor
        result = bench.analyze_pairs(noisy, 0.10, 0.25)
        self.assertEqual(result['status'], 'inconclusive')
        self.assertTrue(any('noise limit' in reason for reason in result['reasons']))
        order_biased = samples([1] * 40)
        for row in order_biased:
            row['candidate']['seconds'] = 1.5 if row['order'][0] == 'baseline' else 1.2
        result = bench.analyze_pairs(order_biased, 0.10, 0.25)
        self.assertEqual(result['status'], 'inconclusive')
        self.assertTrue(any('order-bias' in reason for reason in result['reasons']))

    def test_small_sample_and_short_batch_are_inconclusive(self):
        self.assertEqual(self.analyze([1.2] * 12)['status'], 'inconclusive')
        self.assertEqual(self.analyze([1.2] * 40, seconds=0.01)['status'], 'inconclusive')
        self.assertIsNone(bench.median_interval([1] * 6, 0.05 / 3))

    def test_balancing_is_per_case_and_rejects_unbalanced_data(self):
        for case_index in range(3):
            for index in range(40):
                self.assertEqual(bench.crossover_pair(index, case_index), model_schedule(index, case_index))
            rows = samples([1] * 40, case_index=case_index)
            for start in range(0, 40, 4):
                for slot in ('s0', 's1'):
                    self.assertEqual([row['order'] for row in rows[start:start + 4] if row['slot'] == slot].count(('baseline', 'candidate')), 1)
                    self.assertEqual([row['order'] for row in rows[start:start + 4] if row['slot'] == slot].count(('candidate', 'baseline')), 1)
                for label in ('baseline', 'candidate'):
                    for replica in ('r0', 'r1'):
                        self.assertEqual(sum(row['replicas'][label] == replica for row in rows[start:start + 4]), 2)
            self.assertEqual(bench.analyze_pairs(rows, 0.10, 0.25, case_index=case_index)['status'], 'pass')
        bad = samples([1] * 40)
        bad[1]['order'] = bad[0]['order']
        with self.assertRaises(ValueError):
            bench.analyze_pairs(bad, 0.10, 0.25)

    def test_invalid_timings_are_not_statistics(self):
        for value in (0, -1, math.inf, math.nan):
            with self.subTest(value=value), self.assertRaises(ValueError):
                bench.median_interval([value], 0.05)
            bad = samples([1] * 40)
            bad[0]['candidate']['seconds'] = value
            with self.subTest(value=value), self.assertRaises(ValueError):
                bench.analyze_pairs(bad, 0.10, 0.25)

    def test_calibration_has_headroom_for_an_improving_candidate(self):
        pilots = calibration_pilots([0.01, 0.01], [0.008, 0.008])
        iterations = bench.calibrated_iterations(pilots, 0.25, 128)
        self.assertEqual(iterations, 63)
        # Even another 40% improvement after warmup retains meaningful batches.
        measured = samples([0.48] * 40, seconds=0.01 * iterations)
        analysis = bench.analyze_pairs(measured, 0.10, 0.25)
        self.assertEqual(analysis['status'], 'pass')
        self.assertEqual(bench.calibrated_iterations(pilots, 0.8, 128), 128)

    def test_batched_calibration_normalizes_parent_times_and_records_headroom(self):
        pilots = calibration_pilots([0.00657, 0.00657])
        details = bench.calibration_details(pilots, 1, 512)
        self.assertEqual(details['normalized_rates_seconds_per_iteration'],
                         {slot: {label: [0.00657, 0.00657] for label in bench.LABELS} for slot in bench.SLOTS})
        self.assertEqual(details['minimum_iterations'], 153)
        self.assertEqual(details['requested_iterations'], 305)
        self.assertEqual(details['selected_iterations'], 305)
        self.assertFalse(details['headroom_clipped'])
        self.assertTrue(details['minimum_feasible'])
        self.assertAlmostEqual(details['predicted_selected_batch_seconds'], 305 * 0.00657)
        self.assertEqual(bench.calibrated_iterations(pilots, 1, 512), 305)
        # A further 40% speedup still leaves batches above the declared floor.
        self.assertEqual(bench.analyze_pairs(samples([1] * 40, seconds=305 * 0.00657 * 0.6), 0.10, 1)['status'], 'pass')

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
            numerators = {slot: {label: [generated.randint(1, 1024) for _ in range(2)] for label in bench.LABELS}
                          for slot in bench.SLOTS}
            quarters = generated.choice((1, 2, 4, 8))
            ceiling = generated.choice((1, 8, 32, 64, 256, 512))
            rates = {slot: {label: [value / 4096 for value in numerators[slot][label]] for label in bench.LABELS}
                     for slot in bench.SLOTS}
            pilots = calibration_pilots(None, slot_rates=rates)
            fastest_numerator = min(sum(role) for cells in numerators.values() for role in cells.values())
            minimum = (quarters * 2048 + fastest_numerator - 1) // fastest_numerator
            requested = (quarters * 4096 + fastest_numerator - 1) // fastest_numerator
            with self.subTest(case=case_index, numerators=numerators, quarters=quarters, ceiling=ceiling):
                details = bench.calibration_details(pilots, quarters / 4, ceiling)
                self.assertEqual(details['minimum_iterations'], minimum)
                self.assertEqual(details['requested_iterations'], requested)
                self.assertEqual(details['selected_iterations'], min(requested, ceiling))
                self.assertEqual(details['minimum_feasible'], minimum <= ceiling)
                self.assertEqual(details['headroom_clipped'], requested > ceiling)
                swapped_rates = {slot: dict(zip(bench.LABELS, reversed(list(rates[slot].values())))) for slot in bench.SLOTS}
                swapped = bench.calibration_details(calibration_pilots(None, slot_rates=swapped_rates), quarters / 4, ceiling)
                self.assertEqual(swapped['selected_iterations'], details['selected_iterations'])
                self.assertEqual(swapped['fastest_pilot_median_seconds_per_iteration'],
                                 details['fastest_pilot_median_seconds_per_iteration'])

    def test_gate_and_advisory_exits_are_explicit(self):
        for status, exit_code in (('pass', 0), ('fail', 1), ('inconclusive', 2), ('error', 1)):
            self.assertEqual(bench.comparison_exit(status, True), exit_code)
            self.assertEqual(bench.comparison_exit(status, False), 1 if status == 'error' else 0)

    def test_generated_crossover_cancels_log_additive_slot_replica_and_repeatable_position_factors(self):
        generated = random.Random(20261011)
        for example in range(64):
            artifact = generated.choice((0.7, 1.0, 1.05, 1.2))
            case_index = generated.randrange(3)
            slot_effects = {slot: 1 + generated.randrange(10) / 1000 for slot in ('s0', 's1')}
            replica_effects = {replica: 1 + generated.randrange(10) / 1000 for replica in ('r0', 'r1')}
            # The profile is arbitrary within each slot and repeats across the
            # complementary quads. It is not a separable global position effect.
            position_effects = {slot: [1 + generated.randrange(10) / 1000 for _ in range(4)] for slot in ('s0', 's1')}
            noisy = example % 8 == 0
            if noisy:
                position_effects['s0'][:2] = (0.8, 1.2)
                position_effects['s1'][2:] = (0.8, 1.2)
            rows = samples([1] * 128, seconds=2, case_index=case_index)
            for index, row in enumerate(rows):
                for position, label in enumerate(row['order']):
                    quad_position = (index % 2) * 2 + position
                    row[label]['seconds'] = 2 * slot_effects[row['slot']] * replica_effects[row['replicas'][label]] \
                        * position_effects[row['slot']][quad_position] * (artifact if label == 'candidate' else 1)
            with self.subTest(example=example, artifact=artifact, case=case_index):
                result = bench.analyze_pairs(rows, 0.10, 1, case_index=case_index)
                self.assertEqual(result['inference_unit_count'], 32)
                self.assertTrue(all(math.isclose(value, artifact, rel_tol=1e-12) for value in result['inference_unit_ratios']))
                self.assertEqual(result['status'], 'inconclusive' if noisy else ('fail' if artifact > 1.10 else 'pass'))
                if noisy:
                    self.assertTrue(any('noise limit' in reason for reason in result['reasons']))

    def test_incomplete_malformed_cells_and_indices_are_not_inference_units(self):
        for field, value in (('slot', 's1'), ('replicas', {'baseline': 'r1', 'candidate': 'r0'}),
                             ('order', ('baseline',)), ('pair_index', True), ('unit_index', False),
                             ('quad_index', False), ('iterations', True)):
            rows = samples([1] * 40)
            rows[0][field] = value
            with self.subTest(field=field, value=value), self.assertRaises(ValueError):
                bench.analyze_pairs(rows, 0.10, 1)
        for field in ('candidate', 'slot', 'replicas', 'order'):
            rows = samples([1] * 40)
            del rows[0][field]
            with self.subTest(missing=field), self.assertRaises(ValueError):
                bench.analyze_pairs(rows, 0.10, 1)
        with self.assertRaises(ValueError):
            bench.analyze_pairs(samples([1] * 39), 0.10, 1)
        for index in (True, -1):
            with self.subTest(index=index), self.assertRaises(ValueError):
                bench.crossover_pair(index)
        for change in (('slot', 's1'), ('replicas', {}), ('pair_index', False), ('iterations', True)):
            pilots = calibration_pilots([0.01, 0.01])
            pilots[0][change[0]] = change[1]
            with self.subTest(pilot=change), self.assertRaises(ValueError):
                bench.calibration_details(pilots, 1, 512)
        with self.assertRaises(ValueError):
            bench.calibration_details(calibration_pilots([0.01, 0.01])[:-1], 1, 512)

    def test_extreme_finite_times_cannot_emit_nonfinite_or_zero_ratio_statistics(self):
        for baseline, candidate in ((1e-308, 1e308), (1e308, 1e-308), (1e308, 1e308), (10 ** 400, 1), (1, 10 ** 400)):
            rows = samples([1] * 40)
            for row in rows:
                row['baseline']['seconds'], row['candidate']['seconds'] = baseline, candidate
            with self.subTest(baseline=baseline, candidate=candidate), self.assertRaises(ValueError):
                bench.analyze_pairs(rows, 0.10, 1)
        # Model an exp-log boundary failure after valid raw paired ratios.
        with patch.object(bench.math, 'exp', side_effect=OverflowError('controlled exponent boundary')):
            with self.assertRaisesRegex(ValueError, 'Crossover ratio overflowed'):
                bench.analyze_pairs(samples([1] * 40), 0.10, 1)
        with patch.object(bench.math, 'exp', return_value=0):
            with self.assertRaisesRegex(ValueError, 'Crossover ratio overflowed or underflowed'):
                bench.analyze_pairs(samples([1] * 40), 0.10, 1)

    def test_diagnostic_telemetry_cannot_change_calibration_or_inference(self):
        for ratios in ([1] * 40, [1.2] * 40, ([1.08] * 4 + [1.12] * 4) * 5):
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
                    sample[label]['parent_timing'] = [math.nan, -1, math.inf]
            for pilot in pilots:
                for label in bench.LABELS:
                    pilot[label]['commands'] = [{'reported_build_timing': diagnostic}] * 32
            self.assertEqual(bench.analyze_pairs(measured, 0.10, 0.25), analysis)
            self.assertEqual(bench.calibrated_iterations(pilots, 1, 512), iterations)

    def test_parent_clock_model_preserves_existing_seconds_without_native_processes(self):
        with tempfile.TemporaryDirectory(prefix='test-parent-clock-') as temporary:
            work = Path(temporary)
            captures = [work / 'stdout', work / 'stderr']
            for path in captures:
                path.touch()
            process = Mock(command_id=1, capture_paths=captures, launch_timing=[10] * 5)
            process.wait.return_value = 0
            runner = bench.Supervisor(work, 1, 100)
            with patch.object(runner, 'remaining', return_value=1), patch.object(runner, 'start', return_value=process), \
                    patch.object(runner, 'stop'), patch.object(runner, 'captured', return_value=('ok', '', False)), \
                    patch.object(bench.time, 'monotonic', side_effect=(10 + value / 10000 for value in itertools.count())):
                result = runner.execute([], work, {})
            start, launch, end = result['parent_timing']
            self.assertEqual(start, 10)
            self.assertGreaterEqual(end, launch)
            self.assertEqual(result['seconds'], end - start)
            self.assertFalse(any(path.exists() for path in captures))
            runner.close()


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
        timing = result['command_timing']
        self.assertEqual(len(timing), 12)
        self.assertEqual(timing, sorted(timing))
        self.assertEqual(result['parent_timing'], [timing[index] for index in (0, 6, 9)])

    def test_one_persistent_waiter_is_reused_and_shutdown_is_idempotent(self):
        workers = []
        for _ in range(12):
            self.runner.execute([sys.executable, '-c', 'pass'], self.work, os.environ.copy())
            workers.append(self.runner._waiter)
        self.assertEqual(len({id(worker) for worker in workers}), 1)
        self.assertTrue(workers[0].is_alive())
        self.runner.close()
        self.runner.close()
        self.assertFalse(workers[0].is_alive())
        with self.assertRaisesRegex(bench.BenchmarkError, 'closed'):
            self.runner.execute([], self.work, {})

    def test_idle_waiter_does_not_retain_the_previous_process(self):
        references = []
        original_start = self.runner.start

        def start(*args):
            process = original_start(*args)
            references.append(weakref.ref(process))
            return process

        with patch.object(self.runner, 'start', side_effect=start):
            self.runner.execute([sys.executable, '-c', 'pass'], self.work, os.environ.copy())
        bench.gc.collect()
        self.assertIsNone(references[0]())

    def test_waiter_failure_is_reported_and_can_accept_the_next_command(self):
        captures = [self.work / 'mock.stdout', self.work / 'mock.stderr']
        for path in captures:
            path.touch()
        process = Mock(command_id=1, capture_paths=captures)
        process.launch_timing = [time.monotonic()] * 5
        process.wait.side_effect = OSError('controlled wait failure')
        with patch.object(self.runner, 'start', return_value=process), patch.object(self.runner, 'stop'):
            with self.assertRaisesRegex(bench.BenchmarkError, 'Process waiter failed'):
                self.runner.execute([], self.work, {})
        self.runner.execute([sys.executable, '-c', 'pass'], self.work, os.environ.copy())
        self.assertTrue(self.runner._waiter.is_alive())

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
    def test_startup_oracle_requires_the_selected_package_version_and_no_pdf(self):
        fixture = {'command': [], 'project': Path('/project'), 'env': {}, 'expected_version': '0.5.0'}
        for stdout, stderr, valid in (('tekai 0.5.0\n', '', True), ('tekai 0.4.0\n', '', False),
                                      ('tekai 0.5.0\nextra\n', '', False), ('tekai 0.5.0\n', 'warning', False)):
            runner = Mock()
            runner.execute.return_value = {'stdout': stdout, 'stderr_tail': stderr}
            with self.subTest(stdout=stdout, stderr=stderr), patch.object(bench, 'check_pdf') as pdf:
                if valid:
                    result = bench.execute_fixture(runner, fixture, 'cli-startup')
                    self.assertEqual(result['output_validation'], {'expected_stdout': 'tekai 0.5.0\n',
                                      'stdout_verified': True, 'stderr_empty': True, 'expected_version': '0.5.0'})
                else:
                    with self.assertRaises(bench.BenchmarkError):
                        bench.execute_fixture(runner, fixture, 'cli-startup')
                pdf.assert_not_called()
        fixture['expected_version'] = '0.6.0-rc.1+test'
        runner = Mock()
        runner.execute.return_value = {'stdout': 'tekai 0.6.0-rc.1+test\n', 'stderr_tail': ''}
        self.assertTrue(bench.execute_fixture(runner, fixture, 'cli-startup')['output_validation']['stdout_verified'])

    def test_config_and_real_dependency_workload_are_hashed_and_explicit(self):
        with tempfile.TemporaryDirectory(prefix='test-cache-dependencies-') as temporary:
            root = Path(temporary)
            (root / 'tekai.toml').write_text('[build]\nforce = true\n', encoding='utf-8')
            fixture = bench.make_fixture(root / 'owned', Path('/tekai'), 'warm-build-cache')
            self.assertEqual(len(fixture['dependencies']), bench.CACHE_DEPENDENCY_COUNT)
            source = (fixture['project'] / 'main.tex').read_text()
            for path in fixture['dependencies']:
                self.assertIn('\\input{' + path.relative_to(fixture['project']).as_posix() + '}', source)
            self.assertFalse((fixture['project'] / 'unused-tree').exists())
            self.assertEqual(fixture['config'].read_bytes(), bench.EMPTY_CONFIG_BYTES)
            self.assertEqual(bench.executable_sha256(fixture['config']), bench.EMPTY_CONFIG_SHA256)
            position = fixture['command'].index('--config')
            self.assertEqual(fixture['command'][position + 1], fixture['config'])
            before = fixture['input_sha256']
            fixture['config'].write_text('[build]\nforce = true\n', encoding='utf-8')
            self.assertNotEqual(before, bench.fixture_sha256(fixture['project']))
            self.assertEqual({key: fixture['env'][key] for key in ('LC_ALL', 'LANG', 'TZ')},
                             {'LC_ALL': 'C', 'LANG': 'C', 'TZ': 'UTC'})

    def test_state_oracle_generated_dependency_counts_and_mutation_properties(self):
        rng = random.Random(8706)
        with tempfile.TemporaryDirectory(prefix='test-cache-state-') as temporary:
            work = Path(temporary)
            for trial in range(16):
                root = work / str(trial)
                project, out = root / 'project', root / 'out'
                dependencies = bench.dependency_document(project, rng.randrange(1, 32))
                out.mkdir()
                fixture = {'project': project, 'out': out, 'dependencies': dependencies}
                inputs = [project / 'main.tex', *dependencies]
                rng.shuffle(inputs)
                state = out / '.tekai-main.state.toml'
                source = 'version = 1\n' + '\n'.join('[[inputs]]\npath = ' + json.dumps(str(path.resolve())) + '\nlen = 0\n'
                                                        for path in inputs)
                state.write_text(source, encoding='utf-8')
                initial = bench.cache_state_snapshot(fixture, establish=True)
                self.assertEqual(initial['referenced_dependency_count'], len(dependencies))
                self.assertEqual(initial['recorded_input_count'], len(inputs))
                self.assertEqual(bench.cache_state_snapshot(fixture), initial)
                # Same bytes rewritten are still a mutation because identity
                # timestamps detect state writes on the claimed no-write route.
                state.write_text(source, encoding='utf-8')
                with self.assertRaisesRegex(bench.BenchmarkError, 'mutated'):
                    bench.cache_state_snapshot(fixture)
                state.write_text('version = 1\n[[inputs]]\npath = ' + json.dumps(str((project / 'main.tex').resolve())) + '\n', encoding='utf-8')
                with self.assertRaisesRegex(bench.BenchmarkError, 'omits'):
                    bench.cache_state_snapshot(fixture, establish=True)
                with patch.object(bench, 'MAX_STATE_BYTES', 4), self.assertRaisesRegex(bench.BenchmarkError, 'finite size'):
                    bench.cache_state_snapshot(fixture, establish=True)

    def test_parent_resource_snapshot_is_diagnostic_and_reports_cumulative_scope(self):
        snapshot = bench.parent_snapshot()
        self.assertFalse(snapshot['used_for_gate'])
        self.assertIn('cumulative', snapshot['scope'])
        self.assertEqual(len(snapshot['gc_counts']), 3)
        self.assertEqual(len(snapshot['gc_generations']), 3)
        self.assertGreaterEqual(snapshot['active_thread_count'], 1)
        self.assertEqual(snapshot['peak_rss_raw_unit'], 'bytes' if sys.platform == 'darwin' else 'KiB')
        json.dumps(snapshot, allow_nan=False)

    def test_native_darwin_parent_memory_has_explicit_abi_and_unavailable_path(self):
        def observe(_pid, flavor, pointer):
            self.assertEqual(flavor, 0)
            pointer._obj.ri_resident_size = 123456
            pointer._obj.ri_phys_footprint = 654321
            pointer._obj.ri_pageins = 9
            return 0

        self.assertEqual(bench.ctypes.sizeof(bench.DarwinUsageV0), 96)
        with patch.object(bench, '_darwin_rusage', side_effect=observe):
            observed = bench.darwin_parent_memory()
        self.assertEqual(observed['current_rss_bytes'], 123456)
        self.assertEqual(observed['physical_footprint_bytes'], 654321)
        self.assertEqual(observed['parent_pageins'], 9)
        self.assertIn('proc_pid_rusage', observed['current_rss_source'])
        with patch.object(bench.platform, 'system', return_value='Darwin'), \
                patch.object(bench, 'darwin_parent_memory', side_effect=OSError('unsupported ABI')):
            observed = bench.parent_snapshot()
        self.assertEqual(observed['current_rss_status'], 'unavailable')
        self.assertIn('unsupported ABI', observed['current_rss_unavailable_reason'])

    def test_journal_releases_commands_and_restores_exact_evidence_once(self):
        rng = random.Random(97622)
        with tempfile.TemporaryDirectory(prefix='test-command-journal-') as temporary:
            journal = bench.CommandJournal(Path(temporary) / 'report.json')
            originals, batches = [], []
            try:
                for sequence in range(12):
                    count = rng.randrange(1, 33)
                    commands = [{'command_id': sequence * 100 + index, 'seconds': rng.random()}
                                for index in range(count)]
                    originals.append(commands)
                    batch = {'commands': commands, 'iterations': count}
                    batches.append(batch)
                    journal.append(0, 'samples', [(sequence, 'baseline', batch)])
                    self.assertEqual(batch['commands'], [])
                    self.assertEqual(batch['command_journal'], {'record': sequence, 'batch': 0})
                binding = journal.binding()
                self.assertEqual(binding['sha256'], bench.executable_sha256(journal.path))
                self.assertEqual(binding['bytes'], journal.path.stat().st_size)
                self.assertEqual(binding['records'], 12)
                journal.restore()
                self.assertEqual([batch['commands'] for batch in batches], originals)
            finally:
                journal.close()

    def test_journal_overflow_and_tampering_preserve_owned_evidence_and_fail_closed(self):
        with tempfile.TemporaryDirectory(prefix='test-command-journal-errors-') as temporary:
            journal = bench.CommandJournal(Path(temporary) / 'report.json')
            first = {'commands': [{'command_id': 1}], 'iterations': 1}
            second = {'commands': [{'command_id': 2}], 'iterations': 1}
            try:
                journal.append(0, 'samples', [(0, 'baseline', first)])
                previous = journal.path.read_bytes()
                with patch.object(bench, 'MAX_REPORT_BYTES', len(previous)), self.assertRaises(bench.ReportTooLarge):
                    journal.append(0, 'samples', [(1, 'candidate', second)])
                self.assertEqual(journal.path.read_bytes(), previous)
                self.assertEqual(second['commands'], [{'command_id': 2}])
                with journal.path.open('ab') as handle:
                    handle.write(b'partial')
                with self.assertRaisesRegex(bench.BenchmarkError, 'bytes or hash'):
                    journal.restore()
            finally:
                journal.close()

    def test_journal_overflow_retains_active_unit_in_compact_error_checkpoint(self):
        with tempfile.TemporaryDirectory(prefix='test-journal-active-unit-overflow-') as temporary:
            work = Path(temporary)
            source = work / 'binary'
            source.write_bytes(b'executable fixture')
            source.chmod(0o700)
            metadata, output = work / 'metadata.json', work / 'report.json'
            metadata.write_text(json.dumps({'runner_label': 'test', 'toolchain': 'exact',
                                 **{label: {'revision': 'a' * 40, 'build_command': 'test', 'expected_version': '0.5.0'}
                                    for label in bench.LABELS}}), encoding='utf-8')
            original_append, original_persist = bench.CommandJournal.append, bench.persist
            journal_before = []

            def append(journal, case_index, phase, rows):
                if phase == 'samples':
                    journal_before.append(journal.path.read_bytes())
                    with patch.object(bench, 'MAX_REPORT_BYTES', journal.bytes):
                        return original_append(journal, case_index, phase, rows)
                return original_append(journal, case_index, phase, rows)

            def persist(report, path):
                # A final full artifact may exceed its cap even though the
                # prior compact error checkpoint fits. Force that distinct
                # path without changing the normal compact checkpoint limit.
                if report.get('report_kind') == 'complete-report':
                    with patch.object(bench, 'MAX_REPORT_BYTES', 1):
                        return original_persist(report, path)
                return original_persist(report, path)

            def initialize(_runner, _fixture, _case, initializing=False):
                return {'command_id': 0, 'seconds': 1, 'parent_timing': [0, 0.1, 1],
                        'command_timing': [0] * 12, 'output_validation': {'verified': True}}

            def measured(_runner, _fixture, _case, iterations):
                return {'iterations': iterations, 'seconds': iterations * 0.01,
                        'commands': [{'command_id': index, 'seconds': 0.01} for index in range(iterations)],
                        'output_validation': {'verified': True}}

            with patch('benchmark_runtime.shutil.which', return_value=sys.executable), \
                    patch.object(bench, 'make_fixture', side_effect=mocked_fixture), \
                    patch.object(bench, 'execute_fixture', side_effect=initialize), \
                    patch.object(bench, 'batch', side_effect=measured), \
                    patch.object(bench.CommandJournal, 'append', autospec=True, side_effect=append), \
                    patch.object(bench, 'persist', side_effect=persist), \
                    contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(io.StringIO()):
                result = bench.main(['--baseline', str(source), '--candidate', str(source),
                                     '--metadata', str(metadata), '--output', str(output), '--pairs', '40', '--gate'])
            self.assertEqual(result, 1)
            checkpoint = json.loads(output.read_text())
            manifest = json.loads(output.with_suffix('.overflow.json').read_text())
            self.assertEqual((checkpoint['status'], checkpoint['exit_code'], checkpoint['report_kind']),
                             ('error', 1, 'compact-checkpoint'))
            self.assertEqual(manifest['retained_checkpoint'], str(output.resolve()))
            self.assertFalse(manifest['samples_dropped'])
            active = checkpoint['results'][0]['samples']
            self.assertEqual(len(active), 4)
            self.assertTrue(all(len(sample[label]['commands']) == sample['iterations']
                                for sample in active for label in bench.LABELS))
            journal = Path(checkpoint['raw_command_journal']['path'])
            self.assertEqual(journal.read_bytes(), journal_before[0])
            self.assertEqual(bench.executable_sha256(journal), checkpoint['raw_command_journal']['sha256'])

    def test_future_cli_defaults_and_finite_upper_bounds(self):
        args = bench.parser().parse_args(['--baseline', 'A', '--candidate', 'B', '--metadata', 'M'])
        self.assertEqual((args.pairs, args.budget), (128, 3600))
        for flag, value in (('--pairs', '132'), ('--budget', '3601')):
            with patch.object(bench, 'load_metadata') as load, contextlib.redirect_stderr(io.StringIO()):
                with self.assertRaises(SystemExit):
                    bench.main(['--baseline', 'A', '--candidate', 'B', '--metadata', 'M', flag, value])
            load.assert_not_called()

    def test_host_snapshot_is_outside_timing_and_handles_unavailable_values(self):
        for value in ([1.0, 2.0, 3.0], [math.nan, 2, 3], [True, 2, 3]):
            with patch.object(bench.os, 'getloadavg', return_value=value), patch.object(bench.time, 'monotonic', return_value=7):
                row = bench.host_snapshot('nested-lookup', 0)
            self.assertEqual(row['monotonic_seconds'], 7)
            self.assertFalse(row['used_for_gate'])
            self.assertEqual(row['load_average_status'], 'reported' if value == [1.0, 2.0, 3.0] and type(value[0]) is float else 'unavailable')
            self.assertTrue(all(row[key] == 'unavailable' for key in ('memory_status', 'swap_status', 'cpu_utilization_status')))
        with patch.object(bench.os, 'getloadavg', side_effect=OSError('unsupported')):
            self.assertEqual(bench.host_snapshot('image-compile', 1)['load_average_status'], 'unavailable')

    def test_atomic_checkpoint_and_overflow_keep_prior_evidence(self):
        with tempfile.TemporaryDirectory(prefix='test-checkpoint-') as temporary:
            output = Path(temporary) / 'report.json'
            report = {'gate': True, 'status': 'incomplete', 'results': [], 'errors': [],
                      'policy': {'method': 'test', 'assumptions': 'test', 'relative_threshold': 0.10}}
            bench.persist(report, output)
            previous = output.read_bytes()
            report['partial_completed_side'] = 'x' * 2000
            with patch.object(bench, 'MAX_REPORT_BYTES', 1024), self.assertRaises(bench.ReportTooLarge):
                bench.persist(report, output)
            self.assertEqual(output.read_bytes(), previous)
            manifest = json.loads(output.with_suffix('.overflow.json').read_text())
            self.assertEqual((manifest['status'], manifest['exit_code']), ('error', 1))
            self.assertFalse(manifest['samples_dropped'])
            self.assertEqual(list(Path(temporary).glob('*.tmp')), [])
            with patch.object(bench.os, 'replace', side_effect=OSError('controlled replacement failure')):
                with self.assertRaises(OSError):
                    bench.atomic_text(output, 'new evidence')
            self.assertEqual(output.read_bytes(), previous)
            self.assertEqual(list(Path(temporary).glob('*.tmp')), [])

    def test_maximum_sampling_requires_the_finite_journal_and_final_report_guards(self):
        # A full 512-command batch bounds the largest command IDs and long
        # finite float spellings. Compute repetition exactly without allocating
        # all 524288 commands. Only the cache case has reported build telemetry.
        commands = []
        for index in range(512):
            start = 100000.123456789 + index
            end = start + 0.987654321012345
            commands.append({'command_id': 999999 + index, 'seconds': end - start,
                             'parent_timing': [start, start + 0.123456789012345, end],
                             'command_timing': [start + number / 100000 for number in range(12)]})
        batch = {'iterations': 512, 'seconds': sum(item['seconds'] for item in commands),
                 'commands': commands, 'output_validation': {'verified': True}}
        ordinary_size = len(json.dumps(batch, separators=(',', ':'), allow_nan=False).encode())
        for command in commands:
            command['reported_build_timing'] = bench.reported_build_timing({'elapsed_ms': 1.7976931348623157e308}, command['seconds'])
        cache_size = len(json.dumps(batch, separators=(',', ':'), allow_nan=False).encode())
        # 128 pairs have two batches apiece. A generous 8MiB covers all fixed
        # inventories, pilots, schedules, unit/host metadata and UTF-8 provenance.
        bounded_bytes = 256 * (3 * ordinary_size + cache_size) + 8 * 1024 * 1024
        # Legal iteration ceilings do not promise unlimited evidence storage.
        # Hard caps reject an oversized report, preserving its raw journal and
        # last compact checkpoint instead of trimming any command records.
        self.assertGreater(bounded_bytes, bench.MAX_REPORT_BYTES)
        self.assertLess(8 * cache_size, bench.MAX_JOURNAL_RECORD_BYTES)
    def test_sides_have_equal_inputs_and_disjoint_private_caches(self):
        with tempfile.TemporaryDirectory(prefix='test-paired-isolation-') as temporary:
            work = Path(temporary)
            with patch.dict(os.environ, {'TEKAI_EMBEDDED_ENGINE_RUNNER': '/host/engine', 'TEXINPUTS': '/host/files',
                                         'TEKAI_DIAGNOSTIC_PROFILE': '1',
                                         'TEKAI_FORMAT_CACHE': '/host/cache', 'HOME': '/host/home',
                                         **{name: '/host/search-root' for name in bench.SEARCH_ENV_VARS}}):
                fixtures = [bench.make_fixture(work / slot / replica, Path('/binary'), 'nested-lookup')
                            for slot in ('s0', 's1') for replica in ('r0', 'r1')]
            self.assertEqual(len({fixture['input_sha256'] for fixture in fixtures}), 1)
            self.assertEqual(len({len(str(fixture['root'])) for fixture in fixtures}), 1)
            for fixture in fixtures:
                env = fixture['env']
                self.assertNotIn('TEKAI_EMBEDDED_ENGINE_RUNNER', env)
                self.assertNotIn('TEKAI_DIAGNOSTIC_PROFILE', env)
                for name in bench.SEARCH_ENV_VARS:
                    self.assertNotIn(name, env)
                self.assertEqual(env['PATH'], '')
                for key in ('HOME', 'TEKAI_ENGINE_CACHE', 'TEKAI_FORMAT_CACHE', 'TEKAI_AUX_CACHE', 'TEKAI_BIBTEX_CACHE'):
                    self.assertTrue(Path(env[key]).is_relative_to(fixture['root']))
                    self.assertEqual(len({cell['env'][key] for cell in fixtures}), 4)
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
                            patch.object(bench, 'check_fixture_output', return_value={'verified': True}), \
                            patch.object(bench, 'cache_state_snapshot', return_value={'verified': True}):
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
                      'parent_timing': [float(index), float(index), float(index) + seconds],
                      'command_timing': [float(index)] * 12,
                      'reported_build_timing': diagnostic, 'output_validation': {'verified': True}}
                     for index, (seconds, diagnostic) in enumerate(zip((0.01, 0.02), diagnostics))]
        with patch.object(bench, 'execute_fixture', side_effect=responses), \
                patch.object(bench, 'cache_state_snapshot', return_value={'verified': True}):
            result = bench.batch(None, {}, 'warm-build-cache', 2)
        self.assertEqual(result['seconds'], 0.03)
        self.assertEqual(result['iterations'], 2)
        self.assertEqual([command['reported_build_timing'] for command in result['commands']], diagnostics)
        self.assertEqual([command['seconds'] for command in result['commands']], [0.01, 0.02])
        self.assertEqual([command['parent_timing'] for command in result['commands']], [response['parent_timing'] for response in responses])
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
                               **{label: {'revision': 'a' * 40, 'build_command': 'test', 'expected_version': '0.5.0'} for label in bench.LABELS}}), encoding='utf-8')
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
                        **{label: {'revision': 'a' * 40, 'build_command': 'cargo build --release', 'expected_version': '0.5.0'} for label in bench.LABELS}}
            path.write_text(json.dumps(metadata), encoding='utf-8')
            self.assertEqual(bench.load_metadata(path), metadata)
            metadata['baseline']['expected_version'] = '0.6.0-rc.1+test'
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
                        **{label: {'revision': 'a' * 40, 'build_command': 'test', 'expected_version': '0.5.0'} for label in bench.LABELS}}
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
                              **{label: {'revision': 'a' * 40, 'build_command': 'test', 'expected_version': '0.5.0'} for label in bench.LABELS}}
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
                self.assertEqual(len(report['executables']), 4)
                self.assertEqual({(row['label'], row['slot'], row['replica']) for row in report['executables']},
                                 {('baseline', 's0', 'r0'), ('candidate', 's0', 'r1'),
                                  ('baseline', 's1', 'r1'), ('candidate', 's1', 'r0')})
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
                              **{label: {'revision': 'a' * 40, 'build_command': 'test', 'artifact_sha256': actual, 'expected_version': '0.5.0'}
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
                               **{label: {'revision': 'a' * 40, 'build_command': 'test', 'expected_version': '0.5.0',
                                          'artifact_sha256': bench.executable_sha256(source)} for label in bench.LABELS}}), encoding='utf-8')
            output = work / 'report.json'
            argv = ['--baseline', str(source), '--candidate', str(source), '--metadata', str(metadata),
                    '--output', str(output), '--pairs', '40', '--min-sample-seconds', '0.25', '--gate']
            events = []
            warm_seconds = 0.01
            pilot_seconds = 0.01

            def initialize(_runner, fixture, case, initializing=False):
                self.assertTrue(initializing)
                events.append((case, 'initialize', fixture['label']))
                result = {'command_id': 0, 'seconds': 100, 'parent_timing': [0, 1, 100],
                          'command_timing': [0] * 12, 'output_validation': {'verified': True}}
                if case == 'warm-build-cache':
                    result['reported_build_timing'] = bench.reported_build_timing({'elapsed_ms': 125}, 100)
                return result

            def batch(_runner, fixture, case, iterations):
                events.append((case, 'batch', fixture['label'], iterations))
                rate = warm_seconds if iterations == 1 else pilot_seconds
                elapsed = rate if fixture['label'] == 'baseline' else rate * 0.8
                return dict(seconds=elapsed * iterations, iterations=iterations,
                            commands=[{'command_id': index + 1, 'seconds': elapsed} for index in range(iterations)],
                            output_validation={'verified': True})

            for mutate, warm_seconds, pilot_seconds, expected in ((False, 0.01, 0.01, 0), (True, 0.01, 0.01, 1),
                                                                  (False, 0.014, 0.0065, 0), (False, 0.001, 0.0001, 2)):
                events.clear()
                checkpoints = []
                original_persist = bench.persist

                def checkpoint(report, path):
                    checkpoints.append({row['case']: len(row['samples']) for row in report['results']})
                    original_persist(report, path)

                def measured(*args):
                    result = batch(*args)
                    if mutate and args[-1] > 1:
                        (args[1]['project'] / 'main.tex').write_text('changed during measurement', encoding='utf-8')
                    return result

                with patch('benchmark_runtime.shutil.which', return_value=sys.executable), \
                        patch.object(bench, 'make_fixture', side_effect=mocked_fixture), \
                        patch.object(bench, 'execute_fixture', side_effect=initialize), \
                        patch.object(bench, 'batch', side_effect=measured), \
                        patch.object(bench, 'persist', side_effect=checkpoint), contextlib.redirect_stdout(io.StringIO()):
                    self.assertEqual(bench.main(argv), expected)
                report = json.loads(output.read_text())
                self.assertEqual(report['schema_version'], 6)
                self.assertEqual(report['policy']['calibration_pairs'], 2)
                self.assertEqual(report['policy']['calibration_iterations'], 32)
                self.assertEqual(report['policy']['calibration_method'], 'balanced-batched-slot-pilot-v1')
                self.assertEqual(report['policy']['max_iterations'], 512)
                self.assertEqual(report['policy']['inference_units_per_case'], 10)
                self.assertEqual(report['policy']['pairs_per_inference_unit'], 4)
                self.assertEqual(report['policy']['batches_per_inference_unit'], 8)
                self.assertFalse(report['diagnostic_telemetry']['reported_build_timing']['used_for_gate'])
                for row in report['results']:
                    case_events = [event for event in events if event[0] == row['case']]
                    recorded_counts = sorted({state.get(row['case'], 0) for state in checkpoints})
                    self.assertEqual(recorded_counts, [0] if expected == 2 else list(range(0, 41, 4)))
                    self.assertEqual([event[1] for event in case_events[:4]], ['initialize'] * 4)
                    self.assertEqual([event[3] for event in case_events[4:12]], [1] * 8)
                    self.assertEqual([event[3] for event in case_events[12:20]], [32] * 8)
                    self.assertEqual(row['min_sample_seconds'], 1 if row['case'] == 'warm-build-cache' else 0.25)
                    self.assertEqual(row['fixture_unchanged'], not mutate)
                    self.assertEqual(len(row['fixture_inventory']), 4)
                    self.assertEqual(len({item['root'] for item in row['fixture_inventory']}), 4)
                    self.assertEqual(len({len(item['root']) for item in row['fixture_inventory']}), 1)
                    for cache in ('ENGINE', 'FORMAT', 'AUX', 'BIBTEX'):
                        self.assertEqual(len({item['cache_paths'][cache] for item in row['fixture_inventory']}), 4)
                    self.assertEqual(len(row['warmups']), 4)
                    self.assertEqual(len(row['calibration_pilots']), 4)
                    case_index = bench.CASES.index(row['case'])
                    expected_schedule = [model_schedule(index, case_index) for index in range(40)]
                    self.assertEqual(row['slot_schedule'], json.loads(json.dumps(expected_schedule)))
                    expected_cells = [{'slot': pair['slot'], 'label': label, 'replica': pair['replicas'][label]}
                                      for pair in expected_schedule[:2] for label in pair['order']]
                    self.assertEqual(row['initialization_order'], expected_cells)
                    for index, pilot in enumerate(row['calibration_pilots']):
                        self.assertEqual(pilot['pair_index'], index // 2)
                        self.assertEqual(pilot['slot'], ('s0', 's1')[index % 2])
                        self.assertEqual(pilot['order'], list(('baseline', 'candidate') if (index // 2 + index % 2 + case_index) % 2 == 0
                                                              else ('candidate', 'baseline')))
                        self.assertEqual(pilot['iterations'], 32)
                        self.assertEqual([pilot[label]['iterations'] for label in bench.LABELS], [32, 32])
                    self.assertEqual(row['calibration'], bench.calibration_details(row['calibration_pilots'], row['min_sample_seconds'], 512))
                    for initialization in (value for slot in row['initialization'].values() for value in slot.values()):
                        if row['case'] == 'warm-build-cache':
                            self.assertEqual(initialization['reported_build_timing'],
                                             bench.reported_build_timing({'elapsed_ms': 125}, 100))
                        else:
                            self.assertNotIn('reported_build_timing', initialization)
                    if expected == 2:
                        self.assertEqual(len(case_events), 20)
                        self.assertEqual(row['samples'], [])
                        self.assertEqual(row['analysis']['status'], 'inconclusive')
                        self.assertEqual(row['calibration_infeasible'], row['calibration'])
                        self.assertEqual(row['calibration_infeasible']['iteration_ceiling'], 512)
                        self.assertGreater(row['calibration_infeasible']['minimum_iterations'], 512)
                        self.assertLess(row['calibration_infeasible']['predicted_ceiling_batch_seconds'], row['min_sample_seconds'])
                    else:
                        expected_counts = (97, 385) if pilot_seconds == 0.0065 else (63, 250)
                        self.assertEqual(row['iterations_per_batch'], expected_counts[row['case'] == 'warm-build-cache'])
                        self.assertEqual(len(row['samples']), 40)
                        self.assertEqual(row['analysis']['inference_unit_count'], 10)
                        self.assertEqual(row['analysis']['median_ratio_interval']['order_statistic'], 1)
                        self.assertEqual(len(row['unit_timing']), 10)
                        self.assertTrue(all(unit['complete'] for unit in row['unit_timing']))
                        self.assertEqual(len([host for host in report['host_observations'] if host['case'] == row['case']]), 10)
                if mutate:
                    self.assertEqual(report['status'], 'error')
                else:
                    self.assertEqual(report['status'], 'inconclusive' if expected == 2 else 'pass')
                    self.assertEqual(report['errors'], [])
                    self.assertEqual(len(report['results']), 4)

    def test_later_side_failures_preserve_partial_initialization_warmups_pilots_and_samples(self):
        with tempfile.TemporaryDirectory(prefix='test-paired-partial-calibration-') as temporary:
            work = Path(temporary)
            source = work / 'binary'
            source.write_bytes(b'executable fixture')
            source.chmod(0o700)
            metadata = work / 'metadata.json'
            metadata.write_text(json.dumps({'runner_label': 'test', 'toolchain': 'exact',
                               **{label: {'revision': 'a' * 40, 'build_command': 'test', 'expected_version': '0.5.0'} for label in bench.LABELS}}), encoding='utf-8')
            output = work / 'report.json'

            for phase, failure_iterations in (('initialization', None), ('warmups', 1), ('calibration_pilots', 32), ('samples', 200)):
                def initialize(_runner, fixture, _case, initializing=False):
                    if phase == 'initialization' and fixture['label'] == 'candidate':
                        raise bench.BenchmarkError('controlled later-side ' + phase + ' failure')
                    return {'command_id': 0, 'seconds': 1, 'parent_timing': [0, 0.1, 1],
                            'command_timing': [0] * 12, 'output_validation': {'verified': True}}

                def measured(_runner, fixture, _case, iterations):
                    if iterations == failure_iterations and fixture['label'] == 'candidate':
                        raise bench.BenchmarkError('controlled later-side ' + phase + ' failure')
                    return {'seconds': iterations * 0.01, 'iterations': iterations,
                            'commands': [{'command_id': index + 1, 'seconds': 0.01} for index in range(iterations)],
                            'output_validation': {'verified': True}}

                with self.subTest(phase=phase), patch('benchmark_runtime.shutil.which', return_value=sys.executable), \
                        patch.object(bench, 'make_fixture', side_effect=mocked_fixture), \
                        patch.object(bench, 'execute_fixture', side_effect=initialize), \
                        patch.object(bench, 'batch', side_effect=measured), contextlib.redirect_stdout(io.StringIO()):
                    self.assertEqual(bench.main(['--baseline', str(source), '--candidate', str(source),
                                                '--metadata', str(metadata), '--output', str(output),
                                                '--cli-startup-min-sample-seconds', '1', '--gate']), 1)
                report = json.loads(output.read_text())
                row = report['results'][0]
                self.assertEqual(report['status'], 'error')
                self.assertIn('controlled later-side ' + phase + ' failure', report['errors'][0])
                partial = row[phase]['s0'] if phase == 'initialization' else row[phase][0]
                self.assertIn('baseline', partial)
                self.assertNotIn('candidate', partial)
                if phase == 'initialization':
                    self.assertEqual(row[phase]['s1'], {})
                else:
                    self.assertEqual(len(row[phase]), 1)
                    self.assertEqual(partial['baseline']['seconds'], failure_iterations * 0.01)
                if phase != 'samples':
                    self.assertEqual(row['samples'], [])
                    self.assertEqual(row['unit_timing'], [])
                else:
                    self.assertEqual(len(row['unit_timing']), 1)
                    self.assertFalse(row['unit_timing'][0]['complete'])
                    self.assertNotIn('monotonic_end_seconds', row['unit_timing'][0])
                self.assertEqual(report['host_observations'], [])
                self.assertTrue(all(binary['unchanged'] for binary in report['executables']))


if __name__ == '__main__':
    unittest.main()
