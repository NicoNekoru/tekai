"""Test audit-runner permissions and cleanup without launching a compiler."""

import contextlib
import io
import os
from pathlib import Path
import signal
import subprocess
import sys
import tempfile
import time
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
        with patch.object(Audit, 'run', return_value={'code': 0, 'stdout': 'Test CPU\n'}):
            self.audit = Audit(self.args, self.work)
        self.audit.rss_probe = {'available': True, 'stderr_tail': ''}
        captures = patch.object(self.audit, 'captured', side_effect=lambda process: (*process.captured_output, False))
        captures.start()
        self.addCleanup(captures.stop)

    def test_cpu_metadata_permission_failure_keeps_architecture(self):
        with patch.object(Audit, 'run', side_effect=PermissionError()), \
                patch('audit_runtime.platform.machine', return_value='arm64'):
            audit = Audit(self.args, self.work)
        self.assertEqual(audit.report['machine'], 'arm64')

    def test_caches_are_fixture_scoped_and_search_overrides_are_cleared(self):
        with patch.dict(os.environ, {'TEXINPUTS': '/unused/shared/tree'}), \
                patch.object(Audit, 'run', return_value={'code': 0, 'stdout': 'Test CPU'}):
            audit = Audit(self.args, self.work)
        self.assertNotIn('TEXINPUTS', audit.env)
        self.assertEqual(Path(audit.env['TEKAI_ENGINE_CACHE']), self.work / 'runtime-cache')
        env = audit.environment(self.work / 'project')
        for key in ('TEKAI_AUX_CACHE', 'TEKAI_BIBTEX_CACHE', 'TEKAI_FORMAT_CACHE'):
            self.assertTrue(Path(env[key]).is_relative_to(self.work / 'caches/project'))

    def test_process_inspection_denial_records_skip(self):
        with patch.object(self.audit, 'run', side_effect=PermissionError()), \
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
        process = Mock(returncode=0, completion=time.monotonic())
        process.captured_output = ('output', 'time: Operation not permitted')
        with patch.object(self.audit, 'start', return_value=process), \
                patch('audit_runtime.stop_group') as stop:
            result = self.audit.run(['unused'], self.work, measured=True)
        self.assertIsNone(result['peak_mib'])
        self.assertFalse(result['rss_available'])
        stop.assert_called_once_with(process)

    def test_rss_units_and_success_cleanup(self):
        process = Mock(returncode=0, completion=time.monotonic())
        process.captured_output = ('', '1048576 maximum resident set size')
        with patch.object(self.audit, 'start', return_value=process), \
                patch('audit_runtime.stop_group') as stop:
            result = self.audit.run(['unused'], self.work, measured=True)
        self.assertEqual(result['peak_mib'], 1)
        self.assertTrue(result['rss_available'])
        stop.assert_called_once_with(process)

    def test_denied_timer_does_not_replace_command_exit_status(self):
        self.audit.rss_probe = {'available': False, 'stderr_tail': 'sysctl denied'}
        process = Mock(returncode=0, completion=time.monotonic())
        process.captured_output = ('output', '')
        with patch.object(self.audit, 'start', return_value=process) as start, \
                patch('audit_runtime.stop_group'):
            result = self.audit.run(['unused'], self.work, measured=True)
        self.assertFalse(start.call_args.args[-1])
        self.assertEqual(result['code'], 0)
        self.assertEqual(result['status_source'], 'command')
        self.assertFalse(result['rss_available'])
        self.assertEqual(result['rss_unavailable_reason'], 'sysctl denied')

    def test_timer_permission_probe_is_cached_and_cleans_up(self):
        self.audit.rss_probe = None
        process = Mock(returncode=1)
        process.captured_output = ('', 'sysctl denied')
        with patch.object(self.audit, 'start', return_value=process) as start, \
                patch('audit_runtime.stop_group') as stop:
            self.assertFalse(self.audit.rss_timer_available())
            self.assertFalse(self.audit.rss_timer_available())
        start.assert_called_once()
        stop.assert_called_once_with(process)

    def test_timer_launch_denial_uses_direct_measurement(self):
        self.audit.rss_probe = None
        with patch.object(self.audit, 'start', side_effect=PermissionError()), \
                patch('audit_runtime.stop_group') as stop:
            self.assertFalse(self.audit.rss_timer_available())
        stop.assert_not_called()

    def test_timeout_always_cleans_up_the_owned_group(self):
        process = Mock()
        process.wait_for_exit.side_effect = subprocess.TimeoutExpired('unused', 1)
        with patch.object(self.audit, 'start', return_value=process), \
                patch('audit_runtime.stop_group') as stop:
            result = self.audit.run(['unused'], self.work)
        self.assertTrue(result['timeout'])
        stop.assert_called_once_with(process)

    def test_cleanup_targets_only_the_process_group_it_started(self):
        process = Mock(pid=12345)
        stop_group(process)
        process.finish.assert_called_once_with()

    def test_cleanup_reaps_parent_if_group_already_exited(self):
        process = Mock(pid=12345)
        stop_group(process)
        process.finish.assert_called_once_with()


@unittest.skipUnless(os.name == 'posix' and hasattr(os, 'waitid'), 'POSIX waitid required')
class NativeCaptureTests(unittest.TestCase):
    def test_escaped_output_descriptor_cannot_hold_audit_capture_open(self):
        with tempfile.TemporaryDirectory(prefix='test-audit-native-') as temporary:
            root = Path(temporary)
            args = SimpleNamespace(engine=Path(sys.executable), quick=True, timeout=1,
                                   report=root / 'report.json')
            with patch.object(Audit, 'run', return_value={'code': 0, 'stdout': 'Test CPU'}):
                audit = Audit(args, root)
            read_lease, write_lease = os.pipe()
            native = subprocess.Popen

            def launch(*args, **kwargs):
                return native(*args, pass_fds=(read_lease,), **kwargs)

            # The test owns the lease, not a borrowed numeric PID. Closing it
            # stops the escaped fixture without PID-tree snapshots or signals.
            code = ('import os,select\n'
                    'pid=os.fork()\n'
                    'if pid == 0:\n'
                    ' os.setsid()\n'
                    f' select.select([{read_lease}],[],[],5)\n'
                    'else:\n print("parent output",flush=True)\n')
            try:
                started = time.monotonic()
                with patch('audit_runtime.subprocess.Popen', side_effect=launch):
                    result = audit.run([sys.executable, '-c', code], root, os.environ.copy())
                self.assertEqual(result['code'], 0)
                self.assertEqual(result['stdout'], 'parent output\n')
                self.assertLess(time.monotonic() - started, 2)
                self.assertFalse(audit.owned)
            finally:
                os.close(write_lease)
                os.close(read_lease)
                audit.close()


if __name__ == '__main__':
    unittest.main()
