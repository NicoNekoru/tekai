"""Test audit-runner permissions and cleanup without launching a compiler."""

import contextlib
import io
import os
from pathlib import Path
import signal
import subprocess
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import Mock, patch

from audit_runtime import Audit, stop_group


class AuditRunnerTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix='tekai-audit-runner-test-')
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name).resolve()
        self.args = SimpleNamespace(engine=Path('/unused/tekai'), quick=True,
                                    timeout=1, report=self.work / 'report.json')
        with patch('audit_runtime.subprocess.check_output', return_value='Test CPU\n'):
            self.audit = Audit(self.args, self.work)

    def test_cpu_metadata_permission_failure_keeps_architecture(self):
        with patch('audit_runtime.subprocess.check_output', side_effect=PermissionError()), \
                patch('audit_runtime.platform.machine', return_value='arm64'):
            audit = Audit(self.args, self.work)
        self.assertEqual(audit.report['machine'], 'arm64')

    def test_caches_are_fixture_scoped_and_search_overrides_are_cleared(self):
        with patch.dict(os.environ, {'TEXINPUTS': '/unused/shared/tree'}), \
                patch('audit_runtime.subprocess.check_output', return_value='Test CPU'):
            audit = Audit(self.args, self.work)
        self.assertNotIn('TEXINPUTS', audit.env)
        self.assertEqual(Path(audit.env['TEKAI_ENGINE_CACHE']), self.work / 'runtime-cache')
        env = audit.environment(self.work / 'project')
        for key in ('TEKAI_AUX_CACHE', 'TEKAI_BIBTEX_CACHE', 'TEKAI_FORMAT_CACHE'):
            self.assertTrue(Path(env[key]).is_relative_to(self.work / 'caches/project'))

    def test_process_inspection_denial_records_skip(self):
        with patch('audit_runtime.subprocess.check_output', side_effect=PermissionError()), \
                contextlib.redirect_stdout(io.StringIO()):
            self.assertFalse(self.audit.process_inspection_available('cancel'))
        row = self.audit.report['results'][-1]
        self.assertEqual(row['case'], 'cancel')
        self.assertTrue(row['skipped'])

    def test_denied_process_probe_starts_no_fixture(self):
        with patch.object(self.audit, 'process_inspection_available', return_value=False), \
                patch.object(self.audit, 'project') as project:
            self.audit.cancel()
            self.audit.aux_concurrency()
        project.assert_not_called()

    def test_missing_rss_is_unavailable_rather_than_zero(self):
        process = Mock(returncode=0)
        process.communicate.return_value = ('output', 'time: Operation not permitted')
        with patch.object(self.audit, 'start', return_value=process), \
                patch('audit_runtime.stop_group') as stop:
            result = self.audit.run(['unused'], self.work, measured=True)
        self.assertIsNone(result['peak_mib'])
        self.assertFalse(result['rss_available'])
        stop.assert_called_once_with(process)

    def test_rss_units_and_success_cleanup(self):
        process = Mock(returncode=0)
        process.communicate.return_value = ('', '1048576 maximum resident set size')
        with patch.object(self.audit, 'start', return_value=process), \
                patch('audit_runtime.stop_group') as stop:
            result = self.audit.run(['unused'], self.work, measured=True)
        self.assertEqual(result['peak_mib'], 1)
        self.assertTrue(result['rss_available'])
        stop.assert_called_once_with(process)

    def test_timeout_always_cleans_up_the_owned_group(self):
        process = Mock()
        process.communicate.side_effect = subprocess.TimeoutExpired('unused', 1)
        with patch.object(self.audit, 'start', return_value=process), \
                patch('audit_runtime.stop_group') as stop:
            result = self.audit.run(['unused'], self.work)
        self.assertTrue(result['timeout'])
        stop.assert_called_once_with(process)

    def test_cleanup_targets_only_the_process_group_it_started(self):
        process = Mock(pid=12345)
        with patch('audit_runtime.os.killpg') as kill:
            stop_group(process)
        kill.assert_called_once_with(12345, signal.SIGKILL)
        process.communicate.assert_called_once()

    def test_cleanup_reaps_parent_if_group_already_exited(self):
        process = Mock(pid=12345)
        with patch('audit_runtime.os.killpg', side_effect=ProcessLookupError()):
            stop_group(process)
        process.communicate.assert_called_once()


if __name__ == '__main__':
    unittest.main()
