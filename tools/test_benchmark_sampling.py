"""Independent bounded models for the prospective plan, without timing samples."""

from fractions import Fraction
import itertools
import math
import unittest

import benchmark_sampling as sampling


class SamplingTests(unittest.TestCase):
    def test_interval_rank_matches_independent_coin_enumeration_and_future_plan(self):
        for n in range(4, 13):
            tails = [sum(sum(bits) < k or sum(bits) > n - k for bits in itertools.product((0, 1), repeat=n)) / 2 ** n
                     for k in range(1, (n + 1) // 2 + 1)]
            valid = [k for k, value in enumerate(tails, 1) if value <= 0.05 / 3]
            plan = sampling.sign_interval_plan(n, 0.05 / 3)
            self.assertEqual(None if plan is None else plan['order_statistic'], max(valid) if valid else None)
        plan = sampling.sign_interval_plan(32, 0.05 / 3)
        self.assertEqual(plan['order_statistic'], 9)
        self.assertAlmostEqual(plan['coverage_at_least'], 1 - 2 * sum(math.comb(32, i) for i in range(9)) / 2 ** 32)
        self.assertGreater(2 * sum(math.comb(32, i) for i in range(10)) / 2 ** 32, 0.05 / 3)

    def test_power_matches_independent_bernoulli_recursion(self):
        study = sampling.prospective_power_study()
        self.assertFalse(study['uses_observed_samples'])
        self.assertIn('conditional', study['scope'])
        self.assertIn('independent', study['assumptions'])
        self.assertEqual([plan['logical_pairs'] for plan in study['plans']], [40, 128])
        for plan in study['plans']:
            n, k = plan['inference_units'], plan['order_statistic']
            previous = -1
            for row in plan['rows']:
                q = Fraction(str(row['probability_unit_ratio_at_or_below_boundary']))
                distribution = [Fraction(1)]
                for _ in range(n):
                    next_distribution = [Fraction()] * (len(distribution) + 1)
                    for index, probability in enumerate(distribution):
                        next_distribution[index] += probability * (1 - q)
                        next_distribution[index + 1] += probability * q
                    distribution = next_distribution
                supported_pass = float(sum(distribution[n - k + 1:]))
                supported_fail = float(sum(distribution[:k]))
                self.assertEqual(row['interval_supports_pass_probability'], supported_pass)
                self.assertEqual(row['interval_supports_fail_probability'], supported_fail)
                self.assertAlmostEqual(sum(row[key] for key in ('interval_supports_pass_probability',
                    'interval_supports_fail_probability', 'interval_crosses_boundary_probability')), 1)
                self.assertGreater(supported_pass, previous)
                previous = supported_pass
        for old, future in zip(study['plans'][0]['rows'], study['plans'][1]['rows']):
            if old['probability_unit_ratio_at_or_below_boundary'] >= 0.8:
                self.assertGreater(future['interval_supports_pass_probability'], old['interval_supports_pass_probability'])

    def test_finite_policy_and_invalid_counts(self):
        self.assertEqual((sampling.DEFAULT_PAIRS, sampling.MAX_PAIRS), (128, 128))
        self.assertEqual((sampling.DEFAULT_BUDGET, sampling.MAX_BUDGET), (2700, 2700))
        for value in (False, 0, -1, 129, 10 ** 400, 10.0):
            with self.subTest(value=value), self.assertRaises(ValueError):
                sampling.sign_interval_plan(value, 0.05 / 3)
        for alpha in (False, 0, -1, 1, 10 ** 400, math.nan, math.inf, '0.05'):
            with self.subTest(alpha=alpha), self.assertRaises(ValueError):
                sampling.sign_interval_plan(32, alpha)


if __name__ == '__main__':
    unittest.main()
