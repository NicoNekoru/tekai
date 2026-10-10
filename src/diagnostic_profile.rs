//! Bounded, opt-in wall-time observations. These never affect build decisions.

use std::ffi::OsStr;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use serde::Serialize;

pub const ENV: &str = "TEKAI_DIAGNOSTIC_PROFILE";
const PREFIX: &str = "TEKAI_PROFILE ";
const MAX_RECORD_BYTES: usize = 16384;
static SESSION: OnceLock<Option<Session>> = OnceLock::new();

#[derive(Copy, Clone)]
pub enum Phase {
    CliParse,
    CliConfigSetup,
    BuildTotal,
    BuildSetup,
    BuildStateLoad,
    BuildCacheChecks,
    InputFreshness,
    SettledCacheRestore,
    TexSubprocess,
    BuildStateRead,
    BuildStateParse,
    InputMetadata,
    InputContentHash,
    InputEffectiveHash,
    InputFreshnessMemo,
    LookupSessionReset,
    CliReportSerialization,
    BuildModeKey,
}

const PHASES: [(Phase, &str, &str); 18] = [
    (
        Phase::CliParse,
        "cli_parse",
        "main::run_cli argument parsing",
    ),
    (
        Phase::CliConfigSetup,
        "cli_config_setup",
        "main CLI configuration and option setup",
    ),
    (Phase::BuildTotal, "build_total", "compiler::build"),
    (
        Phase::BuildSetup,
        "build_setup",
        "compiler direct build path, directory and mode-key setup",
    ),
    (
        Phase::BuildStateLoad,
        "build_state_load",
        "compiler::read_build_state_if_exists",
    ),
    (
        Phase::BuildCacheChecks,
        "build_cache_checks",
        "compiler direct build initial cache decision",
    ),
    (
        Phase::InputFreshness,
        "input_freshness",
        "compiler::build_state_inputs_are_fresh",
    ),
    (
        Phase::SettledCacheRestore,
        "settled_cache_restore",
        "compiler::restore_settled_aux_cache_if_fresh",
    ),
    (
        Phase::TexSubprocess,
        "tex_subprocess",
        "compiler TeX command.status calls, including preamble compilation",
    ),
    (
        Phase::BuildStateRead,
        "build_state_read",
        "compiler build-state file read",
    ),
    (
        Phase::BuildStateParse,
        "build_state_parse",
        "compiler build-state TOML parse",
    ),
    (
        Phase::InputMetadata,
        "input_metadata",
        "compiler::file_metadata_fingerprint",
    ),
    (
        Phase::InputContentHash,
        "input_content_hash",
        "compiler::file_content_hash_hex read and hash",
    ),
    (
        Phase::InputEffectiveHash,
        "input_effective_hash",
        "compiler freshness fallback effective-source read and hash",
    ),
    (
        Phase::InputFreshnessMemo,
        "input_freshness_memo",
        "compiler ordered freshness memo lookup and update",
    ),
    (
        Phase::LookupSessionReset,
        "lookup_session_reset",
        "compiler::clear_kpathsea_resolution_cache",
    ),
    (
        Phase::CliReportSerialization,
        "cli_report_serialization",
        "main build-report JSON conversion and serialization",
    ),
    (
        Phase::BuildModeKey,
        "build_mode_key",
        "compiler::direct_mode_key",
    ),
];

#[derive(Copy, Clone)]
pub enum Counter {
    StateBytesRead,
    StateDependencies,
    FreshnessRequests,
    FreshnessMemoHits,
    FreshnessMemoMisses,
    MetadataChecks,
    MetadataMatches,
    ContentHashFallbacks,
    EffectiveHashFallbacks,
    VirtualFreshnessChecks,
    StaleInputs,
    WarmCacheHits,
}

const COUNTERS: [(Counter, &str, &str); 12] = [
    (Counter::StateBytesRead, "state_bytes_read", "bytes"),
    (
        Counter::StateDependencies,
        "state_dependencies",
        "recorded_inputs",
    ),
    (Counter::FreshnessRequests, "freshness_requests", "requests"),
    (
        Counter::FreshnessMemoHits,
        "freshness_memo_hits",
        "requests",
    ),
    (
        Counter::FreshnessMemoMisses,
        "freshness_memo_misses",
        "requests",
    ),
    (Counter::MetadataChecks, "metadata_checks", "calls"),
    (
        Counter::MetadataMatches,
        "metadata_matches",
        "freshness_routes",
    ),
    (
        Counter::ContentHashFallbacks,
        "content_hash_fallbacks",
        "freshness_routes",
    ),
    (
        Counter::EffectiveHashFallbacks,
        "effective_hash_fallbacks",
        "freshness_routes",
    ),
    (
        Counter::VirtualFreshnessChecks,
        "virtual_freshness_checks",
        "freshness_routes",
    ),
    (
        Counter::StaleInputs,
        "stale_inputs",
        "observed_stale_inputs",
    ),
    (Counter::WarmCacheHits, "warm_cache_hits", "builds"),
];

#[derive(Copy, Clone, Default)]
struct Totals {
    elapsed: Duration,
    calls: u64,
    active_calls: u64,
}

struct Session {
    started: Instant,
    totals: Mutex<[Totals; PHASES.len()]>,
    counters: Mutex<[u64; COUNTERS.len()]>,
    emitted: AtomicBool,
}

/// Initialize only at CLI entry, before project configuration can change env.
/// Library callers retain disabled instrumentation unless they opt in here.
pub fn initialize() {
    SESSION.get_or_init(|| enabled(std::env::var_os(ENV).as_deref()).then(Session::new));
}

fn enabled(value: Option<&OsStr>) -> bool {
    value == Some(OsStr::new("1"))
}

pub struct Span<'a> {
    state: Option<(&'a Session, Phase, Instant)>,
}

/// Disabled spans do not read the environment, allocate or start a clock.
pub fn span(phase: Phase) -> Span<'static> {
    SESSION
        .get()
        .and_then(Option::as_ref)
        .map(|session| session.span(phase))
        .unwrap_or(Span { state: None })
}

pub fn measure<T>(phase: Phase, work: impl FnOnce() -> T) -> T {
    let _span = span(phase);
    work()
}

/// Disabled counters use no clock, allocation or lock. Enabled state has a
/// fixed inventory, regardless of project size or the number of dependencies.
pub fn count(counter: Counter, amount: u64) {
    if let Some(session) = SESSION.get().and_then(Option::as_ref) {
        session.count(counter, amount);
    }
}

/// Best-effort stderr only. A broken diagnostic sink must not change exit codes.
pub fn emit(status: &'static str) {
    if let Some(session) = SESSION.get().and_then(Option::as_ref) {
        let _ = session.emit_to(status, &mut io::stderr().lock());
    }
}

impl Session {
    fn new() -> Self {
        Self {
            started: Instant::now(),
            totals: Mutex::new([Totals::default(); PHASES.len()]),
            counters: Mutex::new([0; COUNTERS.len()]),
            emitted: AtomicBool::new(false),
        }
    }

    fn span(&self, phase: Phase) -> Span<'_> {
        if let Ok(mut totals) = self.totals.lock() {
            totals[phase as usize].active_calls =
                totals[phase as usize].active_calls.saturating_add(1);
            Span {
                state: Some((self, phase, Instant::now())),
            }
        } else {
            Span { state: None }
        }
    }

    fn complete(&self, phase: Phase, elapsed: Duration) {
        if let Ok(mut totals) = self.totals.lock() {
            let total = &mut totals[phase as usize];
            total.active_calls = total.active_calls.saturating_sub(1);
            total.calls = total.calls.saturating_add(1);
            total.elapsed = total.elapsed.saturating_add(elapsed);
        }
    }

    fn count(&self, counter: Counter, amount: u64) {
        if let Ok(mut counters) = self.counters.lock() {
            counters[counter as usize] = counters[counter as usize].saturating_add(amount);
        }
    }

    fn emit_to(&self, status: &'static str, writer: &mut impl Write) -> io::Result<()> {
        if self.emitted.swap(true, Ordering::Relaxed) {
            return Ok(());
        }
        let totals = *self
            .totals
            .lock()
            .map_err(|_| io::Error::other("profile state poisoned"))?;
        let mut phases = Vec::with_capacity(PHASES.len() + 2);
        for (phase, name, source) in PHASES {
            let total = totals[phase as usize];
            phases.push(PhaseRecord {
                name,
                status: if total.calls == 0 {
                    "unavailable"
                } else {
                    "available"
                },
                elapsed_ms: (total.calls > 0).then_some(total.elapsed.as_secs_f64() * 1000.0),
                calls: total.calls,
                active_calls: total.active_calls,
                source,
                scope: if matches!(phase, Phase::TexSubprocess) {
                    "subprocess_launch_and_wait"
                } else {
                    "inclusive_wall_time_current_process"
                },
                reason: (total.calls == 0).then_some("no_completed_call_in_this_process"),
            });
        }
        for (name, reason) in [
            (
                "embedded_engine_entry",
                "native_engine_exits_without_returning_to_Rust",
            ),
            (
                "format_initialization",
                "not_separately_instrumented_inside_native_engine",
            ),
        ] {
            phases.push(PhaseRecord {
                name,
                status: "unavailable",
                elapsed_ms: None,
                calls: 0,
                active_calls: 0,
                source: "native exact engine, outside parent process instrumentation",
                scope: "native_engine_internal",
                reason: Some(reason),
            });
        }
        let counters = *self
            .counters
            .lock()
            .map_err(|_| io::Error::other("profile counters poisoned"))?;
        let record = Record {
            schema_version: 2,
            producer: "tekai-rust",
            scope: "cli_process",
            source: "opt_in_rust_instrumentation",
            untrusted: true,
            status,
            elapsed_ms: self.started.elapsed().as_secs_f64() * 1000.0,
            completed_spans_only: true,
            phases_overlap: true,
            phases,
            counters: COUNTERS
                .into_iter()
                .map(|(counter, name, unit)| CounterRecord {
                    name,
                    value: counters[counter as usize],
                    unit,
                    scope: "aggregate_observations_current_process",
                })
                .collect(),
        };
        let json = serde_json::to_vec(&record).map_err(io::Error::other)?;
        if PREFIX.len() + json.len() + 1 > MAX_RECORD_BYTES {
            return Err(io::Error::other("profile record exceeds fixed bound"));
        }
        let mut line = Vec::with_capacity(PREFIX.len() + json.len() + 1);
        line.extend_from_slice(PREFIX.as_bytes());
        line.extend_from_slice(&json);
        line.push(b'\n');
        writer.write_all(&line)
    }
}

impl Drop for Span<'_> {
    fn drop(&mut self) {
        if let Some((session, phase, started)) = self.state {
            session.complete(phase, started.elapsed());
        }
    }
}

#[derive(Serialize)]
struct Record {
    schema_version: u32,
    producer: &'static str,
    scope: &'static str,
    source: &'static str,
    untrusted: bool,
    status: &'static str,
    elapsed_ms: f64,
    completed_spans_only: bool,
    phases_overlap: bool,
    phases: Vec<PhaseRecord>,
    counters: Vec<CounterRecord>,
}

#[derive(Serialize)]
struct CounterRecord {
    name: &'static str,
    value: u64,
    unit: &'static str,
    scope: &'static str,
}

#[derive(Serialize)]
struct PhaseRecord {
    name: &'static str,
    status: &'static str,
    elapsed_ms: Option<f64>,
    calls: u64,
    active_calls: u64,
    source: &'static str,
    scope: &'static str,
    reason: Option<&'static str>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    fn record(session: &Session) -> Value {
        let mut output = Vec::new();
        session.emit_to("success", &mut output).unwrap();
        let line = std::str::from_utf8(&output).unwrap();
        assert_eq!(line.lines().count(), 1);
        assert!(output.len() <= MAX_RECORD_BYTES);
        serde_json::from_str(line.strip_prefix(PREFIX).unwrap()).unwrap()
    }

    #[test]
    fn only_exact_one_enables_profiling() {
        assert!(enabled(Some(OsStr::new("1"))));
        for value in [
            None,
            Some(""),
            Some("0"),
            Some("true"),
            Some(" 1"),
            Some("1\n"),
        ] {
            assert!(!enabled(value.map(OsStr::new)));
        }
    }

    #[test]
    fn inventory_is_bounded_and_unobserved_phases_are_unavailable() {
        let record = record(&Session::new());
        assert_eq!(record["schema_version"], 2);
        assert_eq!(record["untrusted"], true);
        assert_eq!(record["phases_overlap"], true);
        let phases = record["phases"].as_array().unwrap();
        assert_eq!(phases.len(), PHASES.len() + 2);
        for phase in phases {
            assert_eq!(phase["status"], "unavailable");
            assert!(phase["elapsed_ms"].is_null());
            assert_eq!(phase["calls"], 0);
            assert!(!phase["reason"].as_str().unwrap().is_empty());
        }
        assert!(record["elapsed_ms"].as_f64().unwrap().is_finite());
        assert_eq!(record["counters"].as_array().unwrap().len(), COUNTERS.len());
    }

    #[test]
    fn completed_spans_aggregate_and_active_spans_stay_explicit() {
        let session = Session::new();
        let active = session.span(Phase::BuildTotal);
        drop(session.span(Phase::CliParse));
        drop(session.span(Phase::CliParse));
        let record = record(&session);
        let phases = record["phases"].as_array().unwrap();
        assert_eq!(phases[0]["calls"], 2);
        assert_eq!(phases[0]["active_calls"], 0);
        assert!(phases[0]["elapsed_ms"].as_f64().unwrap() >= 0.0);
        assert_eq!(phases[2]["calls"], 0);
        assert_eq!(phases[2]["active_calls"], 1);
        assert!(phases[2]["elapsed_ms"].is_null());
        drop(active);
    }

    #[test]
    fn error_returns_drop_the_span_without_changing_the_error() {
        let session = Session::new();
        let work = || -> Result<(), &'static str> {
            let _span = session.span(Phase::BuildStateLoad);
            Err("unchanged error")
        };
        assert_eq!(work(), Err("unchanged error"));
        assert_eq!(record(&session)["phases"][4]["calls"], 1);
    }

    #[test]
    fn emission_is_once_even_when_the_sink_fails() {
        struct FailedSink;
        impl Write for FailedSink {
            fn write(&mut self, _: &[u8]) -> io::Result<usize> {
                Err(io::Error::other("closed stderr"))
            }
            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }
        let session = Session::new();
        assert!(session.emit_to("error", &mut FailedSink).is_err());
        let mut output = Vec::new();
        session.emit_to("success", &mut output).unwrap();
        assert!(output.is_empty());
        let session = Session::new();
        session.emit_to("success", &mut output).unwrap();
        session.emit_to("error", &mut output).unwrap();
        assert_eq!(std::str::from_utf8(&output).unwrap().lines().count(), 1);
    }

    #[test]
    fn concurrent_calls_share_only_fixed_totals() {
        let session = Session::new();
        std::thread::scope(|scope| {
            for _ in 0..4 {
                let session = &session;
                scope.spawn(move || {
                    for _ in 0..8 {
                        drop(session.span(Phase::TexSubprocess));
                    }
                });
            }
        });
        let record = record(&session);
        assert_eq!(record["phases"][8]["calls"], 32);
        assert_eq!(record["phases"][8]["active_calls"], 0);
        assert_eq!(record["phases"][8]["scope"], "subprocess_launch_and_wait");
    }

    #[test]
    fn totals_saturate_without_wrapping_or_nonfinite_output() {
        let session = Session::new();
        session.totals.lock().unwrap()[0] = Totals {
            elapsed: Duration::MAX,
            calls: u64::MAX,
            active_calls: 0,
        };
        session.complete(Phase::CliParse, Duration::from_secs(1));
        let record = record(&session);
        assert_eq!(record["phases"][0]["calls"].as_u64(), Some(u64::MAX));
        assert!(
            record["phases"][0]["elapsed_ms"]
                .as_f64()
                .unwrap()
                .is_finite()
        );
    }

    #[test]
    fn counters_have_fixed_inventory_and_saturate() {
        let session = Session::new();
        session.count(Counter::MetadataChecks, u64::MAX);
        session.count(Counter::MetadataChecks, 1);
        let record = record(&session);
        let counters = record["counters"].as_array().unwrap();
        assert_eq!(counters.len(), COUNTERS.len());
        assert_eq!(
            counters[Counter::MetadataChecks as usize]["value"].as_u64(),
            Some(u64::MAX)
        );
        assert_eq!(counters[Counter::WarmCacheHits as usize]["value"], 0);
    }
}
