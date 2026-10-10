"""Fixed prospective sampling policy and exact, hypothetical sign-test power.

No observed timings enter this study. Its probabilities omit operational guards.
"""

from fractions import Fraction
import math

DEFAULT_PAIRS = 128
MAX_PAIRS = 128
DEFAULT_BUDGET = 3600.0
MAX_BUDGET = 3600.0
SCHEMA_VERSION = 6
PREDECLARED_CASES = 4
MAX_REPORT_BYTES = 128 * 1024 * 1024


def sign_interval_plan(n, alpha):
    try:
        valid_alpha = type(alpha) in (int, float) and math.isfinite(alpha) and 0 < alpha < 1
    except OverflowError:
        valid_alpha = False
    if type(n) is not int or not 1 <= n <= MAX_PAIRS or not valid_alpha:
        raise ValueError('Sign interval requires a bounded positive count and finite alpha in (0, 1)')
    selected, tail = None, 0
    for k in range(1, (n + 1) // 2 + 1):
        tail += math.comb(n, k - 1)
        noncoverage = Fraction(2 * tail, 2 ** n)
        if noncoverage <= Fraction(alpha):
            selected = {'order_statistic': k, 'coverage_at_least': float(1 - noncoverage)}
        else:
            break
    return selected


def prospective_power_study():
    """Exact binomial decision probabilities under declared hypothetical models."""
    plans = []
    for n in (10, DEFAULT_PAIRS // 4):
        plan = sign_interval_plan(n, 0.05 / PREDECLARED_CASES)
        k = plan['order_statistic']
        rows = []
        for q in (Fraction(1, 2), Fraction(3, 5), Fraction(7, 10), Fraction(4, 5),
                  Fraction(9, 10), Fraction(19, 20), Fraction(99, 100)):
            probabilities = [math.comb(n, count) * q ** count * (1 - q) ** (n - count)
                             for count in range(n + 1)]
            supported_pass = sum(probabilities[n - k + 1:], Fraction())
            supported_fail = sum(probabilities[:k], Fraction())
            rows.append({'probability_unit_ratio_at_or_below_boundary': float(q),
                         'interval_supports_pass_probability': float(supported_pass),
                         'interval_supports_fail_probability': float(supported_fail),
                         'interval_crosses_boundary_probability': float(1 - supported_pass - supported_fail)})
        plans.append({'inference_units': n, 'logical_pairs': 4 * n, **plan,
                      'units_at_or_below_boundary_needed_for_pass': n - k + 1, 'rows': rows})
    return {'method': 'exact-prospective-binomial-v1', 'uses_observed_samples': False, 'used_for_gate': False,
            'scope': 'Hypothetical interval decisions conditional on every unchanged operational guard passing.',
            'assumptions': 'Complete unit ratios are independent and identically distributed with a stable '
                'continuous distribution. q is a specified probability that a unit ratio is at or below '
                'the fixed practical boundary, not an estimate from this or any historical run. '
                'Serial dependence, drift and noise/order/duration guards can reduce actual decision probability.',
            'familywise_confidence': 0.95, 'predeclared_cases': PREDECLARED_CASES, 'plans': plans}


if __name__ == '__main__':
    import json
    print(json.dumps(prospective_power_study(), indent=2, allow_nan=False))
