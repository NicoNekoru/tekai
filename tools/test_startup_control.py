"""Schedule properties, bounded diagnostic driver and a tiny native smoke."""

from collections import Counter
import contextlib
import copy
import io
import itertools
import json
import os
from pathlib import Path
import random
import shutil
import signal
import subprocess
import sys
import tempfile
import time
import unittest
from unittest import mock

import benchmark_runtime as benchmark
import startup_control as control


def evidence_model(blocks=8, repeats=8):
    """A complete fixed plan with one recorded observation per launch."""
    report = {'policy': {'blocks': blocks, 'repeats': repeats},
              'planned_balance': control.schedule_balance(blocks, repeats),
              'epochs': [], 'commands': []}
    for block in range(blocks):
        for planned in control.block_schedule(block):
            epoch_id = len(report['epochs'])
            epoch = {**planned, 'epoch_id': epoch_id, 'replacement_generation': epoch_id + 1,
                     'status': 'complete', 'completed_rounds': 1 + repeats, 'copies': []}
            for ordinal, path_id in enumerate(planned['copy_order']):
                epoch['copies'].append({'path_id': path_id, 'label': planned['assignment'][path_id],
                    'copy_ordinal': ordinal, 'sha256_start': 'fixture-bound-hash',
                    'source_sha256_start': 'fixture-bound-hash', 'unchanged': True,
                    'source_unchanged': True})
            report['epochs'].append(epoch)
            rounds = [('first_after_copy_hash_verification', 0, planned['first_launch_order'])]
            rounds.extend(('later', repetition, control.later_order(block, planned['assignment_index'], repetition))
                          for repetition in range(repeats))
            for phase, repetition, order in rounds:
                for ordinal, path_id in enumerate(order):
                    report['commands'].append({'epoch_id': epoch_id, 'block_index': block,
                        'assignment_id': planned['assignment_id'], 'path_id': path_id,
                        'label': planned['assignment'][path_id], 'phase': phase, 'repetition': repetition,
                        'launch_ordinal': ordinal, 'path_launch_order': list(order), 'status': 'verified',
                        'command_id': len(report['commands']) + 1,
                        'seconds_since_copy_verification': 0.1 + ordinal,
                        **dict.fromkeys(control.STAGES, .01)})
    return report


class ScheduleTests(unittest.TestCase):
    def test_all_assignments_and_equal_artifact_exposure_each_block(self):
        for block in range(8):
            rows = control.block_schedule(block)
            self.assertEqual(len(rows), 8)
            self.assertEqual({row['assignment_id'] for row in rows}, {name for name, _ in control.ASSIGNMENTS})
            mixed = [row for row in rows if row['kind'] == 'mixed']
            self.assertEqual(len(mixed), 6)
            self.assertEqual(len({tuple(row['assignment'].values()) for row in mixed}), 6)
            for row in mixed:
                self.assertEqual(Counter(row['assignment'].values()), {'baseline': 2, 'candidate': 2})
            for path in control.PATH_IDS:
                self.assertEqual(Counter(row['assignment'][path] for row in rows), {'baseline': 4, 'candidate': 4})
                self.assertEqual(Counter(row['assignment'][path] for row in mixed), {'baseline': 3, 'candidate': 3})
            for row in rows:
                self.assertEqual(row['copy_order'], list(reversed(row['first_launch_order'])))
                self.assertEqual(set(row['copy_order']), set(control.PATH_IDS))

    def test_williams_epoch_positions_and_all_ordered_predecessors(self):
        schedule = [control.block_schedule(block) for block in range(8)]
        for name, _ in control.ASSIGNMENTS:
            counts = Counter(row['epoch_ordinal'] for block in schedule for row in block if row['assignment_id'] == name)
            self.assertEqual(counts, dict.fromkeys(range(8), 1))
        predecessors = Counter((left['assignment_id'], right['assignment_id'])
                               for block in schedule for left, right in zip(block, block[1:]))
        self.assertEqual(len(predecessors), 56)
        self.assertEqual(set(predecessors.values()), {1})
        self.assertTrue(all(left != right for left, right in predecessors))

    def test_cyclic_reverse_orders_are_exactly_position_balanced(self):
        for block in range(8):
            for assignment in range(8):
                for repeats in (8, 16):
                    orders = [control.later_order(block, assignment, index) for index in range(repeats)]
                    for index in range(0, repeats, 2):
                        self.assertEqual(orders[index + 1], tuple(reversed(orders[index])))
                    for path in control.PATH_IDS:
                        self.assertEqual(Counter(order.index(path) for order in orders), dict.fromkeys(range(4), repeats // 4))

    def test_randomized_exhaustive_bounded_schedule_flags_and_exposure(self):
        cases = list(itertools.product(range(1, 9), range(1, 17)))
        random.Random(731095).shuffle(cases)
        for blocks, repeats in cases:
            with self.subTest(blocks=blocks, repeats=repeats):
                result = control.schedule_balance(blocks, repeats)
                self.assertTrue(result['artifact_exposure_balanced'])
                self.assertEqual(result['assignment_epoch_positions_balanced'], blocks == 8)
                self.assertEqual(result['within_block_predecessors_balanced'], blocks == 8)
                self.assertEqual(result['within_epoch_later_positions_balanced'], repeats % 8 == 0)
                self.assertEqual(result['within_epoch_later_reversal_pairs_complete'], repeats % 2 == 0)
                self.assertTrue(all(result['path_positions_balanced'].values()))
                for counts in result['artifact_counts_per_path'].values():
                    self.assertEqual(counts, {'baseline': blocks * 4, 'candidate': blocks * 4})

    def test_schedule_rejects_nonexact_indices(self):
        for value in (-1, 8, True, 1.5, float('nan')):
            with self.assertRaises(ValueError):
                control.block_schedule(value)
        for blocks, repeats in ((0, 1), (9, 1), (1, 0), (1, 17), (True, 1), (1, True)):
            with self.assertRaises(ValueError):
                control.schedule_balance(blocks, repeats)


class ObservedProgressTests(unittest.TestCase):
    def test_complete_plan_counts_observed_commands_and_balance(self):
        completion, balance = control.observed_progress(evidence_model())
        self.assertTrue(completion['schedule_complete'])
        self.assertEqual(completion['completed_epochs'], 64)
        self.assertEqual(completion['verified_commands'], 2304)
        self.assertEqual(completion['completed_verified_rounds'], 576)
        self.assertEqual(completion['verified_replacements'], 256)
        self.assertEqual(completion['completed_block_indices'], list(range(8)))
        self.assertTrue(balance['artifact_exposure_balanced'])
        self.assertTrue(balance['assignment_epoch_positions_balanced'])
        self.assertTrue(balance['within_block_predecessors_balanced'])
        self.assertTrue(all(balance['path_positions_balanced'].values()))

    def test_missing_final_epoch_preserves_plan_but_reports_actual_imbalance(self):
        report = evidence_model()
        report['epochs'].pop()
        report['commands'] = [row for row in report['commands'] if row['epoch_id'] < 63]
        completion, balance = control.observed_progress(report)
        self.assertTrue(report['planned_balance']['assignment_epoch_positions_balanced'])
        self.assertFalse(completion['schedule_complete'])
        self.assertEqual(completion['planned_epochs'], 64)
        self.assertEqual(completion['completed_epochs'], 63)
        self.assertEqual(completion['verified_commands'], 2268)
        self.assertEqual(completion['first_launch_verified_commands'], 252)
        self.assertEqual(completion['completed_block_indices'], list(range(7)))
        self.assertFalse(balance['artifact_exposure_balanced'])
        self.assertFalse(balance['assignment_epoch_positions_balanced'])
        self.assertFalse(balance['within_block_predecessors_balanced'])

    def test_partial_later_round_counts_only_verified_observations(self):
        report = evidence_model()
        report['epochs'] = report['epochs'][:1]
        report['epochs'][0].update(status='incomplete', completed_rounds=1)
        report['commands'] = report['commands'][:7]
        report['commands'][-1]['status'] = 'error'
        completion, balance = control.observed_progress(report)
        self.assertFalse(completion['schedule_complete'])
        self.assertEqual(completion['attempted_commands'], 7)
        self.assertEqual(completion['verified_commands'], 6)
        self.assertEqual(completion['failed_commands'], 1)
        self.assertEqual(completion['completed_epochs'], 0)
        self.assertEqual(completion['completed_verified_rounds'], 1)
        self.assertEqual(completion['later_verified_commands'], 2)
        self.assertFalse(balance['within_epoch_later_positions_balanced'])
        self.assertFalse(balance['within_epoch_later_reversal_pairs_complete'])

    def test_empty_evidence_never_claims_full_balance(self):
        report = evidence_model()
        report.update(epochs=[], commands=[])
        completion, balance = control.observed_progress(report)
        self.assertFalse(completion['schedule_complete'])
        self.assertFalse(balance['artifact_exposure_balanced'])
        self.assertFalse(balance['assignment_epoch_positions_balanced'])
        self.assertFalse(balance['within_block_predecessors_balanced'])
        self.assertFalse(balance['within_epoch_later_positions_balanced'])
        self.assertFalse(balance['within_epoch_later_reversal_pairs_complete'])
        self.assertFalse(any(balance['path_positions_balanced'].values()))

    def test_complete_epoch_status_cannot_hide_a_missing_command(self):
        report = evidence_model()
        report['commands'].pop()
        completion, _ = control.observed_progress(report)
        self.assertEqual(completion['completed_epochs'], 63)
        self.assertFalse(completion['schedule_complete'])
        self.assertEqual(completion['completed_verified_rounds'], 575)

    def test_every_fixed_plan_command_prefix_has_exact_observed_counts(self):
        complete = evidence_model()
        per_epoch = 4 * (1 + complete['policy']['repeats'])
        observed_positions = {phase: {path: Counter() for path in control.PATH_IDS}
                              for phase in ('first_after_copy_hash_verification', 'later')}
        for length in range(len(complete['commands']) + 1):
            with self.subTest(length=length):
                epoch_count = (length + per_epoch - 1) // per_epoch
                report = {**complete, 'epochs': complete['epochs'][:epoch_count],
                          'commands': complete['commands'][:length]}
                if length:
                    newest = complete['commands'][length - 1]
                    observed_positions[newest['phase']][newest['path_id']][newest['launch_ordinal']] += 1
                completion, balance = control.observed_progress(report)
                full_epochs, partial_commands = divmod(length, per_epoch)
                first_commands = 4 * full_epochs + min(partial_commands, 4)
                self.assertEqual(completion['attempted_epochs'], epoch_count)
                self.assertEqual(completion['completed_epochs'], full_epochs)
                self.assertEqual(completion['attempted_commands'], length)
                self.assertEqual(completion['verified_commands'], length)
                self.assertEqual(completion['completed_verified_rounds'], length // 4)
                self.assertEqual(completion['verified_replacements'], 4 * epoch_count)
                self.assertEqual(completion['first_launch_verified_commands'], first_commands)
                self.assertEqual(completion['later_verified_commands'], length - first_commands)
                self.assertEqual(completion['completed_block_indices'], list(range(full_epochs // 8)))
                self.assertEqual(completion['schedule_complete'], length == 2304)
                self.assertEqual(balance['assignment_epoch_positions_balanced'], full_epochs == 64)
                self.assertEqual(balance['within_block_predecessors_balanced'], full_epochs == 64)
                for phase, paths in observed_positions.items():
                    for path, counts in paths.items():
                        self.assertEqual(balance['path_ordinal_counts'][phase][path], dict(counts))


class ContrastTests(unittest.TestCase):
    def test_first_later_contrast_keeps_copy_path_and_first_launch_covariates(self):
        report = evidence_model(blocks=1, repeats=2)
        path = control.PATH_IDS[0]
        for row in report['commands']:
            if row['epoch_id'] == 0 and row['path_id'] == path:
                row['parent_seconds'] = .02 if row['phase'] == 'first_after_copy_hash_verification' else .01
        described = control.summaries(report)
        pairs = described['first_later_comparisons']
        self.assertEqual(len(pairs), 8 * 4)
        pair = next(row for row in pairs if row['epoch_id'] == 0 and row['path_id'] == path)
        first = next(row for row in report['commands'] if row['epoch_id'] == 0 and row['path_id'] == path
                     and row['phase'] == 'first_after_copy_hash_verification')
        self.assertTrue(pair['complete'])
        self.assertEqual(pair['label'], report['epochs'][0]['assignment'][path])
        self.assertEqual(pair['first_verified_commands'], 1)
        self.assertEqual(pair['later_verified_commands'], 2)
        self.assertEqual(pair['first_launch_ordinal'], first['launch_ordinal'])
        self.assertEqual(pair['seconds_since_copy_verification'], first['seconds_since_copy_verification'])
        self.assertAlmostEqual(pair['stages']['parent_seconds']['first_over_later'], 2)
        self.assertAlmostEqual(pair['stages']['parent_seconds']['first_minus_later_seconds'], .01)
        self.assertTrue(all(row['used_for_gate'] is False for row in pairs))

    def test_incomplete_epoch_or_missing_later_command_has_no_first_later_ratio(self):
        for reason in ('validation-incomplete', 'later-missing', 'later-error', 'first-error'):
            with self.subTest(reason=reason):
                report = evidence_model(blocks=1, repeats=2)
                path = control.PATH_IDS[0]
                if reason == 'validation-incomplete':
                    report['epochs'][0]['status'] = 'incomplete'
                else:
                    phase = 'first_after_copy_hash_verification' if reason == 'first-error' else 'later'
                    selected = next(row for row in report['commands'] if row['epoch_id'] == 0
                                    and row['path_id'] == path and row['phase'] == phase)
                    if reason == 'later-missing':
                        report['commands'].remove(selected)
                    else:
                        selected['status'] = 'error'
                pairs = control.summaries(report)['first_later_comparisons']
                pair = next(row for row in pairs if row['epoch_id'] == 0 and row['path_id'] == path)
                self.assertFalse(pair['complete'])
                self.assertTrue(all(stage['first_over_later'] is None
                                    and stage['first_minus_later_seconds'] is None
                                    for stage in pair['stages'].values()))

    def test_zero_later_stage_has_no_ratio(self):
        report = evidence_model(blocks=1, repeats=2)
        path = control.PATH_IDS[0]
        for row in report['commands']:
            if row['epoch_id'] == 0 and row['path_id'] == path and row['phase'] == 'later':
                row['capture_setup_seconds'] = 0
        pair = next(row for row in control.summaries(report)['first_later_comparisons']
                    if row['epoch_id'] == 0 and row['path_id'] == path)
        self.assertTrue(pair['complete'])
        self.assertIsNone(pair['stages']['capture_setup_seconds']['first_over_later'])

    def test_mixed_contrast_uses_one_median_per_replacement(self):
        report = evidence_model(blocks=1, repeats=3)
        path = control.PATH_IDS[0]
        mixed_epochs = [epoch for epoch in report['epochs'] if epoch['kind'] == 'mixed']
        baseline = [epoch for epoch in mixed_epochs if epoch['assignment'][path] == 'baseline']
        candidate = [epoch for epoch in mixed_epochs if epoch['assignment'][path] == 'candidate']
        values = {baseline[0]['epoch_id']: (.001, .01, 1),
                  baseline[1]['epoch_id']: (.02, .03, .04),
                  baseline[2]['epoch_id']: (.05, .06, .07),
                  **{epoch['epoch_id']: (.09, .09, .09) for epoch in candidate}}
        for row in report['commands']:
            if row['epoch_id'] in values and row['path_id'] == path and row['phase'] == 'later':
                row['parent_seconds'] = values[row['epoch_id']][row['repetition']]
        described = control.summaries(report)
        pairs = described['mixed_path_phase_comparisons']
        self.assertEqual(len(pairs), 4 * 2)
        pair = next(row for row in pairs if row['path_id'] == path and row['phase'] == 'later')
        self.assertTrue(pair['complete'])
        self.assertEqual(pair['replacement_counts'], {'baseline': 3, 'candidate': 3})
        self.assertEqual({row['epoch_id'] for row in pair['replacements']}, set(values))
        self.assertAlmostEqual(pair['stages']['parent_seconds']['baseline']['median'], .03)
        self.assertAlmostEqual(pair['stages']['parent_seconds']['candidate']['median'], .09)
        self.assertAlmostEqual(pair['stages']['parent_seconds']['candidate_over_baseline'], 3)
        self.assertTrue(all(len(row['launch_ordinals']) == 3
                            and len(row['seconds_since_copy_verification']) == 3
                            for row in pair['replacements']))
        self.assertTrue(all(row['used_for_gate'] is False for row in pairs))
        self.assertIn('order-confounded', described['mixed_contrast_scope'])

    def test_missing_or_incomplete_mixed_replacement_prevents_contrast(self):
        for reason in ('missing', 'incomplete', 'failed-command'):
            with self.subTest(reason=reason):
                report = evidence_model(blocks=1, repeats=2)
                epoch = next(row for row in report['epochs'] if row['kind'] == 'mixed')
                if reason == 'missing':
                    report['epochs'].remove(epoch)
                    report['commands'] = [row for row in report['commands'] if row['epoch_id'] != epoch['epoch_id']]
                elif reason == 'incomplete':
                    epoch['status'] = 'incomplete'
                else:
                    selected = next(row for row in report['commands'] if row['epoch_id'] == epoch['epoch_id'])
                    selected['status'] = 'error'
                pairs = control.summaries(report)['mixed_path_phase_comparisons']
                affected = pairs if reason != 'failed-command' else [row for row in pairs
                    if row['path_id'] == selected['path_id'] and row['phase'] == selected['phase']]
                self.assertTrue(affected)
                self.assertTrue(all(row['complete'] is False for row in affected))
                self.assertTrue(all(stage['candidate_over_baseline'] is None
                                    and stage['candidate_minus_baseline_seconds'] is None
                                    for row in affected for stage in row['stages'].values()))

    def test_contrast_completion_uses_the_same_epoch_validators_as_progress(self):
        for reason in ('round-count', 'copy-changed', 'source-changed', 'copy-missing', 'copy-duplicate',
                       'copy-binding', 'duplicate-repetition'):
            with self.subTest(reason=reason):
                report = evidence_model(blocks=1, repeats=2)
                epoch = next(row for row in report['epochs'] if row['kind'] == 'mixed')
                if reason == 'round-count':
                    epoch['completed_rounds'] -= 1
                elif reason == 'copy-changed':
                    epoch['copies'][0]['unchanged'] = False
                elif reason == 'source-changed':
                    epoch['copies'][0]['source_unchanged'] = False
                elif reason == 'copy-missing':
                    epoch['copies'].pop()
                elif reason == 'copy-duplicate':
                    epoch['copies'][-1] = dict(epoch['copies'][0])
                elif reason == 'copy-binding':
                    epoch['copies'][0]['sha256_start'] = 'different-from-bound-source'
                else:
                    selected = next(row for row in report['commands'] if row['epoch_id'] == epoch['epoch_id']
                                    and row['phase'] == 'later' and row['repetition'] == 1)
                    selected['repetition'] = 0
                completion, _ = control.observed_progress(report)
                self.assertEqual(completion['completed_epochs'], 7)
                self.assertFalse(completion['schedule_complete'])
                described = control.summaries(report)
                first_later = [row for row in described['first_later_comparisons'] if row['epoch_id'] == epoch['epoch_id']]
                self.assertEqual(len(first_later), 4)
                self.assertTrue(all(row['complete'] is False for row in first_later))
                self.assertTrue(all(stage['first_over_later'] is None
                                    and stage['first_minus_later_seconds'] is None
                                    for row in first_later for stage in row['stages'].values()))
                mixed = described['mixed_path_phase_comparisons']
                self.assertEqual(len(mixed), 8)
                self.assertTrue(all(row['complete'] is False for row in mixed))
                self.assertTrue(all(stage['candidate_over_baseline'] is None
                                    and stage['candidate_minus_baseline_seconds'] is None
                                    for row in mixed for stage in row['stages'].values()))


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
        self.metadata_path = self.root / 'metadata.json'
        self.primary_path = self.root / 'primary.json'
        self.output = self.root / 'startup.json'
        self.bind_sources()

    def bind_sources(self, versions=None):
        versions = versions or {'baseline': '0.5.0', 'candidate': '0.6.0-rc.1+fixture'}
        self.metadata = {'runner_label': 'fixture', 'toolchain': 'fixture',
                         **{label: {'revision': 'fixture-' + label, 'build_command': 'fixture-only',
                                    'expected_version': versions[label], 'artifact_sha256': benchmark.executable_sha256(path)}
                            for label, path in self.sources.items()}}
        self.primary = {'schema_version': 5, 'gate': True, 'status': 'inconclusive', 'exit_code': 2,
                        'metadata': self.metadata, 'executables': []}
        for label, path in self.sources.items():
            digest = benchmark.executable_sha256(path)
            self.primary['executables'].append({'label': label, 'source_path': str(path),
                'source_sha256_start': digest, 'source_sha256_end': digest,
                'sha256_start': digest, 'sha256_end': digest, 'unchanged': True})
        self.metadata_path.write_text(json.dumps(self.metadata))
        self.primary_path.write_text(json.dumps(self.primary))

    def argv(self, *extra):
        return ['--baseline', str(self.sources['baseline']), '--candidate', str(self.sources['candidate']),
                '--metadata', str(self.metadata_path), '--primary-report', str(self.primary_path),
                '--output', str(self.output), '--blocks', '1', '--repeats', '2', *extra]

    def read_report(self):
        return control.read_input_json(self.output)


class BindingAndBoundsTests(FixtureFiles):
    def test_bounded_finite_cli_inputs_reject_before_any_output(self):
        for options in (('--blocks', '0'), ('--blocks', '9'), ('--repeats', '0'), ('--repeats', '17'),
                        ('--timeout', 'nan'), ('--timeout', '31'), ('--budget', 'inf'),
                        ('--budget', '0'), ('--budget', '601'), ('--output', str(self.root / 'bad.md'))):
            with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as error:
                control.main(self.argv(*options))
            self.assertEqual(error.exception.code, 2)
            self.assertFalse(self.output.exists())

    def test_json_rejects_nonfinite_constants_and_exponent_overflow(self):
        for content in ('{"value":NaN}', '{"value":Infinity}', '{"value":1e999}', '[0]', '{'):
            self.metadata_path.write_text(content)
            with self.assertRaises(ValueError):
                control.read_input_json(self.metadata_path)
            self.assertEqual(control.main(self.argv()), 1)
            self.assertEqual(self.read_report()['status'], 'error')

    def test_output_aliases_sources_primary_metadata_markdown_and_journal(self):
        companion = self.primary_path.with_suffix('.md')
        companion.write_text('unchanged primary markdown')
        journal = self.root / 'primary.raw.fixture.jsonl'
        journal.write_bytes(b'{"sequence":0}\n')
        self.primary.update(schema_version=6, report_kind='complete-report',
                            raw_command_journal={'path': str(journal), 'bytes': journal.stat().st_size,
                                'sha256': benchmark.executable_sha256(journal), 'format': 'tekai-command-journal-v1'})
        self.primary_path.write_text(json.dumps(self.primary))
        for target in (*self.sources.values(), self.metadata_path, self.primary_path, companion, journal):
            original = target.read_bytes()
            for symlink in (False, True):
                if self.output.exists():
                    self.output.unlink()
                if symlink:
                    self.output.symlink_to(target)
                else:
                    os.link(target, self.output)
                with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
                    control.main(self.argv())
                self.assertEqual(target.read_bytes(), original)
        self.output.unlink()
        self.output.with_suffix('.md').symlink_to(self.sources['candidate'])
        with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
            control.main(self.argv())
        self.assertFalse(self.output.exists())

    def test_output_pair_cannot_alias_itself(self):
        self.output.write_text('do not overwrite')
        os.link(self.output, self.output.with_suffix('.md'))
        with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
            control.main(self.argv())
        self.assertEqual(self.output.read_text(), 'do not overwrite')

    def test_invalid_primary_is_preserved_and_produces_partial_report(self):
        self.primary_path.write_text('{')
        self.assertEqual(control.main(self.argv()), 1)
        report = self.read_report()
        self.assertEqual(report['status'], 'error')
        self.assertTrue(report['primary']['unchanged'])
        self.assertEqual(self.primary_path.read_text(), '{')

    def test_xattrs_denied_are_explicit(self):
        with mock.patch.object(control.os, 'listxattr', side_effect=OSError('denied'), create=True), \
                mock.patch.object(control.os, 'getxattr', create=True):
            observed = control.xattrs(self.sources['baseline'])
        self.assertEqual(observed['status'], 'unavailable')
        self.assertIn('denied', observed['reason'])


class DriverTests(FixtureFiles):
    def run_model(self, *, fail_at=None, failure='oracle', mutation=None, copy_mismatch=False, extra=()):
        calls = []
        original_primary = self.primary_path.read_bytes()
        real_copy = control.shutil.copy2
        initial_bytes = {path.read_bytes(): label for label, path in self.sources.items()}

        def execute(supervisor, command, cwd, env):
            supervisor.counter += 1
            path = str(command[0])
            label = initial_bytes[Path(path).read_bytes()]
            calls.append({'command': list(map(str, command)), 'cwd': str(cwd), 'env': dict(env), 'label': label})
            if mutation:
                mutation(Path(path), len(calls))
            start = 10.0 + len(calls)
            times = [start + offset for offset in (0, 0, .0001, .0002, .0008, .0009,
                                                   .001, .0011, .0012, .01, .0101, .0102)]
            result = {'command_id': supervisor.counter, 'seconds': times[9] - times[0],
                      'parent_timing': [times[0], times[6], times[9]], 'command_timing': times,
                      'stdout': f"tekai {self.metadata[label]['expected_version']}\n", 'stderr_tail': ''}
            if len(calls) == fail_at:
                if failure == 'interrupt':
                    raise KeyboardInterrupt
                if failure == 'sigterm':
                    os.kill(os.getpid(), signal.SIGTERM)
                if failure == 'deadline':
                    raise benchmark.BenchmarkError('Whole-run deadline exceeded')
                if failure == 'stderr':
                    result['stderr_tail'] = 'unexpected\n'
                else:
                    result['stdout'] = 'tekai wrong-version\n'
            return result

        def copied(source, destination):
            result = real_copy(source, destination)
            if copy_mismatch:
                destination.write_bytes(b'wrong owned copy')
            return result

        with mock.patch.object(control.ControlSupervisor, 'execute', execute), \
                mock.patch.object(control.shutil, 'copy2', side_effect=copied), \
                mock.patch.dict(os.environ, {'TEKAI_DIAGNOSTIC_PROFILE': '1', 'TEXINPUTS': '/inherited'}):
            code = control.main(self.argv(*extra))
        self.assertEqual(self.primary_path.read_bytes(), original_primary)
        return code, self.read_report(), calls

    def test_fixed_schedule_copy_binding_exact_version_and_separate_first_launches(self):
        code, report, calls = self.run_model()
        self.assertEqual(code, 0)
        self.assertEqual(report['status'], 'complete')
        self.assertEqual(report['primary']['status'], 'inconclusive')
        self.assertEqual(report['primary']['exit_code'], 2)
        self.assertTrue(report['primary']['unchanged'])
        self.assertFalse(report['used_for_gate'])
        self.assertEqual(len(report['epochs']), 8)
        self.assertEqual(len(calls), 8 * 4 * 3)
        self.assertEqual(len(report['summaries']['cohorts']), 8 * 4 * 2)
        self.assertEqual(len(report['summaries']['same_binary_slot_comparisons']), 2 * 2 * 2)
        self.assertTrue(all(row['status'] == 'verified' for row in report['commands']))
        self.assertFalse(report['balance']['assignment_epoch_positions_balanced'])
        self.assertFalse(report['balance']['within_epoch_later_positions_balanced'])
        self.assertEqual(len({row['isolated_path'] for epoch in report['epochs'] for row in epoch['copies']}), 4)
        for epoch in report['epochs']:
            self.assertEqual(epoch['completed_rounds'], 3)
            self.assertEqual(epoch['replacement_generation'], epoch['epoch_id'] + 1)
            self.assertTrue(all(row['unchanged'] and row['source_unchanged'] for row in epoch['copies']))
            self.assertTrue(all(row['sha256_start'] == self.metadata[row['label']]['artifact_sha256'] for row in epoch['copies']))
            self.assertTrue(all(row['identity_end'] == row['identity_start'] for row in epoch['copies']))
            self.assertEqual(len(epoch['copies']), 4)
            self.assertGreaterEqual(epoch['copy_hash_elapsed_seconds'], 0)
            self.assertEqual(epoch['copy_bytes_read'], sum(self.sources[label].stat().st_size for label in epoch['assignment'].values()))
            for phase in ('first_after_copy_hash_verification', 'later'):
                rows = [row for row in report['commands'] if row['epoch_id'] == epoch['epoch_id'] and row['phase'] == phase]
                self.assertEqual(len(rows), 4 if phase.startswith('first') else 8)
        self.assertTrue(all(len(call['command']) == 2 and call['command'][1] == '--version' for call in calls))
        self.assertTrue(all('TEKAI_DIAGNOSTIC_PROFILE' not in call['env'] and 'TEXINPUTS' not in call['env'] for call in calls))
        for slot in benchmark.SLOTS:
            selected = [call for call in calls if Path(call['command'][0]).parts[-4] == slot]
            self.assertEqual(len({call['cwd'] for call in selected}), 1)
            self.assertTrue(all(call['env'] == selected[0]['env'] for call in selected))
        for row in report['commands']:
            self.assertEqual(row['output_validation']['expected_version'], self.metadata[row['label']]['expected_version'])
            self.assertGreaterEqual(row['seconds_since_copy_verification'], 0)
            self.assertTrue(row['finish_monotonic'] >= row['launch_monotonic'])
            self.assertFalse(row['used_for_gate'])
        self.assertEqual(report['io_totals']['copy_bytes_read'], sum(epoch['copy_bytes_read'] for epoch in report['epochs']))
        self.assertTrue(self.output.with_suffix('.md').exists())
        self.assertIn('no confidence score, primary rescoring or causal attribution', report['summaries']['scope'])

    def test_full_production_plan_remains_bounded_and_protects_primary_and_raw_journal(self):
        journal = self.root / 'primary.raw.fixture.jsonl'
        journal.write_bytes(b'{"sequence":0,"diagnostic_fixture":true}\n')
        self.primary.update(schema_version=6, report_kind='complete-report',
                            raw_command_journal={'path': str(journal), 'bytes': journal.stat().st_size,
                                'sha256': benchmark.executable_sha256(journal), 'format': 'tekai-command-journal-v1'})
        self.primary_path.write_text(json.dumps(self.primary))
        self.primary_path.with_suffix('.md').write_text('Original primary verdict unchanged')
        original = {path: path.read_bytes() for path in (*self.sources.values(), self.metadata_path,
                    self.primary_path, self.primary_path.with_suffix('.md'), journal)}
        code, report, calls = self.run_model(extra=('--blocks', '8', '--repeats', '8'))
        self.assertEqual(code, 0)
        self.assertEqual(report['status'], 'complete')
        self.assertEqual(len(report['epochs']), 64)
        self.assertEqual(len(calls), 2304)
        self.assertEqual(len(report['commands']), 2304)
        self.assertEqual(sum(row['phase'] == 'first_after_copy_hash_verification' for row in report['commands']), 256)
        self.assertTrue(all(epoch['status'] == 'complete' and epoch['completed_rounds'] == 9 for epoch in report['epochs']))
        self.assertTrue(report['balance']['assignment_epoch_positions_balanced'])
        self.assertTrue(report['balance']['within_block_predecessors_balanced'])
        self.assertTrue(report['balance']['within_epoch_later_positions_balanced'])
        self.assertEqual(sum(epoch['block_boundary'] for epoch in report['epochs']), 7)
        self.assertLess(self.output.stat().st_size, control.MAX_OUTPUT_BYTES)
        self.assertFalse(report['used_for_gate'])
        self.assertEqual(report['primary']['status'], 'inconclusive')
        self.assertEqual(report['primary']['exit_code'], 2)
        self.assertEqual(report['primary']['raw_command_journal'], self.primary['raw_command_journal'])
        self.assertTrue(all(row['unchanged'] for row in report['protected_inputs']))
        self.assertNotIn('gate', report)
        self.assertNotIn('results', report)
        self.assertNotIn('confidence_score', report)
        for path, expected in original.items():
            self.assertEqual(path.read_bytes(), expected)

    def test_failed_oracle_stderr_interrupt_sigterm_and_deadline_stop_future_launches(self):
        previous = signal.getsignal(signal.SIGTERM)
        for failure in ('oracle', 'stderr', 'interrupt', 'sigterm', 'deadline'):
            with self.subTest(failure=failure):
                code, report, calls = self.run_model(fail_at=3, failure=failure)
                self.assertEqual(code, 1)
                self.assertEqual(report['status'], 'error')
                self.assertEqual(len(calls), 3)
                self.assertEqual(len(report['commands']), 3)
                self.assertEqual(len(report['epochs']), 1)
                self.assertEqual(report['epochs'][0]['status'], 'incomplete')
                self.assertEqual(report['commands'][-1]['status'], 'error')
                self.assertTrue(all(row['unchanged'] for row in report['epochs'][0]['copies']))
                self.assertTrue(report['primary']['unchanged'])
                self.assertEqual(signal.getsignal(signal.SIGTERM), previous)

    def test_copy_mismatch_stops_before_launch(self):
        code, report, calls = self.run_model(copy_mismatch=True)
        self.assertEqual(code, 1)
        self.assertEqual(calls, [])
        self.assertEqual(len(report['epochs']), 1)
        self.assertEqual(len(report['epochs'][0]['copies']), 1)

    def test_changed_copy_is_detected_before_next_epoch_replacement(self):
        changed = []

        def mutation(path, count):
            if count == 12:
                path.write_bytes(b'changed copy')
                changed.append(path)

        code, report, calls = self.run_model(mutation=mutation)
        self.assertEqual(code, 1)
        self.assertEqual(len(calls), 12)
        self.assertEqual(len(report['epochs']), 1)
        self.assertTrue(any(not row['unchanged'] for row in report['epochs'][0]['copies']))

    def test_same_size_copy_change_with_restored_mtime_still_fails(self):
        def mutation(path, count):
            if count == 12:
                before = path.stat()
                content = path.read_bytes()
                path.write_bytes(bytes([content[0] ^ 1]) + content[1:])
                os.utime(path, ns=(before.st_atime_ns, before.st_mtime_ns))

        code, report, calls = self.run_model(mutation=mutation)
        self.assertEqual(code, 1)
        self.assertEqual(len(calls), 12)
        changed = [row for row in report['epochs'][0]['copies'] if not row['unchanged']]
        self.assertEqual(len(changed), 1)
        self.assertEqual(changed[0]['identity_start']['size_bytes'], changed[0]['identity_end']['size_bytes'])
        self.assertEqual(changed[0]['identity_start']['mtime_ns'], changed[0]['identity_end']['mtime_ns'])
        self.assertNotEqual(changed[0]['identity_start']['ctime_ns'], changed[0]['identity_end']['ctime_ns'])
        self.assertNotEqual(changed[0]['sha256_start'], changed[0]['sha256_end'])

    def test_source_identity_mutations_stop_before_next_replacement(self):
        source = self.sources['baseline']
        original = source.read_bytes()
        for change in ('same-size-restored-mtime', 'replacement-inode', 'mode'):
            with self.subTest(change=change):
                source.write_bytes(original)
                source.chmod(0o700)

                def mutation(_path, count):
                    if count != 12:
                        return
                    before = source.stat()
                    if change == 'same-size-restored-mtime':
                        source.write_bytes(bytes([original[0] ^ 1]) + original[1:])
                        os.utime(source, ns=(before.st_atime_ns, before.st_mtime_ns))
                    elif change == 'replacement-inode':
                        replacement = self.root / 'replacement-source'
                        replacement.write_bytes(original)
                        replacement.chmod(0o700)
                        replacement.replace(source)
                    else:
                        source.chmod(0o600)

                code, report, calls = self.run_model(mutation=mutation)
                self.assertEqual(code, 1)
                self.assertEqual(len(calls), 12)
                self.assertEqual(len(report['epochs']), 1)
                self.assertTrue(all(row['unchanged'] for row in report['epochs'][0]['copies']))
                self.assertTrue(any(not row['source_unchanged'] for row in report['epochs'][0]['copies']))
                self.assertTrue(any('Selected source identity' in error for error in report['errors']))

    def test_source_xattr_change_fails_without_an_identity_change(self):
        changed = False
        source = self.sources['baseline']
        real_xattrs = control.xattrs

        def observation(path):
            if Path(path) == source:
                return {'status': 'observed', 'values': [{'name': 'fixture.xattr', 'bytes': 1,
                        'sha256': 'changed' if changed else 'original'}]}
            return real_xattrs(path)

        def mutation(_path, count):
            nonlocal changed
            if count == 12:
                changed = True

        with mock.patch.object(control, 'xattrs', side_effect=observation):
            code, report, calls = self.run_model(mutation=mutation)
        self.assertEqual(code, 1)
        self.assertEqual(len(calls), 12)
        self.assertEqual(len(report['epochs']), 1)
        self.assertTrue(all(row['source_identity'] == row['source_identity_end']
                            for row in report['epochs'][0]['copies']))
        self.assertTrue(all(row['source_xattrs_unchanged'] is False
                            for row in report['epochs'][0]['copies']))

    def test_unused_source_xattr_change_in_same_binary_epoch_is_detected(self):
        changed = False
        unused_source = self.sources['candidate']
        real_xattrs = control.xattrs

        def observation(path):
            if Path(path) == unused_source:
                return {'status': 'observed', 'values': [{'name': 'fixture.xattr', 'bytes': 1,
                        'sha256': 'changed' if changed else 'original'}]}
            return real_xattrs(path)

        def mutation(_path, count):
            nonlocal changed
            if count == 12:
                changed = True

        with mock.patch.object(control, 'xattrs', side_effect=observation):
            code, report, calls = self.run_model(mutation=mutation)
        self.assertEqual(code, 1)
        self.assertEqual(len(calls), 12)
        self.assertEqual(len(report['epochs']), 1)
        epoch = report['epochs'][0]
        self.assertEqual(epoch['assignment_id'], 'baseline-aa')
        self.assertTrue(all(row['label'] == 'baseline' and row['unchanged'] and row['source_unchanged']
                            for row in epoch['copies']))
        self.assertEqual(set(epoch['source_observations_start']), set(benchmark.LABELS))
        self.assertEqual(set(epoch['source_observations_end']), set(benchmark.LABELS))
        self.assertFalse(epoch['source_observations_end']['candidate']['unchanged'])
        self.assertFalse(epoch['source_observations_end']['candidate']['xattrs_unchanged'])
        self.assertTrue(any('Selected source identity' in error and 'candidate' in error
                            for error in report['errors']))

    def test_epoch_xattr_observation_can_recover_from_initial_unavailability(self):
        changed = False
        observation_calls = 0
        unused_source = self.sources['candidate']
        real_xattrs = control.xattrs

        def observation(path):
            nonlocal observation_calls
            if Path(path) == unused_source:
                observation_calls += 1
                if observation_calls == 1:
                    return {'status': 'unavailable', 'values': [], 'reason': 'fixture initial denial'}
                return {'status': 'observed', 'values': [{'name': 'fixture.xattr', 'bytes': 1,
                        'sha256': 'changed' if changed else 'original'}]}
            return real_xattrs(path)

        def mutation(_path, count):
            nonlocal changed
            if count == 12:
                changed = True

        with mock.patch.object(control, 'xattrs', side_effect=observation):
            code, report, calls = self.run_model(mutation=mutation)
        self.assertEqual(code, 1)
        self.assertEqual(len(calls), 12)
        self.assertEqual(len(report['epochs']), 1)
        epoch = report['epochs'][0]
        self.assertEqual(epoch['source_observations_start']['candidate']['xattrs']['status'], 'observed')
        self.assertFalse(epoch['source_observations_end']['candidate']['unchanged'])
        self.assertFalse(epoch['source_observations_end']['candidate']['xattrs_unchanged'])

    def test_final_source_hash_remains_a_required_check(self):
        source = self.sources['baseline']
        captured = control.identity(source)
        original = source.read_bytes()
        real_identity = control.identity

        def observation(path):
            return captured if Path(path) == source else real_identity(path)

        def mutation(_path, count):
            if count == 96:
                source.write_bytes(bytes([original[0] ^ 1]) + original[1:])

        with mock.patch.object(control, 'identity', side_effect=observation):
            code, report, calls = self.run_model(mutation=mutation)
        self.assertEqual(code, 1)
        self.assertEqual(len(calls), 96)
        observed = next(row for row in report['protected_inputs'] if row['path'] == str(source))
        self.assertFalse(observed['unchanged'])
        self.assertNotEqual(observed['sha256_start'], observed['sha256_end'])
        self.assertEqual(report['status'], 'error')
        self.assertTrue(report['primary']['unchanged'])

    def test_source_disappearing_after_final_digest_does_not_prevent_report(self):
        source = self.sources['baseline']
        observed_hashes = 0
        real_hash = benchmark.executable_sha256

        def observation(path):
            nonlocal observed_hashes
            digest = real_hash(path)
            if Path(path) == source:
                observed_hashes += 1
                if observed_hashes == 2:
                    source.unlink()
            return digest

        with mock.patch.object(benchmark, 'executable_sha256', side_effect=observation):
            code, report, calls = self.run_model()
        self.assertEqual(code, 1)
        self.assertEqual(len(calls), 96)
        self.assertEqual(report['status'], 'error')
        protected = next(row for row in report['protected_inputs'] if row['path'] == str(source))
        self.assertFalse(protected['unchanged'])
        self.assertIsNone(protected['sha256_end'])
        self.assertTrue(report['primary']['unchanged'])
        self.assertIn('summaries', report)
        self.assertTrue(self.output.with_suffix('.md').exists())

    def test_missing_copy_or_source_during_finalization_preserves_partial_report(self):
        source = self.sources['baseline']
        original = source.read_bytes()
        for target in ('copy', 'source'):
            with self.subTest(target=target):
                source.write_bytes(original)
                source.chmod(0o700)

                def mutation(path, count):
                    if count == 12:
                        (path if target == 'copy' else source).unlink()

                code, report, calls = self.run_model(mutation=mutation)
                self.assertEqual(code, 1)
                self.assertEqual(len(calls), 12)
                self.assertEqual(report['status'], 'error')
                self.assertEqual(report['epochs'][0]['status'], 'incomplete')
                self.assertFalse(report['completion']['schedule_complete'])
                self.assertTrue(report['primary']['unchanged'])
                self.assertTrue(self.output.with_suffix('.md').exists())

    def test_hash_inventory_keeps_owned_boundaries_and_removes_epoch_source_reads(self):
        hashed = []
        real_hash = benchmark.executable_sha256

        def observe(path):
            hashed.append(str(Path(path).resolve()))
            return real_hash(path)

        with mock.patch.object(benchmark, 'executable_sha256', side_effect=observe):
            code, report, _ = self.run_model()
        self.assertEqual(code, 0)
        for source in self.sources.values():
            self.assertEqual(hashed.count(str(source)), 2)
        owned = {row['isolated_path'] for epoch in report['epochs'] for row in epoch['copies']}
        self.assertEqual(len(owned), 4)
        self.assertEqual(Counter(path for path in hashed if path in owned), dict.fromkeys(owned, 16))
        self.assertTrue(all('source_sha256_end' not in row
                            for epoch in report['epochs'] for row in epoch['copies']))

    def test_checkpoints_do_not_recompute_final_summaries(self):
        with mock.patch.object(control, 'summaries', wraps=control.summaries) as describe:
            code, report, _ = self.run_model()
        self.assertEqual(code, 0)
        self.assertEqual(describe.call_count, 1)
        self.assertIn('cohorts', report['summaries'])

    def test_whole_deadline_expiring_in_setup_never_launches(self):
        code, report, calls = self.run_model(extra=('--budget', '0.000000001'))
        self.assertEqual(code, 1)
        self.assertEqual(calls, [])
        self.assertTrue(any('deadline' in error for error in report['errors']))


class TimingTests(unittest.TestCase):
    def result(self):
        values = [10 + offset for offset in (0, 0, .0001, .0002, .0008, .0009,
                                              .001, .0011, .0012, .01, .0101, .0102)]
        return {'seconds': values[9] - values[0], 'parent_timing': [values[0], values[6], values[9]],
                'command_timing': values}

    def test_timing_inventory_and_invalid_models(self):
        result = self.result()
        stages = control.timing_stages(result)
        self.assertAlmostEqual(stages['popen_seconds'], .0006)
        self.assertAlmostEqual(stages['waiter_dispatch_seconds'], .0001)
        for edit in (lambda row: row.update(seconds=float('nan')),
                     lambda row: row.update(seconds=0),
                     lambda row: row.update(seconds=.02),
                     lambda row: row.update(parent_timing=[10, 11, 12]),
                     lambda row: row['command_timing'].pop(),
                     lambda row: row['command_timing'].__setitem__(4, float('inf')),
                     lambda row: row['command_timing'].__setitem__(4, 9)):
            value = copy.deepcopy(result)
            edit(value)
            with self.assertRaises(ValueError):
                control.timing_stages(value)

    def test_failed_supervision_retains_available_launch_and_cleanup(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            supervisor = control.ControlSupervisor(root, .1, time.monotonic() + 5)
            self.addCleanup(supervisor.close)
            with self.assertRaises(benchmark.BenchmarkError):
                supervisor.execute([sys.executable, '-c', 'import time; time.sleep(5)'], root, os.environ)
            evidence = supervisor.failed_evidence()
            self.assertIn('popen_return', evidence['partial_timing'])
            self.assertNotIn('completion', evidence['partial_timing'])
            self.assertIn('completion', evidence['unavailable_timing_fields'])
            self.assertIn('cleanup_end', evidence['partial_timing'])
            self.assertFalse(supervisor.owned)


class NativeSmokeTests(FixtureFiles):
    def test_tiny_native_executables_full_eight_assignments(self):
        compiler = shutil.which('cc')
        if not compiler:
            self.skipTest('Native C compiler unavailable')
        source = r'''#include <stdio.h>
#include <stdlib.h>
#include <string.h>
int main(int argc, char **argv) {
    if (getenv("TEKAI_DIAGNOSTIC_PROFILE")) return 2;
    if (argc != 2 || strcmp(argv[1], "--version")) return 3;
    puts("tekai " VERSION); return 0;
}
'''
        for label, path in self.sources.items():
            completed = subprocess.run([compiler, '-x', 'c', '-', '-DVERSION="' + self.metadata[label]['expected_version'] + '"',
                                        '-o', str(path)], input=source, text=True, capture_output=True, timeout=30)
            self.assertEqual(completed.returncode, 0, completed.stderr)
        self.bind_sources()
        primary = self.primary_path.read_bytes()
        with mock.patch.dict(os.environ, {'TEKAI_DIAGNOSTIC_PROFILE': '1'}):
            self.assertEqual(control.main(self.argv('--repeats', '1')), 0)
        report = self.read_report()
        self.assertEqual(len(report['epochs']), 8)
        self.assertEqual(len(report['commands']), 64)
        self.assertTrue(all(row['status'] == 'verified' for row in report['commands']))
        self.assertTrue(all(row['unchanged'] for epoch in report['epochs'] for row in epoch['copies']))
        self.assertEqual(self.primary_path.read_bytes(), primary)


if __name__ == '__main__':
    unittest.main()
