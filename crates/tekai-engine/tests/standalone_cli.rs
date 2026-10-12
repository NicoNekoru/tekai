//! Execute the standalone entry point, not just its shared supervisor helper.
#![cfg(unix)]

use rustix::process::{
    getpgid, kill_process, kill_process_group, waitid, Pid, Signal, WaitId, WaitIdOptions,
};
use rustix::termios::tcgetpgrp;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::os::fd::{AsRawFd, FromRawFd};
use std::os::unix::process::{CommandExt, ExitStatusExt};
use std::path::PathBuf;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant};

static SERIAL: AtomicU64 = AtomicU64::new(0);
const START_LIMIT: Duration = Duration::from_secs(30);
const EXIT_LIMIT: Duration = Duration::from_secs(3);

struct Project(PathBuf);

impl Project {
    fn new() -> Self {
        for _ in 0..128 {
            let directory = std::env::temp_dir().join(format!(
                "tekai-standalone-cli-{}-{}",
                std::process::id(),
                SERIAL.fetch_add(1, Ordering::Relaxed)
            ));
            match fs::create_dir(&directory) {
                Ok(()) => {
                    for name in ["home", "tmp", "cache"] {
                        fs::create_dir(directory.join(name)).unwrap();
                    }
                    return Self(directory);
                }
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(error) => panic!("standalone fixture directory failed {error}"),
            }
        }
        panic!("standalone fixture directory collision bound");
    }

    fn write(&self, name: &str, contents: &str) {
        fs::write(self.0.join(name), contents).unwrap();
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

struct Standalone<'a> {
    child: Option<Child>,
    group: Pid,
    master: File,
    output: String,
    project: &'a Project,
}

fn observe_exit(pid: Pid) -> io::Result<bool> {
    loop {
        match waitid(
            WaitId::Pid(pid),
            WaitIdOptions::EXITED | WaitIdOptions::NOWAIT | WaitIdOptions::NOHANG,
        ) {
            Ok(status) => return Ok(status.is_some()),
            Err(rustix::io::Errno::INTR) => continue,
            Err(error) => return Err(error.into()),
        }
    }
}

impl<'a> Standalone<'a> {
    fn spawn(project: &'a Project, args: &[&str]) -> Self {
        let mut master = -1;
        let mut slave = -1;
        // SAFETY Successful PTY descriptors are transferred to exact File owners.
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
        let mut command = Command::new(env!("CARGO_BIN_EXE_tekai-engine"));
        command
            .args(args)
            .current_dir(&project.0)
            .env_clear()
            .env("PATH", "")
            .env("HOME", project.0.join("home"))
            .env("TMPDIR", project.0.join("tmp"))
            .env("TEKAI_ENGINE_CACHE", project.0.join("cache"))
            .env("TEKAI_TEXMF_MODE", "bundled")
            .env("TZ", "UTC0")
            .env("TEST_ROOT", &project.0)
            .stdin(slave.try_clone().unwrap())
            .stdout(slave.try_clone().unwrap())
            .stderr(slave.try_clone().unwrap());
        let raw_slave = slave.as_raw_fd();
        // SAFETY The child-only OS hook starts a fresh controlling session
        // before exec, with no Rust allocations or locks. The test host's
        // session, terminal and signal registrations remain untouched.
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
        drop(slave);
        let group = Pid::from_raw(child.id() as i32).unwrap();
        Self {
            child: Some(child),
            group,
            master,
            output: String::new(),
            project,
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
                Err(error) => panic!("PTY read failed {error}, {}", self.output),
            }
        }
    }

    fn wait_for(&mut self, receipt: &str) {
        let deadline = Instant::now() + START_LIMIT;
        loop {
            self.collect();
            if self.output.lines().any(|line| line.trim_end() == receipt) {
                return;
            }
            assert!(
                !observe_exit(self.group).unwrap(),
                "standalone exited before {receipt}, {}",
                self.output
            );
            assert!(
                Instant::now() < deadline,
                "standalone readiness bound for {receipt}, {}",
                self.output
            );
            thread::sleep(Duration::from_millis(5));
        }
    }

    fn wait_exit(&mut self) {
        let deadline = Instant::now() + EXIT_LIMIT;
        loop {
            self.collect();
            if observe_exit(self.group).unwrap() {
                return;
            }
            assert!(
                Instant::now() < deadline,
                "standalone exit bound, {}",
                self.output
            );
            thread::sleep(Duration::from_millis(5));
        }
    }

    fn reap(mut self) -> (ExitStatus, String) {
        self.wait_exit();
        // The direct wrapper remains unreaped until its own group is cleaned.
        let _ = kill_process_group(self.group, Signal::KILL);
        let status = self.child.take().unwrap().wait().unwrap();
        (status, std::mem::take(&mut self.output))
    }

    fn pipe_receipt(&mut self) -> Vec<Pid> {
        let deadline = Instant::now() + START_LIMIT;
        loop {
            if let Ok(file) = File::open(self.project.0.join("pipe.pid")) {
                let mut text = String::new();
                file.take(256).read_to_string(&mut text).unwrap();
                let raw = text
                    .split_whitespace()
                    .map(str::parse::<i32>)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();
                if raw.len() == 4 {
                    let pids = raw
                        .into_iter()
                        .map(|raw| Pid::from_raw(raw).unwrap())
                        .collect::<Vec<_>>();
                    let group = pids[3];
                    assert_ne!(
                        group, self.group,
                        "standalone worker must have its own group"
                    );
                    for &pid in &pids {
                        assert_eq!(getpgid(Some(pid)).unwrap(), group);
                    }
                    let terminal_deadline = Instant::now() + EXIT_LIMIT;
                    while tcgetpgrp(&self.master).unwrap() != group {
                        assert!(
                            Instant::now() < terminal_deadline,
                            "worker foreground bound"
                        );
                        thread::sleep(Duration::from_millis(5));
                    }
                    return pids;
                }
            }
            self.collect();
            assert!(
                !observe_exit(self.group).unwrap(),
                "standalone exited before pipe receipt, {}",
                self.output
            );
            assert!(
                Instant::now() < deadline,
                "pipe receipt bound, {}",
                self.output
            );
            thread::sleep(Duration::from_millis(5));
        }
    }
}

impl Drop for Standalone<'_> {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            // The bounded same-group watchdog owns descendant cleanup even
            // when the implementation under test cannot process cancellation.
            let _ = fs::write(self.project.0.join("cleanup"), b"stop");
            let _ = kill_process(self.group, Signal::TERM);
            let _ = kill_process(self.group, Signal::CONT);
            let deadline = Instant::now() + EXIT_LIMIT;
            while Instant::now() < deadline && !observe_exit(self.group).unwrap_or(true) {
                thread::sleep(Duration::from_millis(5));
            }
            let _ = kill_process_group(self.group, Signal::KILL);
            // KILL cannot guarantee an uninterruptible kernel wait has ended.
            // Never turn failure cleanup into an unbounded supervisory wait.
            let deadline = Instant::now() + EXIT_LIMIT;
            while Instant::now() < deadline {
                if observe_exit(self.group).unwrap_or(true) {
                    let _ = child.wait();
                    break;
                }
                thread::sleep(Duration::from_millis(5));
            }
        }
    }
}

fn stopped(pid: Pid) -> bool {
    let mut child = Command::new("/bin/ps")
        .args(["-o", "stat=", "-p", &pid.as_raw_pid().to_string()])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(1);
    loop {
        if child.try_wait().unwrap().is_some() {
            let mut state = String::new();
            child
                .stdout
                .take()
                .unwrap()
                .take(128)
                .read_to_string(&mut state)
                .unwrap();
            return state.trim().is_empty() || state.trim().starts_with('Z');
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            panic!("process-state observation bound");
        }
        thread::sleep(Duration::from_millis(5));
    }
}

#[test]
fn standalone_cli_reads_foreground_terminal_and_emits_engine_output() {
    let project = Project::new();
    project.write(
        "interactive.tex",
        "\\catcode123=1\\catcode125=2\\relax\n\\immediate\\write16{TEKAI_STANDALONE_READY}\n\\read-1 to\\reply\n\\immediate\\write16{TEKAI_STANDALONE_REPLY=\\reply}\n\\end\n",
    );
    let mut cli = Standalone::spawn(
        &project,
        &[
            "-ini",
            "-interaction=errorstopmode",
            "-jobname=interactive",
            "interactive.tex",
        ],
    );
    cli.wait_for("TEKAI_STANDALONE_READY");
    cli.master.write_all(b"owned terminal\n").unwrap();
    cli.wait_for("TEKAI_STANDALONE_REPLY=owned terminal");
    let (status, output) = cli.reap();
    assert!(status.success(), "standalone interactive status, {output}");
    assert!(
        !output.lines().any(|line| line.starts_with("! ")),
        "interactive TeX diagnostic, {output}"
    );
    assert_ne!(status.signal(), Some(libc::SIGABRT));
    let log = fs::read_to_string(project.0.join("interactive.log")).unwrap();
    assert!(log.contains("TEKAI_STANDALONE_REPLY=owned terminal"));
}

#[test]
fn standalone_cli_term_cleans_blocked_pipe_and_stubborn_descendants() {
    let project = Project::new();
    project.write(
        "blocked.sh",
        "trap '' INT TERM\n(\n  trap '' INT TERM\n  attempt=0\n  while [ \"$attempt\" -lt 80 ] && [ ! -e \"$TEST_ROOT/cleanup\" ]; do\n    /bin/sleep 0.1\n    attempt=$((attempt + 1))\n  done\n  kill -KILL 0\n) >/dev/null 2>&1 &\nwatchdog=$!\n/bin/sleep 8 >/dev/null 2>&1 &\nleaf=$!\nprintf '%s %s %s %s\\n' \"$$\" \"$watchdog\" \"$leaf\" \"$TEKAI_SUPERVISED_ENGINE_GROUP\" > \"$TEST_ROOT/pipe.pid.part\"\n/bin/mv \"$TEST_ROOT/pipe.pid.part\" \"$TEST_ROOT/pipe.pid\"\nwait \"$leaf\"\n",
    );
    project.write(
        "blocked.tex",
        "\\openin0=\"|/bin/sh ./blocked.sh\"\n\\read0 to\\reply\n\\immediate\\write16{UNEXPECTED_PIPE_RETURN}\n\\end\n",
    );
    let mut cli = Standalone::spawn(
        &project,
        &[
            "-ini",
            "-shell-escape",
            "-interaction=nonstopmode",
            "-jobname=blocked",
            "blocked.tex",
        ],
    );
    let descendants = cli.pipe_receipt();
    let started = Instant::now();
    kill_process(cli.group, Signal::TERM).unwrap();
    cli.wait_exit();
    assert!(
        started.elapsed() < EXIT_LIMIT,
        "TERM completion exceeded its bound"
    );
    let deadline = Instant::now() + Duration::from_secs(1);
    loop {
        if descendants.iter().copied().all(stopped) {
            break;
        }
        assert!(
            Instant::now() < deadline,
            "standalone TERM left live descendants before fallback cleanup, {}",
            cli.output
        );
        thread::sleep(Duration::from_millis(5));
    }
    let (status, output) = cli.reap();
    assert_eq!(status.code(), Some(128 + libc::SIGTERM), "{output}");
    assert!(!output.contains("UNEXPECTED_PIPE_RETURN"), "{output}");
}
