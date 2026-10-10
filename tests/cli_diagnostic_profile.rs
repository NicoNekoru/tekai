//! Protocol tests use CLI-only paths and never launch a TeX engine.

use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

use serde_json::Value;

const PREFIX: &str = "TEKAI_PROFILE ";
static NEXT: AtomicU64 = AtomicU64::new(0);

struct Fixture(PathBuf);

impl Fixture {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "tekai-cli-profile-{}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        Self(root)
    }

    fn run(&self, args: &[&str], opt_in: Option<&str>) -> Output {
        let mut command = Command::new(env!("CARGO_BIN_EXE_tekai"));
        command
            .current_dir(&self.0)
            .args(args)
            .env_remove("TEKAI_DIAGNOSTIC_PROFILE")
            .env("HOME", self.0.join("home"))
            .env("TEKAI_ENGINE_CACHE", self.0.join("cache-engine"))
            .env("TEKAI_FORMAT_CACHE", self.0.join("cache-format"))
            .env("TEKAI_AUX_CACHE", self.0.join("cache-aux"))
            .env("TEKAI_BIBTEX_CACHE", self.0.join("cache-bibtex"))
            .env(
                "TEKAI_EMBEDDED_ENGINE_RUNNER",
                self.0.join("must-not-launch"),
            );
        if let Some(value) = opt_in {
            command.env("TEKAI_DIAGNOSTIC_PROFILE", value);
        }
        command.output().expect("CLI did not start")
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn records(output: &Output) -> Vec<Value> {
    std::str::from_utf8(&output.stderr)
        .unwrap()
        .lines()
        .filter_map(|line| line.strip_prefix(PREFIX))
        .map(|json| {
            assert!(PREFIX.len() + json.len() < 8192);
            serde_json::from_str(json).expect("invalid profile JSON")
        })
        .collect()
}

fn only_record(output: &Output) -> Value {
    let records = records(output);
    assert_eq!(records.len(), 1, "{output:#?}");
    let record = records.into_iter().next().unwrap();
    assert_eq!(record["schema_version"], 1);
    assert_eq!(record["producer"], "tekai-rust");
    assert_eq!(record["scope"], "cli_process");
    assert_eq!(record["untrusted"], true);
    assert_eq!(record["completed_spans_only"], true);
    assert_eq!(record["phases_overlap"], true);
    assert!(record["elapsed_ms"].as_f64().unwrap().is_finite());
    assert!(record["elapsed_ms"].as_f64().unwrap() >= 0.0);
    let phases = record["phases"].as_array().unwrap();
    assert_eq!(phases.len(), 11);
    for phase in phases {
        assert!(phase["calls"].as_u64().is_some());
        assert!(phase["active_calls"].as_u64().is_some());
        if let Some(elapsed) = phase["elapsed_ms"].as_f64() {
            assert!(elapsed.is_finite() && elapsed >= 0.0);
        } else {
            assert_eq!(phase["status"], "unavailable");
        }
    }
    for name in ["embedded_engine_entry", "format_initialization"] {
        assert_eq!(phase(&record, name)["status"], "unavailable");
        assert!(phase(&record, name)["elapsed_ms"].is_null());
    }
    record
}

fn phase<'a>(record: &'a Value, name: &str) -> &'a Value {
    record["phases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|phase| phase["name"] == name)
        .expect("fixed phase was omitted")
}

fn ordinary_stderr(output: &Output) -> String {
    std::str::from_utf8(&output.stderr)
        .unwrap()
        .split_inclusive('\n')
        .filter(|line| !line.starts_with(PREFIX))
        .collect()
}

#[test]
fn missing_source_error_is_unchanged_and_only_exact_opt_in_emits() {
    let fixture = Fixture::new();
    let args = ["build", "missing.tex", "--report-json"];
    let default = fixture.run(&args, None);
    assert_eq!(default.status.code(), Some(1));
    assert!(records(&default).is_empty());
    for value in ["", "0", "true", " 1", "1\n"] {
        let disabled = fixture.run(&args, Some(value));
        assert_eq!(disabled.status.code(), default.status.code());
        assert_eq!(disabled.stdout, default.stdout);
        assert_eq!(disabled.stderr, default.stderr);
    }
    let enabled = fixture.run(&args, Some("1"));
    assert_eq!(enabled.status.code(), default.status.code());
    assert_eq!(enabled.stdout, default.stdout);
    assert_eq!(ordinary_stderr(&enabled).as_bytes(), default.stderr);
    let record = only_record(&enabled);
    assert_eq!(record["status"], "error");
    for name in [
        "cli_parse",
        "cli_config_setup",
        "build_total",
        "build_setup",
    ] {
        assert_eq!(phase(&record, name)["calls"], 1);
    }
    assert_eq!(phase(&record, "tex_subprocess")["calls"], 0);
}

#[test]
fn successful_json_stdout_is_identical_with_one_stderr_record() {
    let fixture = Fixture::new();
    fs::write(fixture.0.join("main.tex"), "Text $x$.\n").unwrap();
    let args = ["lint", "main.tex", "--report-json", "--allow-warnings"];
    let default = fixture.run(&args, None);
    let enabled = fixture.run(&args, Some("1"));
    assert!(default.status.success() && enabled.status.success());
    assert!(default.stderr.is_empty());
    assert_eq!(default.stdout, enabled.stdout);
    assert!(ordinary_stderr(&enabled).is_empty());
    let report: Value = serde_json::from_slice(&enabled.stdout).unwrap();
    assert_eq!(report["warning_count"], 2);
    assert_eq!(only_record(&enabled)["status"], "success");
}

#[test]
fn existing_check_exit_preserves_json_and_emits_after_work() {
    let fixture = Fixture::new();
    fs::write(fixture.0.join("main.tex"), "\\begin{proof}\n").unwrap();
    let args = ["check", "main.tex", "--report-json"];
    let default = fixture.run(&args, None);
    let enabled = fixture.run(&args, Some("1"));
    assert_eq!(default.status.code(), Some(1));
    assert_eq!(enabled.status.code(), default.status.code());
    assert_eq!(enabled.stdout, default.stdout);
    assert_eq!(ordinary_stderr(&enabled).as_bytes(), default.stderr);
    let report: Value = serde_json::from_slice(&enabled.stdout).unwrap();
    assert!(report["error_count"].as_u64().unwrap() > 0);
    let record = only_record(&enabled);
    assert_eq!(record["status"], "exit_failure");
    assert_eq!(phase(&record, "cli_config_setup")["calls"], 1);
    assert_eq!(phase(&record, "build_total")["calls"], 0);
}

#[test]
fn malformed_state_observes_failed_load_without_launching_engine() {
    let fixture = Fixture::new();
    fs::write(fixture.0.join("main.tex"), "Unused source.\n").unwrap();
    fs::create_dir(fixture.0.join("build")).unwrap();
    fs::write(fixture.0.join("build/.tekai-main.state.toml"), "[").unwrap();
    let args = ["build", "main.tex", "--report-json"];
    let default = fixture.run(&args, None);
    let enabled = fixture.run(&args, Some("1"));
    assert_eq!(default.status.code(), Some(1));
    assert_eq!(enabled.status.code(), default.status.code());
    assert_eq!(enabled.stdout, default.stdout);
    assert_eq!(ordinary_stderr(&enabled).as_bytes(), default.stderr);
    assert!(ordinary_stderr(&enabled).contains("failed to parse build state"));
    let record = only_record(&enabled);
    assert_eq!(record["status"], "error");
    assert_eq!(phase(&record, "build_state_load")["calls"], 1);
    assert_eq!(phase(&record, "tex_subprocess")["calls"], 0);
}

#[test]
fn clap_early_exit_is_unchanged_and_does_not_claim_completed_work() {
    let fixture = Fixture::new();
    let default = fixture.run(&["--version"], None);
    let enabled = fixture.run(&["--version"], Some("1"));
    assert!(default.status.success() && enabled.status.success());
    assert_eq!(default.stdout, enabled.stdout);
    assert_eq!(default.stderr, enabled.stderr);
    assert!(records(&enabled).is_empty());
}
