//! Owned subprocess groups for builds and optional auxiliary tools.
//!
//! Library callers keep control of their signal policy. The CLI opts into
//! cancellation forwarding, and version/cache-only commands do not start a
//! signal thread. SIGKILL cannot run cleanup in the parent.
//! Callers must not independently reap these children or ignore SIGCHLD.

use std::io;
use std::process::{Command, ExitStatus};

#[cfg(unix)]
mod unix {
    use super::*;
    use rustix::process::{Pid, Signal, WaitId, WaitIdOptions, kill_process_group, waitid};
    use signal_hook::consts::{SIGINT, SIGTERM};
    use signal_hook::iterator::Signals;
    use std::os::unix::process::CommandExt;
    use std::process::Child;
    use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
    use std::sync::{Condvar, Mutex, MutexGuard, OnceLock};
    use std::time::{Duration, Instant};

    // One shared limit covers TeX, converters, bibliography and index tools.
    pub const MAX_CHILDREN: usize = 8;
    static CLI: AtomicBool = AtomicBool::new(false);
    static CANCELLED: AtomicI32 = AtomicI32::new(0);
    static GROUPS: Mutex<Vec<Pid>> = Mutex::new(Vec::new());
    static CHANGED: Condvar = Condvar::new();
    static SIGNALS: OnceLock<Result<(), String>> = OnceLock::new();

    fn groups() -> MutexGuard<'static, Vec<Pid>> {
        GROUPS.lock().unwrap_or_else(|error| error.into_inner())
    }

    pub fn enable_cli_cancellation() {
        CLI.store(true, Ordering::Release);
    }

    pub fn initialize_watch_cancellation() -> io::Result<()> {
        if !CLI.load(Ordering::Acquire) {
            return Ok(());
        }
        let result = SIGNALS.get_or_init(|| {
            let mut signals = Signals::new([SIGINT, SIGTERM]).map_err(|error| error.to_string())?;
            std::thread::Builder::new()
                .name("tekai-cancellation".into())
                .spawn(move || {
                    if let Some(number) = signals.forever().next() {
                        cancel_and_exit(number);
                    }
                })
                .map_err(|error| error.to_string())?;
            Ok(())
        });
        result
            .as_ref()
            .map_err(|error| io::Error::other(error.clone()))
            .copied()
    }

    fn cancelled() -> io::Result<()> {
        if CANCELLED.load(Ordering::Acquire) == 0 {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "build cancelled",
            ))
        }
    }

    fn signal_all(active: &[Pid], signal: Signal) {
        for &pid in active {
            // Only registered, unreaped children can enter this list. Their
            // IDs cannot be reused while the group is being signalled.
            let _ = kill_process_group(pid, signal);
        }
    }

    fn cancel_and_exit(number: i32) -> ! {
        CANCELLED.store(number, Ordering::Release);
        let mut active = groups();
        signal_all(
            &active,
            if number == SIGINT {
                Signal::INT
            } else {
                Signal::TERM
            },
        );
        CHANGED.notify_all();
        for (signal, grace) in [
            (Signal::TERM, Duration::from_millis(250)),
            (Signal::KILL, Duration::from_secs(1)),
        ] {
            if signal == Signal::KILL {
                signal_all(&active, signal);
            }
            let deadline = Instant::now() + grace;
            while !active.is_empty() && Instant::now() < deadline {
                let remaining = deadline.saturating_duration_since(Instant::now());
                active = CHANGED
                    .wait_timeout(active, remaining)
                    .unwrap_or_else(|error| error.into_inner())
                    .0;
            }
        }
        // Active waiters normally reap their children before this point. An
        // uninterruptible kernel wait may outlast the grace period, but every
        // owned group has received SIGKILL and no new launch is permitted.
        std::process::exit(128 + number)
    }

    pub fn finish_cli_cancellation() {
        if CANCELLED.load(Ordering::Acquire) != 0 {
            // The signal worker owns shutdown, including idle watch mode.
            // Do not let main return before it has stopped concurrent groups.
            loop {
                std::thread::park_timeout(Duration::from_millis(50));
            }
        }
    }

    struct OwnedChild {
        child: Child,
        pid: Pid,
        registered: bool,
    }

    impl OwnedChild {
        fn spawn(command: &mut Command) -> io::Result<Self> {
            initialize_watch_cancellation()?;
            let mut active = groups();
            while active.len() >= MAX_CHILDREN {
                cancelled()?;
                active = CHANGED
                    .wait(active)
                    .unwrap_or_else(|error| error.into_inner());
            }
            cancelled()?;
            command.process_group(0);
            // Serialize spawn and registration with cancellation. A signal
            // cannot miss the child between these two operations.
            let child = command.spawn()?;
            let raw = i32::try_from(child.id()).expect("Unix child PID fits in i32");
            let pid = Pid::from_raw(raw)
                .filter(|pid| *pid != Pid::INIT)
                .expect("spawned child has a positive non-init PID");
            active.push(pid);
            Ok(Self {
                child,
                pid,
                registered: true,
            })
        }

        fn wait(mut self) -> io::Result<ExitStatus> {
            loop {
                match waitid(
                    WaitId::Pid(self.pid),
                    WaitIdOptions::EXITED | WaitIdOptions::NOWAIT,
                ) {
                    Ok(Some(_)) => break,
                    Ok(None) => continue,
                    Err(rustix::io::Errno::INTR) => continue,
                    Err(rustix::io::Errno::CHILD) => {
                        // A caller's external reaper has taken ownership. Do
                        // not signal a process ID that may already be reused.
                        self.unregister();
                        return Err(rustix::io::Errno::CHILD.into());
                    }
                    Err(error) => return Err(error.into()),
                }
            }
            let status = self.finish();
            cancelled()?;
            status
        }

        fn unregister(&mut self) {
            let mut active = groups();
            active.retain(|pid| *pid != self.pid);
            self.registered = false;
            CHANGED.notify_all();
        }

        fn finish(&mut self) -> io::Result<ExitStatus> {
            let mut active = groups();
            // Leave the direct child unreaped until descendants have been
            // stopped. This also prevents signalling a recycled process ID.
            let _ = kill_process_group(self.pid, Signal::KILL);
            let status = self.child.wait();
            active.retain(|pid| *pid != self.pid);
            self.registered = false;
            CHANGED.notify_all();
            status
        }
    }

    impl Drop for OwnedChild {
        fn drop(&mut self) {
            if self.registered {
                let _ = self.finish();
            }
        }
    }

    pub fn status(command: &mut Command) -> io::Result<ExitStatus> {
        OwnedChild::spawn(command)?.wait()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs;
        use std::sync::atomic::AtomicU64;

        static SERIAL: AtomicU64 = AtomicU64::new(0);

        struct Project(std::path::PathBuf);

        impl Project {
            fn new() -> Self {
                let directory = std::env::temp_dir().join(format!(
                    "tekai-owned-process-{}-{}",
                    std::process::id(),
                    SERIAL.fetch_add(1, Ordering::Relaxed)
                ));
                fs::create_dir(&directory).unwrap();
                Self(directory)
            }

            fn wait_for_pid(&self, name: &str) -> Pid {
                let deadline = Instant::now() + Duration::from_secs(10);
                loop {
                    if let Ok(text) = fs::read_to_string(self.0.join(name))
                        && let Ok(raw) = text.trim().parse()
                        && let Some(pid) = Pid::from_raw(raw)
                    {
                        return pid;
                    }
                    assert!(Instant::now() < deadline, "missing {name}");
                    std::thread::sleep(Duration::from_millis(10));
                }
            }
        }

        impl Drop for Project {
            fn drop(&mut self) {
                let _ = fs::remove_dir_all(&self.0);
            }
        }

        fn assert_stopped(pid: Pid) {
            let deadline = Instant::now() + Duration::from_secs(2);
            loop {
                let snapshot = Command::new("/bin/ps")
                    .args(["-o", "stat=", "-p", &pid.as_raw_pid().to_string()])
                    .output()
                    .unwrap();
                let state = String::from_utf8_lossy(&snapshot.stdout);
                if state.trim().is_empty() || state.trim().starts_with('Z') {
                    return;
                }
                assert!(
                    Instant::now() < deadline,
                    "owned descendant still running: {state}"
                );
                std::thread::sleep(Duration::from_millis(10));
            }
        }

        #[test]
        fn owned_status_preserves_exit_codes_and_releases_slots() {
            for _ in 0..MAX_CHILDREN + 2 {
                assert_eq!(
                    status(Command::new("/bin/sh").args(["-c", "exit 7"]))
                        .unwrap()
                        .code(),
                    Some(7)
                );
            }
        }

        #[test]
        fn failed_spawn_does_not_claim_a_slot() {
            assert!(status(&mut Command::new("/tekai-test-missing-executable")).is_err());
            assert!(
                status(&mut Command::new("/usr/bin/true"))
                    .unwrap()
                    .success()
            );
        }

        #[test]
        fn normal_exit_stops_background_descendants() {
            let project = Project::new();
            let result = status(
                Command::new("/bin/sh")
                    .current_dir(&project.0)
                    .args(["-c", "sleep 30 & echo $! > child.pid; exit 0"]),
            );
            assert!(result.unwrap().success());
            assert_stopped(project.wait_for_pid("child.pid"));
        }

        #[test]
        fn unwinding_stops_owned_groups() {
            let project = Project::new();
            let result = std::panic::catch_unwind(|| {
                let _child = OwnedChild::spawn(
                    Command::new("/bin/sh")
                        .current_dir(&project.0)
                        .args(["-c", "echo $$ > owner.pid; sleep 30"]),
                )
                .unwrap();
                project.wait_for_pid("owner.pid");
                panic!("exercise child drop");
            });
            assert!(result.is_err());
            assert_stopped(project.wait_for_pid("owner.pid"));
        }

        #[test]
        fn independently_reaped_child_does_not_retain_ownership() {
            let mut child = OwnedChild::spawn(&mut Command::new("/usr/bin/true")).unwrap();
            child.child.wait().unwrap();
            assert_eq!(
                child.wait().unwrap_err().raw_os_error(),
                Some(rustix::io::Errno::CHILD.raw_os_error())
            );
        }

        #[test]
        fn concurrent_launches_obey_the_shared_limit() {
            std::thread::scope(|scope| {
                let handles: Vec<_> = (0..MAX_CHILDREN * 2 + 1)
                    .map(|_| {
                        scope.spawn(|| {
                            status(Command::new("/bin/sh").args(["-c", "sleep 0.05"]))
                                .unwrap()
                                .success()
                        })
                    })
                    .collect();
                while handles.iter().any(|handle| !handle.is_finished()) {
                    assert!(groups().len() <= MAX_CHILDREN);
                    std::thread::sleep(Duration::from_millis(1));
                }
                for handle in handles {
                    assert!(handle.join().unwrap());
                }
            });
        }

        #[test]
        #[ignore = "subprocess fixture invoked by cancellation tests"]
        fn cancellation_fixture() {
            assert_eq!(
                std::env::var("TEKAI_PROCESS_RUNNER_FIXTURE").as_deref(),
                Ok("1")
            );
            enable_cli_cancellation();
            let _ = status(Command::new("/bin/sh").args([
                "-c",
                "trap '' INT TERM; sleep 30 & echo $! > child.pid; echo $$ > owner.pid; wait",
            ]));
            finish_cli_cancellation();
            panic!("cancellation fixture returned normally");
        }

        fn cancellation_stops_stubborn_descendants(signal: Signal) {
            let project = Project::new();
            let mut fixture = Command::new(std::env::current_exe().unwrap())
                .current_dir(&project.0)
                .env("TEKAI_PROCESS_RUNNER_FIXTURE", "1")
                .args([
                    "--ignored",
                    "--exact",
                    "process_runner::unix::tests::cancellation_fixture",
                ])
                .spawn()
                .unwrap();
            let owner = project.wait_for_pid("owner.pid");
            let descendant = project.wait_for_pid("child.pid");
            rustix::process::kill_process(Pid::from_raw(fixture.id() as i32).unwrap(), signal)
                .unwrap();
            let deadline = Instant::now() + Duration::from_secs(5);
            let exit = loop {
                if let Some(exit) = fixture.try_wait().unwrap() {
                    break exit;
                }
                if Instant::now() >= deadline {
                    fixture.kill().unwrap();
                    fixture.wait().unwrap();
                    panic!("CLI cancellation did not finish");
                }
                std::thread::sleep(Duration::from_millis(10));
            };
            assert_eq!(exit.code(), Some(128 + signal.as_raw()));
            assert_stopped(owner);
            assert_stopped(descendant);
        }

        #[test]
        fn sigterm_stops_stubborn_descendants() {
            cancellation_stops_stubborn_descendants(Signal::TERM);
        }

        #[test]
        fn sigint_stops_stubborn_descendants() {
            cancellation_stops_stubborn_descendants(Signal::INT);
        }
    }
}

#[cfg(unix)]
pub(crate) use unix::status;
#[cfg(unix)]
pub use unix::{enable_cli_cancellation, finish_cli_cancellation, initialize_watch_cancellation};

#[cfg(not(unix))]
pub fn enable_cli_cancellation() {}
#[cfg(not(unix))]
pub fn finish_cli_cancellation() {}
#[cfg(not(unix))]
pub fn initialize_watch_cancellation() -> io::Result<()> {
    Ok(())
}
#[cfg(not(unix))]
pub(crate) fn status(command: &mut Command) -> io::Result<ExitStatus> {
    command.status()
}
