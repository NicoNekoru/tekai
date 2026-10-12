// Native PTY tests run in a re-executed process, never the host's terminal.
use super::super::*;
use rustix::process::{getpgrp, kill_process, kill_process_group};
use rustix::termios::{tcgetattr, tcgetpgrp, tcsetattr, LocalModes, OptionalActions};
use std::ffi::CString;
use std::mem::MaybeUninit;
use std::os::fd::{FromRawFd, OwnedFd};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Condvar;
use std::time::{Duration, Instant};

fn mask_snapshot() -> libc::sigset_t {
    let mut mask = MaybeUninit::<libc::sigset_t>::zeroed();
    // SAFETY This query changes no mask and covers one complete SDK object.
    assert_eq!(
        unsafe { libc::pthread_sigmask(libc::SIG_SETMASK, ptr::null(), mask.as_mut_ptr()) },
        0
    );
    unsafe { mask.assume_init() }
}

fn assert_mask_unchanged(before: &libc::sigset_t) {
    let after = mask_snapshot();
    for signal in 1..=128 {
        assert_eq!(unsafe { libc::sigismember(before, signal) }, unsafe {
            libc::sigismember(&after, signal)
        });
    }
}

struct MonitorWatchdog {
    done: Arc<(Mutex<bool>, Condvar)>,
    fired: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl MonitorWatchdog {
    fn new(child: &PipeChild) -> Self {
        let state = Arc::clone(&child.monitor.as_ref().expect("private TTY monitor").state);
        let done = Arc::new((Mutex::new(false), Condvar::new()));
        let fired = Arc::new(AtomicBool::new(false));
        let worker_done = Arc::clone(&done);
        let worker_fired = Arc::clone(&fired);
        let thread = thread::spawn(move || {
            let (lock, changed) = &*worker_done;
            let (done, _) = changed
                .wait_timeout_while(lock.lock().unwrap(), Duration::from_secs(3), |done| !*done)
                .unwrap();
            if !*done {
                let mut observation = observation_lock(&state);
                if observation.authority.is_some() {
                    worker_fired.store(true, Ordering::SeqCst);
                    stop_observed_child(&mut observation);
                }
            }
        });
        Self {
            done,
            fired,
            thread: Some(thread),
        }
    }

    fn disarm(&mut self) -> bool {
        let (lock, changed) = &*self.done;
        *lock.lock().unwrap() = true;
        changed.notify_all();
        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
        // The watchdog's observation Arc is gone before pclose reaps the PID.
        !self.fired.load(Ordering::SeqCst)
    }
}

impl Drop for MonitorWatchdog {
    fn drop(&mut self) {
        let _ = self.disarm();
    }
}

fn assert_terminal_failure(child: &PipeChild, signal: i32) {
    let error = child
        .io_error()
        .expect("terminal stop is an error, not EOF");
    assert_eq!(
        error
            .get_ref()
            .unwrap()
            .downcast_ref::<PrivateTerminalStop>()
            .unwrap()
            .0,
        signal
    );
}

unsafe fn private_read(reopen_terminal: bool) {
    let command = if reopen_terminal {
        c"printf prefix; IFS= read -r line </dev/tty"
    } else {
        c"printf prefix; IFS= read -r line"
    };
    let handle = unsafe { popen(command.as_ptr(), c"r".as_ptr()) };
    assert!(!handle.is_null());
    let Backing::ReadPipe(child, _) = &(unsafe { &*handle }).backing else {
        panic!()
    };
    let pid = child.id();
    let mut watchdog = MonitorWatchdog::new(child);
    let mut bytes = [0; 32];
    assert_eq!(
        unsafe { fread(bytes.as_mut_ptr().cast(), 1, bytes.len(), handle) },
        6
    );
    assert_eq!(&bytes[..6], b"prefix");
    assert_eq!(unsafe { ferror(handle) }, 1);
    assert_eq!(unsafe { feof(handle) }, 0);
    assert_eq!(errno(), libc::EIO);
    let Backing::ReadPipe(child, _) = &(unsafe { &*handle }).backing else {
        panic!()
    };
    assert_terminal_failure(child, libc::SIGTTIN);
    assert!(watchdog.disarm(), "private read needed fallback cleanup");
    assert_eq!(unsafe { pclose(handle) }, -1);
    assert_eq!(errno(), libc::EIO);
    super::assert_pipe_child_reaped(pid);
}

struct RedirectedConsole(Vec<OwnedFd>);

impl RedirectedConsole {
    fn new() -> Self {
        let saved = [
            io::stdin().as_raw_fd(),
            io::stdout().as_raw_fd(),
            io::stderr().as_raw_fd(),
        ]
        .into_iter()
        .map(|fd| {
            // SAFETY The three live console descriptors remain borrowed
            // only for this duplication; each result has its own owner.
            rustix::io::fcntl_dupfd_cloexec(unsafe { std::os::fd::BorrowedFd::borrow_raw(fd) }, 3)
                .unwrap()
        })
        .collect();
        let null = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/null")
            .unwrap();
        io::stdout().flush().unwrap();
        io::stderr().flush().unwrap();
        for fd in 0..=2 {
            assert_eq!(unsafe { libc::dup2(null.as_raw_fd(), fd) }, fd);
        }
        Self(saved)
    }
}

impl Drop for RedirectedConsole {
    fn drop(&mut self) {
        for (fd, saved) in self.0.iter().enumerate() {
            let _ = unsafe { libc::dup2(saved.as_raw_fd(), fd as i32) };
        }
    }
}

struct ClosedInput(OwnedFd);

impl ClosedInput {
    fn new() -> Self {
        let saved = rustix::io::fcntl_dupfd_cloexec(io::stdin(), 3).unwrap();
        assert_eq!(unsafe { libc::close(0) }, 0);
        Self(saved)
    }
}

impl Drop for ClosedInput {
    fn drop(&mut self) {
        let _ = unsafe { libc::dup2(self.0.as_raw_fd(), 0) };
    }
}

struct Marker(std::path::PathBuf);

impl Marker {
    fn new(mode: &str) -> Self {
        let directory =
            std::env::temp_dir().join(format!("tekai-stdio-pty-{}-{mode}", std::process::id()));
        std::fs::create_dir(&directory).unwrap();
        Self(directory)
    }

    fn file(&self) -> std::path::PathBuf {
        self.0.join("receipt")
    }

    fn quoted(&self) -> String {
        format!("'{}'", self.file().to_string_lossy().replace('\'', "'\\''"))
    }
}

impl Drop for Marker {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
#[ignore]
fn fixture() {
    let mode = std::env::var("TEKAI_STDIO_PTY_MODE").unwrap();
    // This re-executed fixture is an isolated process. Make the actual native
    // terminal stop behavior explicit rather than inherit shell dispositions.
    unsafe {
        libc::signal(libc::SIGTTIN, libc::SIG_DFL);
        libc::signal(libc::SIGTTOU, libc::SIG_DFL);
        let mut stops = MaybeUninit::<libc::sigset_t>::zeroed();
        assert_eq!(libc::sigemptyset(stops.as_mut_ptr()), 0);
        assert_eq!(libc::sigaddset(stops.as_mut_ptr(), libc::SIGTTIN), 0);
        assert_eq!(libc::sigaddset(stops.as_mut_ptr(), libc::SIGTTOU), 0);
        assert_eq!(
            libc::pthread_sigmask(libc::SIG_UNBLOCK, stops.as_ptr(), ptr::null_mut()),
            0
        );
    }
    let group = getpgrp();
    let mask = mask_snapshot();
    assert_eq!(tcgetpgrp(io::stdin()).unwrap(), group);
    if mode == "drop" {
        let marker = Marker::new("drop");
        let command = CString::new(format!("(/bin/sleep 2; printf fallback > {}; kill -KILL 0) >/dev/null 2>&1 & exec /bin/sleep 8", marker.quoted())).unwrap();
        let mut session = StreamSession::new();
        let (pid, state) = session.as_mut().scope(|| unsafe {
            let handle = popen(command.as_ptr(), c"w".as_ptr());
            assert!(!handle.is_null());
            let Backing::WritePipe(child, _) = &(*handle).backing else {
                panic!()
            };
            (
                child.id(),
                Arc::clone(&child.monitor.as_ref().unwrap().state),
            )
        });
        session.as_mut().finish().unwrap();
        super::assert_pipe_child_reaped(pid);
        assert!(session.streams.borrow().is_empty());
        assert!(observation_lock(&state).authority.is_none());
        assert!(
            !marker.file().exists(),
            "abandonment needed fallback watchdog cleanup"
        );
        assert_eq!(tcgetpgrp(io::stdin()).unwrap(), group);
        assert_mask_unchanged(&mask);
        println!("RESTORED");
        io::stdout().flush().unwrap();
        return;
    }
    let rounds = if mode == "read" || mode == "caller" {
        2
    } else {
        1
    };
    for round in 0..rounds {
        let policy = if mode == "caller" {
            PipeProcessPolicy::CallerManagedProcess
        } else {
            PipeProcessPolicy::InvocationOwned
        };
        let mut session = StreamSession::new_with_pipe_policy(policy);
        session.as_mut().scope(|| unsafe {
            match mode.as_str() {
                "read" => private_read(false),
                "reopen" => {
                    let redirected = RedirectedConsole::new();
                    assert!(rustix::termios::tcgetpgrp(io::stdin()).is_err());
                    private_read(true);
                    drop(redirected);
                }
                "closed" => {
                    let closed = ClosedInput::new();
                    assert_eq!(rustix::termios::tcgetpgrp(io::stdin()).unwrap_err(), rustix::io::Errno::BADF);
                    private_read(true);
                    drop(closed);
                }
                "write" => {
                    let before = tcgetattr(io::stdin()).unwrap();
                    let mut settings = before.clone();
                    settings.local_modes.insert(LocalModes::TOSTOP);
                    tcsetattr(io::stdin(), OptionalActions::Now, &settings).unwrap();
                    let handle = popen(
                        c"printf stopped >/dev/tty; /bin/sleep 8".as_ptr(),
                        c"w".as_ptr(),
                    );
                    assert!(!handle.is_null());
                    assert_eq!(setvbuf(handle, ptr::null_mut(), 2, 0), 0);
                    let Backing::WritePipe(child, _) = &(*handle).backing else {
                        panic!()
                    };
                    let pid = child.id();
                    let mut watchdog = MonitorWatchdog::new(child);
                    let payload = vec![b'x'; 1024 * 1024];
                    let accepted = fwrite(payload.as_ptr().cast(), 1, payload.len(), handle);
                    assert!(
                        accepted < payload.len(),
                        "stopped writer cannot consume the full input"
                    );
                    assert_eq!(ferror(handle), 1);
                    assert_eq!(errno(), libc::EIO);
                    let Backing::WritePipe(child, _) = &(*handle).backing else {
                        panic!()
                    };
                    assert_terminal_failure(child, libc::SIGTTOU);
                    assert!(watchdog.disarm(), "private write needed fallback cleanup");
                    assert_eq!(pclose(handle), -1);
                    assert_eq!(errno(), libc::EIO);
                    super::assert_pipe_child_reaped(pid);
                    tcsetattr(io::stdin(), OptionalActions::Now, &before).unwrap();
                }
                "system" => {
                    // The watchdog signals its own live group, not any numeric
                    // PID receipt, and expires if a regression hangs system.
                    let status = system(
                        c"(/bin/sleep 2; kill -KILL 0) >/dev/null 2>&1 & IFS= read -r line"
                            .as_ptr(),
                    );
                    assert_eq!(status, -1);
                    assert_eq!(errno(), libc::EIO);
                }
                "caller" => {
                    let handle = popen(
                        c"IFS= read -r line; printf 'reply:%s' \"$line\"".as_ptr(),
                        c"r".as_ptr(),
                    );
                    assert!(!handle.is_null());
                    let Backing::ReadPipe(child, _) = &(*handle).backing else {
                        panic!()
                    };
                    let pid = child.id();
                    assert!(child.group.is_none());
                    assert!(child.monitor.is_none());
                    assert_eq!(
                        rustix::process::getpgid(Pid::from_raw(pid as i32)).unwrap(),
                        group
                    );
                    println!("CALLER_READY {round}");
                    io::stdout().flush().unwrap();
                    let mut bytes = [0; 64];
                    let count = fread(bytes.as_mut_ptr().cast(), 1, bytes.len(), handle);
                    assert_eq!(&bytes[..count], b"reply:owned terminal");
                    assert_eq!(ferror(handle), 0);
                    assert_eq!(pclose(handle), 0);
                    super::assert_pipe_child_reaped(pid);
                    println!("CALLER_RESULT {round}");
                    io::stdout().flush().unwrap();
                }
                "background" => {
                    let marker = Marker::new("background");
                    let command = CString::new(format!("(/bin/sleep 0.1; printf done > {}; kill -KILL 0) >/dev/null 2>&1 & printf normal", marker.quoted())).unwrap();
                    let handle = popen(command.as_ptr(), c"r".as_ptr());
                    assert!(!handle.is_null());
                    let Backing::ReadPipe(child, _) = &(*handle).backing else { panic!() };
                    let pid = child.id();
                    let state = Arc::clone(&child.monitor.as_ref().unwrap().state);
                    let mut bytes = [0; 32];
                    assert_eq!(fread(bytes.as_mut_ptr().cast(), 1, bytes.len(), handle), 6);
                    assert_eq!(&bytes[..6], b"normal");
                    assert_eq!(ferror(handle), 0);
                    assert_eq!(pclose(handle), 0);
                    super::assert_pipe_child_reaped(pid);
                    assert!(observation_lock(&state).authority.is_none());
                    let deadline = Instant::now() + Duration::from_secs(2);
                    while !marker.file().exists() {
                        assert!(Instant::now() < deadline, "normal pclose killed intentional background work");
                        thread::sleep(Duration::from_millis(5));
                    }
                    assert_eq!(std::fs::read(marker.file()).unwrap(), b"done");
                }
                _ => panic!("unknown PTY fixture mode"),
            }
        });
        session.as_mut().finish().unwrap();
        assert!(session.streams.borrow().is_empty());
        assert_eq!(tcgetpgrp(io::stdin()).unwrap(), group);
        assert_mask_unchanged(&mask);
    }
    println!("RESTORED");
    io::stdout().flush().unwrap();
}

struct PtyChild {
    child: Option<Child>,
    group: Pid,
    master: File,
    output: String,
}

impl PtyChild {
    fn spawn(mode: &str) -> Self {
        let mut master = -1;
        let mut slave = -1;
        assert_eq!(
            unsafe {
                libc::openpty(
                    &mut master,
                    &mut slave,
                    ptr::null_mut(),
                    ptr::null_mut(),
                    ptr::null_mut(),
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
                "stdio::tests::terminal_tests::fixture",
                "--ignored",
                "--nocapture",
            ])
            .env("TEKAI_STDIO_PTY_MODE", mode)
            .stdin(slave.try_clone().unwrap())
            .stdout(slave.try_clone().unwrap())
            .stderr(slave.try_clone().unwrap());
        let raw_slave = slave.as_raw_fd();
        // SAFETY Child-only native terminal setup before exec, no allocations
        // or locks, and no modification of the test host's terminal/session.
        unsafe {
            command.pre_exec(move || {
                if libc::login_tty(raw_slave) == 0 {
                    Ok(())
                } else {
                    Err(io::Error::last_os_error())
                }
            });
        }
        let child = command.spawn().unwrap();
        let group = Pid::from_raw(child.id() as i32).unwrap();
        drop(slave);
        Self {
            child: Some(child),
            group,
            master,
            output: String::new(),
        }
    }

    fn collect(&mut self) {
        let mut bytes = [0; 4096];
        loop {
            match self.master.read(&mut bytes) {
                Ok(0) => return,
                Ok(count) => {
                    assert!(self.output.len() + count <= 64 * 1024, "PTY output bound");
                    self.output
                        .push_str(&String::from_utf8_lossy(&bytes[..count]));
                }
                Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
                Err(error)
                    if error.kind() == io::ErrorKind::WouldBlock
                        || error.raw_os_error() == Some(libc::EIO) =>
                {
                    return
                }
                Err(error) => panic!("PTY read failed {error}"),
            }
        }
    }

    fn exited(&self) -> bool {
        loop {
            match waitid(
                WaitId::Pid(self.group),
                WaitIdOptions::EXITED | WaitIdOptions::NOWAIT | WaitIdOptions::NOHANG,
            ) {
                Ok(status) => return status.is_some(),
                Err(rustix::io::Errno::INTR) => continue,
                Err(error) => panic!("PTY observation failed {error}"),
            }
        }
    }

    fn wait_for(&mut self, text: &str) {
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            self.collect();
            if self.output.contains(text) {
                return;
            }
            assert!(
                !self.exited(),
                "fixture exited before {text}, {}",
                self.output
            );
            assert!(
                Instant::now() < deadline,
                "fixture bound for {text}, {}",
                self.output
            );
            thread::sleep(Duration::from_millis(5));
        }
    }

    fn finish(mut self, diagnostic: Option<&str>) {
        self.wait_for("RESTORED");
        if let Some(signal) = diagnostic {
            assert!(
                self.output
                    .contains(&format!("private engine command stopped by {signal}")),
                "missing typed stop diagnostic, {}",
                self.output
            );
        }
        let deadline = Instant::now() + Duration::from_secs(2);
        while !self.exited() {
            assert!(Instant::now() < deadline, "PTY fixture exit bound");
            thread::sleep(Duration::from_millis(5));
        }
        let _ = kill_process_group(self.group, Signal::KILL);
        let status = self.child.take().unwrap().wait().unwrap();
        assert!(status.success(), "PTY fixture failed, {}", self.output);
    }
}

impl Drop for PtyChild {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            // Fixture operations have owner-backed private-group watchdogs.
            // First allow them to wake blocked IO and run scoped cleanup.
            let deadline = Instant::now() + Duration::from_secs(4);
            while Instant::now() < deadline && !self.exited() {
                thread::sleep(Duration::from_millis(5));
            }
            let _ = kill_process(self.group, Signal::CONT);
            let _ = kill_process_group(self.group, Signal::KILL);
            let deadline = Instant::now() + Duration::from_secs(2);
            while Instant::now() < deadline {
                if self.exited() {
                    let _ = child.wait();
                    break;
                }
                thread::sleep(Duration::from_millis(5));
            }
        }
    }
}

#[test]
fn private_read_reports_terminal_stop_without_discarding_transferred_prefix() {
    PtyChild::spawn("read").finish(Some("SIGTTIN"));
}

#[test]
fn private_write_terminal_stop_wakes_pipe_io_and_reports_failure() {
    PtyChild::spawn("write").finish(Some("SIGTTOU"));
}

#[test]
fn private_system_terminal_stop_is_a_normal_failure() {
    PtyChild::spawn("system").finish(Some("SIGTTIN"));
}

#[test]
fn redirected_console_with_reopened_controlling_terminal_is_monitored() {
    PtyChild::spawn("reopen").finish(None);
}

#[test]
fn closed_standard_input_does_not_hide_the_controlling_terminal() {
    PtyChild::spawn("closed").finish(Some("SIGTTIN"));
}

#[test]
fn monitored_normal_pclose_preserves_intentional_background_work() {
    PtyChild::spawn("background").finish(None);
}

#[test]
fn abandoning_a_monitored_pipe_joins_and_reaps_without_fallback() {
    PtyChild::spawn("drop").finish(None);
}

#[test]
fn caller_managed_pipes_read_foreground_and_preserve_host_terminal_and_masks() {
    let mut fixture = PtyChild::spawn("caller");
    for round in 0..2 {
        fixture.wait_for(&format!("CALLER_READY {round}"));
        assert_eq!(tcgetpgrp(&fixture.master).unwrap(), fixture.group);
        fixture.master.write_all(b"owned terminal\n").unwrap();
        fixture.wait_for(&format!("CALLER_RESULT {round}"));
    }
    fixture.finish(None);
}
