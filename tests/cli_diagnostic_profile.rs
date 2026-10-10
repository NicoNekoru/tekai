//! Protocol tests use CLI-only paths and never launch a TeX engine.

use std::collections::BTreeSet;
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
            // Leave one byte within the fixed record bound for its newline.
            assert!(PREFIX.len() + json.len() < 16384);
            serde_json::from_str(json).expect("invalid profile JSON")
        })
        .collect()
}

fn only_record(output: &Output) -> Value {
    let records = records(output);
    assert_eq!(records.len(), 1, "{output:#?}");
    let record = records.into_iter().next().unwrap();
    assert_eq!(record["schema_version"], 2);
    assert_eq!(record["producer"], "tekai-rust");
    assert_eq!(record["scope"], "cli_process");
    assert_eq!(record["source"], "opt_in_rust_instrumentation");
    assert_eq!(record["untrusted"], true);
    assert_eq!(record["completed_spans_only"], true);
    assert_eq!(record["phases_overlap"], true);
    assert!(record["elapsed_ms"].as_f64().unwrap().is_finite());
    assert!(record["elapsed_ms"].as_f64().unwrap() >= 0.0);
    let phases = record["phases"].as_array().unwrap();
    let expected_phases = BTreeSet::from([
        "cli_parse",
        "cli_config_setup",
        "build_total",
        "build_setup",
        "build_state_load",
        "build_cache_checks",
        "input_freshness",
        "settled_cache_restore",
        "tex_subprocess",
        "build_state_read",
        "build_state_parse",
        "input_metadata",
        "input_content_hash",
        "input_effective_hash",
        "input_freshness_memo",
        "lookup_session_reset",
        "cli_report_serialization",
        "build_mode_key",
        "embedded_engine_entry",
        "format_initialization",
    ]);
    assert_eq!(phases.len(), expected_phases.len());
    assert_eq!(
        phases
            .iter()
            .map(|phase| phase["name"].as_str().unwrap())
            .collect::<BTreeSet<_>>(),
        expected_phases
    );
    for phase in phases {
        let name = phase["name"].as_str().unwrap();
        let (scope, unavailable_reason) = match name {
            "embedded_engine_entry" => (
                "native_engine_internal",
                "native_engine_exits_without_returning_to_Rust",
            ),
            "format_initialization" => (
                "native_engine_internal",
                "not_separately_instrumented_inside_native_engine",
            ),
            "tex_subprocess" => (
                "subprocess_launch_and_wait",
                "no_completed_call_in_this_process",
            ),
            _ => (
                "inclusive_wall_time_current_process",
                "no_completed_call_in_this_process",
            ),
        };
        assert_eq!(phase["scope"], scope);
        assert!(!phase["source"].as_str().unwrap().is_empty());
        assert!(phase["calls"].as_u64().is_some());
        assert_eq!(phase["active_calls"], 0);
        if let Some(elapsed) = phase["elapsed_ms"].as_f64() {
            assert_eq!(phase["status"], "available");
            assert_eq!(phase.get("reason"), Some(&Value::Null));
            assert!(phase["calls"].as_u64().unwrap() > 0);
            assert!(elapsed.is_finite() && elapsed >= 0.0);
        } else {
            assert_eq!(phase["status"], "unavailable");
            assert_eq!(phase.get("elapsed_ms"), Some(&Value::Null));
            assert_eq!(phase["calls"], 0);
            assert_eq!(phase["reason"], unavailable_reason);
        }
    }
    let expected_counters = BTreeSet::from([
        ("state_bytes_read", "bytes"),
        ("state_dependencies", "recorded_inputs"),
        ("freshness_requests", "requests"),
        ("freshness_memo_hits", "requests"),
        ("freshness_memo_misses", "requests"),
        ("metadata_checks", "calls"),
        ("metadata_matches", "freshness_routes"),
        ("content_hash_fallbacks", "freshness_routes"),
        ("effective_hash_fallbacks", "freshness_routes"),
        ("virtual_freshness_checks", "freshness_routes"),
        ("stale_inputs", "observed_stale_inputs"),
        ("warm_cache_hits", "builds"),
    ]);
    let counters = record["counters"].as_array().unwrap();
    assert_eq!(counters.len(), expected_counters.len());
    assert_eq!(
        counters
            .iter()
            .map(|counter| (
                counter["name"].as_str().unwrap(),
                counter["unit"].as_str().unwrap()
            ))
            .collect::<BTreeSet<_>>(),
        expected_counters
    );
    for counter in counters {
        assert!(counter["value"].as_u64().is_some());
        assert_eq!(counter["scope"], "aggregate_observations_current_process");
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
    assert_eq!(phase(&record, "build_state_read")["calls"], 1);
    assert_eq!(phase(&record, "build_state_parse")["calls"], 1);
    let counter = |name| {
        record["counters"]
            .as_array()
            .unwrap()
            .iter()
            .find(|counter| counter["name"] == name)
            .unwrap()["value"]
            .as_u64()
            .unwrap()
    };
    assert_eq!(counter("state_bytes_read"), 1);
    assert_eq!(counter("state_dependencies"), 0);
    assert_eq!(counter("freshness_requests"), 0);
    assert_eq!(counter("warm_cache_hits"), 0);
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
