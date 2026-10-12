//! CLI-only worker supervision. The core library never installs signal handlers.
//!
//! A worker and its inherited pipe commands stay in one owned process group.
//! The parent can clean that group even when the worker is killed without Drop.
//! CLI descendants are job-owned through worker completion. This differs from
//! reusable-library explicit pclose/system, which preserve intentional background
//! side effects beyond the call. CLI job exit cleans those residual descendants.
//! Killing the supervising parent itself, independently reaping its child, or
//! deliberately escaping the owned group is outside this cleanup contract.

#![warn(clippy::all)]

use crate::process_group::signal_owned_group;
use rustix::process::{getpgrp, getpid, kill_process, waitid, Pid, Signal, WaitId, WaitIdOptions};
use rustix::termios::{tcgetpgrp, tcsetpgrp};
use signal_hook::consts::{SIGINT, SIGTERM};
use signal_hook::iterator::{Handle, Signals};
use std::ffi::{OsStr, OsString};
use std::io;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::os::fd::OwnedFd;
use std::os::unix::process::CommandExt;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::rc::Rc;
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::thread::{self, JoinHandle};
use std::time::Duration;

/// Trusted CLI routing metadata, not authentication or a security boundary.
pub const SUPERVISED_GROUP_ENV: &str = "TEKAI_SUPERVISED_ENGINE_GROUP";

/// Use this value in the worker's invocation-owned environment overlay.
/// The marker names the actual group, which may differ from a nested CLI's PID.
pub fn current_group_marker() -> OsString {
    OsString::from(getpgrp().as_raw_pid().to_string())
}

fn marker_matches_group(marker: &OsStr, group: Pid) -> bool {
    marker == OsStr::new(&group.as_raw_pid().to_string())
}

/// A nested CLI may run inline only when the inherited marker still names its
/// actual process group. The core invocation's private default is unaffected.
pub fn inherited_group_is_supervised() -> bool {
    std::env::var_os(SUPERVISED_GROUP_ENV)
        .is_some_and(|marker| marker_matches_group(&marker, getpgrp()))
}

const TERM_GRACE: Duration = Duration::from_millis(250);

// tcsetpgrp from the background supervisor otherwise stops the whole CLI group.
// Block only SIGTTOU in this thread and restore the caller's complete mask.
struct TerminalSignalMask {
    original: Option<libc::sigset_t>,
    _thread: PhantomData<Rc<()>>,
}

impl TerminalSignalMask {
    fn block() -> io::Result<Self> {
        let mut blocked = MaybeUninit::<libc::sigset_t>::zeroed();
        let mut original = MaybeUninit::<libc::sigset_t>::zeroed();
        // SAFETY Both SDK objects cover sigset_t. These essential OS calls
        // affect only the calling thread, and SIGTTOU is a valid signal.
        unsafe {
            if libc::sigemptyset(blocked.as_mut_ptr()) != 0
                || libc::sigaddset(blocked.as_mut_ptr(), libc::SIGTTOU) != 0
            {
                return Err(io::Error::last_os_error());
            }
            let error =
                libc::pthread_sigmask(libc::SIG_BLOCK, blocked.as_ptr(), original.as_mut_ptr());
            if error != 0 {
                return Err(io::Error::from_raw_os_error(error));
            }
            Ok(Self {
                original: Some(original.assume_init()),
                _thread: PhantomData,
            })
        }
    }

    fn restore(&mut self) -> io::Result<()> {
        if let Some(original) = &self.original {
            // SAFETY The successful query initialized the complete mask. The
            // non-Send guard restores it in the thread that acquired it.
            let error =
                unsafe { libc::pthread_sigmask(libc::SIG_SETMASK, original, std::ptr::null_mut()) };
            if error != 0 {
                return Err(io::Error::from_raw_os_error(error));
            }
            self.original = None;
        }
        Ok(())
    }
}

impl Drop for TerminalSignalMask {
    fn drop(&mut self) {
        let _ = self.restore();
    }
}

struct TerminalLease {
    terminal: OwnedFd,
    supervisor: Pid,
    worker: Pid,
    reserved: bool,
}

impl TerminalLease {
    fn inherited_stdin(worker: Pid) -> io::Result<Option<Self>> {
        let stdin = io::stdin();
        match tcgetpgrp(&stdin) {
            Ok(_) => Ok(Some(Self {
                terminal: rustix::io::fcntl_dupfd_cloexec(&stdin, 3)?,
                supervisor: getpgrp(),
                worker,
                reserved: true,
            })),
            // A redirected or closed stdin has no terminal ownership to move.
            Err(rustix::io::Errno::NOTTY | rustix::io::Errno::BADF) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    fn set_foreground(&self, group: Pid) -> io::Result<()> {
        let mut mask = TerminalSignalMask::block()?;
        let result = tcsetpgrp(&self.terminal, group).map_err(io::Error::from);
        let restored = mask.restore();
        result.and(restored)
    }

    fn handoff(&self) -> io::Result<bool> {
        if tcgetpgrp(&self.terminal)? == self.supervisor {
            self.set_foreground(self.worker)?;
            Ok(true)
        } else {
            // A background launch or another job's terminal is never stolen.
            Ok(false)
        }
    }

    fn worker_is_foreground(&self) -> io::Result<bool> {
        Ok(tcgetpgrp(&self.terminal)? == self.worker)
    }

    fn restore(&self) -> io::Result<()> {
        if self.reserved && self.worker_is_foreground()? {
            self.set_foreground(self.supervisor)?;
        }
        Ok(())
    }
}

impl Drop for TerminalLease {
    fn drop(&mut self) {
        // No panic during an unexpected unwind, and no change if the shell or
        // another foreground job has already taken over the terminal.
        let _ = self.restore();
    }
}

struct Authority {
    // An unreaped direct child reserves this ID. Signals are sent while the
    // mutex is held, never from a PID snapshot after releasing the lock.
    group: Option<Pid>,
    observed_exit: bool,
    closing: bool,
    signal_error: Option<io::Error>,
}

struct Shared {
    authority: Mutex<Authority>,
    changed: Condvar,
}

impl Shared {
    fn lock(&self) -> MutexGuard<'_, Authority> {
        self.authority
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

fn forward(authority: &mut Authority, signal: Signal) {
    if let Some(group) = authority.group {
        if let Err(error) = signal_owned_group(group, signal) {
            authority.signal_error.get_or_insert(error);
        }
    }
}

fn signal_worker(mut signals: Signals, shared: Arc<Shared>) {
    for number in signals.forever() {
        let mut authority = shared.lock();
        if authority.closing {
            break;
        }
        match number {
            SIGINT => forward(&mut authority, Signal::INT),
            SIGTERM => {
                forward(&mut authority, Signal::TERM);
                // This is an owner-completion notification, not child polling.
                // The leader remains unreaped through the grace and escalation.
                let (mut authority, _) = shared
                    .changed
                    .wait_timeout_while(authority, TERM_GRACE, |authority| {
                        !authority.observed_exit && !authority.closing
                    })
                    .unwrap_or_else(|poisoned| poisoned.into_inner());
                if !authority.observed_exit && !authority.closing {
                    forward(&mut authority, Signal::KILL);
                }
            }
            _ => unreachable!("only INT and TERM are registered"),
        }
    }
}

struct Worker {
    child: Option<Child>,
    shared: Arc<Shared>,
    signals: Option<Handle>,
    signal_thread: Option<JoinHandle<()>>,
    terminal: Option<TerminalLease>,
    startup_terminal_stop: bool,
}

impl Worker {
    fn lost_child(&mut self) -> io::Error {
        // An external reaper violated the ownership contract. Neither signal
        // forwarding nor terminal routing may use its now-unreserved group ID.
        let mut authority = self.shared.lock();
        authority.group = None;
        authority.observed_exit = true;
        self.shared.changed.notify_all();
        if let Some(mut terminal) = self.terminal.take() {
            terminal.reserved = false;
        }
        rustix::io::Errno::CHILD.into()
    }

    fn spawn(command: &mut Command, signals: Signals) -> io::Result<Self> {
        // Inherit rather than capture output. A full stdout pipe must not block
        // this parent in a relay while it should be supervising cancellation.
        command
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .process_group(0);
        let child = command.spawn()?;
        let raw = i32::try_from(child.id()).expect("Unix child PID fits in i32");
        let group = Pid::from_raw(raw)
            .filter(|pid| *pid != Pid::INIT)
            .expect("spawned worker has a positive non-init PID");
        let shared = Arc::new(Shared {
            authority: Mutex::new(Authority {
                group: Some(group),
                observed_exit: false,
                closing: false,
                signal_error: None,
            }),
            changed: Condvar::new(),
        });
        // Own the child before spawning the signal worker. A thread-creation
        // failure drops this guard and still stops/reaps the process group.
        let mut worker = Self {
            child: Some(child),
            shared: Arc::clone(&shared),
            signals: Some(signals.handle()),
            signal_thread: None,
            terminal: None,
            startup_terminal_stop: false,
        };
        worker.terminal = TerminalLease::inherited_stdin(group)?;
        worker.signal_thread = Some(
            thread::Builder::new()
                .name("tekai-engine-signals".into())
                .spawn(move || signal_worker(signals, shared))?,
        );
        if let Some(terminal) = &worker.terminal {
            worker.startup_terminal_stop = terminal.handoff()?;
            if worker.startup_terminal_stop {
                // The worker may have attempted its first read between spawn
                // and handoff. CONT clears a pending TTIN/TTOU and resumes a
                // stopped worker while its unreaped leader still reserves PGID.
                signal_owned_group(group, Signal::CONT)?;
            }
        }
        Ok(worker)
    }

    fn observe_exit(&mut self) -> io::Result<()> {
        // Only this thread can reap. The signal worker holds group authority
        // under the mutex, and the main observer deliberately uses NOWAIT.
        let group = Pid::from_raw(
            i32::try_from(self.child.as_ref().expect("owned worker").id())
                .expect("Unix child PID fits in i32"),
        )
        .expect("owned worker PID is positive");
        loop {
            match waitid(
                WaitId::Pid(group),
                WaitIdOptions::EXITED | WaitIdOptions::STOPPED | WaitIdOptions::NOWAIT,
            ) {
                Ok(Some(status)) if status.stopped() => {
                    // Consume only the stop report, never an exit reservation.
                    // A racing CONT/exit may have replaced the report already.
                    let consumed = loop {
                        match waitid(
                            WaitId::Pid(group),
                            WaitIdOptions::STOPPED | WaitIdOptions::NOHANG,
                        ) {
                            Err(rustix::io::Errno::INTR) => continue,
                            Err(rustix::io::Errno::CHILD) => return Err(self.lost_child()),
                            result => break result?,
                        }
                    };
                    if consumed.is_none() {
                        continue;
                    }
                    let terminal_stop = matches!(
                        status.stopping_signal(),
                        Some(libc::SIGTTIN | libc::SIGTTOU)
                    );
                    if self.startup_terminal_stop
                        && terminal_stop
                        && self
                            .terminal
                            .as_ref()
                            .map(TerminalLease::worker_is_foreground)
                            .transpose()?
                            .unwrap_or(false)
                    {
                        // A waitable stop can outlive the startup CONT. Allow
                        // one startup recovery, then respect later job stops.
                        self.startup_terminal_stop = false;
                        signal_owned_group(group, Signal::CONT)?;
                        continue;
                    }
                    self.startup_terminal_stop = false;
                    if let Some(terminal) = &self.terminal {
                        terminal.restore()?;
                    }
                    // Stop this CLI PID, not the inherited shell's entire
                    // process group. Its job-control shell observes the stop,
                    // reclaims the TTY, and chooses foreground or background
                    // continuation. STOP cannot be ignored or handler-masked.
                    kill_process(getpid(), Signal::STOP)?;
                    if let Some(terminal) = &self.terminal {
                        terminal.handoff()?;
                    }
                    signal_owned_group(group, Signal::CONT)?;
                }
                Ok(Some(_)) => {
                    self.shared.lock().observed_exit = true;
                    self.shared.changed.notify_all();
                    return Ok(());
                }
                Ok(None) | Err(rustix::io::Errno::INTR) => continue,
                Err(rustix::io::Errno::CHILD) => {
                    return Err(self.lost_child());
                }
                Err(error) => return Err(error.into()),
            }
        }
    }

    fn stop_signal_worker(&mut self) -> io::Result<()> {
        {
            let mut authority = self.shared.lock();
            authority.closing = true;
            self.shared.changed.notify_all();
        }
        if let Some(handle) = &self.signals {
            handle.close();
        }
        let joined = self.signal_thread.take().map_or(Ok(()), |thread| {
            thread
                .join()
                .map_err(|_| io::Error::other("engine signal forwarding worker panicked"))
        });
        // Signals and this final Handle share the delivery state. Dropping
        // only the iterator leaves registrations alive, so release both here.
        self.signals.take();
        joined
    }

    fn finish(&mut self) -> io::Result<ExitStatus> {
        let mut first_error = self.stop_signal_worker().err();
        // Restore while the direct leader is still unreaped. Reaping first
        // would let its numeric process-group ID be recycled before this check.
        if let Some(terminal) = self.terminal.take() {
            if let Err(error) = terminal.restore() {
                first_error.get_or_insert(error);
            }
            // Any infallible Drop retry also runs before ending reservation.
            drop(terminal);
        }
        let mut authority = self.shared.lock();
        if let Some(group) = authority.group {
            if let Err(error) = signal_owned_group(group, Signal::KILL) {
                first_error.get_or_insert(error);
            }
            // Also stop a direct worker that changed its own group. Its Child
            // still owns the unreaped PID; no forwarded snapshot can race it.
            if !authority.observed_exit {
                if let Err(error) = self.child.as_mut().expect("owned worker").kill() {
                    first_error.get_or_insert(error);
                }
            }
        }
        let status = loop {
            match self.child.as_mut().expect("owned worker").wait() {
                Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
                result => break result,
            }
        };
        if status.is_ok()
            || status.as_ref().is_err_and(|error| {
                error.raw_os_error() == Some(rustix::io::Errno::CHILD.raw_os_error())
            })
        {
            authority.group = None;
            self.child.take();
        }
        if let Some(error) = authority.signal_error.take().or(first_error) {
            Err(error)
        } else {
            status
        }
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        if self.child.is_some() {
            // This also handles unexpected unwind and setup/wait failures.
            // No signal worker survives to act on a reaped/recycled PID.
            let _ = self.finish();
        }
    }
}

/// Supervise one explicitly configured engine-worker command.
///
/// This CLI adapter installs and owns INT/TERM registrations. It never exits the
/// host process. Callers must not reap its child independently or ignore SIGCHLD.
/// The worker must use inherited pipe groups and must not start another private
/// supervisor for a nested command carrying the matching group marker.
///
/// INT reaches the whole worker group and retains normal TeX interrupt handling.
/// TERM receives a short grace followed by group KILL. Residual descendants are
/// stopped before normal or abnormal worker status is reaped. Worker SIGKILL is
/// covered because this parent survives; supervisor SIGKILL cannot run cleanup.
pub fn supervise(command: &mut Command) -> io::Result<ExitStatus> {
    // Register before spawn. Signals arriving during launch remain queued until
    // the owned worker starts, so no child escapes a registration gap.
    let signals = Signals::new([SIGINT, SIGTERM])?;
    let mut worker = Worker::spawn(command, signals)?;
    worker.observe_exit()?;
    worker.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{Read, Write};
    use std::os::fd::{AsRawFd, FromRawFd};
    use std::os::unix::ffi::OsStringExt;
    use std::os::unix::process::ExitStatusExt;
    use std::time::Instant;

    #[test]
    fn group_marker_is_canonical_and_names_the_group() {
        let group = Pid::from_raw(7123).unwrap();
        assert!(marker_matches_group(OsStr::new("7123"), group));
        for marker in ["7124", "07123", " 7123", "7123\n", "-7123", ""] {
            assert!(!marker_matches_group(OsStr::new(marker), group));
        }
        assert!(!marker_matches_group(
            &OsString::from_vec(vec![0xff]),
            group
        ));
        assert_eq!(
            current_group_marker(),
            OsString::from(getpgrp().as_raw_pid().to_string())
        );
    }

    #[test]
    fn repeated_workers_preserve_status_and_release_signal_owners() {
        for _ in 0..8 {
            let status = supervise(Command::new("/bin/sh").args(["-c", "exit 7"]))
                .expect("supervised shell status");
            assert_eq!(status.code(), Some(7));
        }
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn completed_protected_system_worker_preserves_status() {
        let status = supervise(Command::new("/bin/ps").args([
            "-p",
            &std::process::id().to_string(),
            "-o",
            "pid=",
        ]))
        .expect("completed protected worker cleanup");
        assert!(status.success(), "protected worker status {status}");
    }

    #[test]
    fn failed_spawn_and_worker_sigkill_are_ordinary_results() {
        assert!(supervise(&mut Command::new("/tekai-test-missing-worker")).is_err());
        let status = supervise(Command::new("/bin/sh").args(["-c", "kill -KILL $$"]))
            .expect("killed worker status");
        assert_eq!(status.signal(), Some(signal_hook::consts::SIGKILL));
    }

    fn signal_mask() -> libc::sigset_t {
        let mut mask = MaybeUninit::<libc::sigset_t>::zeroed();
        // SAFETY Query only this thread's mask into one complete SDK object.
        assert_eq!(
            unsafe {
                libc::pthread_sigmask(libc::SIG_SETMASK, std::ptr::null(), mask.as_mut_ptr())
            },
            0
        );
        unsafe { mask.assume_init() }
    }

    fn assert_signal_masks_equal(before: &libc::sigset_t, after: &libc::sigset_t) {
        // Compare signal membership rather than opaque SDK padding bytes.
        for signal in 1..=128 {
            assert_eq!(
                unsafe { libc::sigismember(before, signal) },
                unsafe { libc::sigismember(after, signal) },
                "mask membership changed for signal {signal}"
            );
        }
    }

    // Re-executed in a fresh PTY session. Never mutate the test host's terminal.
    #[test]
    #[ignore]
    fn pty_fixture() {
        let mode = std::env::var("TEKAI_TEST_PTY_MODE").expect("isolated PTY mode");
        let foreground = getpgrp();
        assert_eq!(tcgetpgrp(io::stdin()).unwrap(), foreground);
        let before = signal_mask();
        if mode == "no-steal" {
            let mut child = Command::new("/bin/sh")
                .args(["-c", "/bin/sleep 8"])
                .stdin(Stdio::null())
                .process_group(0)
                .spawn()
                .unwrap();
            let other = Pid::from_raw(child.id() as i32).unwrap();
            let lease = TerminalLease::inherited_stdin(other).unwrap().unwrap();
            lease.set_foreground(other).unwrap();
            assert!(!lease.handoff().unwrap());
            // A lease must not steal from another foreground job even in Drop.
            let unrelated = TerminalLease::inherited_stdin(foreground).unwrap().unwrap();
            unrelated.restore().unwrap();
            drop(unrelated);
            assert_eq!(tcgetpgrp(io::stdin()).unwrap(), other);
            lease.restore().unwrap();
            child.kill().unwrap();
            signal_owned_group(other, Signal::KILL).unwrap();
            child.wait().unwrap();
        } else if mode == "unwind" {
            let result = std::panic::catch_unwind(|| {
                let _worker = Worker::spawn(
                    Command::new("/bin/sh").args([
                        "-c",
                        "(/bin/sleep 8; kill -KILL 0) >/dev/null 2>&1 & read value",
                    ]),
                    Signals::new([SIGINT, SIGTERM]).unwrap(),
                )
                .unwrap();
                panic!("intentional terminal-owner unwind");
            });
            assert!(result.is_err());
        } else {
            let rounds = if mode == "repeat" { 2 } else { 1 };
            for round in 0..rounds {
                // The same-group watchdog bounds abandoned test workers even
                // if a regression prevents supervisor cleanup from executing.
                let body = if mode == "stop" {
                    "printf 'READY %s\\n' $$; kill -STOP $$; printf 'RESUMED\\n'; read value; test \"$value\" = input; exit 7"
                } else if mode == "tstp" {
                    "printf 'READY %s\\n' $$; kill -TSTP $$; printf 'RESUMED\\n'; read value; test \"$value\" = input; exit 7"
                } else if mode == "term" {
                    "trap '' TERM; printf 'READY %s\\n' $$; read value; exit 9"
                } else {
                    "printf 'READY %s\\n' $$; read value; test \"$value\" = input; exit 7"
                };
                let script = format!("(/bin/sleep 8; kill -KILL 0) >/dev/null 2>&1 & {body}");
                let status = supervise(Command::new("/bin/sh").args(["-c", &script])).unwrap();
                if mode == "term" {
                    assert_eq!(status.signal(), Some(libc::SIGKILL));
                } else {
                    assert_eq!(status.code(), Some(7));
                }
                assert_eq!(tcgetpgrp(io::stdin()).unwrap(), foreground);
                assert_signal_masks_equal(&before, &signal_mask());
                println!("RESULT {round}");
            }
        }
        assert_eq!(tcgetpgrp(io::stdin()).unwrap(), foreground);
        assert_signal_masks_equal(&before, &signal_mask());
        println!("RESTORED");
    }

    struct PtyFixture {
        child: Option<Child>,
        master: File,
        output: String,
    }

    impl PtyFixture {
        fn spawn(mode: &str) -> Self {
            let mut master = -1;
            let mut slave = -1;
            // SAFETY The test supplies two valid integer outputs and default
            // terminal settings. Successful descriptors become exact owners.
            assert_eq!(
                unsafe {
                    libc::openpty(
                        &mut master,
                        &mut slave,
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                    )
                },
                0
            );
            let master = unsafe { File::from_raw_fd(master) };
            let slave = unsafe { File::from_raw_fd(slave) };
            let flags = unsafe { libc::fcntl(master.as_raw_fd(), libc::F_GETFL) };
            assert!(flags >= 0);
            assert_eq!(
                unsafe { libc::fcntl(master.as_raw_fd(), libc::F_SETFL, flags | libc::O_NONBLOCK) },
                0
            );
            let mut command = Command::new(std::env::current_exe().unwrap());
            command
                .args([
                    "--exact",
                    "cli_supervisor::tests::pty_fixture",
                    "--ignored",
                    "--nocapture",
                ])
                .env("TEKAI_TEST_PTY_MODE", mode)
                .stdin(slave.try_clone().unwrap())
                .stdout(slave.try_clone().unwrap())
                .stderr(slave.try_clone().unwrap());
            let raw_slave = slave.as_raw_fd();
            // SAFETY This child-only hook uses the essential login_tty syscall
            // adapter before exec. No Rust allocation, locks, or parent terminal
            // mutation occur in the post-fork child.
            unsafe {
                command.pre_exec(move || {
                    if libc::login_tty(raw_slave) != 0 {
                        Err(io::Error::last_os_error())
                    } else {
                        Ok(())
                    }
                });
            }
            let child = command.spawn().unwrap();
            drop(slave);
            Self {
                child: Some(child),
                master,
                output: String::new(),
            }
        }

        fn pid(&self) -> Pid {
            Pid::from_raw(self.child.as_ref().unwrap().id() as i32).unwrap()
        }

        fn collect(&mut self) {
            let mut buffer = [0; 4096];
            loop {
                match self.master.read(&mut buffer) {
                    Ok(0) => return,
                    Ok(count) => {
                        assert!(self.output.len() + count <= 64 * 1024, "PTY output bound");
                        self.output
                            .push_str(&String::from_utf8_lossy(&buffer[..count]));
                    }
                    Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
                    Err(error)
                        if error.kind() == io::ErrorKind::WouldBlock
                            || error.raw_os_error() == Some(libc::EIO) =>
                    {
                        return;
                    }
                    Err(error) => panic!("PTY read failed {error}"),
                }
            }
        }

        fn wait_for(&mut self, text: &str) {
            let deadline = Instant::now() + Duration::from_secs(3);
            loop {
                self.collect();
                if self.output.contains(text) {
                    return;
                }
                assert!(
                    Instant::now() < deadline,
                    "missing {text}, output {}",
                    self.output
                );
                thread::sleep(Duration::from_millis(5));
            }
        }

        fn worker(&self, round: usize) -> Pid {
            let line = self
                .output
                .lines()
                .filter_map(|line| line.strip_prefix("READY "))
                .nth(round)
                .expect("worker receipt");
            Pid::from_raw(line.trim().parse().unwrap()).unwrap()
        }

        fn send_input(&mut self) {
            self.master.write_all(b"input\n").unwrap();
        }

        fn wait_foreground(&mut self, group: Pid) {
            let deadline = Instant::now() + Duration::from_secs(3);
            loop {
                if tcgetpgrp(&self.master).unwrap() == group {
                    return;
                }
                self.collect();
                assert!(
                    Instant::now() < deadline,
                    "foreground handoff bound, {}",
                    self.output
                );
                thread::sleep(Duration::from_millis(5));
            }
        }

        fn wait_stopped(&mut self) {
            let deadline = Instant::now() + Duration::from_secs(3);
            loop {
                let status = waitid(
                    WaitId::Pid(self.pid()),
                    WaitIdOptions::STOPPED | WaitIdOptions::NOWAIT | WaitIdOptions::NOHANG,
                )
                .unwrap();
                if status.is_some_and(|status| status.stopped()) {
                    return;
                }
                self.collect();
                assert!(
                    Instant::now() < deadline,
                    "supervisor did not stop, {}",
                    self.output
                );
                thread::sleep(Duration::from_millis(5));
            }
        }

        fn finish(mut self) {
            self.wait_for("RESTORED");
            let deadline = Instant::now() + Duration::from_secs(3);
            loop {
                if waitid(
                    WaitId::Pid(self.pid()),
                    WaitIdOptions::EXITED | WaitIdOptions::NOWAIT | WaitIdOptions::NOHANG,
                )
                .unwrap()
                .is_some()
                {
                    break;
                }
                assert!(
                    Instant::now() < deadline,
                    "fixture exit bound, {}",
                    self.output
                );
                thread::sleep(Duration::from_millis(5));
            }
            signal_owned_group(self.pid(), Signal::KILL).unwrap();
            let status = self.child.take().unwrap().wait().unwrap();
            assert!(status.success(), "PTY fixture failed, {}", self.output);
        }
    }

    impl Drop for PtyFixture {
        fn drop(&mut self) {
            if let Some(mut child) = self.child.take() {
                let pid = Pid::from_raw(child.id() as i32).unwrap();
                let _ = kill_process(pid, Signal::TERM);
                let _ = kill_process(pid, Signal::CONT);
                let deadline = Instant::now() + Duration::from_secs(2);
                while Instant::now() < deadline {
                    if waitid(
                        WaitId::Pid(pid),
                        WaitIdOptions::EXITED | WaitIdOptions::NOWAIT | WaitIdOptions::NOHANG,
                    )
                    .ok()
                    .flatten()
                    .is_some()
                    {
                        break;
                    }
                    thread::sleep(Duration::from_millis(5));
                }
                // The unreaped fixture reserves this group through cleanup.
                let _ = signal_owned_group(pid, Signal::KILL);
                let _ = child.wait();
            }
        }
    }

    #[test]
    fn pty_worker_reads_and_repeated_foreground_leases_restore() {
        let mut fixture = PtyFixture::spawn("repeat");
        fixture.wait_for("READY ");
        fixture.wait_foreground(fixture.worker(0));
        fixture.send_input();
        fixture.wait_for("RESULT 0");
        let deadline = Instant::now() + Duration::from_secs(3);
        while fixture.output.matches("READY ").count() < 2 {
            fixture.collect();
            assert!(
                Instant::now() < deadline,
                "second worker receipt, {}",
                fixture.output
            );
            thread::sleep(Duration::from_millis(5));
        }
        fixture.wait_foreground(fixture.worker(1));
        fixture.send_input();
        fixture.finish();
    }

    #[test]
    fn pty_worker_stop_is_visible_and_foreground_continuation_reads() {
        for mode in ["stop", "tstp"] {
            let mut fixture = PtyFixture::spawn(mode);
            fixture.wait_for("READY ");
            fixture.wait_stopped();
            fixture.wait_foreground(fixture.pid());
            kill_process(fixture.pid(), Signal::CONT).unwrap();
            fixture.wait_for("RESUMED");
            fixture.wait_foreground(fixture.worker(0));
            fixture.send_input();
            fixture.finish();
        }
    }

    #[test]
    fn pty_term_escalation_restores_foreground_and_mask() {
        let mut fixture = PtyFixture::spawn("term");
        fixture.wait_for("READY ");
        kill_process(fixture.pid(), Signal::TERM).unwrap();
        fixture.finish();
    }

    #[test]
    fn pty_terminal_lease_does_not_steal_another_foreground_group() {
        PtyFixture::spawn("no-steal").finish();
    }

    #[test]
    fn pty_unexpected_unwind_restores_foreground_and_mask() {
        PtyFixture::spawn("unwind").finish();
    }

    #[test]
    fn temporary_terminal_mask_preserves_already_blocked_signals() {
        let before = signal_mask();
        let mut blocked = MaybeUninit::<libc::sigset_t>::zeroed();
        // SAFETY Private test thread mask, restored by a non-Send guard.
        unsafe {
            assert_eq!(libc::sigemptyset(blocked.as_mut_ptr()), 0);
            assert_eq!(libc::sigaddset(blocked.as_mut_ptr(), libc::SIGUSR1), 0);
            assert_eq!(libc::sigaddset(blocked.as_mut_ptr(), libc::SIGTTOU), 0);
            assert_eq!(
                libc::pthread_sigmask(libc::SIG_BLOCK, blocked.as_ptr(), std::ptr::null_mut()),
                0
            );
        }
        let mut outer = TerminalSignalMask {
            original: Some(before),
            _thread: PhantomData,
        };
        let expected = signal_mask();
        {
            let _inner = TerminalSignalMask::block().unwrap();
        }
        assert_signal_masks_equal(&expected, &signal_mask());
        outer.restore().unwrap();
        assert_signal_masks_equal(&before, &signal_mask());
    }
}
