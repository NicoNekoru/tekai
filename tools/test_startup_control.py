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

        def execute(supervisor, command, cwd, env):
            supervisor.counter += 1
            path = str(command[0])
            label = next(label for label, source in self.sources.items()
                         if benchmark.executable_sha256(Path(path)) == benchmark.executable_sha256(source))
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
        self.assertIn('No A/B pooled ratio', report['summaries']['scope'])

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
