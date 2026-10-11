//! Rust formatting for the engine's fixed printf grammar.
//!
//! Decimal conversion uses Rust's correctly rounded binary-to-decimal formatter.
//! The C presentation rules are applied here, not delegated to native printf.
//! Formats are ASCII and locale independent. Hex floats, grouping, positional
//! arguments, dynamic `*` fields, `%n`, wide strings and long doubles are
//! deliberately rejected. No engine output caller uses those extensions.
//! Rounding is nearest with ties to even, independent of process locale or
//! native floating-environment rounding mode, as required by stable PDF bytes.

use libc::{c_char, c_void};
use std::ffi::CStr;
use std::fmt;
use std::ptr;

// Fixed engine literals use small fields. A malformed or externally supplied
// format must not request an unbounded padding/precision allocation.
const MAX_FIELD: usize = 1024 * 1024;

#[derive(Clone, Copy, Debug)]
pub enum PrintfArg {
    Signed(i64),
    Unsigned(u64),
    Float(f64),
    Ptr(*const c_void),
}

macro_rules! signed_arg {
    ($($ty:ty),* $(,)?) => {
        $(impl From<$ty> for PrintfArg {
            fn from(value: $ty) -> Self { Self::Signed(value as i64) }
        })*
    };
}

macro_rules! unsigned_arg {
    ($($ty:ty),* $(,)?) => {
        $(impl From<$ty> for PrintfArg {
            fn from(value: $ty) -> Self { Self::Unsigned(value as u64) }
        })*
    };
}

signed_arg!(i8, i16, i32, i64, isize);
unsigned_arg!(u8, u16, u32, u64, usize);

impl From<f32> for PrintfArg {
    fn from(value: f32) -> Self {
        Self::Float(f64::from(value))
    }
}

impl From<f64> for PrintfArg {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl<T> From<*const T> for PrintfArg {
    fn from(value: *const T) -> Self {
        Self::Ptr(value.cast())
    }
}

impl<T> From<*mut T> for PrintfArg {
    fn from(value: *mut T) -> Self {
        Self::Ptr(value.cast())
    }
}

impl PrintfArg {
    fn as_i64(self) -> i64 {
        match self {
            Self::Signed(value) => value,
            Self::Unsigned(value) => value as i64,
            Self::Float(value) => value as i64,
            Self::Ptr(value) => value as isize as i64,
        }
    }

    fn as_u64(self) -> u64 {
        match self {
            Self::Signed(value) => value as u64,
            Self::Unsigned(value) => value,
            Self::Float(value) => value as u64,
            Self::Ptr(value) => value as usize as u64,
        }
    }

    fn as_f64(self) -> f64 {
        match self {
            Self::Signed(value) => value as f64,
            Self::Unsigned(value) => value as f64,
            Self::Float(value) => value,
            Self::Ptr(value) => value as usize as f64,
        }
    }

    fn as_ptr(self) -> *const c_void {
        match self {
            Self::Ptr(value) => value,
            Self::Signed(value) => value as isize as *const c_void,
            Self::Unsigned(value) => value as usize as *const c_void,
            Self::Float(_) => ptr::null(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FormatError {
    offset: usize,
    reason: &'static str,
}

impl fmt::Display for FormatError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "printf format at byte {} is unsupported: {}",
            self.offset, self.reason
        )
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum Length {
    #[default]
    Int,
    Char,
    Short,
    Long,
    LongLong,
    IntMax,
    Size,
    PtrDiff,
}

#[derive(Default)]
struct Spec {
    alternate: bool,
    zero: bool,
    left: bool,
    plus: bool,
    space: bool,
    width: usize,
    precision: Option<usize>,
    length: Length,
    conversion: u8,
}

fn fail(offset: usize, reason: &'static str) -> FormatError {
    FormatError { offset, reason }
}

fn number(source: &[u8], cursor: &mut usize, start: usize) -> Result<usize, FormatError> {
    let mut number = 0usize;
    while source.get(*cursor).is_some_and(u8::is_ascii_digit) {
        number = number
            .checked_mul(10)
            .and_then(|value| value.checked_add(usize::from(source[*cursor] - b'0')))
            .filter(|value| *value <= MAX_FIELD)
            .ok_or_else(|| fail(start, "width or precision exceeds the bounded field limit"))?;
        *cursor += 1;
    }
    Ok(number)
}

fn parse(source: &[u8], cursor: &mut usize, start: usize) -> Result<Spec, FormatError> {
    let mut spec = Spec::default();
    while let Some(&flag) = source.get(*cursor) {
        match flag {
            b'#' => spec.alternate = true,
            b'0' => spec.zero = true,
            b'-' => spec.left = true,
            b'+' => spec.plus = true,
            b' ' => spec.space = true,
            b'\'' => {
                return Err(fail(
                    start,
                    "locale grouping is not part of the engine grammar",
                ))
            }
            _ => break,
        }
        *cursor += 1;
    }
    spec.width = number(source, cursor, start)?;
    if source.get(*cursor) == Some(&b'.') {
        *cursor += 1;
        spec.precision = Some(number(source, cursor, start)?);
    }
    spec.length = match source.get(*cursor) {
        Some(b'h') => {
            *cursor += 1;
            if source.get(*cursor) == Some(&b'h') {
                *cursor += 1;
                Length::Char
            } else {
                Length::Short
            }
        }
        Some(b'l') => {
            *cursor += 1;
            if source.get(*cursor) == Some(&b'l') {
                *cursor += 1;
                Length::LongLong
            } else {
                Length::Long
            }
        }
        Some(b'j') => {
            *cursor += 1;
            Length::IntMax
        }
        Some(b'z') => {
            *cursor += 1;
            Length::Size
        }
        Some(b't') => {
            *cursor += 1;
            Length::PtrDiff
        }
        Some(b'L') => return Err(fail(start, "long double has no typed engine argument")),
        _ => Length::Int,
    };
    spec.conversion = *source
        .get(*cursor)
        .ok_or_else(|| fail(start, "unterminated conversion"))?;
    *cursor += 1;
    if !matches!(
        spec.conversion,
        b'd' | b'i'
            | b'u'
            | b'o'
            | b'x'
            | b'X'
            | b'c'
            | b's'
            | b'p'
            | b'f'
            | b'F'
            | b'e'
            | b'E'
            | b'g'
            | b'G'
    ) {
        return Err(fail(
            start,
            "conversion is outside the fixed engine grammar",
        ));
    }
    if matches!(spec.conversion, b'c' | b's' | b'p') && spec.length != Length::Int {
        return Err(fail(
            start,
            "wide characters and pointer lengths are not supported",
        ));
    }
    if matches!(spec.conversion, b'f' | b'F' | b'e' | b'E' | b'g' | b'G')
        && !matches!(spec.length, Length::Int | Length::Long)
    {
        return Err(fail(start, "floating length has no typed engine argument"));
    }
    Ok(spec)
}

/// Only parsing and output formatting happen here; no native formatter runs.
///
/// # Safety
/// Non-null `%s` argument pointers must remain valid C strings for the call.
pub unsafe fn format(source: &[u8], args: &[PrintfArg]) -> Result<Vec<u8>, FormatError> {
    let mut output = Vec::with_capacity(1024);
    let mut cursor = 0usize;
    let mut argument = 0usize;
    while cursor < source.len() {
        if source[cursor] != b'%' {
            output.push(source[cursor]);
            cursor += 1;
            continue;
        }
        let start = cursor;
        cursor += 1;
        if source.get(cursor) == Some(&b'%') {
            output.push(b'%');
            cursor += 1;
            continue;
        }
        let spec = parse(source, &mut cursor, start)?;
        let value = *args
            .get(argument)
            .ok_or_else(|| fail(start, "missing typed argument"))?;
        argument += 1;
        output.extend(format_one(&spec, value));
    }
    Ok(output)
}

fn signed(value: i64, length: Length) -> i64 {
    match length {
        Length::Char => value as i8 as i64,
        Length::Short => value as i16 as i64,
        Length::Int => value as i32 as i64,
        Length::Long => value as libc::c_long as i64,
        Length::Size | Length::PtrDiff => value as isize as i64,
        Length::LongLong | Length::IntMax => value,
    }
}

fn unsigned(value: u64, length: Length) -> u64 {
    match length {
        Length::Char => value as u8 as u64,
        Length::Short => value as u16 as u64,
        Length::Int => value as u32 as u64,
        Length::Long => value as libc::c_ulong as u64,
        Length::Size | Length::PtrDiff => value as usize as u64,
        Length::LongLong | Length::IntMax => value,
    }
}

fn sign(negative: bool, spec: &Spec) -> &'static [u8] {
    if negative {
        b"-"
    } else if spec.plus {
        b"+"
    } else if spec.space {
        b" "
    } else {
        b""
    }
}

fn pad(prefix: &[u8], body: &[u8], spec: &Spec, zero: bool) -> Vec<u8> {
    let padding = spec.width.saturating_sub(prefix.len() + body.len());
    let mut output = Vec::with_capacity(prefix.len() + body.len() + padding);
    if !spec.left && !zero {
        output.resize(padding, b' ');
    }
    output.extend_from_slice(prefix);
    if !spec.left && zero {
        output.resize(output.len() + padding, b'0');
    }
    output.extend_from_slice(body);
    if spec.left {
        output.resize(output.len() + padding, b' ');
    }
    output
}

fn integer(spec: &Spec, value: u64, prefix: &[u8]) -> Vec<u8> {
    let mut digits = match spec.conversion {
        b'o' => format!("{value:o}"),
        b'x' | b'p' => format!("{value:x}"),
        b'X' => format!("{value:X}"),
        _ => value.to_string(),
    }
    .into_bytes();
    if value == 0 && spec.precision == Some(0) {
        digits.clear();
    }
    let mut precision = spec.precision.unwrap_or(0);
    if spec.alternate && spec.conversion == b'o' {
        precision = precision.max(digits.len() + usize::from(digits.first() != Some(&b'0')));
    }
    let mut body = vec![b'0'; precision.saturating_sub(digits.len())];
    body.extend(digits);
    pad(prefix, &body, spec, spec.zero && spec.precision.is_none())
}

fn scientific(value: f64, precision: usize) -> (String, i32) {
    let decimal = format!("{value:.precision$e}");
    let (mantissa, exponent) = decimal.split_once('e').expect("Rust scientific exponent");
    (
        mantissa.to_owned(),
        exponent.parse().expect("Rust decimal exponent"),
    )
}

fn strip_fraction_zeroes(decimal: &mut String) {
    if decimal.contains('.') {
        while decimal.ends_with('0') {
            decimal.pop();
        }
        if decimal.ends_with('.') {
            decimal.pop();
        }
    }
}

fn exponent(mantissa: String, exponent: i32, uppercase: bool) -> String {
    format!(
        "{mantissa}{}{exponent:+03}",
        if uppercase { 'E' } else { 'e' }
    )
}

fn decimal(spec: &Spec, value: f64) -> Vec<u8> {
    // Darwin's printf suppresses every NaN sign, including explicit +/space.
    // Glibc keeps the sign bit and sign flags. Preserve each supported ABI's
    // existing spelling without delegating decimal conversion to it.
    let prefix = if cfg!(target_os = "macos") && value.is_nan() {
        b"".as_slice()
    } else {
        sign(value.is_sign_negative(), spec)
    };
    let value = value.abs();
    let uppercase = spec.conversion.is_ascii_uppercase();
    let body = if value.is_nan() {
        if uppercase {
            "NAN".into()
        } else {
            "nan".into()
        }
    } else if value.is_infinite() {
        if uppercase {
            "INF".into()
        } else {
            "inf".into()
        }
    } else {
        let precision = spec.precision.unwrap_or(6);
        match spec.conversion.to_ascii_lowercase() {
            b'f' => {
                let mut body = format!("{value:.precision$}");
                if spec.alternate && precision == 0 {
                    body.push('.');
                }
                body
            }
            b'e' => {
                let (mut mantissa, power) = scientific(value, precision);
                if spec.alternate && precision == 0 {
                    mantissa.push('.');
                }
                exponent(mantissa, power, uppercase)
            }
            b'g' => {
                let precision = precision.max(1);
                // Selection follows the exponent *after* rounding to the
                // requested significant digits, including 9.999 -> 10 cases.
                let (mut mantissa, power) = scientific(value, precision - 1);
                if power < -4 || power >= precision as i32 {
                    if !spec.alternate {
                        strip_fraction_zeroes(&mut mantissa);
                    } else if !mantissa.contains('.') {
                        mantissa.push('.');
                    }
                    exponent(mantissa, power, uppercase)
                } else {
                    let places = (precision as i32 - power - 1) as usize;
                    let mut body = format!("{value:.places$}");
                    if !spec.alternate {
                        strip_fraction_zeroes(&mut body);
                    } else if !body.contains('.') {
                        body.push('.');
                    }
                    body
                }
            }
            _ => unreachable!("validated floating conversion"),
        }
    };
    pad(
        prefix,
        body.as_bytes(),
        spec,
        spec.zero && value.is_finite(),
    )
}

fn format_one(spec: &Spec, arg: PrintfArg) -> Vec<u8> {
    match spec.conversion {
        b'd' | b'i' => {
            let value = signed(arg.as_i64(), spec.length);
            integer(spec, value.unsigned_abs(), sign(value < 0, spec))
        }
        b'u' | b'o' | b'x' | b'X' => {
            let value = unsigned(arg.as_u64(), spec.length);
            let prefix: &[u8] = if spec.alternate && value != 0 {
                match spec.conversion {
                    b'x' => b"0x",
                    b'X' => b"0X",
                    _ => b"",
                }
            } else {
                b""
            };
            integer(spec, value, prefix)
        }
        b'c' => pad(b"", &[arg.as_i64() as u8], spec, false),
        b's' => {
            let pointer = arg.as_ptr().cast::<c_char>();
            let bytes = if pointer.is_null() {
                b"(null)".as_slice()
            } else {
                unsafe { CStr::from_ptr(pointer).to_bytes() }
            };
            let end = spec.precision.unwrap_or(bytes.len()).min(bytes.len());
            pad(b"", &bytes[..end], spec, false)
        }
        b'p' => {
            let value = arg.as_ptr() as usize as u64;
            if cfg!(target_os = "linux") && value == 0 {
                pad(b"", b"(nil)", spec, false)
            } else {
                integer(spec, value, b"0x")
            }
        }
        _ => decimal(spec, arg.as_f64()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    fn format(source: &[u8], args: &[PrintfArg]) -> Result<Vec<u8>, FormatError> {
        // Test pointers come from live C literals or buffers in their scope.
        unsafe { super::format(source, args) }
    }

    // Native formatting is deliberately confined to this compatibility oracle.
    unsafe fn oracle(spec: &CStr, arg: PrintfArg, wide: bool) -> Vec<u8> {
        let mut output = vec![0u8; 4096];
        let count = unsafe {
            match arg {
                PrintfArg::Float(value) => libc::snprintf(
                    output.as_mut_ptr().cast(),
                    output.len(),
                    spec.as_ptr(),
                    value,
                ),
                PrintfArg::Signed(value) if wide => libc::snprintf(
                    output.as_mut_ptr().cast(),
                    output.len(),
                    spec.as_ptr(),
                    value,
                ),
                PrintfArg::Signed(value) => libc::snprintf(
                    output.as_mut_ptr().cast(),
                    output.len(),
                    spec.as_ptr(),
                    value as i32,
                ),
                PrintfArg::Unsigned(value) if wide => libc::snprintf(
                    output.as_mut_ptr().cast(),
                    output.len(),
                    spec.as_ptr(),
                    value,
                ),
                PrintfArg::Unsigned(value) => libc::snprintf(
                    output.as_mut_ptr().cast(),
                    output.len(),
                    spec.as_ptr(),
                    value as u32,
                ),
                PrintfArg::Ptr(value) => libc::snprintf(
                    output.as_mut_ptr().cast(),
                    output.len(),
                    spec.as_ptr(),
                    value,
                ),
            }
        };
        assert!(count >= 0 && (count as usize) < output.len());
        output.truncate(count as usize);
        output
    }

    fn compare(spec: &str, arg: PrintfArg) {
        let c = CString::new(spec).unwrap();
        let wide = spec.contains(['l', 'j', 'z', 't']);
        assert_eq!(
            format(spec.as_bytes(), &[arg]).unwrap(),
            unsafe { oracle(&c, arg, wide) },
            "format {spec}, argument {arg:?}"
        );
    }

    #[test]
    fn exact_pdf_and_diagnostic_fragments() {
        assert_eq!(
            format(
                b"/Name /%s %04X %ld %.2x %%",
                &[
                    PrintfArg::from(c"Font".as_ptr()),
                    15u32.into(),
                    123i64.into(),
                    10u32.into()
                ]
            )
            .unwrap(),
            b"/Name /Font 000F 123 0a %"
        );
        assert_eq!(
            format(
                b"/Matrix [%.6g %.6g %.6g]\n\\%03o /Length %llu\n",
                &[
                    (-0.0).into(),
                    0.00001.into(),
                    1_000_000.0.into(),
                    10u32.into(),
                    u64::MAX.into()
                ]
            )
            .unwrap(),
            b"/Matrix [-0 1e-05 1e+06]\n\\012 /Length 18446744073709551615\n"
        );
        assert_eq!(
            format(b"%+03d'%02d'", &[(-7).into(), 30.into()]).unwrap(),
            b"-07'30'"
        );
    }

    #[test]
    fn zero_precision_prefix_sign_and_length_boundaries() {
        for value in [
            0,
            1,
            -1,
            i64::MIN,
            i64::MAX,
            255,
            256,
            32768,
            i32::MAX as i64 + 1,
        ] {
            for spec in [
                "%d", "%i", "%hhd", "%hd", "%ld", "%lld", "%jd", "%zd", "%td", "%+010d", "% 10.4i",
                "%#08x", "%#08X", "%#08o", "%#.0o", "%#.0x", "%#.8o", "%-10.0d", "%hhu", "%hu",
                "%lu", "%llu",
            ] {
                let arg = if spec.ends_with(['d', 'i']) {
                    PrintfArg::Signed(value)
                } else {
                    PrintfArg::Unsigned(value as u64)
                };
                compare(spec, arg);
            }
        }
    }

    #[test]
    fn floating_rounding_exponents_and_ieee_specials() {
        let negative_nan = f64::from_bits(f64::NAN.to_bits() | (1 << 63));
        for value in [
            0.0,
            -0.0,
            0.5,
            1.5,
            2.5,
            3.5,
            1.75,
            9.999,
            99.999,
            0.000099999,
            0.0000099999,
            1e99,
            1e100,
            f64::MIN_POSITIVE,
            f64::from_bits(1),
            f64::MAX,
            f64::NAN,
            negative_nan,
            f64::INFINITY,
            f64::NEG_INFINITY,
        ] {
            for spec in [
                "%f", "%.0f", "%#.0f", "%+020.4F", "% e", "%.0e", "%#.0E", "%.17e", "%g", "%.0g",
                "%.1g", "%.4g", "%#.4g", "%#.1G", "%-20.17G", "%020g",
            ] {
                compare(spec, value.into());
            }
        }
    }

    #[test]
    fn bytes_precision_and_character_padding() {
        for spec in ["%s", "%10s", "%-10s", "%.0s", "%.2s", "%10.4s"] {
            compare(spec, PrintfArg::from(c"\xC3\xA9abc".as_ptr()));
        }
        for spec in ["%c", "%4c", "%-4c"] {
            compare(spec, 0.into());
            compare(spec, 255.into());
        }
        for spec in ["%p", "%20p", "%-20p", "%020p"] {
            compare(spec, PrintfArg::Ptr(ptr::null()));
            compare(spec, PrintfArg::Ptr(0xabcdusize as *const c_void));
        }
    }

    #[test]
    fn unsupported_extensions_and_malformed_fields_are_explicit() {
        for source in [
            b"%a".as_slice(),
            b"%A",
            b"%'d",
            b"%*d",
            b"%.*f",
            b"%1$d",
            b"%n",
            b"%ls",
            b"%lc",
            b"%Lf",
            b"%",
            b"%1048577d",
            b"%.1048577f",
        ] {
            assert!(format(source, &[1.into()]).is_err(), "{:?}", source);
        }
        assert!(format(b"%s", &[]).is_err());
    }

    proptest::proptest! {
        #[test]
        fn arbitrary_integer_values_match_native_fixed_grammar(
            value in proptest::prelude::any::<u64>(),
            width in 0usize..32,
            precision in 0usize..24,
            flag in 0usize..8,
            conversion in 0usize..6,
            length in 0usize..8,
            explicit_precision in proptest::prelude::any::<bool>(),
        ) {
            let flags = ["", "0", "-", "+", " ", "#", "+0", "#0"][flag];
            let conversion = ["d", "i", "u", "o", "x", "X"][conversion];
            let length = ["", "hh", "h", "l", "ll", "j", "z", "t"][length];
            let field = if explicit_precision { format!(".{precision}") } else { String::new() };
            let spec = format!("%{flags}{width}{field}{length}{conversion}");
            let arg = if matches!(conversion, "d" | "i") { PrintfArg::Signed(value as i64) }
                else { PrintfArg::Unsigned(value) };
            compare(&spec, arg);
        }

        #[test]
        fn arbitrary_float_bits_match_native_decimal_grammar(
            bits in proptest::prelude::any::<u64>(),
            width in 0usize..32,
            precision in 0usize..18,
            flag in 0usize..8,
            conversion in 0usize..6,
            explicit_precision in proptest::prelude::any::<bool>(),
        ) {
            let flags = ["", "0", "-", "+", " ", "#", "+0", "#0"][flag];
            let conversion = ["f", "F", "e", "E", "g", "G"][conversion];
            let field = if explicit_precision { format!(".{precision}") } else { String::new() };
            let spec = format!("%{flags}{width}{field}{conversion}");
            compare(&spec, f64::from_bits(bits).into());
        }

        #[test]
        fn arbitrary_string_bytes_match_native_precision_and_padding(
            bytes in proptest::collection::vec(1u8..=255, 0..128),
            width in 0usize..160,
            precision in 0usize..160,
            left in proptest::prelude::any::<bool>(),
        ) {
            let string = CString::new(bytes).unwrap();
            let flag = if left { "-" } else { "" };
            let spec = format!("%{flag}{width}.{precision}s");
            compare(&spec, PrintfArg::from(string.as_ptr()));
        }
    }
}
