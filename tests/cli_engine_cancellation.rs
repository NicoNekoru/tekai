#![cfg(unix)]

//! Exercise the CLI, compiler, engine pipe policy and nested PGF routing together.
//! A parent exit alone is insufficient. Every recorded fixture process must stop.

use rustix::process::{
    Pid, Signal, WaitId, WaitIdOptions, kill_process, kill_process_group, waitid,
};
use std::fs::{self, File};
use std::io::Read;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

const READY_BUDGET: Duration = Duration::from_secs(30);
const CANCELLATION_BUDGET: Duration = Duration::from_secs(3);

struct Project(PathBuf);

impl Project {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        for _ in 0..128 {
            let root = std::env::temp_dir().join(format!(
                "tekai-cli-cancellation-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            match fs::create_dir(&root) {
                Ok(()) => {
                    for name in ["build", "bin", "home", "tmp"] {
                        fs::create_dir(root.join(name)).unwrap();
                    }
                    let project = Self(root);
                    project.write("receipt.sh", RECEIPT_SCRIPT);
                    project.write("block.sh", BLOCK_SCRIPT);
                    project.write("bin/pdflatex", PGF_SHIM);
                    fs::set_permissions(
                        project.0.join("bin/pdflatex"),
                        fs::Permissions::from_mode(0o700),
                    )
                    .unwrap();
                    return project;
                }
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
                Err(error) => panic!("cannot reserve cancellation fixture: {error}"),
            }
        }
        panic!("cannot reserve cancellation fixture after 128 attempts");
    }

    fn write(&self, name: &str, text: &str) {
        fs::write(self.0.join(name), text).unwrap();
    }

    fn command(&self) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_tekai"));
        command
            .current_dir(&self.0)
            .env_clear()
            .env("PATH", self.0.join("bin"))
            .env("HOME", self.0.join("home"))
            .env("TMPDIR", self.0.join("tmp"))
            .env("TZ", "UTC0")
            .env("TEKAI_TEXMF_MODE", "bundled")
            .env("TEKAI_ENGINE_CACHE", self.0.join("cache/engine"))
            .env("TEKAI_FORMAT_CACHE", self.0.join("cache/formats"))
            .env("TEKAI_AUX_CACHE", self.0.join("cache/aux"))
            .env("TEKAI_BIBTEX_CACHE", self.0.join("cache/bibtex"))
            .env("TEST_TRACE_ROOT", &self.0)
            .env("TEST_TEKAI_BIN", env!("CARGO_BIN_EXE_tekai"))
            .stdin(Stdio::null());
        command
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        // This exact directory was exclusively created by the test.
        let _ = fs::remove_dir_all(&self.0);
    }
}

const RECEIPT_SCRIPT: &str = r#"record_pid() {
  group=$(/bin/ps -o pgid= -p "$2")
  printf '%s %s %s\n' "$2" "$group" "${TEKAI_SUPERVISED_ENGINE_GROUP:-missing}" > "$TEST_TRACE_ROOT/$1.pid"
}
"#;

const BLOCK_SCRIPT: &str = r#"#!/bin/sh
set -eu
. "$TEST_TRACE_ROOT/receipt.sh"
role=$1
mode=$2
trap '' INT TERM
/bin/sleep 15 >/dev/null 2>&1 &
leaf=$!
record_pid "$role-leaf" "$leaf"
(
  # Failure cleanup is fixture-owned, not a signal sent using a stale PID.
  # This watchdog remains a member of the exact group it stops with kill 0.
  n=0
  while [ "$n" -lt 100 ]; do
    if [ -f "$TEST_TRACE_ROOT/stop-fixtures" ]; then
      kill -KILL 0
    fi
    /bin/sleep 0.1
    n=$((n + 1))
  done
  kill -KILL 0
) >/dev/null 2>&1 &
watchdog=$!
record_pid "$role-watchdog" "$watchdog"
if [ "$mode" = cooperative ]; then
  trap 'exit 130' INT
fi
# This final receipt proves signal policy is installed before the test signals.
record_pid "$role" "$$"
# No stdout and no close: TeX is blocked reading an unclosed pipe.
wait "$leaf"
"#;

const PGF_SHIM: &str = r#"#!/bin/sh
set -eu
. "$TEST_TRACE_ROOT/receipt.sh"
record_pid pgf-wrapper "$$"
exec "$TEST_TEKAI_BIN" __tekai-engine -8bit "$@"
"#;

fn blocking_document(role: &str, mode: &str) -> String {
    format!(
        "\\documentclass{{article}}\n\\begin{{document}}\n\
         \\newread\\blocking\n\
         \\openin\\blocking=\"|/bin/sh ./block.sh {role} {mode}\"\n\
         \\read\\blocking to\\result\n\
         \\typeout{{CANCELLATION-FIXTURE-READ-RETURNED}}\n\
         \\closein\\blocking\nNever complete this job.\n\\end{{document}}\n"
    )
}

const PGF_DOCUMENT: &str = r#"\documentclass{article}
\usepackage{tikz}
\usetikzlibrary{external}
\tikzexternalize
\tikzifexternalizing{
  \newread\blocking
  \openin\blocking="|/bin/sh ./block.sh nested stubborn"
  \read\blocking to\result
  \closein\blocking
}{}
\begin{document}
\begin{tikzpicture}
\draw (0,0) -- (1,1);
\end{tikzpicture}
\end{document}
"#;

#[derive(Clone, Copy, Debug)]
struct Receipt {
    pid: Pid,
    group: Pid,
    marker: Pid,
}

fn receipt(path: &Path) -> Option<Receipt> {
    let text = fs::read_to_string(path).ok()?;
    if text.len() > 128 {
        return None;
    }
    let mut fields = text.split_whitespace();
    let mut next = || {
        fields
            .next()?
            .parse::<i32>()
            .ok()
            .and_then(Pid::from_raw)
            .filter(|pid| *pid != Pid::INIT)
    };
    let value = Receipt {
        pid: next()?,
        group: next()?,
        marker: next()?,
    };
    fields.next().is_none().then_some(value)
}

fn stopped(pid: Pid) -> bool {
    let output = Command::new("/bin/ps")
        .args(["-o", "stat=", "-p", &pid.as_raw_pid().to_string()])
        .output()
        .expect("cannot inspect fixture process");
    let state = String::from_utf8_lossy(&output.stdout);
    // A zombie can no longer execute or keep a descriptor open. Its eventual
    // reaping belongs to its actual parent/init, not this test's PID snapshot.
    state.trim().is_empty() || state.trim().starts_with('Z')
}

struct OwnedCli {
    child: Option<Child>,
    group: Pid,
    root: PathBuf,
}

impl OwnedCli {
    fn spawn(project: &Project, mut command: Command) -> Self {
        command
            .process_group(0)
            .stdout(File::create(project.0.join("stdout.log")).unwrap())
            .stderr(File::create(project.0.join("stderr.log")).unwrap());
        let child = command.spawn().unwrap();
        let group = Pid::from_raw(i32::try_from(child.id()).unwrap()).unwrap();
        assert_ne!(group, Pid::INIT);
        Self {
            child: Some(child),
            group,
            root: project.0.clone(),
        }
    }

    fn diagnostics(&self) -> String {
        ["stdout.log", "stderr.log", "build/main.log"]
            .into_iter()
            .map(|name| {
                let mut text = String::new();
                if let Ok(file) = File::open(self.root.join(name)) {
                    let _ = file.take(64 * 1024).read_to_string(&mut text);
                }
                format!("{name}\n{text}")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn exited_without_reaping(&self) -> bool {
        loop {
            match waitid(
                WaitId::Pid(self.group),
                WaitIdOptions::EXITED | WaitIdOptions::NOWAIT | WaitIdOptions::NOHANG,
            ) {
                Ok(status) => return status.is_some(),
                Err(rustix::io::Errno::INTR) => {}
                Err(error) => panic!("cannot observe owned CLI without reaping: {error}"),
            }
        }
    }

    fn ready(&self, name: &str) -> Receipt {
        let deadline = Instant::now() + READY_BUDGET;
        loop {
            if let Some(value) = receipt(&self.root.join(format!("{name}.pid"))) {
                assert_eq!(value.marker, value.group, "{name}: {value:?}");
                assert_eq!(
                    rustix::process::getpgid(Some(value.pid)).unwrap(),
                    value.group,
                    "{name}: {value:?}"
                );
                return value;
            }
            assert!(
                !self.exited_without_reaping() && Instant::now() < deadline,
                "fixture {name} never became ready\n{}",
                self.diagnostics()
            );
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    fn cancel(&self, signal: Signal) {
        assert!(
            !self.exited_without_reaping(),
            "CLI exited before cancellation"
        );
        // The Child remains unreaped, reserving this positive non-init PID.
        kill_process(self.group, signal).unwrap();
        let deadline = Instant::now() + CANCELLATION_BUDGET;
        while !self.exited_without_reaping() {
            assert!(
                Instant::now() < deadline,
                "CLI cancellation did not finish within its bound\n{}",
                self.diagnostics()
            );
            std::thread::sleep(Duration::from_millis(10));
        }
        assert!(
            Instant::now() < deadline,
            "CLI exit was not observed within the cancellation bound\n{}",
            self.diagnostics()
        );
    }

    fn assert_stopped(&self, processes: &[Receipt]) {
        let deadline = Instant::now() + Duration::from_secs(1);
        loop {
            if processes.iter().all(|process| stopped(process.pid)) {
                return;
            }
            assert!(
                Instant::now() < deadline,
                "CLI exited but fixture descendants survived: {processes:?}\n{}",
                self.diagnostics()
            );
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    fn reap(mut self) -> ExitStatus {
        assert!(self.exited_without_reaping());
        // Assert production cleanup before this fallback can mask a leak.
        // Signal only our still-reserved direct child's group, then reap.
        let _ = kill_process_group(self.group, Signal::KILL);
        self.child.take().unwrap().wait().unwrap()
    }
}

impl Drop for OwnedCli {
    fn drop(&mut self) {
        let Some(child) = self.child.as_mut() else {
            return;
        };
        // Trusted watchdogs stop their own groups, even if a faulty nested CLI
        // created an escaping group. Never kill a descendant PID after reaping.
        let _ = fs::write(self.root.join("stop-fixtures"), b"stop");
        let _ = kill_process(self.group, Signal::TERM);
        let deadline = Instant::now() + Duration::from_secs(2);
        loop {
            match waitid(
                WaitId::Pid(self.group),
                WaitIdOptions::EXITED | WaitIdOptions::NOWAIT | WaitIdOptions::NOHANG,
            ) {
                Ok(Some(_)) => break,
                Ok(None) | Err(rustix::io::Errno::INTR) if Instant::now() < deadline => {
                    std::thread::sleep(Duration::from_millis(10));
                }
                _ => break,
            }
        }
        let _ = kill_process_group(self.group, Signal::KILL);
        let _ = child.wait();
        // Let the private cleanup marker reach any still-running watchdog
        // before Project removes this test-owned directory.
        let deadline = Instant::now() + Duration::from_secs(2);
        loop {
            let live = fs::read_dir(&self.root).ok().is_some_and(|entries| {
                entries.filter_map(Result::ok).take(32).any(|entry| {
                    entry
                        .path()
                        .extension()
                        .is_some_and(|extension| extension == "pid")
                        && receipt(&entry.path()).is_some_and(|process| !stopped(process.pid))
                })
            });
            if !live || Instant::now() >= deadline {
                break;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
    }
}

fn pipe_processes(cli: &OwnedCli, role: &str) -> [Receipt; 3] {
    let leader = cli.ready(role);
    let leaf = cli.ready(&format!("{role}-leaf"));
    let watchdog = cli.ready(&format!("{role}-watchdog"));
    assert_ne!(
        leader.group, cli.group,
        "engine worker did not isolate its group"
    );
    assert_eq!(leader.group, leaf.group);
    assert_eq!(leader.group, watchdog.group);
    [leader, leaf, watchdog]
}

fn managed_build_cancellation(signal: Signal) {
    let project = Project::new();
    project.write("main.tex", &blocking_document("managed", "stubborn"));
    let mut command = project.command();
    command.args(["build", "main.tex", "--once", "--shell-escape", "--force"]);
    let cli = OwnedCli::spawn(&project, command);
    let processes = pipe_processes(&cli, "managed");
    cli.cancel(signal);
    cli.assert_stopped(&processes);
    let status = cli.reap();
    assert_eq!(status.code(), Some(128 + signal.as_raw()));
}

#[test]
fn managed_build_sigint_stops_unclosed_pipe_and_stubborn_descendants() {
    managed_build_cancellation(Signal::INT);
}

#[test]
fn managed_build_sigterm_escalates_and_stops_stubborn_descendants() {
    managed_build_cancellation(Signal::TERM);
}

#[test]
fn direct_engine_sigint_ends_blocked_read_and_stops_pipe_descendants() {
    let project = Project::new();
    project.write("main.tex", &blocking_document("direct", "cooperative"));
    let mut command = project.command();
    command.args([
        "__tekai-engine",
        "-interaction=nonstopmode",
        "-halt-on-error",
        "-shell-escape",
        "-output-directory=build",
        "main.tex",
    ]);
    let cli = OwnedCli::spawn(&project, command);
    let processes = pipe_processes(&cli, "direct");
    cli.cancel(Signal::INT);
    cli.assert_stopped(&processes);
    let status = cli.reap();
    assert_eq!(
        status.code(),
        Some(1),
        "SIGINT should remain a normal TeX interruption"
    );
}

#[test]
fn nested_pgf_wrapper_stays_in_outer_group_and_term_cleans_its_descendants() {
    let project = Project::new();
    project.write("main.tex", PGF_DOCUMENT);
    let mut command = project.command();
    command.args([
        "__tekai-engine",
        "-interaction=nonstopmode",
        "-halt-on-error",
        "-shell-escape",
        "-output-directory=build",
        "main.tex",
    ]);
    let cli = OwnedCli::spawn(&project, command);
    let wrapper = cli.ready("pgf-wrapper");
    let processes = pipe_processes(&cli, "nested");
    assert_eq!(
        processes[0].group, wrapper.group,
        "nested __tekai-engine escaped the parent's supervised group"
    );
    assert_ne!(
        wrapper.pid, wrapper.group,
        "PGF shim unexpectedly became group leader"
    );
    cli.cancel(Signal::TERM);
    let observed = [wrapper, processes[0], processes[1], processes[2]];
    cli.assert_stopped(&observed);
    let status = cli.reap();
    assert!(!status.success(), "cancelled PGF job returned success");
}
