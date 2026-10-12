"""Unreaped-leader authority and bounded CLI cleanup, without TeX."""

import ctypes
import errno
import os
from pathlib import Path
import signal
import subprocess
import sys
import tempfile
import threading
import time
from types import SimpleNamespace
import unittest
from unittest.mock import Mock, patch

import owned_process as owned


def status(pid, code=0):
    return SimpleNamespace(si_pid=pid, si_code=os.CLD_EXITED, si_status=code)


@unittest.skipUnless(os.name == 'posix' and hasattr(os, 'waitid'), 'POSIX waitid required')
class AuthorityTests(unittest.TestCase):
    def setUp(self):
        grace = patch.object(owned, 'TERM_GRACE_SECONDS', 0)
        grace.start()
        self.addCleanup(grace.stop)
        self.native = Mock(pid=12345, args=['fixture'], returncode=None)
        self.native.wait.return_value = 7
        self.process = owned.OwnedProcess(self.native)

    def test_status_is_observed_without_reaping_and_cleanup_precedes_only_wait(self):
        events = []
        self.native.wait.side_effect = lambda **kwargs: events.append(('reap', kwargs)) or 7
        with patch.object(owned.os, 'waitid', side_effect=lambda *args: events.append(('observe', args)) or status(12345, 7)), \
                patch.object(owned.os, 'killpg', side_effect=lambda *args: events.append(('signal', args))):
            self.assertEqual(self.process.poll_exit(), 7)
            self.assertEqual(self.process.wait_for_exit().code, 7)
            self.native.wait.assert_not_called()
            self.assertIsNone(self.native.returncode)
            self.assertEqual(self.process.finish(), 7)
            self.assertEqual(self.process.finish(), 7)
            self.assertEqual(self.process.poll_exit(), 7)
        self.assertEqual([event[0] for event in events].count('reap'), 1)
        self.assertLess(next(i for i, event in enumerate(events) if event[0] == 'signal'),
                        next(i for i, event in enumerate(events) if event[0] == 'reap'))
        self.assertTrue(all(args[2] & os.WNOWAIT for event, args in events if event == 'observe'))

    def test_echild_revokes_signalling_even_after_a_cached_exit(self):
        with patch.object(owned.os, 'waitid', side_effect=[status(12345, 7), ChildProcessError(errno.ECHILD, 'already reaped')]), \
                patch.object(owned.os, 'killpg') as kill:
            self.assertEqual(self.process.poll_exit(), 7)
            for action in (self.process.finish, self.process.terminate, self.process.finish):
                with self.assertRaises(owned.OwnershipLost):
                    action()
            kill.assert_not_called()
            self.native.wait.assert_not_called()

    def test_permission_failure_retains_authority_and_does_not_reap(self):
        with patch.object(owned.os, 'waitid', return_value=status(12345, 7)), \
                patch.object(owned, '_signal_owned_group', side_effect=PermissionError):
            with self.assertRaises(PermissionError):
                self.process.finish()
            self.native.wait.assert_not_called()
        with patch.object(owned.os, 'waitid', return_value=status(12345, 7)), \
                patch.object(owned, '_signal_owned_group'):
            self.assertEqual(self.process.finish(), 7)

    def test_observer_publication_barrier_runs_before_actual_reap(self):
        order = []
        self.process.reap_barrier = lambda timeout: order.append(('publish', timeout)) or True
        self.native.wait.side_effect = lambda **kwargs: order.append(('reap', kwargs)) or 7
        with patch.object(owned.os, 'waitid', return_value=status(12345, 7)), \
                patch.object(owned, '_signal_owned_group'):
            self.process.finish()
        self.assertEqual([event for event, _ in order], ['publish', 'reap'])

    def test_failed_observer_barrier_retains_unreaped_leader(self):
        self.process.reap_barrier = lambda _: False
        with patch.object(owned.os, 'waitid', return_value=status(12345, 7)), \
                patch.object(owned, '_signal_owned_group'):
            with self.assertRaisesRegex(RuntimeError, 'publish'):
                self.process.finish()
            self.native.wait.assert_not_called()

    def test_signal_status_is_not_confused_with_normal_exit(self):
        value = SimpleNamespace(si_pid=12345, si_code=os.CLD_KILLED, si_status=signal.SIGTERM)
        with patch.object(owned.os, 'waitid', return_value=value):
            self.assertEqual(self.process.wait_for_exit().code, -signal.SIGTERM)

    def test_signal_check_serializes_known_loss_publication(self):
        entered, release, generated = threading.Event(), threading.Event(), threading.Event()
        failures = []

        def waitid(_kind, _pid, flags):
            if not flags & os.WNOHANG:
                entered.set()
                release.wait(2)
                generated.set()
                raise ChildProcessError(errno.ECHILD, 'controlled lost reservation')
            return status(12345, 7)

        def observe():
            try:
                self.process.wait_for_exit()
            except owned.OwnershipLost as error:
                failures.append(error)

        def signal_group(*_args):
            # A blocking observer is ready to publish a loss. The signal pair
            # holds the routing lock, so it cannot signal after a published loss.
            self.assertTrue(self.process._condition._is_owned())
            release.set()
            self.assertTrue(generated.wait(2))
            self.assertFalse(self.process._lost)
            return False

        with patch.object(owned.os, 'waitid', side_effect=waitid), \
                patch.object(owned, '_signal_owned_group', side_effect=signal_group) as send:
            waiter = threading.Thread(target=observe)
            waiter.start()
            self.assertTrue(entered.wait(2))
            self.process.terminate()
            waiter.join(2)
            self.assertFalse(waiter.is_alive())
            self.assertEqual(len(failures), 1)
            with self.assertRaises(owned.OwnershipLost):
                self.process.terminate()
            self.assertEqual(send.call_count, 1)


class DarwinClassificationTests(unittest.TestCase):
    def library(self, lists, records):
        library = Mock()
        outputs = iter(lists)

        def listing(selector, group, pointer, size):
            self.assertEqual(selector, 2)
            self.assertEqual(group, 12345)
            self.assertEqual(size, owned.MAX_GROUP_MEMBERS * ctypes.sizeof(ctypes.c_int))
            values = next(outputs)
            if isinstance(values, int):
                return values
            ids = ctypes.cast(pointer, ctypes.POINTER(ctypes.c_int))
            for index, pid in enumerate(values):
                ids[index] = pid
            return len(values) * ctypes.sizeof(ctypes.c_int)

        def info(pid, flavor, include_zombies, pointer, size):
            self.assertEqual((flavor, include_zombies, size), (3, 1, 136))
            record = ctypes.cast(pointer, ctypes.POINTER(owned._DarwinBSDInfo)).contents
            returned_pid, group, state = records[pid]
            record.pbi_pid, record.pbi_pgid, record.pbi_status = returned_pid, group, state
            return size

        library.proc_listpids.side_effect = listing
        library.proc_pidinfo.side_effect = info
        return library

    def test_exact_zombies_require_two_complete_identical_group_lists(self):
        library = self.library([[12345, 12346], [12346, 12345]],
                               {12345: (12345, 12345, 5), 12346: (12346, 12345, 5)})
        with patch.object(owned.ctypes, 'CDLL', return_value=library):
            self.assertTrue(owned._darwin_only_zombies(12345))
        self.assertEqual(library.proc_listpids.call_count, 2)

    def test_malformed_truncated_or_changed_lists_fail_closed(self):
        for lists in ([0], [-1], [3], [4096], [[12345, 12345]], [[-1]], [[12345], [12345, 12346]]):
            library = self.library(lists, {12345: (12345, 12345, 5)})
            with self.subTest(lists=lists), patch.object(owned.ctypes, 'CDLL', return_value=library):
                self.assertFalse(owned._darwin_only_zombies(12345))

    def test_inexact_or_live_record_does_not_suppress_permission_error(self):
        for record in ((12346, 12345, 5), (12345, 12346, 5), (12345, 12345, 2), (12345, 12345, 0)):
            library = self.library([[12345]], {12345: record})
            with self.subTest(record=record), patch.object(owned.ctypes, 'CDLL', return_value=library):
                self.assertFalse(owned._darwin_only_zombies(12345))
        with patch.object(owned.os, 'killpg', side_effect=PermissionError), \
                patch.object(owned.sys, 'platform', 'darwin'), \
                patch.object(owned, '_darwin_only_zombies', return_value=False):
            with self.assertRaises(PermissionError):
                owned._signal_owned_group(12345, signal.SIGKILL)


@unittest.skipUnless(os.name == 'posix' and hasattr(os, 'waitid'), 'POSIX waitid required')
class NativeOwnershipTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix='test-owned-process-')
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)

    def launch(self, code, **kwargs):
        log = tempfile.TemporaryFile()
        self.addCleanup(log.close)
        child = subprocess.Popen([sys.executable, '-c', code], start_new_session=True,
                                 stdout=log, stderr=log, **kwargs)
        process = owned.OwnedProcess(child)
        self.addCleanup(process.finish)
        return process

    def test_real_completed_leader_is_unreaped_until_finish(self):
        process = self.launch('raise SystemExit(7)')
        observation = process.wait_for_exit(2)
        self.assertEqual(observation.code, 7)
        self.assertIsNone(process._process.returncode)
        self.assertEqual(os.waitid(os.P_PID, process.pid, os.WEXITED | os.WNOWAIT).si_status, 7)
        if sys.platform == 'darwin':
            self.assertTrue(owned._darwin_only_zombies(process.pid))
        self.assertEqual(process.finish(), 7)
        with self.assertRaises(ChildProcessError):
            os.waitid(os.P_PID, process.pid, os.WEXITED | os.WNOWAIT | os.WNOHANG)

    def test_real_external_reap_loses_authority_without_a_signal(self):
        owner = owned.OwnedProcess()
        child = subprocess.Popen([sys.executable, '-c', 'pass'], start_new_session=True,
                                 stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        owner.attach(child)
        child.wait(timeout=2)  # Deliberate contract violation to test fail-closed.
        with patch.object(owned.os, 'killpg') as kill:
            with self.assertRaises(owned.OwnershipLost):
                owner.finish()
            kill.assert_not_called()

    def test_real_blocking_observer_publishes_before_reap(self):
        process = self.launch('import time; time.sleep(10)')
        completed = threading.Event()
        observations = []

        def observe():
            observations.append(process.wait_for_exit())
            completed.set()

        waiter = threading.Thread(target=observe)
        waiter.start()
        process.reap_barrier = completed.wait
        process.finish()
        waiter.join(2)
        self.assertFalse(waiter.is_alive())
        self.assertEqual(observations[0].code, -signal.SIGTERM)
        self.assertIsNotNone(process._process.returncode)

    def test_completed_parent_residual_child_does_not_delay_exit_observation(self):
        heartbeat = self.root / 'heartbeat'
        process = self.launch('import os,time\nfrom pathlib import Path\n'
                              f'path=Path({str(heartbeat)!r})\n'
                              'pid=os.fork()\n'
                              'if pid == 0:\n'
                              ' end=time.monotonic()+5\n'
                              ' while time.monotonic()<end:\n'
                              '  path.write_text(str(time.monotonic())); time.sleep(.01)\n'
                              'else:\n time.sleep(.05)\n')
        observation = process.wait_for_exit(2)
        self.assertEqual(observation.code, 0)
        self.assertTrue(heartbeat.exists())
        process.finish()
        time.sleep(.05)
        after = heartbeat.read_bytes()
        time.sleep(.05)
        self.assertEqual(heartbeat.read_bytes(), after)

    def test_term_grace_allows_private_inner_group_cleanup(self):
        # The inner child has an independent six-second self-deadline as a
        # bounded supervisory backstop if the fixture's outer cleanup fails.
        heartbeat, ready, cleaned = [self.root / name for name in ('heartbeat', 'ready', 'cleaned')]
        inner = ('import time\nfrom pathlib import Path\n'
                 f'path=Path({str(heartbeat)!r})\nend=time.monotonic()+6\n'
                 'while time.monotonic()<end:\n path.write_text(str(time.monotonic())); time.sleep(.01)\n')
        code = ('import os,signal,subprocess,sys,time\nfrom pathlib import Path\n'
                f'child=subprocess.Popen([sys.executable,"-c",{inner!r}],start_new_session=True)\n'
                'def stop(number,frame):\n'
                ' time.sleep(.3)\n os.killpg(child.pid,signal.SIGKILL)\n child.wait()\n'
                f' Path({str(cleaned)!r}).touch()\n raise SystemExit(0)\n'
                'signal.signal(signal.SIGTERM,stop)\n'
                f'Path({str(ready)!r}).touch()\n'
                'time.sleep(10)\n')
        process = self.launch(code)
        deadline = time.monotonic() + 2
        while not ready.exists() or not heartbeat.exists():
            self.assertLess(time.monotonic(), deadline)
            time.sleep(.005)
        started = time.monotonic()
        self.assertEqual(process.finish(), 0)
        self.assertLess(time.monotonic() - started, 3)
        self.assertTrue(cleaned.exists())
        after = heartbeat.read_bytes()
        time.sleep(.05)
        self.assertEqual(heartbeat.read_bytes(), after)

    def test_live_group_ignoring_term_is_killed_after_bounded_grace(self):
        ready = self.root / 'ready'
        process = self.launch('import signal,time\nfrom pathlib import Path\n'
                              'signal.signal(signal.SIGTERM,signal.SIG_IGN)\n'
                              f'Path({str(ready)!r}).touch()\ntime.sleep(10)\n')
        deadline = time.monotonic() + 2
        while not ready.exists():
            self.assertLess(time.monotonic(), deadline)
            time.sleep(.005)
        started = time.monotonic()
        self.assertEqual(process.finish(), -signal.SIGKILL)
        elapsed = time.monotonic() - started
        self.assertGreaterEqual(elapsed, owned.TERM_GRACE_SECONDS)
        self.assertLess(elapsed, 3)

    def test_exited_timer_leader_allows_cli_descendant_private_group_cleanup(self):
        self.assert_timer_cli_cleanup(stopped=False)

    def test_stopped_cli_descendant_resumes_to_clean_its_private_group(self):
        self.assert_timer_cli_cleanup(stopped=True)

    def assert_timer_cli_cleanup(self, stopped):
        ready, cleaned = self.root / 'ready', self.root / 'cleaned'
        read_lease, write_lease = os.pipe()
        inner = f'import select\nselect.select([{read_lease}],[],[],6)\n'
        cli = ('import os,signal,subprocess,sys,time\nfrom pathlib import Path\n'
               f'child=subprocess.Popen([sys.executable,"-c",{inner!r}],start_new_session=True,pass_fds=({read_lease},))\n'
               'def stop(number,frame):\n'
               ' time.sleep(.3)\n os.killpg(child.pid,signal.SIGKILL)\n child.wait()\n'
               f' Path({str(cleaned)!r}).touch()\n raise SystemExit(0)\n'
               'signal.signal(signal.SIGTERM,stop)\n'
               f'Path({str(ready)!r}).touch()\n'
               + ('os.kill(os.getpid(),signal.SIGSTOP)\n' if stopped else '') +
               'time.sleep(10)\n')
        timer = ('import subprocess,sys,time\nfrom pathlib import Path\n'
                 f'child=subprocess.Popen([sys.executable,"-c",{cli!r}],pass_fds=({read_lease},))\n')
        if stopped:
            timer += ('import os\nwhile True:\n'
                      ' pid,status=os.waitpid(child.pid,os.WUNTRACED|os.WNOHANG)\n'
                      ' if pid and os.WIFSTOPPED(status):\n  break\n'
                      ' time.sleep(.005)\n')
        else:
            timer += f'while not Path({str(ready)!r}).exists():\n time.sleep(.005)\n'
        try:
            process = self.launch(timer, pass_fds=(read_lease,))
            self.assertEqual(process.wait_for_exit(2).code, 0)
            self.assertFalse(cleaned.exists())
            self.assertEqual(process.finish(), 0)
            self.assertTrue(cleaned.exists())
        finally:
            # Release the independent worker lease even if the assertion fails.
            os.close(write_lease)
            os.close(read_lease)


if __name__ == '__main__':
    unittest.main()
