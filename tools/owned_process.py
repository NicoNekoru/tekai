"""Owned POSIX groups whose leader stays unreaped through group cleanup.

No caller may wait/poll/communicate on the underlying Popen. Exit observation
uses WNOWAIT, so the unreaped leader reserves the numeric group until finish.
Read-only Darwin membership queries classify zombie-only EPERM, never grant
signalling authority. Commands that deliberately escape the group remain out
of scope. A live CLI gets time to clean up its separately owned worker group.
"""

import ctypes
import os
import signal
import subprocess
import sys
import threading
import time
from typing import NamedTuple

TERM_GRACE_SECONDS = 1.5
REAP_TIMEOUT_SECONDS = 5
POLL_SECONDS = 0.005
MAX_GROUP_MEMBERS = 1024


class OwnershipLost(RuntimeError):
    pass


class ExitObservation(NamedTuple):
    code: int
    completion: float


class _DarwinBSDInfo(ctypes.Structure):
    # Exact 64-bit SDK sys/proc_info.h proc_bsdinfo, including its unused fields.
    _fields_ = [(name, ctypes.c_uint32) for name in (
        'pbi_flags', 'pbi_status', 'pbi_xstatus', 'pbi_pid', 'pbi_ppid',
        'pbi_uid', 'pbi_gid', 'pbi_ruid', 'pbi_rgid', 'pbi_svuid', 'pbi_svgid', 'rfu_1')]
    _fields_ += [('pbi_comm', ctypes.c_char * 16), ('pbi_name', ctypes.c_char * 32)]
    _fields_ += [(name, ctypes.c_uint32) for name in (
        'pbi_nfiles', 'pbi_pgid', 'pbi_pjobc', 'e_tdev', 'e_tpgid')]
    _fields_ += [('pbi_nice', ctypes.c_int32), ('pbi_start_tvsec', ctypes.c_uint64),
                ('pbi_start_tvusec', ctypes.c_uint64)]


def _darwin_only_zombies(group, leader_exited=False):
    if ctypes.sizeof(_DarwinBSDInfo) != 136 or _DarwinBSDInfo.pbi_pgid.offset != 100:
        return False
    try:
        library = ctypes.CDLL('/usr/lib/libproc.dylib', use_errno=True)
        listing, info = library.proc_listpids, library.proc_pidinfo
        listing.argtypes = [ctypes.c_uint32, ctypes.c_uint32, ctypes.c_void_p, ctypes.c_int]
        listing.restype = ctypes.c_int
        info.argtypes = [ctypes.c_int, ctypes.c_int, ctypes.c_uint64, ctypes.c_void_p, ctypes.c_int]
        info.restype = ctypes.c_int

        def members():
            ids = (ctypes.c_int * MAX_GROUP_MEMBERS)()
            count_bytes = listing(2, group, ids, ctypes.sizeof(ids))  # PROC_PGRP_ONLY
            # Zero is also libproc's error sentinel. A full buffer may truncate.
            if count_bytes <= 0 or count_bytes >= ctypes.sizeof(ids) or count_bytes % ctypes.sizeof(ctypes.c_int):
                return None
            values = sorted(ids[:count_bytes // ctypes.sizeof(ctypes.c_int)])
            return values if all(pid > 0 for pid in values) and len(set(values)) == len(values) else None

        first = members()
        if first is None or group not in first:
            return False
        for pid in first:
            if pid == group and leader_exited:
                # Fresh WNOWAIT already proves this exact reserved child's
                # final exit. Darwin denies BSDINFO for the exited setuid
                # /bin/ps. This exemption never extends to
                # another PID or replaces the complete group membership proof.
                continue
            record = _DarwinBSDInfo()
            # PROC_PIDTBSDINFO with arg=1 includes zombies. Require a full record.
            if info(pid, 3, 1, ctypes.byref(record), ctypes.sizeof(record)) != ctypes.sizeof(record):
                return False
            if record.pbi_pid != pid or record.pbi_pgid != group or record.pbi_status != 5:  # SZOMB
                return False
        return members() == first
    except (OSError, AttributeError):
        return False


def _signal_owned_group(group, number, leader_exited=False):
    while True:
        try:
            os.killpg(group, number)
            return True
        except InterruptedError:
            continue
        except ProcessLookupError:
            return False
        except PermissionError as error:
            if sys.platform == 'darwin' and _darwin_only_zombies(group, leader_exited):
                return False
            raise PermissionError(error.errno,
                                  f'owned group {group} signal {number} denied. '
                                  f'fresh reserved-leader exit proof={leader_exited}. '
                                  'complete zombie-only membership not established') from error


class OwnedProcess:
    """A returned Popen created with start_new_session=True, exclusively owned.

    The wrapper owns all wait and signal operations. Only finish calls Popen.wait.
    A persistent observer may block in waitid, but must not reap. reap_barrier,
    when present, completes the observer's publication before the actual reap.
    Failed cleanup retains ownership so a caller can retry. ECHILD permanently
    revokes authority and prevents any further numeric PID/group signals.
    """

    @staticmethod
    def require_support():
        if os.name != 'posix' or not all(hasattr(os, name) for name in (
                'waitid', 'WNOWAIT', 'WEXITED', 'WNOHANG', 'P_PID', 'CLD_EXITED', 'CLD_KILLED', 'CLD_DUMPED')):
            raise RuntimeError('Owned process groups require POSIX waitid WNOWAIT')

    def __init__(self, process=None):
        self.require_support()
        self._process = process
        self._condition = threading.Condition()
        self._finish_lock = threading.Lock()
        self._active_observers = 0
        self._observation = None
        self._lost = False
        self._reaped = False
        self.reap_barrier = None

    def attach(self, process):
        if self._process is not None and self._process is not process:
            raise RuntimeError('Owned process cannot attach a different child')
        self._process = process
        return self

    @property
    def pid(self):
        return self._process.pid

    @property
    def returncode(self):
        return self._observation.code if self._observation is not None else None

    @property
    def completion(self):
        return self._observation.completion if self._observation is not None else None

    def _observe(self, blocking=False):
        with self._condition:
            if self._lost:
                raise OwnershipLost(f'Unreaped leader authority lost for child {self.pid}')
            if self._reaped:
                return self._observation
            self._active_observers += 1
        try:
            flags = os.WEXITED | os.WNOWAIT | (0 if blocking else os.WNOHANG)
            while True:
                try:
                    status = os.waitid(os.P_PID, self.pid, flags)
                    completion = time.monotonic()
                    break
                except InterruptedError:
                    continue
                except ChildProcessError as error:
                    with self._condition:
                        self._lost = True
                    raise OwnershipLost(f'Unreaped leader authority lost for child {self.pid}') from error
            if status is None or status.si_pid == 0:
                return None
            if status.si_pid != self.pid or status.si_code not in (os.CLD_EXITED, os.CLD_KILLED, os.CLD_DUMPED):
                raise RuntimeError(f'Unexpected waitid exit observation for child {self.pid}')
            code = status.si_status if status.si_code == os.CLD_EXITED else -status.si_status
            with self._condition:
                if self._observation is None:
                    self._observation = ExitObservation(code, completion)
                return self._observation
        finally:
            with self._condition:
                self._active_observers -= 1
                self._condition.notify_all()

    def poll_exit(self):
        status = self._observe()
        return None if status is None else status.code

    def wait_for_exit(self, timeout=None):
        if timeout is None:
            return self._observe(blocking=True)
        deadline = time.monotonic() + timeout
        while True:
            status = self._observe()
            if status is not None:
                return status
            remaining = deadline - time.monotonic()
            if remaining <= 0:
                raise subprocess.TimeoutExpired(self._process.args, timeout)
            time.sleep(min(POLL_SECONDS, remaining))

    def signal_group(self, number):
        with self._finish_lock:
            if self._reaped:
                return
            self._signal_reserved(number)

    def _signal_reserved(self, number):
        with self._condition:
            # Serialize known-loss publication with the fresh authority check
            # and signal. Do not hold this lock across a blocking waitid call.
            status = self._observe()
            return _signal_owned_group(self.pid, number, status is not None)

    def terminate(self):
        self.signal_group(signal.SIGTERM)

    def finish(self):
        with self._finish_lock:
            if self._reaped:
                return self.returncode
            # A timer's leader can already be dead while its CLI descendant is
            # cleaning a private group. TERM every remaining outer member and
            # retain the full grace even if the leader exits during that grace.
            # A stopped CLI must resume to run that cleanup. Each phase gets
            # its own fresh authority check, never a PID-list-derived signal.
            self._signal_reserved(signal.SIGCONT)
            if self._signal_reserved(signal.SIGTERM):
                deadline = time.monotonic() + TERM_GRACE_SECONDS
                try:
                    self.wait_for_exit(TERM_GRACE_SECONDS)
                except subprocess.TimeoutExpired:
                    pass
                remaining = deadline - time.monotonic()
                if remaining > 0:
                    time.sleep(remaining)
            # Kill-before-reap also stops descendants after a successful exit.
            # The live CLI had a bounded TERM grace to clean its private groups.
            self._signal_reserved(signal.SIGKILL)
            self.wait_for_exit(REAP_TIMEOUT_SECONDS)
            if self.reap_barrier is not None and not self.reap_barrier(REAP_TIMEOUT_SECONDS):
                raise RuntimeError('Exit observer did not publish before owned reap')
            deadline = time.monotonic() + REAP_TIMEOUT_SECONDS
            with self._condition:
                while self._active_observers:
                    remaining = deadline - time.monotonic()
                    if remaining <= 0:
                        raise RuntimeError('Exit observer did not finish before owned reap')
                    self._condition.wait(remaining)
                # No new observer may race the final reap. _observe takes this
                # same lock, and later status reads return the saved observation.
                self._observe()
                code = self._process.wait(timeout=REAP_TIMEOUT_SECONDS)
                self._reaped = True
            if code != self.returncode:
                raise RuntimeError('Reaped child status disagrees with WNOWAIT observation')
            return code
