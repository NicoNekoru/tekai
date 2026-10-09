"""Test parity command supervision and cache isolation without compiling TeX."""

import os
from pathlib import Path
import signal
import subprocess
import tempfile
import unittest
from unittest.mock import Mock, call, patch

import verify_bundled_papers as parity


class CommandTests(unittest.TestCase):
    def setUp(self):
        self.process = Mock(pid=12345, returncode=0)
        self.stdout = b"complete output\n"
        self.stderr = b"diagnostic output\n"
        self.launch_options = None

        def launch(argv, **options):
            self.argv = argv
            self.launch_options = options
            options["stdout"].write(self.stdout)
            options["stdout"].flush()
            options["stderr"].write(self.stderr)
            options["stderr"].flush()
            return self.process

        popen_patch = patch.object(parity.subprocess, "Popen", side_effect=launch)
        self.popen = popen_patch.start()
        self.addCleanup(popen_patch.stop)
        kill_patch = patch.object(parity.os, "killpg")
        self.kill = kill_patch.start()
        self.addCleanup(kill_patch.stop)

    def test_success_returns_complete_binary_stdout_and_reaps_owned_group(self):
        env = {"PATH": ""}
        result = parity.command([Path("/fixture/tool"), "argument"], timeout=7,
                                cwd=Path("/fixture"), env=env)
        self.assertEqual(result, self.stdout)
        self.assertEqual(self.argv, ["/fixture/tool", "argument"])
        self.assertTrue(self.launch_options["start_new_session"])
        self.assertEqual(self.launch_options["cwd"], Path("/fixture"))
        self.assertIs(self.launch_options["env"], env)
        self.assertNotEqual(self.launch_options["stdout"], subprocess.PIPE)
        self.assertNotEqual(self.launch_options["stderr"], subprocess.PIPE)
        self.assertTrue(self.launch_options["stdout"].closed)
        self.assertTrue(self.launch_options["stderr"].closed)
        self.kill.assert_called_once_with(12345, signal.SIGKILL)
        self.assertEqual(self.process.wait.call_args_list,
                         [call(timeout=7), call(timeout=parity.REAP_TIMEOUT_SECONDS)])

    def test_timeout_kills_the_group_reaps_and_reports_bounded_logs(self):
        self.process.wait.side_effect = [subprocess.TimeoutExpired("tool", 3), None]
        self.stdout = b"early stdout " * 100 + b"stdout tail"
        self.stderr = b"early stderr " * 100 + b"stderr tail"
        with patch.object(parity, "LOG_TAIL_BYTES", 16):
            with self.assertRaisesRegex(RuntimeError, "timed out after 3 seconds") as error:
                parity.command(["tool"], timeout=3)
        message = str(error.exception)
        self.assertIn("stdout tail", message)
        self.assertIn("stderr tail", message)
        self.assertNotIn("early stdout early stdout", message)
        self.assertLess(len(message), 100)
        self.kill.assert_called_once_with(12345, signal.SIGKILL)
        self.assertEqual(self.process.wait.call_args_list,
                         [call(timeout=3), call(timeout=parity.REAP_TIMEOUT_SECONDS)])

    def test_nonzero_exit_reports_error_tails_and_reaps(self):
        self.process.returncode = 1
        self.stderr = b"bad metadata \xff\n"
        with self.assertRaisesRegex(RuntimeError, "exited with code 1") as error:
            parity.command(["tool"])
        self.assertIn("complete output", str(error.exception))
        self.assertIn("bad metadata", str(error.exception))
        self.kill.assert_called_once_with(12345, signal.SIGKILL)
        self.assertEqual(self.process.wait.call_count, 2)

    def test_an_already_exited_group_does_not_prevent_reaping(self):
        self.kill.side_effect = ProcessLookupError
        self.assertEqual(parity.command(["tool"]), self.stdout)
        self.assertEqual(self.process.wait.call_count, 2)

    def test_interrupted_wait_still_kills_and_reaps(self):
        self.process.wait.side_effect = [KeyboardInterrupt, None]
        with self.assertRaises(KeyboardInterrupt):
            parity.command(["tool"])
        self.kill.assert_called_once_with(12345, signal.SIGKILL)
        self.assertEqual(self.process.wait.call_count, 2)

    def test_successful_output_at_the_limit_is_not_truncated(self):
        self.stdout = b"x" * 64
        with patch.object(parity, "MAX_STDOUT_BYTES", 64):
            self.assertEqual(parity.command(["tool"]), self.stdout)

    def test_oversized_successful_stdout_fails_instead_of_hiding_a_pdf_difference(self):
        self.stdout = b"x" * 65
        with patch.object(parity, "MAX_STDOUT_BYTES", 64):
            with self.assertRaisesRegex(RuntimeError, "stdout exceeds the 64-byte capture limit"):
                parity.command(["tool"])
        self.kill.assert_called_once_with(12345, signal.SIGKILL)

    def test_invalid_deadlines_start_no_process(self):
        for timeout in (0, -1, float("nan"), float("inf"), 301):
            with self.subTest(timeout=timeout):
                with self.assertRaisesRegex(ValueError, "finite, positive"):
                    parity.command(["tool"], timeout=timeout)
        self.popen.assert_not_called()
        self.kill.assert_not_called()

    def test_launch_error_never_targets_a_process_group(self):
        self.popen.side_effect = PermissionError("Launch denied")
        with self.assertRaises(PermissionError):
            parity.command(["tool"])
        self.kill.assert_not_called()

    def test_cleanup_failure_still_attempts_a_bounded_reap(self):
        self.kill.side_effect = PermissionError("Group inspection denied")
        with self.assertRaises(PermissionError):
            parity.command(["tool"])
        self.assertEqual(self.process.wait.call_args_list,
                         [call(timeout=parity.COMMAND_TIMEOUT_SECONDS),
                          call(timeout=parity.REAP_TIMEOUT_SECONDS)])

    def test_sigterm_interrupts_the_command_cleans_up_and_restores_the_handler(self):
        original_handler = object()
        with patch.object(parity.signal, "getsignal", return_value=original_handler), \
                patch.object(parity.signal, "signal") as set_handler, \
                patch.object(parity, "_main", side_effect=lambda: parity.command(["tool"])):
            waits = 0

            def wait(**_kwargs):
                nonlocal waits
                waits += 1
                if waits == 1:
                    handler = set_handler.call_args.args[1]
                    handler(signal.SIGTERM, None)

            self.process.wait.side_effect = wait
            with self.assertRaises(KeyboardInterrupt):
                parity.main()
        self.kill.assert_called_once_with(12345, signal.SIGKILL)
        self.assertEqual(self.process.wait.call_count, 2)
        self.assertEqual(set_handler.call_count, 2)
        set_handler.assert_called_with(signal.SIGTERM, original_handler)

    def test_normal_cli_exit_restores_the_original_sigterm_handler(self):
        original_handler = object()
        with patch.object(parity.signal, "getsignal", return_value=original_handler), \
                patch.object(parity.signal, "signal") as set_handler, \
                patch.object(parity, "_main", return_value=42):
            self.assertEqual(parity.main(), 42)
        self.assertEqual(set_handler.call_count, 2)
        self.assertTrue(callable(set_handler.call_args_list[0].args[1]))
        set_handler.assert_called_with(signal.SIGTERM, original_handler)


class CandidateEnvironmentTests(unittest.TestCase):
    def test_all_four_caches_are_fixture_owned_and_host_overrides_are_absent(self):
        with tempfile.TemporaryDirectory(prefix="tekai-parity-test-") as temporary:
            work = Path(temporary).resolve()
            shared_env = {"PATH": "/host/tools", "HOME": "/host/home", "TEXINPUTS": "/host/tex//",
                          "TEKAI_EMBEDDED_ENGINE_RUNNER": "/host/tekai"}
            shared_env.update({f"TEKAI_{name}_CACHE": f"/host/{name.lower()}"
                               for name in ("ENGINE", "FORMAT", "AUX", "BIBTEX")})
            with patch.dict(os.environ, shared_env):
                env = parity.candidate_environment(work)
            paths = [Path(env[f"TEKAI_{name}_CACHE"])
                     for name in ("ENGINE", "FORMAT", "AUX", "BIBTEX")]
            self.assertEqual(len(set(paths)), 4)
            for path in paths:
                self.assertTrue(path.is_relative_to(work))
                self.assertTrue(path.is_dir())
            self.assertEqual(env["PATH"], "")
            self.assertEqual(env["TEKAI_TEXMF_MODE"], "bundled")
            self.assertEqual(Path(env["HOME"]), work / "home")
            self.assertEqual(Path(env["TMPDIR"]), work / "tmp")
            self.assertNotIn("TEXINPUTS", env)
            self.assertNotIn("TEKAI_EMBEDDED_ENGINE_RUNNER", env)

    def test_repeated_environment_setup_preserves_the_same_owned_cache_paths(self):
        with tempfile.TemporaryDirectory(prefix="tekai-parity-test-") as temporary:
            work = Path(temporary)
            first = parity.candidate_environment(work)
            self.assertEqual(first, parity.candidate_environment(work))


if __name__ == "__main__":
    unittest.main()
