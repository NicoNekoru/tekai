"""Filesystem and state-transition tests. No Git, Cargo or native executables."""

import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import performance_build as build


class BuildTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix='test-performance-build-')
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name).resolve()
        self.workspace = self.root / 'workspace'
        self.workspace.mkdir()
        self.report = self.root / 'tekai-performance-reports'
        self.report.mkdir()
        self.target = self.root / 'tekai-performance-target'
        self.frozen = self.root / 'tekai-performance-baseline-target'
        self.baseline, self.candidate = 'a' * 40, 'b' * 40
        self.cwd = patch.object(Path, 'cwd', return_value=self.workspace)
        self.cwd.start()
        self.addCleanup(self.cwd.stop)
        self.environment = {
            'CARGO_INCREMENTAL': '0', 'CARGO_BUILD_JOBS': '2', 'RUSTUP_TOOLCHAIN': 'pinned-toolchain',
            'CARGO_PROFILE_RELEASE_CODEGEN_UNITS': '1', 'CARGO_PROFILE_RELEASE_LTO': 'fat',
            'CARGO_PROFILE_RELEASE_PANIC': 'abort',
        }
        self.session = self.prepare()

    def prepare(self, **overrides):
        args = dict(report=self.report, root=self.root, workspace=self.workspace, target=self.target,
                    frozen=self.frozen, baseline=self.baseline, candidate=self.candidate,
                    current=self.candidate, runner_label='synthetic-runner', baseline_policy='explicit-sha')
        args.update(overrides)
        return build.Session.prepare(**args)

    def artifact(self, contents=b'Never execute this synthetic artifact'):
        path = self.target / 'release' / 'tekai'
        path.parent.mkdir(parents=True)
        path.write_bytes(contents)
        path.chmod(0o700)
        return path

    def start(self, role='baseline', **overrides):
        args = dict(role=role, revision=getattr(self, role), toolchain='actual synthetic toolchain',
                    environment=self.environment)
        args.update(overrides)
        return self.session.start(**args)

    def finish(self, role='baseline', **overrides):
        args = dict(role=role, revision=getattr(self, role), cargo_exit=0, log_exit=0)
        args.update(overrides)
        return self.session.finish(**args)

    def baseline_ready(self):
        self.start()
        executable = self.artifact(b'baseline bytes')
        expected = build.sha256(executable)
        self.finish()
        state = self.session.isolate()
        return expected, state

    def both_ready(self):
        expected, state = self.baseline_ready()
        self.start('candidate')
        self.artifact(b'candidate bytes')
        self.finish('candidate')
        return expected, state

    def test_complete_sequence_preserves_tree_and_identical_build_paths(self):
        self.start()
        executable = self.artifact(b'baseline bytes')
        expected = build.sha256(executable)
        extra = self.target / 'release' / 'deps' / 'retained-object'
        extra.parent.mkdir()
        extra.write_bytes(b'whole tree must move')
        original_inode = self.target.stat().st_ino
        self.finish()
        state = self.session.isolate()
        self.assertEqual(self.frozen.stat().st_ino, original_inode)
        self.assertEqual((self.frozen / extra.relative_to(self.target)).read_bytes(), b'whole tree must move')
        self.assertFalse(self.target.exists())
        self.assertTrue(state['baseline_verified'])
        self.start('candidate')
        self.artifact(b'candidate bytes')
        self.finish('candidate')
        metadata = self.session.metadata()
        self.assertEqual(build.sha256(self.frozen / 'release' / 'tekai'), expected)
        for role in ('baseline', 'candidate'):
            self.assertEqual(metadata[role]['build_workspace'], str(self.workspace))
            self.assertEqual(metadata[role]['build_target_dir'], str(self.target))
            self.assertEqual(metadata[role]['build_artifact_path'], str(self.target / 'release' / 'tekai'))
            self.assertEqual(metadata[role]['build_command'], build.COMMAND)
        self.assertEqual(metadata['baseline']['artifact_path'], str(self.frozen / 'release' / 'tekai'))
        self.assertEqual(metadata['candidate']['artifact_path'], str(self.target / 'release' / 'tekai'))
        self.assertNotEqual(metadata['baseline']['artifact_sha256'], metadata['candidate']['artifact_sha256'])

    def test_failed_partial_build_is_quarantined_before_candidate_build(self):
        self.start()
        partial = self.target / 'release' / 'deps' / 'partial'
        partial.parent.mkdir(parents=True)
        partial.write_bytes(b'failed build residue')
        record = self.finish(cargo_exit=101)
        self.assertFalse(record['successful'])
        self.assertTrue(record['command_finished'])
        state = self.session.isolate()
        self.assertTrue(state['candidate_target_fresh'])
        self.assertFalse(state['baseline_verified'])
        self.assertFalse(self.target.exists())
        self.assertEqual((self.frozen / partial.relative_to(self.target)).read_bytes(), b'failed build residue')
        self.start('candidate')
        self.artifact()
        self.finish('candidate')
        with self.assertRaises(ValueError):
            self.session.metadata()

    def test_baseline_checkout_failure_without_build_preserves_candidate_correctness(self):
        state = self.session.isolate()
        self.assertTrue(state['candidate_target_fresh'])
        self.assertFalse(state['target_moved'])
        self.start('candidate')
        self.artifact()
        self.assertTrue(self.finish('candidate')['successful'])
        with self.assertRaises((OSError, ValueError)):
            self.session.metadata()

    def test_started_build_without_completion_cannot_rotate_or_start_candidate(self):
        self.start()
        self.artifact()
        with self.assertRaises(ValueError):
            self.session.isolate()
        self.assertTrue(self.target.exists())
        self.assertFalse(self.frozen.exists())
        with self.assertRaises(ValueError):
            self.start('candidate')

    def test_signal_exit_completion_does_not_establish_writer_cleanup(self):
        self.start()
        self.artifact()
        self.finish(cargo_exit=143)
        with self.assertRaises(ValueError):
            self.session.isolate()
        self.assertTrue(self.target.exists())

    def test_missing_successful_executable_still_permits_fresh_candidate(self):
        self.start()
        self.assertFalse(self.finish()['successful'])
        self.assertTrue(self.session.isolate()['candidate_target_fresh'])
        self.start('candidate')
        self.artifact()
        self.assertTrue(self.finish('candidate')['successful'])

    def test_frozen_hash_disagreement_cannot_pass_metadata_but_candidate_can_run(self):
        self.start()
        executable = self.artifact()
        self.finish()
        executable.write_bytes(b'changed after completion')
        state = self.session.isolate()
        self.assertTrue(state['candidate_target_fresh'])
        self.assertFalse(state['baseline_verified'])
        self.start('candidate')
        self.artifact()
        self.finish('candidate')
        with self.assertRaises(ValueError):
            self.session.metadata()

    def test_current_artifact_changes_prevent_comparison(self):
        self.both_ready()
        executable = self.frozen / 'release' / 'tekai'
        executable.write_bytes(b'replaced frozen artifact')
        with self.assertRaises(ValueError):
            self.session.metadata()

    def test_metadata_rejects_boolean_pipeline_status_in_either_role(self):
        self.both_ready()
        for role in ('baseline', 'candidate'):
            path = self.report / (role + '-finished.json')
            original = path.read_text(encoding='utf-8')
            for key in ('cargo_exit', 'log_exit'):
                with self.subTest(role=role, key=key):
                    record = json.loads(original)
                    record[key] = False
                    path.write_text(json.dumps(record), encoding='utf-8')
                    with self.assertRaises(ValueError):
                        self.session.metadata()
                    path.write_text(original, encoding='utf-8')

    def test_restoration_failure_cannot_start_baseline_sources_as_candidate(self):
        self.baseline_ready()
        with self.assertRaises(ValueError):
            self.start('candidate', revision=self.baseline)
        self.assertFalse((self.report / 'candidate-start.json').exists())

    def test_preflight_failure_has_no_session_or_build_permission(self):
        empty = self.root / 'empty-report'
        empty.mkdir()
        with self.assertRaises(ValueError):
            build.Session.load(empty)
        self.assertFalse(self.target.exists())

    def test_source_residue_is_retained_and_rejected(self):
        residue = self.workspace / 'untracked-source'
        residue.write_bytes(b'preserve diagnostics')
        with self.assertRaises(ValueError):
            self.start(source_clean=False)
        self.assertEqual(residue.read_bytes(), b'preserve diagnostics')
        self.start()
        self.artifact()
        self.assertFalse(self.finish(source_clean=False)['successful'])
        self.assertTrue(self.session.isolate()['candidate_target_fresh'])
        with self.assertRaises(ValueError):
            self.start('candidate', source_clean=False)

    def test_wrong_workspace_is_rejected(self):
        with patch.object(Path, 'cwd', return_value=self.root):
            with self.assertRaises(ValueError):
                self.start()

    def test_target_destination_collision_is_not_overwritten(self):
        self.start()
        self.artifact()
        self.finish()
        self.frozen.mkdir()
        sentinel = self.frozen / 'sentinel'
        sentinel.write_bytes(b'preserve collision')
        with self.assertRaises(ValueError):
            self.session.isolate()
        self.assertEqual(sentinel.read_bytes(), b'preserve collision')
        self.assertTrue(self.target.exists())

    def test_dangling_frozen_symlink_is_not_followed(self):
        self.start()
        self.artifact()
        self.finish()
        destination = self.root / 'absent-symlink-target'
        self.frozen.symlink_to(destination, target_is_directory=True)
        with self.assertRaises(ValueError):
            self.session.isolate()
        self.assertFalse(destination.exists())
        self.assertTrue(self.target.exists())

    def test_canonical_target_symlink_is_not_moved(self):
        self.start()
        outside = self.root / 'unowned-directory'
        outside.mkdir()
        self.target.symlink_to(outside, target_is_directory=True)
        self.finish(cargo_exit=101)
        with self.assertRaises(ValueError):
            self.session.isolate()
        self.assertTrue(self.target.is_symlink())
        self.assertTrue(outside.exists())

    def test_target_without_a_start_is_not_claimed(self):
        self.target.mkdir()
        with self.assertRaises(ValueError):
            self.session.isolate()
        self.assertTrue(self.target.exists())

    def test_rename_failure_prevents_candidate_reuse(self):
        self.start()
        self.artifact()
        self.finish()
        with patch.object(Path, 'rename', side_effect=OSError('mock rename failure')):
            with self.assertRaises(OSError):
                self.session.isolate()
        self.assertTrue(self.target.exists())
        self.assertFalse(build.read(self.report / 'target-isolation.json')['candidate_target_fresh'])

    def test_alias_executable_is_not_published(self):
        self.start()
        outside = self.root / 'outside-executable'
        outside.write_bytes(b'outside')
        outside.chmod(0o700)
        path = self.target / 'release' / 'tekai'
        path.parent.mkdir(parents=True)
        path.symlink_to(outside)
        self.assertFalse(self.finish()['successful'])
        self.session.isolate()
        self.assertEqual(outside.read_bytes(), b'outside')

    def test_toolchain_or_settings_difference_cannot_pass(self):
        self.baseline_ready()
        self.start('candidate', toolchain='different compiler')
        self.artifact()
        self.finish('candidate')
        with self.assertRaises(ValueError):
            self.session.metadata()

    def test_start_rejects_different_release_settings(self):
        environment = dict(self.environment, CARGO_PROFILE_RELEASE_LTO='thin')
        with self.assertRaises(ValueError):
            self.start(environment=environment)

    def test_prepare_rejects_non_owned_paths_and_preserves_existing_records(self):
        original = (self.report / 'build-session.json').read_bytes()
        with self.assertRaises(ValueError):
            self.prepare(target=self.root / 'wrong-target')
        with self.assertRaises(FileExistsError):
            self.prepare()
        self.assertEqual((self.report / 'build-session.json').read_bytes(), original)

    def test_finished_record_provenance_change_prevents_rotation(self):
        self.start()
        self.artifact()
        self.finish()
        path = self.report / 'baseline-finished.json'
        record = build.read(path)
        record['revision'] = self.candidate
        path.write_text(json.dumps(record), encoding='utf-8')
        with self.assertRaises(ValueError):
            self.session.isolate()
        self.assertTrue(self.target.exists())


if __name__ == '__main__':
    unittest.main()
