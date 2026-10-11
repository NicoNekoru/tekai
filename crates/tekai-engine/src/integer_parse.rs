//! Allocation-free integer prefixes for the engine's former C-locale parsers.
//!
//! The engine uses 64-bit signed long and unsigned long long values. Overflow
//! saturates at those bounds while the parser still consumes every valid digit.
//! No locale state is read, and legacy base-zero parsing has no `0b` extension.

#![warn(clippy::all)]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum IntegerBase {
    Auto,
    Decimal,
    Hex,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ParseStatus {
    NoDigits,
    InRange,
    Overflow,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ParsedInteger<T> {
    pub(crate) value: T,
    /// End offset within the original bytes, or zero when no digit converted.
    pub(crate) consumed: usize,
    pub(crate) status: ParseStatus,
}

/// The six whitespace bytes recognized by the default C locale.
pub(crate) fn is_c_whitespace(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c)
}

struct Cursor<'a> {
    bytes: &'a [u8],
    at: usize,
    #[cfg(test)]
    inspected: usize,
}

impl Cursor<'_> {
    fn peek(&mut self, ahead: usize) -> Option<u8> {
        #[cfg(test)]
        {
            self.inspected += 1;
        }
        self.bytes.get(self.at..)?.get(ahead).copied()
    }
}

struct Magnitude {
    value: u64,
    negative: bool,
    consumed: usize,
    status: ParseStatus,
    #[cfg(test)]
    inspected: usize,
}

fn digit(byte: u8, radix: u64) -> Option<u64> {
    let value = match byte {
        b'0'..=b'9' => byte - b'0',
        b'a'..=b'f' => byte - b'a' + 10,
        b'A'..=b'F' => byte - b'A' + 10,
        _ => return None,
    };
    (u64::from(value) < radix).then_some(u64::from(value))
}

fn magnitude(bytes: &[u8], base: IntegerBase, signed: bool) -> Magnitude {
    let mut cursor = Cursor {
        bytes,
        at: 0,
        #[cfg(test)]
        inspected: 0,
    };
    while cursor.peek(0).is_some_and(is_c_whitespace) {
        cursor.at += 1;
    }
    let negative = cursor.peek(0) == Some(b'-');
    if negative || cursor.peek(0) == Some(b'+') {
        cursor.at += 1;
    }
    let mut radix = match base {
        IntegerBase::Hex => 16,
        IntegerBase::Auto | IntegerBase::Decimal => 10,
    };
    if base != IntegerBase::Decimal && cursor.peek(0) == Some(b'0') {
        if matches!(cursor.peek(1), Some(b'x' | b'X'))
            && cursor.peek(2).and_then(|byte| digit(byte, 16)).is_some()
        {
            radix = 16;
            cursor.at += 2;
        } else if base == IntegerBase::Auto {
            radix = 8;
        }
    }
    let digits_start = cursor.at;
    let limit = if signed {
        (i64::MAX as u64) + u64::from(negative)
    } else {
        u64::MAX
    };
    let cutoff = limit / radix;
    let last_digit = limit % radix;
    let mut value = 0u64;
    let mut status = ParseStatus::InRange;
    while let Some(next) = cursor.peek(0).and_then(|byte| digit(byte, radix)) {
        if status != ParseStatus::Overflow {
            if value > cutoff || (value == cutoff && next > last_digit) {
                value = limit;
                status = ParseStatus::Overflow;
            } else {
                value = value * radix + next;
            }
        }
        cursor.at += 1;
    }
    let consumed = if cursor.at == digits_start {
        status = ParseStatus::NoDigits;
        0
    } else {
        cursor.at
    };
    Magnitude {
        value,
        negative,
        consumed,
        status,
        #[cfg(test)]
        inspected: cursor.inspected,
    }
}

pub(crate) fn signed_prefix(bytes: &[u8], base: IntegerBase) -> ParsedInteger<i64> {
    let parsed = magnitude(bytes, base, true);
    ParsedInteger {
        value: if parsed.negative {
            parsed.value.wrapping_neg() as i64
        } else {
            parsed.value as i64
        },
        consumed: parsed.consumed,
        status: parsed.status,
    }
}

pub(crate) fn unsigned_prefix(bytes: &[u8], base: IntegerBase) -> ParsedInteger<u64> {
    let parsed = magnitude(bytes, base, false);
    ParsedInteger {
        value: if parsed.negative && parsed.status != ParseStatus::Overflow {
            parsed.value.wrapping_neg()
        } else {
            parsed.value
        },
        consumed: parsed.consumed,
        status: parsed.status,
    }
}

/// Preserve defined `atoi` inputs and explicitly choose wrapping i32 narrowing
/// after signed-long saturation for out-of-range inputs. C leaves atoi overflow
/// undefined, so only representable i32 inputs have a portable native oracle.
pub(crate) fn decimal_i32_legacy(bytes: &[u8]) -> ParsedInteger<i32> {
    let parsed = signed_prefix(bytes, IntegerBase::Decimal);
    ParsedInteger {
        value: parsed.value as i32,
        consumed: parsed.consumed,
        status: if parsed.status == ParseStatus::InRange && i32::try_from(parsed.value).is_err() {
            ParseStatus::Overflow
        } else {
            parsed.status
        },
    }
}

/// Preserve SOURCE_DATE_EPOCH's full-consumption rule, including its historical
/// empty-string zero and unsigned negative spelling. Trailing whitespace fails.
pub(crate) fn source_date_epoch(bytes: &[u8]) -> Option<u64> {
    let parsed = unsigned_prefix(bytes, IntegerBase::Decimal);
    (parsed.consumed == bytes.len() && parsed.status != ParseStatus::Overflow)
        .then_some(parsed.value)
}

#[cfg(test)]
mod tests {
    use super::{
        decimal_i32_legacy, is_c_whitespace, magnitude, signed_prefix, source_date_epoch,
        unsigned_prefix, IntegerBase, ParseStatus, ParsedInteger,
    };
    use proptest::prelude::*;

    #[cfg(target_os = "macos")]
    unsafe fn errno() -> *mut libc::c_int {
        unsafe { libc::__error() }
    }

    #[cfg(target_os = "linux")]
    unsafe fn errno() -> *mut libc::c_int {
        unsafe { libc::__errno_location() }
    }

    fn base_from_index(index: u8) -> IntegerBase {
        match index {
            0 => IntegerBase::Auto,
            1 => IntegerBase::Decimal,
            _ => IntegerBase::Hex,
        }
    }

    fn native_base(base: IntegerBase) -> libc::c_int {
        match base {
            IntegerBase::Auto => 0,
            IntegerBase::Decimal => 10,
            IntegerBase::Hex => 16,
        }
    }

    fn native(bytes: &[u8], base: IntegerBase) -> (ParsedInteger<i64>, ParsedInteger<u64>) {
        assert_eq!(
            std::mem::size_of::<libc::c_long>(),
            std::mem::size_of::<i64>()
        );
        let mut buffer = bytes.to_vec();
        buffer.push(0);
        let start = buffer.as_ptr().cast();
        let mut end = std::ptr::null_mut();
        // The buffer is NUL terminated. Both native end pointers stay within it.
        unsafe {
            let saved_errno = *errno();
            *errno() = 0;
            let signed = libc::strtol(start, &mut end, native_base(base));
            let signed_consumed = end.cast::<u8>().offset_from(buffer.as_ptr()) as usize;
            let signed_status = if *errno() == libc::ERANGE {
                ParseStatus::Overflow
            } else if signed_consumed == 0 {
                ParseStatus::NoDigits
            } else {
                ParseStatus::InRange
            };
            *errno() = 0;
            let unsigned = libc::strtoull(start, &mut end, native_base(base));
            let unsigned_consumed = end.cast::<u8>().offset_from(buffer.as_ptr()) as usize;
            let unsigned_status = if *errno() == libc::ERANGE {
                ParseStatus::Overflow
            } else if unsigned_consumed == 0 {
                ParseStatus::NoDigits
            } else {
                ParseStatus::InRange
            };
            *errno() = saved_errno;
            (
                ParsedInteger {
                    value: signed,
                    consumed: signed_consumed,
                    status: signed_status,
                },
                ParsedInteger {
                    value: unsigned,
                    consumed: unsigned_consumed,
                    status: unsigned_status,
                },
            )
        }
    }

    #[test]
    fn prefix_edges_match_native_values_end_offsets_and_range_status() {
        let cases: &[&[u8]] = &[
            b"",
            b" \t\n\r\x0b\x0c",
            b"+",
            b" -",
            b"0",
            b"08",
            b"0x",
            b"0xG",
            b"0Xf!",
            b"-0x80tail",
            b"9223372036854775807",
            b"9223372036854775808!",
            b"-9223372036854775808",
            b"-9223372036854775809!",
            b"18446744073709551615",
            b"18446744073709551616!",
            b"-1",
            b"-18446744073709551615",
            b"-18446744073709551616!",
            b"\x0b42\0tail",
            b"\xff12",
            b"00FF",
        ];
        for base in [IntegerBase::Auto, IntegerBase::Decimal, IntegerBase::Hex] {
            for bytes in cases {
                let (signed, unsigned) = native(bytes, base);
                assert_eq!(signed_prefix(bytes, base), signed, "{bytes:?} {base:?}");
                assert_eq!(unsigned_prefix(bytes, base), unsigned, "{bytes:?} {base:?}");
            }
        }
        assert_eq!(
            signed_prefix(b"0b11", IntegerBase::Auto),
            ParsedInteger {
                value: 0,
                consumed: 1,
                status: ParseStatus::InRange
            }
        );
    }

    #[test]
    fn c_whitespace_is_exactly_the_six_default_locale_bytes() {
        for byte in u8::MIN..=u8::MAX {
            assert_eq!(is_c_whitespace(byte), b" \t\n\r\x0b\x0c".contains(&byte));
        }
    }

    #[test]
    fn legacy_i32_overflow_has_an_explicit_saturation_then_narrowing_policy() {
        for (bytes, expected) in [
            (b"2147483648".as_slice(), i32::MIN),
            (b"4294967296".as_slice(), 0),
            (b"-2147483649".as_slice(), i32::MAX),
            (b"9223372036854775808".as_slice(), -1),
            (b"-9223372036854775809".as_slice(), 0),
        ] {
            let parsed = decimal_i32_legacy(bytes);
            assert_eq!(parsed.value, expected);
            assert_eq!(parsed.status, ParseStatus::Overflow);
            assert_eq!(parsed.consumed, bytes.len());
        }
        assert_eq!(decimal_i32_legacy(b"+bad").status, ParseStatus::NoDigits);
    }

    #[test]
    fn epoch_preserves_full_consumption_and_legacy_empty_negative_values() {
        assert_eq!(source_date_epoch(b""), Some(0));
        assert_eq!(source_date_epoch(b" \x0b+42"), Some(42));
        assert_eq!(source_date_epoch(b"-1"), Some(u64::MAX));
        assert_eq!(source_date_epoch(b"18446744073709551615"), Some(u64::MAX));
        for bytes in [
            b" ".as_slice(),
            b"+",
            b"42 ",
            b"0x10",
            b"18446744073709551616",
        ] {
            assert_eq!(source_date_epoch(bytes), None, "{bytes:?}");
        }
    }

    #[test]
    fn overflowing_digit_runs_use_one_pass_and_constant_storage() {
        for length in [1usize, 20, 4096, 1_000_000] {
            let mut bytes = vec![b'9'; length];
            bytes.extend_from_slice(b"!unread suffix");
            for signed in [false, true] {
                let parsed = magnitude(&bytes, IntegerBase::Decimal, signed);
                assert_eq!(parsed.consumed, length);
                assert!(parsed.inspected <= length + 8);
                if length >= 20 {
                    assert_eq!(parsed.status, ParseStatus::Overflow);
                }
            }
        }
    }

    proptest! {
        #[test]
        fn arbitrary_byte_prefixes_match_native(
            bytes in prop::collection::vec(any::<u8>(), 0..512),
            base_index in 0u8..3,
        ) {
            let base = base_from_index(base_index);
            let (signed, unsigned) = native(&bytes, base);
            prop_assert_eq!(signed_prefix(&bytes, base), signed);
            prop_assert_eq!(unsigned_prefix(&bytes, base), unsigned);
            for signed in [false, true] {
                let parsed = magnitude(&bytes, base, signed);
                prop_assert!(parsed.consumed <= bytes.len());
                prop_assert!(parsed.inspected <= parsed.consumed.max(bytes.iter().take_while(|&&byte| is_c_whitespace(byte)).count()) + 9);
            }
        }

        #[test]
        fn structured_numeric_prefixes_match_native(
            whitespace in prop::collection::vec(prop::sample::select(b" \t\n\r\x0b\x0c".to_vec()), 0..16),
            sign in prop::sample::select(vec![b"".as_slice(), b"+", b"-"]),
            prefix in prop::sample::select(vec![b"".as_slice(), b"0", b"0x", b"0X"]),
            digits in prop::collection::vec(prop::sample::select(b"0123456789abcdefABCDEF".to_vec()), 0..256),
            suffix in prop::sample::select(vec![b"".as_slice(), b"!tail", b"\0hidden", b" "]),
            base_index in 0u8..3,
        ) {
            let mut bytes = whitespace;
            bytes.extend_from_slice(sign);
            bytes.extend_from_slice(prefix);
            bytes.extend_from_slice(&digits);
            bytes.extend_from_slice(suffix);
            let base = base_from_index(base_index);
            let (signed, unsigned) = native(&bytes, base);
            prop_assert_eq!(signed_prefix(&bytes, base), signed);
            prop_assert_eq!(unsigned_prefix(&bytes, base), unsigned);
        }

        #[test]
        fn representable_atoi_values_and_suffixes_match_native(
            value in any::<i32>(),
            leading in prop::sample::select(vec!["", "\x0b", " \t\r\n\x0c"]),
            plus in any::<bool>(),
            suffix in prop::sample::select(vec!["", "!tail", " trailing"]),
        ) {
            let bytes = format!("{leading}{}{value}{suffix}", if plus && value >= 0 { "+" } else { "" }).into_bytes();
            let parsed = decimal_i32_legacy(&bytes);
            let mut buffer = bytes.clone();
            buffer.push(0);
            let native = unsafe { libc::atoi(buffer.as_ptr().cast()) };
            prop_assert_eq!(parsed.value, native);
            prop_assert_eq!(parsed.value, value);
            prop_assert_eq!(parsed.status, ParseStatus::InRange);
            prop_assert_eq!(parsed.consumed, bytes.len() - suffix.len());
        }

        #[test]
        fn epoch_matches_native_full_conversion_without_errno_dependency(
            bytes in prop::collection::vec(prop::sample::select(b"0123456789+- \t\n\r\x0b\x0c!x".to_vec()), 0..256),
        ) {
            let (_, native) = native(&bytes, IntegerBase::Decimal);
            let expected = (native.consumed == bytes.len() && native.status != ParseStatus::Overflow).then_some(native.value);
            prop_assert_eq!(source_date_epoch(&bytes), expected);
        }
    }
}
