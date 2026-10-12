//! Invocation-owned clocks and timezone rules, with no native date buffers.
//!
//! Jiff parses owned TZif/POSIX rules and converts Gregorian dates in Rust.
//! Its system-zone and database caches are deliberately unused. A timezone is
//! snapshot belongs to the invocation. Local rules are parsed only on their
//! first use, and that result remains fixed until teardown. UTC-only dates and
//! help/version exits do not read or validate irrelevant local rules.
//! Leap-bearing TZif files and implicit DST rules that require reinterpreting
//! host transitions are configuration errors, rather than guessed timestamps.

#![warn(clippy::all)]

use jiff::{tz::TimeZone, Timestamp};
use std::cell::Cell;
use std::ffi::{c_char, OsStr, OsString};
use std::fmt;
use std::io::{self, Read};
use std::marker::{PhantomData, PhantomPinned};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::ptr::NonNull;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_TZIF_BYTES: usize = 1024 * 1024;
const MAX_TZ_STRING_BYTES: usize = 4096;

#[derive(Clone, Debug)]
pub(crate) enum TimeError {
    Missing,
    Nested,
    Busy,
    InvalidEpoch(OsString),
    Configuration(String),
    ClockRange,
    CalendarRange,
}

impl fmt::Display for TimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing => f.write_str("no active time invocation scope"),
            Self::Nested => f.write_str("nested time invocation scope"),
            Self::Busy => f.write_str("recursive time state access"),
            Self::InvalidEpoch(value) => write!(f, "invalid epoch-seconds-timezone value for environment variable $SOURCE_DATE_EPOCH: {}", value.to_string_lossy()),
            Self::Configuration(message) => write!(f, "timezone configuration: {message}"),
            Self::ClockRange => f.write_str("system clock is outside the supported signed-second range"),
            Self::CalendarRange => f.write_str("date is outside the supported calendar range; PDF dates require years 0001 through 9999"),
        }
    }
}

impl std::error::Error for TimeError {}

/// A copied, NUL-terminated PDF date. Pointers borrow the caller's stack value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PdfDate {
    bytes: [u8; 30],
    len: usize,
}

impl PdfDate {
    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }

    pub(crate) fn as_ptr(&self) -> *const c_char {
        self.bytes.as_ptr().cast()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Calendar {
    pub(crate) year: i32,
    pub(crate) month: i32,
    pub(crate) day: i32,
    pub(crate) hour: i32,
    pub(crate) minute: i32,
    pub(crate) second: i32,
    pub(crate) offset_seconds: i32,
}

fn calendar(seconds: i64, zone: &TimeZone) -> Result<Calendar, TimeError> {
    let stamp = Timestamp::from_second(seconds).map_err(|_| TimeError::CalendarRange)?;
    let offset = zone.to_offset(stamp);
    let date = offset.to_datetime(stamp);
    Ok(Calendar {
        year: i32::from(date.year()),
        month: i32::from(date.month()),
        day: i32::from(date.day()),
        hour: i32::from(date.hour()),
        minute: i32::from(date.minute()),
        second: i32::from(date.second()),
        offset_seconds: offset.seconds(),
    })
}

fn put_digits(destination: &mut [u8], mut value: u32) {
    for digit in destination.iter_mut().rev() {
        *digit = b'0' + (value % 10) as u8;
        value /= 10;
    }
}

fn pdf_date(seconds: i64, zone: &TimeZone) -> Result<PdfDate, TimeError> {
    let date = calendar(seconds, zone)?;
    if !(1..=9999).contains(&date.year) {
        return Err(TimeError::CalendarRange);
    }
    let mut result = PdfDate {
        bytes: [0; 30],
        len: 16,
    };
    result.bytes[..2].copy_from_slice(b"D:");
    put_digits(&mut result.bytes[2..6], date.year as u32);
    for (at, value) in [
        (6, date.month),
        (8, date.day),
        (10, date.hour),
        (12, date.minute),
        (14, date.second),
    ] {
        put_digits(&mut result.bytes[at..at + 2], value as u32);
    }
    // Preserve the former formatter's minute precision, including historical
    // second offsets. Its value was the difference of the two clock minutes.
    let minutes =
        (seconds + i64::from(date.offset_seconds)).div_euclid(60) - seconds.div_euclid(60);
    if minutes == 0 {
        result.bytes[16] = b'Z';
        result.len = 17;
    } else {
        result.bytes[16] = if minutes < 0 { b'-' } else { b'+' };
        let magnitude = minutes.unsigned_abs();
        put_digits(&mut result.bytes[17..19], (magnitude / 60) as u32);
        result.bytes[19] = b'\'';
        put_digits(&mut result.bytes[20..22], (magnitude % 60) as u32);
        result.bytes[22] = b'\'';
        result.len = 23;
    }
    Ok(result)
}

fn seconds_and_micros_at(clock: SystemTime) -> Result<(i64, i32), TimeError> {
    match clock.duration_since(UNIX_EPOCH) {
        Ok(duration) => Ok((
            i64::try_from(duration.as_secs()).map_err(|_| TimeError::ClockRange)?,
            duration.subsec_micros() as i32,
        )),
        Err(error) => {
            let duration = error.duration();
            let whole = i128::from(duration.as_secs());
            let fraction = duration.subsec_nanos();
            let seconds = -whole - i128::from(fraction != 0);
            let micros = if fraction == 0 {
                0
            } else {
                ((1_000_000_000 - fraction) / 1000) as i32
            };
            Ok((
                i64::try_from(seconds).map_err(|_| TimeError::ClockRange)?,
                micros,
            ))
        }
    }
}

pub(crate) fn seconds_and_micros() -> Result<(i64, i32), TimeError> {
    seconds_and_micros_at(SystemTime::now())
}

struct TimeState {
    tz: Option<OsString>,
    tzdir: Option<OsString>,
    zone: Option<Result<TimeZone, TimeError>>,
    source_epoch: Option<OsString>,
    force_source: Option<OsString>,
    start: Option<i64>,
    creation: Option<PdfDate>,
    force_applied: bool,
}

impl TimeState {
    fn local_zone(&mut self) -> Result<&TimeZone, TimeError> {
        if self.zone.is_none() {
            self.zone = Some(resolve_zone(self.tz.as_deref(), self.tzdir.as_deref()));
        }
        self.zone.as_ref().unwrap().as_ref().map_err(Clone::clone)
    }

    fn start(&mut self) -> Result<i64, TimeError> {
        if let Some(start) = self.start {
            return Ok(start);
        }
        let start = match &self.source_epoch {
            Some(value) => crate::integer_parse::source_date_epoch(value.as_bytes())
                .map(|epoch| epoch as i64)
                .ok_or_else(|| TimeError::InvalidEpoch(value.clone()))?,
            None => seconds_and_micros()?.0,
        };
        self.start = Some(start);
        Ok(start)
    }

    fn creation(&mut self) -> Result<PdfDate, TimeError> {
        if let Some(date) = self.creation {
            return Ok(date);
        }
        let start = self.start()?;
        let zone = if self.source_epoch.is_some() {
            &TimeZone::UTC
        } else {
            self.local_zone()?
        };
        let date = pdf_date(start, zone)?;
        self.creation = Some(date);
        Ok(date)
    }

    fn tex_date(&mut self) -> Result<Calendar, TimeError> {
        if self
            .force_source
            .as_deref()
            .is_some_and(|value| value.as_bytes() == b"1")
        {
            let start = self.start()?;
            self.force_applied = true;
            calendar(start, &TimeZone::UTC)
        } else {
            calendar(seconds_and_micros()?.0, self.local_zone()?)
        }
    }
}

pub(crate) struct TimeSession {
    state: TimeState,
    _thread_confined: PhantomData<Rc<()>>,
    _pinned: PhantomPinned,
}

struct Active {
    pointer: Cell<Option<NonNull<TimeSession>>>,
    busy: Cell<bool>,
}

thread_local! {
    static ACTIVE: Active = const { Active { pointer: Cell::new(None), busy: Cell::new(false) } };
}

struct ScopeGuard;
impl Drop for ScopeGuard {
    fn drop(&mut self) {
        ACTIVE.with(|active| active.pointer.set(None));
    }
}

struct BusyGuard<'a>(&'a Cell<bool>);
impl Drop for BusyGuard<'_> {
    fn drop(&mut self) {
        self.0.set(false);
    }
}

impl TimeSession {
    pub(crate) fn new() -> Result<Pin<Box<Self>>, TimeError> {
        Ok(Box::pin(Self {
            state: TimeState {
                tz: crate::environment::var_os("TZ"),
                tzdir: crate::environment::var_os("TZDIR"),
                zone: None,
                source_epoch: crate::environment::var_os("SOURCE_DATE_EPOCH"),
                force_source: crate::environment::var_os("FORCE_SOURCE_DATE"),
                start: None,
                creation: None,
                force_applied: false,
            },
            _thread_confined: PhantomData,
            _pinned: PhantomPinned,
        }))
    }

    pub(crate) fn scope<T>(
        session: Pin<&mut Self>,
        action: impl FnOnce() -> T,
    ) -> Result<T, TimeError> {
        ACTIVE.with(|active| {
            if active.busy.get() {
                return Err(TimeError::Busy);
            }
            if active.pointer.get().is_some() {
                return Err(TimeError::Nested);
            }
            // SAFETY The exclusive pinned borrow lasts through the private
            // guard and action. Neither function moves the owner's fields.
            active
                .pointer
                .set(Some(NonNull::from(unsafe { session.get_unchecked_mut() })));
            Ok(())
        })?;
        let _guard = ScopeGuard;
        Ok(action())
    }
}

fn with_state<T>(
    action: impl FnOnce(&mut TimeState) -> Result<T, TimeError>,
) -> Result<T, TimeError> {
    ACTIVE.with(|active| {
        if active.busy.get() {
            return Err(TimeError::Busy);
        }
        let mut pointer = active.pointer.get().ok_or(TimeError::Missing)?;
        active.busy.set(true);
        let _guard = BusyGuard(&active.busy);
        // SAFETY A registered pinned thread-confined owner remains borrowed
        // by scope(). BusyGuard rejects a recursively aliased mutable access.
        action(unsafe { &mut pointer.as_mut().state })
    })
}

pub(crate) fn initialize_start() -> Result<(), TimeError> {
    with_state(|state| state.start().map(|_| ()))
}

pub(crate) fn creation_date() -> Result<PdfDate, TimeError> {
    with_state(TimeState::creation)
}

pub(crate) fn tex_date() -> Result<Calendar, TimeError> {
    with_state(TimeState::tex_date)
}

pub(crate) fn modification_date(seconds: i64) -> Result<PdfDate, TimeError> {
    with_state(|state| {
        let utc = TimeZone::UTC;
        pdf_date(
            seconds,
            if state.force_applied && state.source_epoch.is_some() {
                &utc
            } else {
                state.local_zone()?
            },
        )
    })
}

pub(crate) fn invalid_force_source() -> Result<Option<OsString>, TimeError> {
    with_state(|state| {
        Ok(state
            .force_source
            .as_ref()
            .filter(|value| !matches!(value.as_bytes(), b"" | b"0" | b"1"))
            .cloned())
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct FileMetadata {
    pub(crate) len: u64,
    pub(crate) modified_seconds: i64,
}

pub(crate) fn file_metadata(path: &Path) -> io::Result<FileMetadata> {
    let data = std::fs::metadata(crate::working_directory::resolve(path)?)?;
    Ok(FileMetadata {
        len: data.len(),
        modified_seconds: data.mtime(),
    })
}

/// The generated pool reserves one byte and marks a full pool on failure.
pub(crate) fn pool_end(start: i32, capacity: i32, len: usize) -> Option<i32> {
    let end = usize::try_from(start).ok()?.checked_add(len)?;
    (end < usize::try_from(capacity).ok()?)
        .then(|| i32::try_from(end).ok())
        .flatten()
}

fn read_tzif(path: &Path) -> Result<Vec<u8>, TimeError> {
    let path = crate::working_directory::resolve(path)
        .map_err(|error| TimeError::Configuration(error.to_string()))?;
    let file = std::fs::File::open(&path).map_err(|error| {
        TimeError::Configuration(format!("cannot read {}: {error}", path.display()))
    })?;
    if !file
        .metadata()
        .map_err(|error| TimeError::Configuration(error.to_string()))?
        .is_file()
    {
        return Err(TimeError::Configuration(format!(
            "{} is not a regular TZif file",
            path.display()
        )));
    }
    let mut bytes = Vec::new();
    file.take((MAX_TZIF_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|error| TimeError::Configuration(error.to_string()))?;
    if bytes.len() > MAX_TZIF_BYTES {
        return Err(TimeError::Configuration(
            "TZif file exceeds the 1 MiB limit".into(),
        ));
    }
    Ok(bytes)
}

#[derive(Clone, Copy)]
struct TzifBlock {
    time_width: usize,
    time_count: usize,
    type_count: usize,
    indices: usize,
    types: usize,
    end: usize,
    leap_count: usize,
}

fn tzif_blocks(bytes: &[u8]) -> Option<Vec<TzifBlock>> {
    let mut blocks = Vec::with_capacity(2);
    let mut header = 0usize;
    for width in [4usize, 8] {
        let prefix = bytes.get(header..header.checked_add(44)?)?;
        if &prefix[..4] != b"TZif" {
            return None;
        }
        let count = |at: usize| -> Option<usize> {
            Some(u32::from_be_bytes(prefix.get(at..at + 4)?.try_into().ok()?) as usize)
        };
        let leap_count = count(28)?;
        let time_count = count(32)?;
        let type_count = count(36)?;
        let indices = header
            .checked_add(44)?
            .checked_add(time_count.checked_mul(width)?)?;
        let types = indices.checked_add(time_count)?;
        let end = types
            .checked_add(type_count.checked_mul(6)?)?
            .checked_add(count(40)?)?
            .checked_add(leap_count.checked_mul(width + 4)?)?
            .checked_add(count(24)?)?
            .checked_add(count(20)?)?;
        if end > bytes.len() {
            return None;
        }
        blocks.push(TzifBlock {
            time_width: width,
            time_count,
            type_count,
            indices,
            types,
            end,
            leap_count,
        });
        if prefix[4] == 0 || width == 8 {
            break;
        }
        header = end;
    }
    Some(blocks)
}

fn zone_from_bytes(bytes: &[u8]) -> Result<TimeZone, TimeError> {
    let zone = TimeZone::tzif("Etc/Invocation", bytes)
        .map_err(|error| TimeError::Configuration(error.to_string()))?;
    let blocks = tzif_blocks(bytes)
        .ok_or_else(|| TimeError::Configuration("invalid TZif block bounds".into()))?;
    if blocks.iter().any(|block| block.leap_count != 0) {
        return Err(TimeError::Configuration("leap-second TZif files are unsupported; use the corresponding POSIX zone without right/".into()));
    }
    Ok(zone)
}

fn zone_roots(tzdir: Option<&OsStr>) -> Vec<PathBuf> {
    if let Some(directory) = tzdir {
        return vec![PathBuf::from(directory)];
    }
    [
        "/usr/share/zoneinfo",
        "/var/db/timezone/zoneinfo",
        "/usr/share/lib/zoneinfo",
        "/etc/zoneinfo",
    ]
    .map(PathBuf::from)
    .to_vec()
}

fn find_zone_file(name: &OsStr, roots: &[PathBuf]) -> Result<Option<Vec<u8>>, TimeError> {
    let path = Path::new(name);
    if path.is_absolute() {
        return read_tzif(path).map(Some);
    }
    for root in roots {
        let path = crate::working_directory::resolve(&root.join(path))
            .map_err(|error| TimeError::Configuration(error.to_string()))?;
        match std::fs::metadata(&path) {
            Ok(_) => return read_tzif(&path).map(Some),
            Err(error) if error.kind() == io::ErrorKind::NotFound => (),
            Err(error) => {
                return Err(TimeError::Configuration(format!(
                    "cannot read {}: {error}",
                    path.display()
                )))
            }
        }
    }
    Ok(None)
}

/// Recognize only the implicit-DST header. Jiff validates the same header by
/// parsing it with explicit dummy rules before we consider host posixrules.
fn implicit_offsets(text: &str) -> Option<(i32, i32)> {
    fn name(bytes: &[u8], at: &mut usize) -> Option<()> {
        let start = *at;
        if bytes.get(*at) == Some(&b'<') {
            *at += 1;
            while bytes.get(*at).is_some_and(|&byte| byte != b'>') {
                *at += 1;
            }
            if bytes.get(*at) != Some(&b'>') {
                return None;
            }
            *at += 1;
        } else {
            while bytes.get(*at).is_some_and(u8::is_ascii_alphabetic) {
                *at += 1;
            }
        }
        (*at > start).then_some(())
    }
    fn offset(bytes: &[u8], at: &mut usize) -> Option<i32> {
        let sign = match bytes.get(*at) {
            Some(b'-') => {
                *at += 1;
                -1
            }
            Some(b'+') => {
                *at += 1;
                1
            }
            _ => 1,
        };
        let mut total = 0i32;
        for part in 0..3 {
            let start = *at;
            let mut value = 0i32;
            while let Some(byte @ b'0'..=b'9') = bytes.get(*at) {
                value = value
                    .checked_mul(10)?
                    .checked_add(i32::from(*byte - b'0'))?;
                *at += 1;
            }
            if *at == start || (part > 0 && value > 59) {
                return None;
            }
            total = total.checked_add(value.checked_mul([3600, 60, 1][part])?)?;
            if part == 2 || bytes.get(*at) != Some(&b':') {
                break;
            }
            *at += 1;
        }
        total.checked_mul(-sign)
    }
    let bytes = text.as_bytes();
    let mut at = 0;
    name(bytes, &mut at)?;
    let standard = offset(bytes, &mut at)?;
    name(bytes, &mut at)?;
    let dst = if at == bytes.len() {
        standard.checked_add(3600)?
    } else {
        offset(bytes, &mut at)?
    };
    if at != bytes.len() {
        return None;
    }
    TimeZone::posix(&format!("{text},M3.2.0,M11.1.0")).ok()?;
    Some((standard, dst))
}

fn compatible_posixrules(
    mut bytes: Vec<u8>,
    offsets: (i32, i32),
) -> Result<Option<TimeZone>, TimeError> {
    zone_from_bytes(&bytes)?;
    let blocks = tzif_blocks(&bytes)
        .ok_or_else(|| TimeError::Configuration("invalid posixrules TZif".into()))?;
    for block in &blocks {
        // Native tzparse starts with an explicitly standard type. Also reject
        // tables long enough for tzcode's implicit 400-year extrapolation,
        // which Jiff would not infer after we remove the optional footer.
        if bytes[block.types + 4] != 0 {
            return Ok(None);
        }
        if block.time_count > 1 {
            let first = block.indices - block.time_count * block.time_width;
            let last = block.indices - block.time_width;
            let stamp = |at: usize| -> i64 {
                if block.time_width == 4 {
                    i64::from(i32::from_be_bytes(bytes[at..at + 4].try_into().unwrap()))
                } else {
                    i64::from_be_bytes(bytes[at..at + 8].try_into().unwrap())
                }
            };
            if i128::from(stamp(last)) - i128::from(stamp(first)) >= 12_622_780_800 {
                return Ok(None);
            }
        }
        for at in block.indices..block.indices + block.time_count {
            if usize::from(bytes[at]) >= block.type_count {
                return Err(TimeError::Configuration(
                    "invalid posixrules type index".into(),
                ));
            }
            let row = block.types + usize::from(bytes[at]) * 6;
            let dst = bytes[row + 4] != 0;
            let original = i32::from_be_bytes(bytes[row..row + 4].try_into().unwrap());
            if original != if dst { offsets.1 } else { offsets.0 } {
                return Ok(None);
            }
        }
        for kind in 0..block.type_count {
            let row = block.types + kind * 6;
            let offset = if bytes[row + 4] != 0 {
                offsets.1
            } else {
                offsets.0
            };
            bytes[row..row + 4].copy_from_slice(&offset.to_be_bytes());
        }
    }
    let last = blocks.last().unwrap();
    bytes.truncate(last.end);
    // Native tzparse loads default rules without extending their table using
    // the TZif footer. Keeping that footer changes implicit DST after its end.
    if blocks.len() == 2 {
        bytes.extend_from_slice(b"\n\n");
    }
    zone_from_bytes(&bytes).map(Some)
}

fn resolve_zone(tz: Option<&OsStr>, tzdir: Option<&OsStr>) -> Result<TimeZone, TimeError> {
    if tzdir.is_some_and(|directory| directory.as_bytes().len() > MAX_TZ_STRING_BYTES) {
        return Err(TimeError::Configuration(
            "TZDIR exceeds the 4096-byte limit".into(),
        ));
    }
    let Some(tz) = tz else {
        return zone_from_bytes(&read_tzif(Path::new("/etc/localtime"))?);
    };
    let mut bytes = tz.as_bytes();
    if bytes.is_empty() {
        return Ok(TimeZone::UTC);
    }
    if bytes.len() > MAX_TZ_STRING_BYTES {
        return Err(TimeError::Configuration(
            "TZ exceeds the 4096-byte limit".into(),
        ));
    }
    let file_only = bytes[0] == b':';
    if file_only {
        bytes = &bytes[1..];
    }
    let roots = zone_roots(tzdir);
    if let Some(data) = find_zone_file(OsStr::from_bytes(bytes), &roots)? {
        return zone_from_bytes(&data);
    }
    if file_only {
        return Err(TimeError::Configuration(
            "colon-prefixed TZ must name a readable TZif file".into(),
        ));
    }
    let text = std::str::from_utf8(bytes).map_err(|_| {
        TimeError::Configuration("non-UTF-8 TZ must name a readable TZif file".into())
    })?;
    if let Ok(zone) = TimeZone::posix(text) {
        return Ok(zone);
    }
    if let Some(offsets) = implicit_offsets(text) {
        if let Some(data) = find_zone_file(OsStr::new("posixrules"), &roots)? {
            if let Some(zone) = compatible_posixrules(data, offsets)? {
                return Ok(zone);
            }
        }
        return Err(TimeError::Configuration("TZ specifies DST without transition rules; no offset-compatible host posixrules file is available, so supply explicit start/end rules".into()));
    }
    Err(TimeError::Configuration(
        "TZ is neither a readable TZif file nor a supported POSIX timezone rule".into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::{self, EnvironmentSession};
    use proptest::prelude::*;
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn fixture_state(source: Option<&str>, force: Option<&str>, zone: &str) -> TimeState {
        TimeState {
            tz: Some(OsString::from(zone)),
            tzdir: None,
            zone: Some(Ok(TimeZone::posix(zone).unwrap())),
            source_epoch: source.map(OsString::from),
            force_source: force.map(OsString::from),
            start: None,
            creation: None,
            force_applied: false,
        }
    }

    fn environment_scope<T>(action: impl FnOnce() -> T) -> T {
        let mut owner = EnvironmentSession::new().unwrap();
        EnvironmentSession::scope(owner.as_mut(), action).unwrap()
    }

    fn configured_session(tz: &str, epoch: &str, force: &str) -> Pin<Box<TimeSession>> {
        for (name, value) in [
            ("TZ", tz),
            ("SOURCE_DATE_EPOCH", epoch),
            ("FORCE_SOURCE_DATE", force),
        ] {
            environment::set(OsStr::new(name), OsStr::new(value)).unwrap();
        }
        TimeSession::new().unwrap()
    }

    #[test]
    fn source_date_primitives_do_not_leave_creation_cache_empty() {
        let mut state = fixture_state(Some("0"), Some("1"), "XST5");
        assert_eq!(
            state.tex_date().unwrap(),
            calendar(0, &TimeZone::UTC).unwrap()
        );
        assert_eq!(state.creation().unwrap().as_bytes(), b"D:19700101000000Z");
        assert!(state.force_applied);
        assert_eq!(state.start, Some(0));
        assert!(state.creation.is_some());

        let mut state = fixture_state(Some("-1"), None, "XST5");
        assert_eq!(state.start().unwrap(), -1);
        assert_eq!(state.creation().unwrap().as_bytes(), b"D:19691231235959Z");
        let mut state = fixture_state(Some(""), None, "XST5");
        assert_eq!(state.creation().unwrap().as_bytes(), b"D:19700101000000Z");
    }

    #[test]
    fn creation_utc_and_file_modification_force_rules_are_distinct() {
        environment_scope(|| {
            let mut owner = configured_session("XST5", "0", "0");
            TimeSession::scope(owner.as_mut(), || {
                assert_eq!(creation_date().unwrap().as_bytes(), b"D:19700101000000Z");
                assert_eq!(
                    modification_date(0).unwrap().as_bytes(),
                    b"D:19691231190000-05'00'"
                );
            })
            .unwrap();
            let mut owner = configured_session("XST5", "0", "1");
            TimeSession::scope(owner.as_mut(), || {
                assert_eq!(
                    modification_date(0).unwrap().as_bytes(),
                    b"D:19691231190000-05'00'"
                );
                tex_date().unwrap();
                assert_eq!(
                    modification_date(0).unwrap().as_bytes(),
                    b"D:19700101000000Z"
                );
                assert_eq!(creation_date().unwrap().as_bytes(), b"D:19700101000000Z");
            })
            .unwrap();
        });
        let mut state = fixture_state(None, Some("1"), "XST5");
        state.tex_date().unwrap();
        assert!(state.source_epoch.is_none());
        assert!(state.force_applied);
        assert!(state.creation().unwrap().as_bytes().ends_with(b"-05'00'"));
    }

    #[test]
    fn negative_subhour_and_nonhour_offsets_keep_their_sign() {
        for (rule, expected) in [
            ("XST0:30", b"D:19691231233000-00'30'".as_slice()),
            ("XST-0:30", b"D:19700101003000+00'30'".as_slice()),
            ("XST-5:45", b"D:19700101054500+05'45'".as_slice()),
            ("XST3:30", b"D:19691231203000-03'30'".as_slice()),
            ("UTC0", b"D:19700101000000Z".as_slice()),
        ] {
            let date = pdf_date(0, &TimeZone::posix(rule).unwrap()).unwrap();
            assert_eq!(date.as_bytes(), expected, "{rule}");
            assert_eq!(date.bytes[date.len], 0);
            assert!(date.len < date.bytes.len());
        }
        let zone = TimeZone::posix("XST0:00:30").unwrap();
        assert!(pdf_date(0, &zone).unwrap().as_bytes().ends_with(b"-00'01'"));
        assert!(pdf_date(30, &zone).unwrap().as_bytes().ends_with(b"Z"));
    }

    #[test]
    fn clock_microseconds_use_floor_normalization_before_the_epoch() {
        for (delta, expected) in [
            (0, (0, 0)),
            (1, (-1, 999_999)),
            (999, (-1, 999_999)),
            (1000, (-1, 999_999)),
            (1_000_001, (-1, 998_999)),
            (1_000_000_000, (-1, 0)),
            (1_000_000_001, (-2, 999_999)),
        ] {
            assert_eq!(
                seconds_and_micros_at(UNIX_EPOCH - std::time::Duration::from_nanos(delta)).unwrap(),
                expected
            );
        }
        assert_eq!(
            seconds_and_micros_at(UNIX_EPOCH + std::time::Duration::new(12, 987_654_321)).unwrap(),
            (12, 987_654)
        );
    }

    #[test]
    fn date_ranges_and_invalid_epochs_are_ordinary_errors() {
        let mut invalid = fixture_state(Some("0junk"), Some("1"), "UTC0");
        assert!(matches!(invalid.start(), Err(TimeError::InvalidEpoch(_))));
        assert_eq!(invalid.start, None);
        assert_eq!(invalid.creation, None);
        let mut out_of_range = fixture_state(Some("253402300800"), Some("1"), "UTC0");
        assert!(matches!(
            out_of_range.creation(),
            Err(TimeError::CalendarRange)
        ));
        assert!(matches!(
            pdf_date(i64::MAX, &TimeZone::UTC),
            Err(TimeError::CalendarRange)
        ));
        assert!(matches!(
            pdf_date(reference_seconds(0, 1, 1, 0, 0, 0), &TimeZone::UTC),
            Err(TimeError::CalendarRange)
        ));
    }

    #[test]
    fn scopes_own_snapshots_reject_aliases_and_unregister_on_unwind() {
        let parent_tz = std::env::var_os("TZ");
        let parent_epoch = std::env::var_os("SOURCE_DATE_EPOCH");
        environment_scope(|| {
            let mut first = configured_session("UTC0", "0", "1");
            let mut second = configured_session("XST5", "1", "1");
            TimeSession::scope(first.as_mut(), || {
                assert!(matches!(
                    TimeSession::scope(second.as_mut(), || ()),
                    Err(TimeError::Nested)
                ));
                assert_eq!(creation_date().unwrap().as_bytes(), b"D:19700101000000Z");
                with_state(|_| {
                    assert!(matches!(creation_date(), Err(TimeError::Busy)));
                    Ok(())
                })
                .unwrap();
            })
            .unwrap();
            assert!(matches!(creation_date(), Err(TimeError::Missing)));
            assert!(catch_unwind(AssertUnwindSafe(|| {
                TimeSession::scope(second.as_mut(), || {
                    assert_eq!(creation_date().unwrap().as_bytes(), b"D:19700101000001Z");
                    panic!("exercise time unwind");
                })
                .unwrap();
            }))
            .is_err());
            assert!(matches!(creation_date(), Err(TimeError::Missing)));
            let mut third = configured_session("UTC0", "2", "1");
            TimeSession::scope(third.as_mut(), || {
                assert_eq!(creation_date().unwrap().as_bytes(), b"D:19700101000002Z")
            })
            .unwrap();
        });
        assert_eq!(std::env::var_os("TZ"), parent_tz);
        assert_eq!(std::env::var_os("SOURCE_DATE_EPOCH"), parent_epoch);
    }

    #[test]
    fn invalid_local_rules_are_deferred_and_the_first_failure_is_invocation_owned() {
        let directory = Directory::new();
        environment_scope(|| {
            environment::set(OsStr::new("TZDIR"), directory.0.as_os_str()).unwrap();
            let mut owner = configured_session(":missing-zone", "0", "0");
            assert!(owner.state.zone.is_none());
            // An option-only invocation can enter and return without any
            // timezone read, just like help/version before calendar use.
            TimeSession::scope(owner.as_mut(), || ()).unwrap();
            assert!(owner.state.zone.is_none());
            TimeSession::scope(owner.as_mut(), || {
                // SOURCE_DATE_EPOCH creation dates use UTC independently of
                // whether a local file-modification date will later fail.
                assert_eq!(creation_date().unwrap().as_bytes(), b"D:19700101000000Z");
                let first = modification_date(0).unwrap_err().to_string();
                std::fs::write(directory.0.join("missing-zone"), fixed_tzif(-1800, false)).unwrap();
                environment::set(OsStr::new("TZ"), OsStr::new("UTC0")).unwrap();
                assert_eq!(modification_date(0).unwrap_err().to_string(), first);
                with_state(|state| {
                    assert!(matches!(state.zone, Some(Err(_))));
                    assert_eq!(state.tz.as_deref(), Some(OsStr::new(":missing-zone")));
                    Ok(())
                })
                .unwrap();
            })
            .unwrap();
            // A new owner observes the changed environment and has no failed
            // zone cache inherited from the preceding invocation.
            let mut fresh = TimeSession::new().unwrap();
            TimeSession::scope(fresh.as_mut(), || {
                assert_eq!(
                    modification_date(0).unwrap().as_bytes(),
                    b"D:19700101000000Z"
                );
            })
            .unwrap();
        });
    }

    #[test]
    fn forced_source_dates_never_resolve_an_unused_unknown_timezone() {
        environment_scope(|| {
            environment::set(
                OsStr::new("TZDIR"),
                OsStr::new("/nonexistent/tekai-zoneinfo"),
            )
            .unwrap();
            let mut owner = configured_session(":never-readable", "123456", "1");
            assert!(owner.state.zone.is_none());
            TimeSession::scope(owner.as_mut(), || {
                initialize_start().unwrap();
                assert_eq!(
                    tex_date().unwrap(),
                    calendar(123456, &TimeZone::UTC).unwrap()
                );
                assert_eq!(
                    creation_date().unwrap(),
                    pdf_date(123456, &TimeZone::UTC).unwrap()
                );
                assert_eq!(
                    modification_date(-1).unwrap(),
                    pdf_date(-1, &TimeZone::UTC).unwrap()
                );
                with_state(|state| {
                    assert!(state.zone.is_none());
                    assert!(state.force_applied);
                    Ok(())
                })
                .unwrap();
            })
            .unwrap();
            assert!(owner.state.zone.is_none());
        });
    }

    #[test]
    fn lazy_zone_reads_owned_names_once_and_retains_the_parsed_file() {
        let directory = Directory::new();
        std::fs::write(directory.0.join("owned-zone"), fixed_tzif(-1800, false)).unwrap();
        environment_scope(|| {
            environment::set(OsStr::new("TZDIR"), directory.0.as_os_str()).unwrap();
            let mut owner = configured_session(":owned-zone", "0", "0");
            environment::set(
                OsStr::new("TZDIR"),
                OsStr::new("/nonexistent/tekai-zoneinfo"),
            )
            .unwrap();
            environment::set(OsStr::new("TZ"), OsStr::new("UTC0")).unwrap();
            TimeSession::scope(owner.as_mut(), || {
                assert_eq!(
                    modification_date(0).unwrap().as_bytes(),
                    b"D:19691231233000-00'30'"
                );
                std::fs::write(directory.0.join("owned-zone"), fixed_tzif(0, false)).unwrap();
                assert_eq!(
                    modification_date(0).unwrap().as_bytes(),
                    b"D:19691231233000-00'30'"
                );
            })
            .unwrap();
            let mut fresh = TimeSession::new().unwrap();
            TimeSession::scope(fresh.as_mut(), || {
                assert_eq!(
                    modification_date(0).unwrap().as_bytes(),
                    b"D:19700101000000Z"
                );
            })
            .unwrap();
        });
    }

    #[test]
    fn concurrent_scopes_have_independent_date_caches() {
        let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
        let threads = ["0", "123456"].map(|epoch| {
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                environment_scope(|| {
                    let mut owner = configured_session("UTC0", epoch, "1");
                    TimeSession::scope(owner.as_mut(), || {
                        barrier.wait();
                        let expected = pdf_date(epoch.parse().unwrap(), &TimeZone::UTC).unwrap();
                        assert_eq!(creation_date().unwrap(), expected);
                        barrier.wait();
                        assert_eq!(creation_date().unwrap(), expected);
                    })
                    .unwrap();
                })
            })
        });
        for thread in threads {
            thread.join().unwrap();
        }
    }

    struct Directory(PathBuf);
    impl Directory {
        fn new() -> Self {
            static NEXT: AtomicUsize = AtomicUsize::new(0);
            let path = std::env::temp_dir().join(format!(
                "tekai-time-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            std::fs::create_dir(&path).unwrap();
            Self(path)
        }
    }
    impl Drop for Directory {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn fixed_tzif(offset: i32, leap: bool) -> Vec<u8> {
        let mut bytes = b"TZif\0".to_vec();
        bytes.extend_from_slice(&[0; 15]);
        for count in [0u32, 0, u32::from(leap), 0, 1, 4] {
            bytes.extend_from_slice(&count.to_be_bytes());
        }
        bytes.extend_from_slice(&offset.to_be_bytes());
        bytes.extend_from_slice(&[0, 0]);
        bytes.extend_from_slice(b"XST\0");
        if leap {
            bytes.extend_from_slice(&78_796_800i32.to_be_bytes());
            bytes.extend_from_slice(&1i32.to_be_bytes());
        }
        bytes
    }

    fn filesystem_name(directory: &Path, bytes: &[u8], payload: &[u8]) -> OsString {
        let name = OsString::from_vec(bytes.to_vec());
        match std::fs::write(directory.join(&name), payload) {
            Ok(()) => name,
            Err(error) => {
                // APFS rejects invalid UTF-8 names with EILSEQ. Still verify
                // that our raw-path metadata read fails instead of consulting
                // an existing file with the lossy replacement spelling.
                #[cfg(target_os = "macos")]
                assert_eq!(error.raw_os_error(), Some(libc::EILSEQ));
                #[cfg(not(target_os = "macos"))]
                panic!("cannot create non-UTF-8 fixture: {error}");
                let replacement = name.to_string_lossy().into_owned();
                std::fs::write(directory.join(&replacement), payload).unwrap();
                assert!(file_metadata(&directory.join(&name)).is_err());
                OsString::from(replacement)
            }
        }
    }

    #[test]
    fn tzif_paths_are_exact_non_utf8_and_tzdir_is_invocation_owned() {
        let directory = Directory::new();
        let name = filesystem_name(&directory.0, b"custom-\xff", &fixed_tzif(-1800, false));
        let path = directory.0.join(&name);
        let direct = resolve_zone(Some(path.as_os_str()), None).unwrap();
        assert_eq!(
            pdf_date(0, &direct).unwrap().as_bytes(),
            b"D:19691231233000-00'30'"
        );
        let mut colon = b":".to_vec();
        colon.extend_from_slice(path.as_os_str().as_bytes());
        let forced = resolve_zone(Some(OsStr::from_bytes(&colon)), None).unwrap();
        assert_eq!(pdf_date(0, &forced).unwrap(), pdf_date(0, &direct).unwrap());
        let relative = resolve_zone(Some(&name), Some(directory.0.as_os_str())).unwrap();
        assert_eq!(
            pdf_date(0, &relative).unwrap(),
            pdf_date(0, &direct).unwrap()
        );
        std::fs::write(directory.0.join("UTC0"), fixed_tzif(3600, false)).unwrap();
        assert_eq!(
            calendar(
                0,
                &resolve_zone(Some(OsStr::new("UTC0")), Some(directory.0.as_os_str())).unwrap()
            )
            .unwrap()
            .hour,
            1
        );
        assert_eq!(
            calendar(
                0,
                &resolve_zone(Some(OsStr::new("")), Some(directory.0.as_os_str())).unwrap()
            )
            .unwrap()
            .hour,
            0
        );
        assert!(resolve_zone(
            Some(OsStr::new(":UTC0")),
            Some(OsStr::new("/nonexistent/tekai-zoneinfo"))
        )
        .is_err());
        environment_scope(|| {
            environment::set(OsStr::new("TZ"), &name).unwrap();
            environment::set(OsStr::new("TZDIR"), directory.0.as_os_str()).unwrap();
            environment::set(OsStr::new("SOURCE_DATE_EPOCH"), OsStr::new("0")).unwrap();
            let mut owner = TimeSession::new().unwrap();
            assert!(owner.state.zone.is_none());
            TimeSession::scope(owner.as_mut(), || {
                with_state(|state| {
                    assert_eq!(
                        state
                            .local_zone()?
                            .to_offset(Timestamp::UNIX_EPOCH)
                            .seconds(),
                        -1800
                    );
                    Ok(())
                })
                .unwrap();
            })
            .unwrap();
        });
    }

    #[test]
    fn timezone_parse_limits_and_unrepresentable_configurations_are_explicit() {
        assert!(zone_from_bytes(&fixed_tzif(0, true))
            .unwrap_err()
            .to_string()
            .contains("leap-second"));
        let directory = Directory::new();
        assert!(
            resolve_zone(Some(OsStr::new("FOO8BAR")), Some(directory.0.as_os_str()))
                .unwrap_err()
                .to_string()
                .contains("without transition rules")
        );
        let oversized = "X".repeat(MAX_TZ_STRING_BYTES + 1);
        assert!(resolve_zone(Some(OsStr::new(&oversized)), None).is_err());
        std::fs::write(directory.0.join("huge"), vec![0; MAX_TZIF_BYTES + 1]).unwrap();
        assert!(
            resolve_zone(Some(OsStr::new("huge")), Some(directory.0.as_os_str()))
                .unwrap_err()
                .to_string()
                .contains("1 MiB")
        );
        assert!(zone_from_bytes(b"TZif").is_err());
    }

    fn default_rule_tzif() -> Vec<u8> {
        let mut bytes = b"TZif\0".to_vec();
        bytes.extend_from_slice(&[0; 15]);
        for count in [0u32, 0, 0, 3, 3, 12] {
            bytes.extend_from_slice(&count.to_be_bytes());
        }
        for stamp in [0i32, 200, 400] {
            bytes.extend_from_slice(&stamp.to_be_bytes());
        }
        bytes.extend_from_slice(&[1, 2, 1]);
        for (offset, dst, name) in [(-17_762i32, 0, 0), (-18_000, 0, 4), (-14_400, 1, 8)] {
            bytes.extend_from_slice(&offset.to_be_bytes());
            bytes.extend_from_slice(&[dst, name]);
        }
        bytes.extend_from_slice(b"LMT\0XST\0XDT\0");
        bytes
    }

    #[test]
    fn implicit_dst_admission_proves_offsets_and_corrects_the_pre_table_default() {
        assert_eq!(implicit_offsets("FOO5BAR"), Some((-18_000, -14_400)));
        assert_eq!(
            implicit_offsets("<F+O>3:30<D-S>2:30"),
            Some((-12_600, -9000))
        );
        assert_eq!(implicit_offsets("FOO5BAR,M3.2.0,M11.1.0"), None);
        let bytes = default_rule_tzif();
        assert_eq!(
            zone_from_bytes(&bytes)
                .unwrap()
                .to_offset(Timestamp::from_second(-1).unwrap())
                .seconds(),
            -17_762
        );
        let zone = compatible_posixrules(bytes.clone(), (-18_000, -14_400))
            .unwrap()
            .unwrap();
        for (stamp, expected) in [
            (-1, -18_000),
            (0, -18_000),
            (199, -18_000),
            (200, -14_400),
            (399, -14_400),
            (400, -18_000),
            (2_193_134_400, -18_000),
        ] {
            assert_eq!(
                zone.to_offset(Timestamp::from_second(stamp).unwrap())
                    .seconds(),
                expected
            );
        }
        assert!(compatible_posixrules(bytes, (-28_800, -25_200))
            .unwrap()
            .is_none());
        let mut dst_default = default_rule_tzif();
        let block = tzif_blocks(&dst_default).unwrap()[0];
        dst_default[block.types + 4] = 1;
        assert!(compatible_posixrules(dst_default, (-18_000, -14_400))
            .unwrap()
            .is_none());
        let directory = Directory::new();
        std::fs::write(directory.0.join("posixrules"), default_rule_tzif()).unwrap();
        let zone =
            resolve_zone(Some(OsStr::new("FOO5BAR")), Some(directory.0.as_os_str())).unwrap();
        assert_eq!(
            zone.to_offset(Timestamp::from_second(200).unwrap())
                .seconds(),
            -14_400
        );
        assert!(
            resolve_zone(Some(OsStr::new("FOO8BAR")), Some(directory.0.as_os_str()))
                .unwrap_err()
                .to_string()
                .contains("offset-compatible")
        );
    }

    #[test]
    fn explicit_dst_rules_cover_gaps_folds_and_southern_summers() {
        let zone = TimeZone::posix("EST5EDT,M3.2.0,M11.1.0").unwrap();
        for (stamp, hour, minute, second, offset) in [
            (
                reference_seconds(2024, 3, 10, 6, 59, 59),
                1,
                59,
                59,
                -18_000,
            ),
            (reference_seconds(2024, 3, 10, 7, 0, 0), 3, 0, 0, -14_400),
            (
                reference_seconds(2024, 11, 3, 5, 59, 59),
                1,
                59,
                59,
                -14_400,
            ),
            (reference_seconds(2024, 11, 3, 6, 0, 0), 1, 0, 0, -18_000),
        ] {
            let date = calendar(stamp, &zone).unwrap();
            assert_eq!(
                (date.hour, date.minute, date.second, date.offset_seconds),
                (hour, minute, second, offset)
            );
        }
        let south = TimeZone::posix("AEST-10AEDT,M10.1.0,M4.1.0/3").unwrap();
        assert_eq!(
            calendar(reference_seconds(2024, 1, 1, 0, 0, 0), &south)
                .unwrap()
                .offset_seconds,
            39_600
        );
        assert_eq!(
            calendar(reference_seconds(2024, 7, 1, 0, 0, 0), &south)
                .unwrap()
                .offset_seconds,
            36_000
        );
    }

    #[test]
    fn relative_metadata_follows_symlinks_and_retains_non_utf8_paths() {
        use crate::working_directory::WorkingDirectory;
        let directory = Directory::new();
        let file = filesystem_name(&directory.0, b"input-\xfe", b"metadata payload");
        let path = directory.0.join(&file);
        let stamp = UNIX_EPOCH + std::time::Duration::new(1_234_567_890, 123_456_789);
        std::fs::File::options()
            .write(true)
            .open(&path)
            .unwrap()
            .set_times(std::fs::FileTimes::new().set_modified(stamp))
            .unwrap();
        std::os::unix::fs::symlink(&file, directory.0.join("link")).unwrap();
        let parent = std::env::current_dir().unwrap();
        let mut owner = WorkingDirectory::new(Some(directory.0.clone())).unwrap();
        owner
            .as_mut()
            .scope(|| {
                let expected = FileMetadata {
                    len: 16,
                    modified_seconds: 1_234_567_890,
                };
                assert_eq!(file_metadata(Path::new(&file)).unwrap(), expected);
                assert_eq!(file_metadata(Path::new("link")).unwrap(), expected);
                assert_eq!(file_metadata(&path).unwrap(), expected);
                assert!(file_metadata(Path::new("absent")).is_err());
                assert_eq!(std::env::current_dir().unwrap(), parent);
            })
            .unwrap();
    }

    #[test]
    fn metadata_pool_requires_one_spare_byte_without_wrapping() {
        assert_eq!(pool_end(0, 30, 23), Some(23));
        assert_eq!(pool_end(7, 30, 23), None);
        assert_eq!(pool_end(6, 30, 23), Some(29));
        assert_eq!(pool_end(-1, 30, 0), None);
        assert_eq!(pool_end(0, -1, 0), None);
        assert_eq!(pool_end(1, i32::MAX, usize::MAX), None);
        assert_eq!(pool_end(i32::MAX - 1, i32::MAX, 1), None);
    }

    fn leap(year: i32) -> bool {
        year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
    }
    fn month_days(year: i32, month: i32) -> i64 {
        match month {
            2 => {
                if leap(year) {
                    29
                } else {
                    28
                }
            }
            4 | 6 | 9 | 11 => 30,
            _ => 31,
        }
    }

    // Independent, bounded iterative reference. It deliberately does not use
    // Jiff calendar constructors, production conversion, or formatting.
    fn reference_seconds(
        year: i32,
        month: i32,
        day: i32,
        hour: i32,
        minute: i32,
        second: i32,
    ) -> i64 {
        let mut days = 0i64;
        if year >= 1970 {
            for y in 1970..year {
                days += if leap(y) { 366 } else { 365 };
            }
        } else {
            for y in year..1970 {
                days -= if leap(y) { 366 } else { 365 };
            }
        }
        for m in 1..month {
            days += month_days(year, m);
        }
        (days + i64::from(day - 1)) * 86400 + i64::from(hour * 3600 + minute * 60 + second)
    }

    proptest! {
        #[test]
        fn utc_calendar_matches_an_independent_gregorian_reference(
            year in 1i32..=9998, month in 1i32..=12, day in 1i32..=28,
            hour in 0i32..24, minute in 0i32..60, second in 0i32..60,
        ) {
            let stamp = reference_seconds(year, month, day, hour, minute, second);
            let date = calendar(stamp, &TimeZone::UTC).unwrap();
            prop_assert_eq!(date, Calendar { year, month, day, hour, minute, second, offset_seconds: 0 });
            let formatted = pdf_date(stamp, &TimeZone::UTC).unwrap();
            let expected = format!("D:{year:04}{month:02}{day:02}{hour:02}{minute:02}{second:02}Z");
            prop_assert_eq!(formatted.as_bytes(), expected.as_bytes());
        }

        #[test]
        fn fixed_zone_offsets_match_reference_clock_minutes(seconds in -2_000_000_000i64..=2_000_000_000, offset_minutes in -1439i32..=1439) {
            let offset = -offset_minutes;
            let rule = format!("XST{}{hour}:{minute:02}", if offset < 0 { "-" } else { "+" }, hour = offset.unsigned_abs() / 60, minute = offset.unsigned_abs() % 60);
            let zone = TimeZone::posix(&rule).unwrap();
            let local = calendar(seconds, &zone).unwrap();
            let shifted = calendar(seconds + i64::from(offset_minutes) * 60, &TimeZone::UTC).unwrap();
            prop_assert_eq!(local, Calendar { offset_seconds: offset_minutes * 60, ..shifted });
            let date = pdf_date(seconds, &zone).unwrap();
            let expected = if offset_minutes == 0 { "Z".to_owned() } else { format!("{}{hour:02}'{minute:02}'", if offset_minutes < 0 { "-" } else { "+" }, hour = offset_minutes.unsigned_abs() / 60, minute = offset_minutes.unsigned_abs() % 60) };
            prop_assert!(date.as_bytes().ends_with(expected.as_bytes()));
            prop_assert_eq!(date.bytes[date.len], 0);
        }
    }

    #[test]
    fn gregorian_leap_centuries_and_year_boundaries_are_exact() {
        for (year, month, day) in [
            (1, 1, 1),
            (1600, 2, 29),
            (1700, 3, 1),
            (1900, 3, 1),
            (2000, 2, 29),
            (2100, 3, 1),
            (2400, 2, 29),
            (9998, 12, 31),
        ] {
            let seconds = reference_seconds(year, month, day, 23, 59, 59);
            let date = calendar(seconds, &TimeZone::UTC).unwrap();
            assert_eq!(
                (
                    date.year,
                    date.month,
                    date.day,
                    date.hour,
                    date.minute,
                    date.second
                ),
                (year, month, day, 23, 59, 59)
            );
        }
        assert_eq!(
            calendar(
                reference_seconds(2000, 12, 31, 23, 59, 59) + 1,
                &TimeZone::UTC
            )
            .unwrap()
            .year,
            2001
        );
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn native_oracle_child() {
        let Some(seconds) = std::env::var_os("TEKAI_TIME_ORACLE_SECONDS") else {
            return;
        };
        for value in seconds.to_str().unwrap().split(',') {
            let seconds: libc::time_t = value.parse().unwrap();
            let mut calendar = std::mem::MaybeUninit::<libc::tm>::uninit();
            // SAFETY This is a test-only oracle in a dedicated child process.
            // localtime_r initializes the caller-owned calendar on success.
            assert!(!unsafe { libc::localtime_r(&seconds, calendar.as_mut_ptr()) }.is_null());
            let date = unsafe { calendar.assume_init() };
            println!(
                "TEKAI_NATIVE_DATE {} {} {} {} {} {} {}",
                date.tm_year + 1900,
                date.tm_mon + 1,
                date.tm_mday,
                date.tm_hour,
                date.tm_min,
                date.tm_sec,
                date.tm_gmtoff
            );
        }
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn supported_zones_match_serialized_native_child_oracles() {
        static SERIAL: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _serial = SERIAL.lock().unwrap();
        let stamps = [
            -2_840_140_800,
            -1,
            0,
            951_782_400,
            1_710_053_999,
            1_710_054_000,
            1_730_617_199,
            1_730_617_200,
            2_193_134_400,
        ];
        let values = stamps.map(|value| value.to_string()).join(",");
        for name in [
            "UTC0",
            "NST3:30NDT,M3.2.0,M11.1.0",
            "XST-5:45",
            "America/Los_Angeles",
            "Asia/Kathmandu",
            ":/usr/share/zoneinfo/Europe/Dublin",
            "FOO5BAR",
        ] {
            let zone = resolve_zone(Some(OsStr::new(name)), None).unwrap();
            let output = std::process::Command::new(std::env::current_exe().unwrap())
                .args(["--exact", "time::tests::native_oracle_child", "--nocapture"])
                .env("TZ", name)
                .env("TEKAI_TIME_ORACLE_SECONDS", &values)
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "native oracle for {name}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            let stdout = String::from_utf8(output.stdout).unwrap();
            let dates: Vec<_> = stdout
                .lines()
                .filter_map(|line| line.strip_prefix("TEKAI_NATIVE_DATE "))
                .map(|line| {
                    line.split_whitespace()
                        .map(|value| value.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect();
            assert_eq!(dates.len(), stamps.len(), "{name}: {stdout}");
            for (&stamp, expected) in stamps.iter().zip(dates) {
                let actual = calendar(stamp, &zone).unwrap();
                assert_eq!(
                    [
                        actual.year,
                        actual.month,
                        actual.day,
                        actual.hour,
                        actual.minute,
                        actual.second,
                        actual.offset_seconds
                    ]
                    .as_slice(),
                    expected,
                    "{name}, {stamp}"
                );
            }
        }
    }
}
