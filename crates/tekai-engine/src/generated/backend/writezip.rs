// Rust translation and subsequent modifications for Tekai.
// Upstream TeX Live source revision 52dac44dde13a8310bbb8231e81c58be28cebe20.
// Origin texk/web2c/pdftexdir/writezip.c.
// GPL-2.0-or-later.
// Modified for Tekai. Licensing and provenance notice restored 2026-10-13.
// Original copyrights and license terms are retained in the crate's
// NOTICE.md and licenses/upstream-notices.txt.

//! Invocation-owned PDF compression with the legacy output batching protocol.

use crate::generated::pdftex0::getpdfcompresslevel;

use crate::utils::{xfflush, xfwrite};
use flate2::{Compress, Compression, FlushCompress, Status};
use std::cell::Cell;
use std::ffi::CString;
use std::fmt;
use std::marker::{PhantomData, PhantomPinned};
use std::pin::Pin;
use std::ptr::NonNull;
use std::rc::Rc;

const ZIP_BUF_SIZE: usize = 131_072;
const TEKAI_MAX_FAST_COMPRESS_LEVEL: u32 = 1;

#[derive(Debug)]
pub(crate) enum CompressionError {
    MissingScope,
    AlreadyScoped,
    BusyScope,
    InvalidLevel(i32),
    InvalidState(&'static str),
    NoProgress,
    LengthOverflow,
    Codec(flate2::CompressError),
}

impl fmt::Display for CompressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingScope => f.write_str("no active PDF compression scope"),
            Self::AlreadyScoped => f.write_str("nested PDF compression scope"),
            Self::BusyScope => f.write_str("recursive PDF compressor access"),
            Self::InvalidLevel(level) => write!(f, "invalid PDF compression level {level}"),
            Self::InvalidState(message) => f.write_str(message),
            Self::NoProgress => f.write_str("PDF compressor made no progress"),
            Self::LengthOverflow => f.write_str("PDF compression length overflow"),
            Self::Codec(error) => write!(f, "PDF compression failed, {error}"),
        }
    }
}

impl std::error::Error for CompressionError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Phase {
    Idle,
    Active,
    Finished,
}

/// Owns the codec and one bounded output buffer for an engine invocation.
///
/// Pinning keeps the non-owning scoped pointer stable. Dropping this value
/// releases unfinished codec state without writing a trailer or finalizing PDF.
pub(crate) struct CompressionSession {
    compressor: Option<Compress>,
    output: Box<[u8; ZIP_BUF_SIZE]>,
    pending: usize,
    phase: Phase,
    _pinned: PhantomPinned,
}

thread_local! {
    static ACTIVE: ActiveState = const { ActiveState {
        pointer: Cell::new(None),
        busy: Cell::new(false),
    } };
}

struct ActiveState {
    pointer: Cell<Option<NonNull<CompressionSession>>>,
    busy: Cell<bool>,
}

struct BusyScope<'a>(&'a Cell<bool>);

impl Drop for BusyScope<'_> {
    fn drop(&mut self) {
        self.0.set(false);
    }
}

/// Borrows a pinned session and unregisters it on return or unwind.
///
/// This guard stays on its creating thread. The engine must return before the
/// guard is dropped, and the guard must be dropped before its session owner.
struct CompressionScope<'a> {
    pointer: NonNull<CompressionSession>,
    _borrow: PhantomData<&'a mut CompressionSession>,
    _same_thread: PhantomData<Rc<()>>,
}

impl Drop for CompressionScope<'_> {
    fn drop(&mut self) {
        // Pointer removal is independent of the busy flag. Even an unexpected
        // callback drop cannot panic or retain a stale pointer while borrowed.
        let _ = ACTIVE.try_with(|active| {
            if active.pointer.get() == Some(self.pointer) {
                active.pointer.set(None);
            }
        });
    }
}

impl CompressionSession {
    pub(crate) fn new() -> Pin<Box<Self>> {
        let mut output = Box::<[u8; ZIP_BUF_SIZE]>::new_uninit();
        // SAFETY all-zero bytes are a valid byte array. Initialize directly in
        // the allocation, not a 128 KiB temporary on the caller's stack.
        unsafe { std::ptr::write_bytes(output.as_mut_ptr(), 0, 1) };
        let output = unsafe { output.assume_init() };
        Box::pin(Self {
            compressor: None,
            output,
            pending: 0,
            phase: Phase::Idle,
            _pinned: PhantomPinned,
        })
    }

    pub(crate) fn scope<T>(
        session: Pin<&mut Self>,
        action: impl FnOnce() -> T,
    ) -> Result<T, CompressionError> {
        let _scope = Self::enter(session)?;
        Ok(action())
    }

    fn enter(session: Pin<&mut Self>) -> Result<CompressionScope<'_>, CompressionError> {
        ACTIVE.with(|active| {
            if active.busy.get() {
                return Err(CompressionError::BusyScope);
            }
            if active.pointer.get().is_some() {
                return Err(CompressionError::AlreadyScoped);
            }
            // SAFETY The guard retains this exclusive pinned borrow until it
            // unregisters the pointer. No method moves the session or fields.
            let pointer = NonNull::from(unsafe { session.get_unchecked_mut() });
            active.pointer.set(Some(pointer));
            Ok(CompressionScope {
                pointer,
                _borrow: PhantomData,
                _same_thread: PhantomData,
            })
        })
    }

    fn start(&mut self, level: i32) -> Result<(), CompressionError> {
        if level <= 0 {
            return Err(CompressionError::InvalidLevel(level));
        }
        if self.phase == Phase::Active {
            return Err(CompressionError::InvalidState(
                "PDF compression stream restarted before completion",
            ));
        }
        let level = (level as u32).min(TEKAI_MAX_FAST_COMPRESS_LEVEL);
        if let Some(compressor) = &mut self.compressor {
            // All positive requested levels clamp to one, so reset preserves
            // the same codec configuration and starts with zero totals.
            compressor.reset();
        } else {
            self.compressor = Some(Compress::new(Compression::new(level), true));
        }
        self.pending = 0;
        self.phase = Phase::Active;
        Ok(())
    }

    fn emit(
        &mut self,
        sink: &mut impl FnMut(&[u8]) -> Result<(), CompressionError>,
    ) -> Result<(), CompressionError> {
        if self.pending != 0 {
            sink(&self.output[..self.pending])?;
            self.pending = 0;
        }
        Ok(())
    }

    fn process(
        &mut self,
        mut input: &[u8],
        finish: bool,
        sink: &mut impl FnMut(&[u8]) -> Result<(), CompressionError>,
    ) -> Result<u64, CompressionError> {
        if self.phase != Phase::Active {
            return Err(CompressionError::InvalidState(
                "PDF compression input outside an active stream",
            ));
        }
        loop {
            // Preserve the old writer's timing. A full buffer at the end of a
            // nonfinal call remains pending until the next call begins.
            if self.pending == ZIP_BUF_SIZE {
                self.emit(sink)?;
            }
            let compressor = self
                .compressor
                .as_mut()
                .ok_or(CompressionError::InvalidState("missing PDF compressor"))?;
            let old_in = compressor.total_in();
            let old_out = compressor.total_out();
            // Only the codec call is timed. Sink writes, reset/allocation,
            // checked bookkeeping and flushing belong to the enclosing body.
            let mut compression_profile =
                crate::engine_profile::span(crate::engine_profile::Phase::PdfCompression);
            crate::engine_profile::count(crate::engine_profile::Counter::CompressionCalls, 1);
            let status = compressor
                .compress(
                    input,
                    &mut self.output[self.pending..],
                    if finish {
                        FlushCompress::Finish
                    } else {
                        FlushCompress::None
                    },
                )
                .map_err(CompressionError::Codec)?;
            compression_profile.success();
            drop(compression_profile);
            let total_out = compressor.total_out();
            let consumed = usize::try_from(
                compressor
                    .total_in()
                    .checked_sub(old_in)
                    .ok_or(CompressionError::LengthOverflow)?,
            )
            .map_err(|_| CompressionError::LengthOverflow)?;
            let produced = usize::try_from(
                total_out
                    .checked_sub(old_out)
                    .ok_or(CompressionError::LengthOverflow)?,
            )
            .map_err(|_| CompressionError::LengthOverflow)?;
            if consumed > input.len() || produced > ZIP_BUF_SIZE - self.pending {
                return Err(CompressionError::InvalidState(
                    "PDF compressor returned invalid buffer counts",
                ));
            }
            crate::engine_profile::count(
                crate::engine_profile::Counter::CompressionInputBytes,
                consumed,
            );
            crate::engine_profile::count(
                crate::engine_profile::Counter::CompressionOutputBytes,
                produced,
            );
            input = &input[consumed..];
            self.pending += produced;
            if status == Status::StreamEnd {
                if !finish || !input.is_empty() {
                    return Err(CompressionError::InvalidState(
                        "PDF compressor ended before consuming final input",
                    ));
                }
                self.emit(sink)?;
                self.phase = Phase::Finished;
                return Ok(total_out);
            }
            if status != Status::Ok || (consumed == 0 && produced == 0) {
                return Err(CompressionError::NoProgress);
            }
            if !finish && input.is_empty() {
                return Ok(total_out);
            }
        }
    }

    fn abort(&mut self) {
        // RAII drops partial codec state. Cleanup deliberately has no sink and
        // never attempts to complete a stream whose input may be incomplete.
        self.compressor = None;
        self.pending = 0;
        self.phase = Phase::Idle;
    }
}

/// # Safety
/// A registered session must still exist. Its owner must stay alive until the
/// action returns, even if a callback unregisters the scope. No other reference
/// may access that session during the action.
unsafe fn with_current<T>(
    action: impl FnOnce(&mut CompressionSession) -> Result<T, CompressionError>,
) -> Result<T, CompressionError> {
    ACTIVE.with(|active| {
        if active.busy.get() {
            return Err(CompressionError::BusyScope);
        }
        let mut pointer = active.pointer.get().ok_or(CompressionError::MissingScope)?;
        active.busy.set(true);
        let _busy = BusyScope(&active.busy);
        // SAFETY The caller keeps this pinned owner alive without aliases.
        // The busy guard rejects recursive access and new scopes, even after
        // a callback unregisters the pointer, and resets on return or unwind.
        action(unsafe { pointer.as_mut() })
    })
}

unsafe fn fail(error: CompressionError) -> ! {
    let message = CString::new(error.to_string())
        .expect("compression error messages contain no embedded NUL");
    crate::utils::pdftex_fail_args(
        b"%s\0".as_ptr().cast(),
        &[crate::utils::PrintfArg::from(message.as_ptr())],
    )
}

/// Compress the current PDF input buffer using the invocation-owned session.
///
/// # Safety
/// The engine must hold a live CompressionScope on this thread. Its PDF input
/// buffer and file must be valid. The session owner must remain alive until
/// this function returns, and callbacks must not access the session. The caller
/// retains responsibility for clearing pdfptr.
pub unsafe fn writezip(finish: i32) {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    if (*__tekai_state_generated_pdftexextra).pdfptr < 0
        || (*__tekai_state_generated_pdftexextra).pdfbufsize < 0
        || (*__tekai_state_generated_pdftexextra).pdfptr
            > (*__tekai_state_generated_pdftexextra).pdfbufsize
    {
        fail(CompressionError::InvalidState(
            "invalid PDF input buffer length",
        ));
    }
    let count = (*__tekai_state_generated_pdftexextra).pdfptr as usize;
    if count != 0 && (*__tekai_state_generated_pdftexextra).pdfbuf.is_null() {
        fail(CompressionError::InvalidState("missing PDF input buffer"));
    }
    let input = if count == 0 {
        &[]
    } else {
        std::slice::from_raw_parts((*__tekai_state_generated_pdftexextra).pdfbuf, count)
    };
    let level = getpdfcompresslevel();
    if level <= 0 {
        fail(CompressionError::InvalidLevel(level));
    }
    (*__tekai_state_utils).cur_file_name = std::ptr::null_mut();
    let result = with_current(|session| {
        let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
            unsafe { crate::generated::pdftexextra::state() };

        if (*__tekai_state_generated_pdftexextra).pdfstreamlength == 0 {
            session.start(level)?;
        }
        session.process(input, finish != 0, &mut |output| {
            let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
                unsafe { crate::generated::pdftexextra::state() };

            let written = xfwrite(
                output.as_ptr().cast_mut().cast(),
                1,
                output.len(),
                (*__tekai_state_generated_pdftexextra).pdffile,
            );
            if written != output.len() {
                return Err(CompressionError::InvalidState(
                    "short PDF compression write",
                ));
            }
            (*__tekai_state_generated_pdftexextra).pdfgone = (*__tekai_state_generated_pdftexextra)
                .pdfgone
                .checked_add(i64::try_from(written).map_err(|_| CompressionError::LengthOverflow)?)
                .ok_or(CompressionError::LengthOverflow)?;
            (*__tekai_state_generated_pdftexextra).pdflastbyte =
                *output.last().expect("output batches are nonempty");
            Ok(())
        })
    });
    let total_out = match result {
        Ok(total_out) => total_out,
        Err(error) => fail(error),
    };
    if finish != 0 {
        xfflush((*__tekai_state_generated_pdftexextra).pdffile);
    }
    (*__tekai_state_generated_pdftexextra).pdfstreamlength = match i64::try_from(total_out) {
        Ok(length) => length,
        Err(_) => fail(CompressionError::LengthOverflow),
    };
}

/// Release codec state during engine cleanup without finalizing partial PDF.
///
/// # Safety
/// If compression was used, its invocation scope must still be live on this
/// thread. Its owner must remain alive until this function returns, and no
/// callback may access the session.
pub unsafe fn zip_free() {
    match with_current(|session| {
        session.abort();
        Ok(())
    }) {
        Ok(()) | Err(CompressionError::MissingScope) => {}
        Err(error) => fail(error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::read::ZlibDecoder;
    use proptest::prelude::*;
    use std::io::Read;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    #[test]
    fn profile_counts_codec_calls_and_actual_compressed_byte_progress() {
        use crate::engine_profile::Collector;
        let profile = Collector::new();
        let mut control = crate::control::ControlSession::new_with_profile(
            crate::InterruptToken::default(),
            Some(profile.clone()),
        );
        let mut owner = CompressionSession::new();
        let input = b"bounded codec observation\n".repeat(20_000);
        let mut encoded = Vec::new();
        control
            .as_mut()
            .scope(|| {
                CompressionSession::scope(owner.as_mut(), || {
                    with_current(|session| {
                        assert!(session.start(0).is_err());
                        session.start(1)?;
                        let mut sink = |bytes: &[u8]| {
                            encoded.extend_from_slice(bytes);
                            Ok(())
                        };
                        session.process(&input, false, &mut sink)?;
                        session.process(&[], true, &mut sink)?;
                        Ok(())
                    })
                    .unwrap();
                })
                .unwrap();
            })
            .unwrap();
        drop(owner);
        drop(control);
        profile.finish(false);
        let report = profile.snapshot("success");
        let compressed = report
            .phases()
            .iter()
            .find(|phase| phase.name == "pdf_compression")
            .unwrap();
        assert!(compressed.calls >= 2);
        assert_eq!(compressed.failed_calls, 0);
        for (name, expected) in [
            ("compression_calls", compressed.calls),
            ("compression_input_bytes", input.len() as u64),
            ("compression_output_bytes", encoded.len() as u64),
        ] {
            assert_eq!(
                report
                    .counters()
                    .iter()
                    .find(|counter| counter.name == name)
                    .unwrap()
                    .value,
                expected,
                "{name}"
            );
        }
        let mut decoded = Vec::new();
        ZlibDecoder::new(encoded.as_slice())
            .read_to_end(&mut decoded)
            .unwrap();
        assert_eq!(decoded, input);
        assert!(crate::control::profile_collector().is_none());
    }

    fn with_current<T>(
        action: impl FnOnce(&mut CompressionSession) -> Result<T, CompressionError>,
    ) -> Result<T, CompressionError> {
        // SAFETY These tests keep each pinned owner alive around the action
        // and never access it through another reference during that action.
        unsafe { super::with_current(action) }
    }

    // The old Rust libz adapter is a compatibility oracle only. Its C-shaped
    // stream must have a stable address from initialization through teardown.
    struct Oracle {
        stream: Pin<Box<libz_rs_sys::z_stream>>,
        output: Box<[u8; ZIP_BUF_SIZE]>,
        batches: Vec<Vec<u8>>,
    }

    impl Oracle {
        fn new() -> Self {
            let mut stream = Box::pin(libz_rs_sys::z_stream::default());
            let status = unsafe {
                libz_rs_sys::deflateInit_(
                    stream.as_mut().get_mut(),
                    1,
                    libz_rs_sys::zlibVersion(),
                    std::mem::size_of::<libz_rs_sys::z_stream>() as i32,
                )
            };
            assert_eq!(status, libz_rs_sys::Z_OK);
            let mut oracle = Self {
                stream,
                output: Box::new([0; ZIP_BUF_SIZE]),
                batches: Vec::new(),
            };
            oracle.reset_output();
            oracle
        }

        fn reset_output(&mut self) {
            self.stream.next_out = self.output.as_mut_ptr();
            self.stream.avail_out = ZIP_BUF_SIZE as u32;
        }

        fn reset(&mut self) {
            assert_eq!(
                unsafe { libz_rs_sys::deflateReset(self.stream.as_mut().get_mut()) },
                libz_rs_sys::Z_OK
            );
            self.batches.clear();
            self.reset_output();
        }

        fn process(&mut self, input: &[u8], finish: bool) -> u64 {
            self.stream.next_in = input.as_ptr();
            self.stream.avail_in = input.len().try_into().unwrap();
            loop {
                if self.stream.avail_out == 0 {
                    self.batches.push(self.output.to_vec());
                    self.reset_output();
                }
                let status = unsafe {
                    libz_rs_sys::deflate(
                        self.stream.as_mut().get_mut(),
                        if finish {
                            libz_rs_sys::Z_FINISH
                        } else {
                            libz_rs_sys::Z_NO_FLUSH
                        },
                    )
                };
                if finish && status == libz_rs_sys::Z_STREAM_END {
                    break;
                }
                assert_eq!(status, libz_rs_sys::Z_OK);
                if !finish && self.stream.avail_in == 0 {
                    break;
                }
            }
            if finish {
                let pending = ZIP_BUF_SIZE - self.stream.avail_out as usize;
                if pending != 0 {
                    self.batches.push(self.output[..pending].to_vec());
                }
            }
            self.stream.total_out as u64
        }
    }

    impl Drop for Oracle {
        fn drop(&mut self) {
            // An aborted stream may report DATA_ERROR while still freeing its
            // allocation. No stream finalization is attempted during teardown.
            unsafe { libz_rs_sys::deflateEnd(self.stream.as_mut().get_mut()) };
        }
    }

    fn session_mut(session: &mut Pin<Box<CompressionSession>>) -> &mut CompressionSession {
        // SAFETY Test helpers mutate fields but never move the pinned owner.
        unsafe { session.as_mut().get_unchecked_mut() }
    }

    fn run(
        session: &mut CompressionSession,
        input: &[u8],
        chunk: usize,
        level: i32,
    ) -> (Vec<Vec<u8>>, Vec<u64>) {
        session.start(level).unwrap();
        let mut batches = Vec::new();
        let mut totals = Vec::new();
        let mut sink = |output: &[u8]| {
            batches.push(output.to_vec());
            Ok(())
        };
        for input in input.chunks(chunk) {
            totals.push(session.process(input, false, &mut sink).unwrap());
        }
        totals.push(session.process(&[], true, &mut sink).unwrap());
        (batches, totals)
    }

    fn oracle_run(oracle: &mut Oracle, input: &[u8], chunk: usize) -> (Vec<Vec<u8>>, Vec<u64>) {
        let mut totals = Vec::new();
        for input in input.chunks(chunk) {
            totals.push(oracle.process(input, false));
        }
        totals.push(oracle.process(&[], true));
        (oracle.batches.clone(), totals)
    }

    fn decoded(batches: &[Vec<u8>]) -> Vec<u8> {
        let compressed = batches.concat();
        let mut decoder = ZlibDecoder::new(compressed.as_slice());
        let mut result = Vec::new();
        decoder.read_to_end(&mut result).unwrap();
        result
    }

    fn random_bytes(len: usize) -> Vec<u8> {
        let mut state = 0xa832_72d1_u32;
        (0..len)
            .map(|_| {
                state ^= state << 13;
                state ^= state >> 17;
                state ^= state << 5;
                state as u8
            })
            .collect()
    }

    #[test]
    fn chunk_boundaries_preserve_bytes_batches_and_totals() {
        let input = random_bytes(3 * ZIP_BUF_SIZE + 17);
        for chunk in [
            1,
            127,
            ZIP_BUF_SIZE - 1,
            ZIP_BUF_SIZE,
            ZIP_BUF_SIZE + 1,
            input.len(),
        ] {
            let mut session = CompressionSession::new();
            let actual = run(session_mut(&mut session), &input, chunk, 1);
            let expected = oracle_run(&mut Oracle::new(), &input, chunk);
            assert_eq!(actual, expected, "chunk size {chunk}");
            assert_eq!(decoded(&actual.0), input);
            assert!(actual
                .0
                .iter()
                .all(|batch| !batch.is_empty() && batch.len() <= ZIP_BUF_SIZE));
            assert_eq!(
                actual.0.iter().map(Vec::len).sum::<usize>() as u64,
                *actual.1.last().unwrap()
            );
            assert!(actual.0.iter().any(|batch| batch.len() == ZIP_BUF_SIZE));
        }
    }

    #[test]
    fn pending_bytes_are_in_stream_length_but_not_emitted_accounting() {
        let mut session = CompressionSession::new();
        let session = session_mut(&mut session);
        session.start(1).unwrap();
        let input = random_bytes(3 * ZIP_BUF_SIZE + 29);
        let mut gone = 41_u64;
        let mut lastbyte = 0x80;
        let mut batches = Vec::new();
        for input in input.chunks(65_537) {
            let before_gone = gone;
            let before_lastbyte = lastbyte;
            let before_batches = batches.len();
            let total = session
                .process(input, false, &mut |output| {
                    gone += output.len() as u64;
                    lastbyte = *output.last().unwrap();
                    batches.push(output.to_vec());
                    Ok(())
                })
                .unwrap();
            assert_eq!(total, gone - 41 + session.pending as u64);
            if batches.len() == before_batches {
                assert_eq!(gone, before_gone);
                assert_eq!(lastbyte, before_lastbyte);
            }
        }
        let total = session
            .process(&[], true, &mut |output| {
                gone += output.len() as u64;
                lastbyte = *output.last().unwrap();
                batches.push(output.to_vec());
                Ok(())
            })
            .unwrap();
        assert_eq!(total, gone - 41);
        assert_eq!(session.pending, 0);
        assert_eq!(lastbyte, *batches.last().unwrap().last().unwrap());
        assert_eq!(decoded(&batches), input);
    }

    #[test]
    fn empty_nonfinal_input_matches_header_then_no_progress_behavior() {
        let mut session = CompressionSession::new();
        let session = session_mut(&mut session);
        session.start(1).unwrap();
        let mut oracle = Oracle::new();
        let total = session.process(&[], false, &mut |_| Ok(())).unwrap();
        assert_eq!(total, oracle.process(&[], false));
        assert_eq!(session.pending as u64, total);
        assert!(matches!(
            session.process(&[], false, &mut |_| Ok(())),
            Err(CompressionError::NoProgress)
        ));
        assert_eq!(
            unsafe {
                libz_rs_sys::deflate(oracle.stream.as_mut().get_mut(), libz_rs_sys::Z_NO_FLUSH)
            },
            libz_rs_sys::Z_BUF_ERROR
        );
        let mut batches = Vec::new();
        let total = session
            .process(&[], true, &mut |output| {
                batches.push(output.to_vec());
                Ok(())
            })
            .unwrap();
        assert_eq!(total, oracle.process(&[], true));
        assert_eq!(batches, oracle.batches);
        assert!(decoded(&batches).is_empty());
    }

    #[test]
    fn reset_preserves_level_one_clamp_and_empty_streams() {
        let mut session = CompressionSession::new();
        let mut oracle = Oracle::new();
        for (index, level) in [1, 2, 9, i32::MAX].into_iter().enumerate() {
            if index != 0 {
                oracle.reset();
            }
            let input = if index == 2 {
                Vec::new()
            } else {
                vec![b'A'; 19_217]
            };
            let actual = run(session_mut(&mut session), &input, 7, level);
            let expected = oracle_run(&mut oracle, &input, 7);
            assert_eq!(actual, expected);
            assert_eq!(decoded(&actual.0), input);
        }
    }

    #[test]
    fn partial_abort_drops_state_without_output_and_restarts_fresh() {
        let mut session = CompressionSession::new();
        let session = session_mut(&mut session);
        session.start(1).unwrap();
        let mut emitted = Vec::new();
        session
            .process(b"incomplete PDF stream", false, &mut |output| {
                emitted.push(output.to_vec());
                Ok(())
            })
            .unwrap();
        assert!(emitted.is_empty());
        assert!(session.pending > 0);
        session.abort();
        assert!(session.compressor.is_none());
        assert_eq!(session.pending, 0);
        assert_eq!(session.phase, Phase::Idle);
        assert!(emitted.is_empty());
        let input = random_bytes(100_001);
        let actual = run(session, &input, 101, 1);
        let mut fresh = CompressionSession::new();
        assert_eq!(actual, run(session_mut(&mut fresh), &input, 101, 1));
        assert_eq!(decoded(&actual.0), input);
    }

    #[test]
    fn sink_failure_can_abort_then_start_a_fresh_compressor() {
        let mut session = CompressionSession::new();
        let session = session_mut(&mut session);
        session.start(1).unwrap();
        assert!(session
            .process(&random_bytes(2 * ZIP_BUF_SIZE), true, &mut |_| {
                Err(CompressionError::InvalidState("test sink failed"))
            })
            .is_err());
        session.abort();
        let input = b"fresh after failed output";
        let actual = run(session, input, 5, 1);
        assert_eq!(decoded(&actual.0), input);
        assert_eq!(actual, oracle_run(&mut Oracle::new(), input, 5));
    }

    #[test]
    fn scopes_reject_nesting_and_recursion_and_clear_on_unwind() {
        assert!(matches!(
            with_current(|_| Ok(())),
            Err(CompressionError::MissingScope)
        ));
        let mut first = CompressionSession::new();
        {
            let _scope = CompressionSession::enter(first.as_mut()).unwrap();
            let mut second = CompressionSession::new();
            assert!(matches!(
                CompressionSession::enter(second.as_mut()),
                Err(CompressionError::AlreadyScoped)
            ));
            with_current(|session| {
                session.start(1)?;
                assert!(matches!(
                    with_current(|_| Ok(())),
                    Err(CompressionError::BusyScope)
                ));
                Ok(())
            })
            .unwrap();
        }
        assert!(matches!(
            with_current(|_| Ok(())),
            Err(CompressionError::MissingScope)
        ));
        let unwind = catch_unwind(AssertUnwindSafe(|| {
            let _scope = CompressionSession::enter(first.as_mut()).unwrap();
            with_current::<()>(|session| {
                session.process(b"partial", false, &mut |_| Ok(()))?;
                panic!("test invocation unwind")
            })
            .unwrap();
        }));
        assert!(unwind.is_err());
        assert!(matches!(
            with_current(|_| Ok(())),
            Err(CompressionError::MissingScope)
        ));
        drop(first);
        let mut fresh = CompressionSession::new();
        let _scope = CompressionSession::enter(fresh.as_mut()).unwrap();
    }

    #[test]
    fn compression_constructor_fits_a_small_caller_stack() {
        std::thread::Builder::new()
            .stack_size(128 * 1024)
            .spawn(|| {
                let mut session = CompressionSession::new();
                CompressionSession::scope(session.as_mut(), || {
                    with_current(|state| {
                        assert_eq!(state.phase, Phase::Idle);
                        assert!(state.output.iter().all(|&byte| byte == 0));
                        Ok(())
                    })
                    .unwrap();
                })
                .unwrap();
            })
            .unwrap()
            .join()
            .unwrap();
    }

    #[test]
    fn callback_guard_drop_unregisters_even_while_busy_and_after_panic() {
        for panic_after_drop in [false, true] {
            let mut owner = CompressionSession::new();
            let scope = CompressionSession::enter(owner.as_mut()).unwrap();
            let mut other = CompressionSession::new();
            let outcome = catch_unwind(AssertUnwindSafe(|| {
                with_current(|session| {
                    drop(scope);
                    assert!(ACTIVE.with(|active| active.pointer.get().is_none()));
                    assert!(matches!(
                        with_current(|_| Ok(())),
                        Err(CompressionError::BusyScope)
                    ));
                    assert!(matches!(
                        CompressionSession::enter(other.as_mut()),
                        Err(CompressionError::BusyScope)
                    ));
                    session.start(1)?;
                    session.process(b"still live owner", false, &mut |_| Ok(()))?;
                    if panic_after_drop {
                        panic!("test callback panic after scope drop");
                    }
                    Ok(())
                })
            }));
            assert_eq!(outcome.is_err(), panic_after_drop);
            assert!(ACTIVE.with(|active| !active.busy.get()));
            assert!(matches!(
                with_current(|_| Ok(())),
                Err(CompressionError::MissingScope)
            ));
            let _scope = CompressionSession::enter(owner.as_mut()).unwrap();
            with_current(|session| {
                session.abort();
                Ok(())
            })
            .unwrap();
        }
    }

    #[test]
    fn stream_protocol_rejects_invalid_transitions_and_levels() {
        let mut session = CompressionSession::new();
        let session = session_mut(&mut session);
        assert!(session.process(b"x", false, &mut |_| Ok(())).is_err());
        assert!(matches!(
            session.start(0),
            Err(CompressionError::InvalidLevel(0))
        ));
        assert!(matches!(
            session.start(-1),
            Err(CompressionError::InvalidLevel(-1))
        ));
        session.start(1).unwrap();
        assert!(session.start(1).is_err());
        session.process(&[], true, &mut |_| Ok(())).unwrap();
        assert!(session.process(b"x", false, &mut |_| Ok(())).is_err());
        session.start(1).unwrap();
        session.abort();
        session.start(1).unwrap();
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(48))]
        #[test]
        fn bounded_generated_inputs_match_old_rust_codec(
            input in prop::collection::vec(any::<u8>(), 0..32_769),
            chunk in 1usize..8_193,
            level in 1i32..21,
        ) {
            let mut session = CompressionSession::new();
            let actual = run(session_mut(&mut session), &input, chunk, level);
            let expected = oracle_run(&mut Oracle::new(), &input, chunk);
            prop_assert_eq!(&actual, &expected);
            prop_assert_eq!(decoded(&actual.0), input);
            prop_assert!(actual.0.iter().all(|batch| batch.len() <= ZIP_BUF_SIZE));
        }
    }
}
