//! Checked C-locale floating fields for the backend's fixed scan formats.
//!
//! Parsing borrows the initialized input and takes linear work. Hexadecimal
//! rounding keeps at most 64 significant bits and one sticky bit, regardless
//! of token length. Decimal conversion goes directly to the requested Rust
//! float type, avoiding an f64-to-f32 double rounding step. Only finite fields
//! are admitted. Finite underflow, including signed zero, remains valid.

#![warn(clippy::all)]

use crate::integer_parse::is_c_whitespace;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FloatError {
    NoMatch,
    NonFinite,
    Overflow,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ParsedFloat<T> {
    pub(crate) value: T,
    /// Offset in the original initialized bytes, including leading whitespace.
    pub(crate) consumed: usize,
}

struct Number<'a> {
    token: &'a [u8],
    consumed: usize,
    hexadecimal: bool,
}

fn hex_digit(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn number(bytes: &[u8], adjacent_extend: bool) -> Result<Number<'_>, FloatError> {
    let start = bytes
        .iter()
        .take_while(|&&byte| is_c_whitespace(byte))
        .count();
    let mut at = start;
    if matches!(bytes.get(at), Some(b'+' | b'-')) {
        at += 1;
    }
    if bytes
        .get(at..at.saturating_add(3))
        .is_some_and(|word| word.eq_ignore_ascii_case(b"inf") || word.eq_ignore_ascii_case(b"nan"))
    {
        return Err(FloatError::NonFinite);
    }
    let hexadecimal =
        bytes.get(at) == Some(&b'0') && matches!(bytes.get(at + 1), Some(b'x' | b'X'));
    if hexadecimal {
        at += 2;
    }
    let mut digits = 0usize;
    let mut point = false;
    while let Some(&byte) = bytes.get(at) {
        if (hexadecimal && hex_digit(byte).is_some()) || (!hexadecimal && byte.is_ascii_digit()) {
            digits += 1;
            at += 1;
        } else if byte == b'.' && !point {
            point = true;
            at += 1;
        } else {
            break;
        }
    }
    if digits == 0 {
        return Err(FloatError::NoMatch);
    }
    let exponent = if hexadecimal { b'p' } else { b'e' };
    if bytes
        .get(at)
        .is_some_and(|byte| byte.to_ascii_lowercase() == exponent)
        && !(adjacent_extend && !hexadecimal && bytes[at..].starts_with(b"ExtendFont"))
    {
        at += 1;
        if matches!(bytes.get(at), Some(b'+' | b'-')) {
            at += 1;
        }
        let first_digit = at;
        while bytes.get(at).is_some_and(u8::is_ascii_digit) {
            at += 1;
        }
        // POSIX scanf reads the longest possible input item. An unfinished
        // exponent is a matching failure, not a successful shorter decimal.
        if at == first_digit {
            return Err(FloatError::NoMatch);
        }
    }
    Ok(Number {
        token: &bytes[start..at],
        consumed: at,
        hexadecimal,
    })
}

#[derive(Default)]
struct HexBits {
    top: u64,
    kept: u32,
    total: i64,
    sticky: bool,
}

impl HexBits {
    fn digit(&mut self, digit: u8) {
        for shift in (0..4).rev() {
            let bit = (digit >> shift) & 1;
            if self.total == 0 && bit == 0 {
                continue;
            }
            self.total = self.total.saturating_add(1);
            if self.kept < 64 {
                self.top = (self.top << 1) | u64::from(bit);
                self.kept += 1;
            } else {
                self.sticky |= bit != 0;
            }
        }
    }

    fn rounded(&self, wanted: u32) -> u64 {
        if wanted >= self.kept {
            return self.top << (wanted - self.kept);
        }
        let shift = self.kept - wanted;
        let retained = if shift == 64 { 0 } else { self.top >> shift };
        let halfway = self.top & (1u64 << (shift - 1)) != 0;
        let lower_mask = (1u64 << (shift - 1)) - 1;
        let beyond_halfway = self.sticky || self.top & lower_mask != 0;
        retained + u64::from(halfway && (beyond_halfway || retained & 1 != 0))
    }
}

fn exponent_value(bytes: &[u8]) -> i64 {
    let negative = bytes.first() == Some(&b'-');
    let digits = if matches!(bytes.first(), Some(b'+' | b'-')) {
        &bytes[1..]
    } else {
        bytes
    };
    let mut value = 0i64;
    for &digit in digits {
        value = value
            .saturating_mul(10)
            .saturating_add(i64::from(digit - b'0'));
    }
    if negative {
        -value
    } else {
        value
    }
}

/// Round a valid hexadecimal token directly to binary32 or binary64, including
/// ties at the normal/subnormal boundary. No intermediate float is constructed.
fn hexadecimal_bits(
    token: &[u8],
    precision: u32,
    minimum: i64,
    maximum: i64,
    bias: i64,
    sign_bit: u32,
) -> Result<u64, FloatError> {
    let negative = token.first() == Some(&b'-');
    let sign = u64::from(negative) << sign_bit;
    let unsigned = if matches!(token.first(), Some(b'+' | b'-')) {
        &token[1..]
    } else {
        token
    };
    let mut bits = HexBits::default();
    let mut fractional_digits = 0i64;
    let mut fractional = false;
    let mut at = 2;
    while let Some(&byte) = unsigned.get(at) {
        if let Some(digit) = hex_digit(byte) {
            bits.digit(digit);
            if fractional {
                fractional_digits = fractional_digits.saturating_add(1);
            }
        } else if byte == b'.' {
            fractional = true;
        } else {
            break;
        }
        at += 1;
    }
    if bits.total == 0 {
        return Ok(sign);
    }
    let explicit_exponent = unsigned.get(at + 1..).map_or(0, exponent_value);
    let mut exponent = bits
        .total
        .saturating_sub(1)
        .saturating_sub(fractional_digits.saturating_mul(4))
        .saturating_add(explicit_exponent);
    let fraction_bits = precision - 1;
    let least_subnormal = minimum - i64::from(fraction_bits);
    if exponent > maximum {
        return Err(FloatError::Overflow);
    }
    if exponent < least_subnormal - 1 {
        return Ok(sign);
    }
    let wanted = if exponent >= minimum {
        precision
    } else {
        u32::try_from(exponent - least_subnormal + 1).unwrap()
    };
    let mut significand = bits.rounded(wanted);
    if exponent < minimum {
        // Rounding a largest subnormal to the smallest normal also has this
        // integer bit representation, so no second rounding is necessary.
        return Ok(sign | significand);
    }
    if significand == 1u64 << precision {
        significand >>= 1;
        exponent += 1;
        if exponent > maximum {
            return Err(FloatError::Overflow);
        }
    }
    Ok(sign
        | (u64::try_from(exponent + bias).unwrap() << fraction_bits)
        | (significand - (1u64 << fraction_bits)))
}

fn convert_f32(parsed: Number<'_>) -> Result<ParsedFloat<f32>, FloatError> {
    let value = if parsed.hexadecimal {
        f32::from_bits(hexadecimal_bits(parsed.token, 24, -126, 127, 127, 31)? as u32)
    } else {
        std::str::from_utf8(parsed.token)
            .map_err(|_| FloatError::NoMatch)?
            .parse::<f32>()
            .map_err(|_| FloatError::NoMatch)?
    };
    if !value.is_finite() {
        return Err(FloatError::Overflow);
    }
    Ok(ParsedFloat {
        value,
        consumed: parsed.consumed,
    })
}

pub(crate) fn scan_f32(bytes: &[u8]) -> Result<ParsedFloat<f32>, FloatError> {
    convert_f32(number(bytes, false)?)
}

pub(crate) fn scan_f64(bytes: &[u8]) -> Result<ParsedFloat<f64>, FloatError> {
    let parsed = number(bytes, false)?;
    let value = if parsed.hexadecimal {
        f64::from_bits(hexadecimal_bits(parsed.token, 53, -1022, 1023, 1023, 63)?)
    } else {
        std::str::from_utf8(parsed.token)
            .map_err(|_| FloatError::NoMatch)?
            .parse::<f64>()
            .map_err(|_| FloatError::NoMatch)?
    };
    if !value.is_finite() {
        return Err(FloatError::Overflow);
    }
    Ok(ParsedFloat {
        value,
        consumed: parsed.consumed,
    })
}

/// The map's `%f %n` grammar consumes trailing C whitespace. Its historical
/// exponent-letter backup permits a number immediately before `ExtendFont`.
pub(crate) fn map_operand(bytes: &[u8]) -> Result<ParsedFloat<f32>, FloatError> {
    let mut parsed = convert_f32(number(bytes, true)?)?;
    parsed.consumed += bytes[parsed.consumed..]
        .iter()
        .take_while(|&&byte| is_c_whitespace(byte))
        .count();
    Ok(parsed)
}

/// Preserve the Type1 caller's separate cursor grammar. It skipped one ASCII
/// space, then advanced over decimal-looking bytes independently of `%g`.
pub(crate) fn type1_number(bytes: &[u8]) -> Result<ParsedFloat<f32>, FloatError> {
    let leading_space = usize::from(bytes.first() == Some(&b' '));
    let field = &bytes[leading_space..];
    let mut parsed = scan_f32(field)?;
    parsed.consumed = leading_space
        + field
            .iter()
            .take_while(|&&byte| {
                byte.is_ascii_digit() || matches!(byte, b'.' | b'e' | b'E' | b'+' | b'-')
            })
            .count();
    Ok(parsed)
}

/// Keep the Type3 literal prefix, one-byte separator skip and prefix-number
/// conversion. The whole line must be initialized before this adapter runs.
pub(crate) fn font_scale_line(bytes: &[u8]) -> Result<f32, FloatError> {
    let prefix = b"\\pdffontscale";
    if !bytes.starts_with(prefix) {
        return Err(FloatError::NoMatch);
    }
    let field = bytes.get(prefix.len() + 1..).ok_or(FloatError::NoMatch)?;
    let value = scan_f32(field)?.value;
    if value <= 0.0 || value > 1000.0 {
        return Err(FloatError::NoMatch);
    }
    Ok(value)
}

/// Preserve the map's two rounding steps, first its f32 scaled value and then
/// half-away-from-zero integer conversion. Do not silently saturate a cast or
/// admit i32::MIN, which the subsequent signed-absolute-value check cannot use.
pub(crate) fn scaled_map_operand(value: f32) -> Result<i32, FloatError> {
    let scaled = (f64::from(value) * 1000.0) as f32;
    let adjusted = f64::from(scaled) + if scaled > 0.0 { 0.5 } else { -0.5 };
    let rounded = adjusted.trunc();
    if !rounded.is_finite() || rounded <= f64::from(i32::MIN) || rounded > f64::from(i32::MAX) {
        return Err(FloatError::Overflow);
    }
    Ok(rounded as i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use std::ffi::CString;

    unsafe extern "C" {
        fn sscanf(input: *const libc::c_char, format: *const libc::c_char, ...) -> libc::c_int;
        fn strtod(input: *const libc::c_char, end: *mut *mut libc::c_char) -> libc::c_double;
    }

    fn native_f32(bytes: &[u8]) -> (i32, f32, i32) {
        let input = CString::new(bytes).unwrap();
        let mut value = 0.0f32;
        let mut consumed = -1i32;
        let assignments =
            unsafe { sscanf(input.as_ptr(), c"%f %n".as_ptr(), &mut value, &mut consumed) };
        (assignments, value, consumed)
    }

    fn native_f64(bytes: &[u8]) -> (f64, usize) {
        let input = CString::new(bytes).unwrap();
        let mut end = std::ptr::null_mut();
        let value = unsafe { strtod(input.as_ptr(), &mut end) };
        (value, unsafe { end.offset_from(input.as_ptr()) } as usize)
    }

    #[test]
    fn decimal_prefixes_preserve_c_whitespace_sign_zero_and_suffixes() {
        for bytes in [
            b" \t-0tail".as_slice(),
            b"+.5 ",
            b"1.",
            b" 1e-3font",
            b"-12.25\n",
        ] {
            let parsed = scan_f32(bytes).unwrap();
            let (_, expected, _) = native_f32(bytes);
            assert_eq!(parsed.value.to_bits(), expected.to_bits(), "{bytes:?}");
            let map = map_operand(bytes).unwrap();
            let (assigned, _, consumed) = native_f32(bytes);
            assert_eq!(assigned, 1);
            assert_eq!(map.consumed, consumed as usize, "{bytes:?}");
        }
        assert_eq!(
            scan_f32(b"-0").unwrap().value.to_bits(),
            (-0.0f32).to_bits()
        );
        assert_eq!(
            scan_f64(b"-0").unwrap().value.to_bits(),
            (-0.0f64).to_bits()
        );
        assert_eq!(scan_f32(b"1\0ignored").unwrap().consumed, 1);
        assert_eq!(
            map_operand(b"1ExtendFont").unwrap(),
            ParsedFloat {
                value: 1.0,
                consumed: 1
            }
        );
        assert_eq!(map_operand(b"1e-3ExtendFont").unwrap().consumed, 4);
    }

    #[test]
    fn failures_never_publish_nonfinite_or_partial_exponent_values() {
        for bytes in [
            b"".as_slice(),
            b" \t",
            b"+",
            b".",
            b"1e",
            b"1e+",
            b"0x",
            b"0x.p0",
            b"0x1p-",
        ] {
            assert_eq!(scan_f32(bytes), Err(FloatError::NoMatch), "{bytes:?}");
            assert_eq!(scan_f64(bytes), Err(FloatError::NoMatch), "{bytes:?}");
        }
        for bytes in [b"nan".as_slice(), b"-NaN(payload)", b"+Inf", b"INFINITY"] {
            assert_eq!(scan_f32(bytes), Err(FloatError::NonFinite), "{bytes:?}");
            assert_eq!(scan_f64(bytes), Err(FloatError::NonFinite), "{bytes:?}");
        }
        for bytes in [
            b"1e999999999999999999999".as_slice(),
            b"-0x1p999999999999999999999",
        ] {
            assert_eq!(scan_f32(bytes), Err(FloatError::Overflow));
            assert_eq!(scan_f64(bytes), Err(FloatError::Overflow));
        }
        for bytes in [
            b"-1e-999999999999999999999".as_slice(),
            b"-0x1p-999999999999999999999",
        ] {
            assert_eq!(
                scan_f32(bytes).unwrap().value.to_bits(),
                (-0.0f32).to_bits()
            );
            assert_eq!(
                scan_f64(bytes).unwrap().value.to_bits(),
                (-0.0f64).to_bits()
            );
        }
    }

    #[test]
    fn hex_rounding_matches_independent_ieee_boundary_bits() {
        for (bytes, bits) in [
            (b"0x1p-149".as_slice(), 1u32),
            (b"0x1p-150", 0),
            (b"0x1.000001p-150", 1),
            (b"0x1.fffffep-127", 0x0080_0000),
            (b"0x1.000001p0", 1.0f32.to_bits()),
            (b"0x1.00000100000000000001p0", 1.0f32.to_bits() + 1),
            (b"0x1.000003p0", 1.0f32.to_bits() + 2),
            (b"0x1.fffffep127", f32::MAX.to_bits()),
        ] {
            assert_eq!(scan_f32(bytes).unwrap().value.to_bits(), bits, "{bytes:?}");
        }
        assert_eq!(scan_f32(b"0x1.ffffffp127"), Err(FloatError::Overflow));
        for (bytes, bits) in [
            (b"0x1p-1074".as_slice(), 1u64),
            (b"0x1p-1075", 0),
            (b"0x1.0000000000001p-1075", 1),
            (b"0x1.fffffffffffffp-1023", 0x0010_0000_0000_0000),
            (b"0x1.00000000000008p0", 1.0f64.to_bits()),
            (b"0x1.00000000000008000001p0", 1.0f64.to_bits() + 1),
            (b"0x1.00000000000018p0", 1.0f64.to_bits() + 2),
            (b"0x1.fffffffffffffp1023", f64::MAX.to_bits()),
        ] {
            assert_eq!(scan_f64(bytes).unwrap().value.to_bits(), bits, "{bytes:?}");
        }
        assert_eq!(
            scan_f64(b"0x1.fffffffffffff8p1023"),
            Err(FloatError::Overflow)
        );
    }

    #[test]
    fn caller_formats_keep_distinct_literal_and_cursor_rules() {
        assert_eq!(
            type1_number(b" 1.25 trailing").unwrap(),
            ParsedFloat {
                value: 1.25,
                consumed: 5
            }
        );
        assert_eq!(type1_number(b"  1.25").unwrap().consumed, 1);
        assert_eq!(
            type1_number(b"0x1p2").unwrap(),
            ParsedFloat {
                value: 4.0,
                consumed: 1
            }
        );
        assert_eq!(font_scale_line(b"\\pdffontscale 0.001 rest"), Ok(0.001));
        assert_eq!(font_scale_line(b"\\pdffontscaleX1"), Ok(1.0));
        for bytes in [
            b"\\pdffontscale".as_slice(),
            b"\\pdffontscale ",
            b"\\pdffontscale nan",
            b"\\pdffontscale 0",
            b"\\pdffontscale -1",
            b"\\pdffontscale 1001",
            b"wrong 1",
        ] {
            assert!(font_scale_line(bytes).is_err(), "{bytes:?}");
        }
        for value in [
            f32::INFINITY,
            f32::NAN,
            f32::MAX,
            -f32::MAX,
            i32::MIN as f32 / 1000.0,
        ] {
            assert_eq!(scaled_map_operand(value), Err(FloatError::Overflow));
        }
        assert_eq!(scaled_map_operand(0.0005), Ok(1));
        assert_eq!(scaled_map_operand(-0.0005), Ok(-1));
    }

    #[test]
    fn direct_decimal_f32_conversion_avoids_double_rounding() {
        let token = b"1.000000059604644775390625000001";
        assert_eq!(
            scan_f32(token).unwrap().value.to_bits(),
            1.0f32.to_bits() + 1
        );
        assert_eq!(scan_f64(token).unwrap().value as f32, 1.0f32);
    }

    #[test]
    fn long_mantissas_and_exponents_do_not_grow_parser_storage() {
        let mut token = b"0x1.".to_vec();
        token.extend(std::iter::repeat_n(b'0', 131_072));
        token.extend_from_slice(b"1p0");
        assert_eq!(scan_f64(&token).unwrap().value, 1.0);
        let mut bits = HexBits::default();
        for _ in 0..131_072 {
            bits.digit(15);
        }
        assert_eq!(bits.kept, 64);
        assert!(bits.sticky);
        assert!(std::mem::size_of::<HexBits>() <= 32);
        let mut zero = b"0x0p".to_vec();
        zero.extend(std::iter::repeat_n(b'9', 131_072));
        assert_eq!(scan_f32(&zero).unwrap().value, 0.0);
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(256))]
        #[test]
        fn generated_finite_decimal_fields_match_native_scans(
            mantissa in -9_999_999i64..=9_999_999,
            fraction in 0u32..=999_999,
            exponent in -25i32..=25,
            whitespace in prop::collection::vec(prop::sample::select(vec![b' ', b'\t', b'\n', b'\r', 0x0b, 0x0c]), 0..8),
        ) {
            let mut token = whitespace;
            token.extend_from_slice(format!("{mantissa}.{fraction:06}e{exponent:+} suffix").as_bytes());
            let map = map_operand(&token).unwrap();
            let (assigned, value, consumed) = native_f32(&token);
            prop_assert_eq!(assigned, 1);
            prop_assert_eq!(map.value.to_bits(), value.to_bits());
            prop_assert_eq!(map.consumed, consumed as usize);
            let parsed64 = scan_f64(&token).unwrap();
            let (value64, consumed64) = native_f64(&token);
            prop_assert_eq!(parsed64.value.to_bits(), value64.to_bits());
            prop_assert_eq!(parsed64.consumed, consumed64);
        }

        #[test]
        fn generated_hex_fields_match_native_rounding(
            significant in 1u64..=u64::MAX,
            exponent in -120i32..=40,
            negative in any::<bool>(),
        ) {
            let token = format!("{}0x{significant:x}p{exponent:+} suffix", if negative { "-" } else { "" });
            let parsed = map_operand(token.as_bytes()).unwrap();
            let (assigned, value, consumed) = native_f32(token.as_bytes());
            prop_assert_eq!(assigned, 1);
            prop_assert_eq!(parsed.value.to_bits(), value.to_bits());
            prop_assert_eq!(parsed.consumed, consumed as usize);
            let parsed64 = scan_f64(token.as_bytes()).unwrap();
            let (value64, consumed64) = native_f64(token.as_bytes());
            prop_assert_eq!(parsed64.value.to_bits(), value64.to_bits());
            prop_assert_eq!(parsed64.consumed, consumed64);
        }

        #[test]
        fn arbitrary_bounded_bytes_never_emit_nonfinite_values(bytes in prop::collection::vec(any::<u8>(), 0..256)) {
            if let Ok(parsed) = scan_f32(&bytes) {
                prop_assert!(parsed.value.is_finite());
                prop_assert!(parsed.consumed <= bytes.len());
            }
            if let Ok(parsed) = scan_f64(&bytes) {
                prop_assert!(parsed.value.is_finite());
                prop_assert!(parsed.consumed <= bytes.len());
            }
        }
    }
}
