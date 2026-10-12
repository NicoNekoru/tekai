//! Fixed, invocation-owned diagnostic observations. The CLI opts in explicitly.
//! Nested codec observations overlap the library lifecycle. They are not gate
//! samples, and no record is produced when an unexpected panic propagates.

#![warn(clippy::all)]

use std::cell::Cell;
use std::io::{self, Write};
use std::rc::Rc;
use std::time::{Duration, Instant};

const LIMIT: u64 = i64::MAX as u64;
const RECORD_BYTES: usize = 4096;
pub const ENV: &str = "TEKAI_ENGINE_PROFILE";
pub const PREFIX: &str = "TEKAI_ENGINE_PROFILE ";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Phase {
    OwnerConstruction,
    InvocationSetup,
    EngineBody,
    Cleanup,
    OwnerDrop,
    PngDecode,
    PdfCompression,
}

const PHASES: [(Phase, &str); 7] = [
    (Phase::OwnerConstruction, "owner_construction"),
    (Phase::InvocationSetup, "invocation_setup"),
    (Phase::EngineBody, "engine_body"),
    (Phase::Cleanup, "cleanup"),
    (Phase::OwnerDrop, "owner_drop"),
    (Phase::PngDecode, "png_decode"),
    (Phase::PdfCompression, "pdf_compression"),
];

#[derive(Clone, Copy)]
pub(crate) enum Counter {
    DecodeCalls,
    CacheHits,
    DecodedBytes,
    CompressionCalls,
    CompressionInputBytes,
    CompressionOutputBytes,
}

const COUNTERS: [(Counter, &str, &str); 6] = [
    (Counter::DecodeCalls, "decode_calls", "calls"),
    (Counter::CacheHits, "cache_hits", "hits"),
    (Counter::DecodedBytes, "decoded_bytes", "bytes"),
    (Counter::CompressionCalls, "compression_calls", "calls"),
    (
        Counter::CompressionInputBytes,
        "compression_input_bytes",
        "bytes",
    ),
    (
        Counter::CompressionOutputBytes,
        "compression_output_bytes",
        "bytes",
    ),
];

#[derive(Clone, Copy, Debug, Default)]
struct Total {
    calls: u64,
    failed_calls: u64,
    elapsed_ns: u64,
}

/// A completed observation, including ordinary failed calls.
#[derive(Clone, Copy, Debug)]
pub struct PhaseObservation {
    pub name: &'static str,
    pub calls: u64,
    pub failed_calls: u64,
    pub elapsed_ns: Option<u64>,
}

/// A fixed counter. PNG cache hits count the bounded cache, not an already
/// decoded live image. Compression byte counts are actual consumed/produced
/// bytes, rather than input slices offered again after partial progress.
#[derive(Clone, Copy, Debug)]
pub struct CounterObservation {
    pub name: &'static str,
    pub value: u64,
    pub unit: &'static str,
}

/// Immutable diagnostic data returned only after all invocation owners drop.
#[derive(Clone, Debug)]
pub struct EngineProfile {
    status: &'static str,
    /// Library invocation only. CLI preparation is outside this measurement.
    elapsed_ns: u64,
    phases: [PhaseObservation; 7],
    counters: [CounterObservation; 6],
}

pub(crate) struct Collector {
    started: Instant,
    boundary: Cell<Instant>,
    lifecycle: Cell<Option<Phase>>,
    finished: Cell<Option<Instant>>,
    totals: [Cell<Total>; 7],
    counters: [Cell<u64>; 6],
}

fn nanos(duration: Duration) -> u64 {
    duration.as_nanos().min(u128::from(LIMIT)) as u64
}

impl Collector {
    pub(crate) fn new() -> Rc<Self> {
        let started = Instant::now();
        Rc::new(Self {
            started,
            boundary: Cell::new(started),
            lifecycle: Cell::new(Some(Phase::OwnerConstruction)),
            finished: Cell::new(None),
            totals: std::array::from_fn(|_| Cell::new(Total::default())),
            counters: std::array::from_fn(|_| Cell::new(0)),
        })
    }

    fn complete(&self, phase: Phase, elapsed: Duration, failed: bool) {
        let slot = &self.totals[phase as usize];
        let mut total = slot.get();
        total.calls = total.calls.saturating_add(1).min(LIMIT);
        total.failed_calls = total
            .failed_calls
            .saturating_add(u64::from(failed))
            .min(LIMIT);
        total.elapsed_ns = total.elapsed_ns.saturating_add(nanos(elapsed)).min(LIMIT);
        slot.set(total);
    }

    /// One timestamp closes the old segment and starts the next. Error paths
    /// leave unreached phases unavailable instead of inventing zero work.
    pub(crate) fn transition(&self, next: Phase, failed: bool) {
        self.transition_at(next, failed, Instant::now());
    }

    fn transition_at(&self, next: Phase, failed: bool, now: Instant) {
        if let Some(previous) = self.lifecycle.replace(Some(next)) {
            self.complete(previous, now.duration_since(self.boundary.get()), failed);
        }
        self.boundary.set(now);
    }

    pub(crate) fn finish(&self, failed: bool) {
        self.finish_at(failed, Instant::now());
    }

    fn finish_at(&self, failed: bool, now: Instant) {
        if let Some(previous) = self.lifecycle.take() {
            self.complete(previous, now.duration_since(self.boundary.get()), failed);
        }
        self.finished.set(Some(now));
    }

    pub(crate) fn count(&self, counter: Counter, amount: usize) {
        let slot = &self.counters[counter as usize];
        slot.set(slot.get().saturating_add(amount as u64).min(LIMIT));
    }

    pub(crate) fn snapshot(&self, status: &'static str) -> EngineProfile {
        EngineProfile {
            status,
            elapsed_ns: nanos(
                self.finished
                    .get()
                    .expect("profile must finish before snapshot")
                    .duration_since(self.started),
            ),
            phases: PHASES.map(|(phase, name)| {
                let total = self.totals[phase as usize].get();
                PhaseObservation {
                    name,
                    calls: total.calls,
                    failed_calls: total.failed_calls,
                    elapsed_ns: (total.calls != 0).then_some(total.elapsed_ns),
                }
            }),
            counters: COUNTERS.map(|(counter, name, unit)| CounterObservation {
                name,
                value: self.counters[counter as usize].get(),
                unit,
            }),
        }
    }
}

pub(crate) struct Span {
    state: Option<(Rc<Collector>, Phase, Instant)>,
    failed: bool,
}

impl Span {
    fn new(collector: Option<Rc<Collector>>, phase: Phase) -> Self {
        Self {
            state: collector.map(|collector| (collector, phase, Instant::now())),
            failed: true,
        }
    }

    pub(crate) fn success(&mut self) {
        self.failed = false;
    }
}

impl Drop for Span {
    fn drop(&mut self) {
        if let Some((collector, phase, started)) = &self.state {
            collector.complete(*phase, started.elapsed(), self.failed);
        }
    }
}

pub(crate) fn span(phase: Phase) -> Span {
    Span::new(crate::control::profile_collector(), phase)
}

pub(crate) fn transition(next: Phase, failed: bool) {
    if let Some(collector) = crate::control::profile_collector() {
        collector.transition(next, failed);
    }
}

pub(crate) fn count(counter: Counter, amount: usize) {
    if let Some(collector) = crate::control::profile_collector() {
        collector.count(counter, amount);
    }
}

struct RecordBuffer {
    data: [u8; RECORD_BYTES],
    length: usize,
}

impl Write for RecordBuffer {
    fn write(&mut self, input: &[u8]) -> io::Result<usize> {
        let Some(end) = self
            .length
            .checked_add(input.len())
            .filter(|&end| end <= self.data.len())
        else {
            return Err(io::Error::other(
                "engine profile record exceeds fixed bound",
            ));
        };
        self.data[self.length..end].copy_from_slice(input);
        self.length = end;
        Ok(input.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl EngineProfile {
    pub fn status(&self) -> &'static str {
        self.status
    }

    pub fn elapsed_ns(&self) -> u64 {
        self.elapsed_ns
    }

    pub fn phases(&self) -> &[PhaseObservation; 7] {
        &self.phases
    }

    pub fn counters(&self) -> &[CounterObservation; 6] {
        &self.counters
    }

    /// Write one bounded record. Callers decide whether to emit it. Diagnostic
    /// sink errors must not replace the engine's result. No input text or paths
    /// are serialized, and CLI preparation never enters library elapsed time.
    pub fn write_record(
        &self,
        writer: &mut impl Write,
        worker_preparation: Option<Duration>,
    ) -> io::Result<()> {
        let mut record = RecordBuffer {
            data: [0; RECORD_BYTES],
            length: 0,
        };
        write!(record, "TEKAI_ENGINE_PROFILE {{\"schema_version\":1,\"producer\":\"tekai-engine-rust\",\"scope\":\"engine_invocation\",\"source\":\"opt_in_rust_instrumentation\",\"untrusted\":true,\"status\":\"{}\",\"elapsed_ns\":{},\"completed_spans_only\":true,\"phases_overlap\":true,\"lifecycle_partition_on_success\":true,\"worker_preparation\":", self.status, self.elapsed_ns)?;
        if let Some(preparation) = worker_preparation {
            write!(
                record,
                "{{\"scope\":\"cli_worker_before_invocation\",\"elapsed_ns\":{}}}",
                nanos(preparation)
            )?;
        } else {
            write!(record, "null")?;
        }
        write!(record, ",\"phases\":[")?;
        for (index, phase) in self.phases.iter().enumerate() {
            if index != 0 {
                write!(record, ",")?;
            }
            write!(record, "{{\"name\":\"{}\",\"status\":\"{}\",\"calls\":{},\"failed_calls\":{},\"elapsed_ns\":", phase.name, if phase.elapsed_ns.is_some() { "available" } else { "unavailable" }, phase.calls, phase.failed_calls)?;
            if let Some(elapsed) = phase.elapsed_ns {
                write!(record, "{elapsed}")?;
            } else {
                write!(record, "null")?;
            }
            write!(record, "}}")?;
        }
        write!(record, "],\"counters\":[")?;
        for (index, counter) in self.counters.iter().enumerate() {
            if index != 0 {
                write!(record, ",")?;
            }
            write!(
                record,
                "{{\"name\":\"{}\",\"value\":{},\"unit\":\"{}\"}}",
                counter.name, counter.value, counter.unit
            )?;
        }
        writeln!(record, "]}}")?;
        writer.write_all(&record.data[..record.length])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_boundaries_partition_completed_lifecycle_and_leave_unreached_phases_null() {
        let collector = Collector::new();
        let start = collector.started;
        for (index, phase) in [
            Phase::InvocationSetup,
            Phase::EngineBody,
            Phase::Cleanup,
            Phase::OwnerDrop,
        ]
        .into_iter()
        .enumerate()
        {
            collector.transition_at(
                phase,
                false,
                start + Duration::from_nanos((index as u64 + 1) * 10),
            );
        }
        collector.finish_at(false, start + Duration::from_nanos(50));
        let report = collector.snapshot("success");
        assert_eq!(report.elapsed_ns, 50);
        assert_eq!(
            report.phases[..5]
                .iter()
                .map(|phase| phase.elapsed_ns.unwrap())
                .sum::<u64>(),
            50
        );
        assert!(report.phases[..5]
            .iter()
            .all(|phase| phase.calls == 1 && phase.failed_calls == 0));
        assert!(report.phases[5..]
            .iter()
            .all(|phase| phase.calls == 0 && phase.elapsed_ns.is_none()));
    }

    #[test]
    fn disabled_observations_have_no_collector_or_clock() {
        assert!(span(Phase::PngDecode).state.is_none());
        count(Counter::DecodeCalls, 1);
        transition(Phase::EngineBody, false);
    }

    #[test]
    fn records_are_bounded_and_failed_nested_calls_remain_visible() {
        let collector = Collector::new();
        drop(Span::new(Some(collector.clone()), Phase::PngDecode));
        collector.count(Counter::DecodeCalls, 1);
        collector.finish(true);
        let report = collector.snapshot("setup_error");
        assert_eq!(report.phases[5].calls, 1);
        assert_eq!(report.phases[5].failed_calls, 1);
        let mut bytes = Vec::new();
        report
            .write_record(&mut bytes, Some(Duration::from_millis(2)))
            .unwrap();
        assert!(bytes.len() <= RECORD_BYTES);
        assert_eq!(bytes.iter().filter(|&&byte| byte == b'\n').count(), 1);
        assert!(bytes.starts_with(b"TEKAI_ENGINE_PROFILE {"));
        assert!(bytes.ends_with(b"]}\n"));
        assert!(std::str::from_utf8(&bytes)
            .unwrap()
            .contains("\"elapsed_ns\":2000000"));
    }
}
