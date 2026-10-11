//! Checked byte fields for the backend's fixed input grammars.
//!
//! Callers provide the initialized input slice, excluding its C terminator.
//! Failed fields leave the cursor unchanged. Literal and whitespace handling
//! stay explicit so each caller can keep its own suffix-consumption policy.

#![warn(clippy::all)]

use crate::integer_parse::{
    is_c_whitespace, signed_prefix, unsigned_prefix, IntegerBase, ParseStatus,
};
use std::ops::Range;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FieldError {
    NoMatch,
    Overflow,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Cursor<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl<'a> Cursor<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, at: 0 }
    }

    pub(crate) fn offset(&self) -> usize {
        self.at
    }

    pub(crate) fn remaining(&self) -> &'a [u8] {
        &self.bytes[self.at..]
    }

    pub(crate) fn peek(&self) -> Option<u8> {
        self.remaining().first().copied()
    }

    pub(crate) fn skip_whitespace(&mut self) {
        self.at += self
            .remaining()
            .iter()
            .take_while(|&&byte| is_c_whitespace(byte))
            .count();
    }

    pub(crate) fn literal(&mut self, literal: &[u8]) -> Result<(), FieldError> {
        if !self.remaining().starts_with(literal) {
            return Err(FieldError::NoMatch);
        }
        self.at += literal.len();
        Ok(())
    }

    /// Parse a signed long prefix, including automatic octal/hex selection.
    pub(crate) fn integer(&mut self, base: IntegerBase) -> Result<i64, FieldError> {
        let parsed = signed_prefix(self.remaining(), base);
        match parsed.status {
            ParseStatus::NoDigits => Err(FieldError::NoMatch),
            ParseStatus::Overflow => Err(FieldError::Overflow),
            ParseStatus::InRange => {
                self.at += parsed.consumed;
                Ok(parsed.value)
            }
        }
    }

    /// Width counts field bytes after leading whitespace, including its sign
    /// and optional hexadecimal prefix. Callers decide whether signs are valid.
    pub(crate) fn hex(&mut self, width: Option<usize>) -> Result<u64, FieldError> {
        let leading = self
            .remaining()
            .iter()
            .take_while(|&&byte| is_c_whitespace(byte))
            .count();
        let field = &self.remaining()[leading..];
        let field = &field[..width.unwrap_or(field.len()).min(field.len())];
        let parsed = unsigned_prefix(field, IntegerBase::Hex);
        match parsed.status {
            ParseStatus::NoDigits => Err(FieldError::NoMatch),
            ParseStatus::Overflow => Err(FieldError::Overflow),
            ParseStatus::InRange => {
                self.at += leading + parsed.consumed;
                Ok(parsed.value)
            }
        }
    }

    /// Return the word without copying or appending a terminator. A field width
    /// limits the returned prefix, as in the backend's existing string scans.
    pub(crate) fn word(&mut self, width: Option<usize>) -> Result<&'a [u8], FieldError> {
        let leading = self
            .remaining()
            .iter()
            .take_while(|&&byte| is_c_whitespace(byte))
            .count();
        let field = &self.remaining()[leading..];
        let limit = width.unwrap_or(field.len()).min(field.len());
        let length = field[..limit]
            .iter()
            .take_while(|&&byte| !is_c_whitespace(byte))
            .count();
        if length == 0 {
            return Err(FieldError::NoMatch);
        }
        self.at += leading + length;
        Ok(&field[..length])
    }
}

/// Check a whole span before any source or destination array access. An empty
/// span may start at the one-past-the-end position, but a nonempty one may not.
pub(crate) fn checked_span(start: i64, count: i64, slots: usize) -> Option<Range<usize>> {
    let start = usize::try_from(start).ok()?;
    let count = usize::try_from(count).ok()?;
    let end = start.checked_add(count)?;
    (end <= slots).then_some(start..end)
}

/// Check the destination capacity for an inclusive SFD character-code range.
/// Reversed ranges fail even when their arithmetic would yield an empty span.
pub(crate) fn checked_inclusive_span(
    start: i64,
    first: i64,
    last: i64,
    slots: usize,
) -> Option<Range<usize>> {
    if first > last {
        return None;
    }
    let count = last.checked_sub(first)?.checked_add(1)?;
    checked_span(start, count, slots)
}

/// Normalize the ToUnicode sequence into a NUL-terminated fixed buffer. Reject
/// insufficient capacity before writing any byte, including the terminator.
pub(crate) fn copy_without_spaces(input: &[u8], output: &mut [u8]) -> Result<usize, FieldError> {
    let length = input.iter().filter(|&&byte| byte != b' ').count();
    if length >= output.len() {
        return Err(FieldError::Overflow);
    }
    for (slot, &byte) in output
        .iter_mut()
        .zip(input.iter().filter(|&&byte| byte != b' '))
    {
        *slot = byte;
    }
    output[length] = 0;
    Ok(length)
}

#[cfg(test)]
mod tests {
    use super::{checked_inclusive_span, checked_span, copy_without_spaces, Cursor, FieldError};
    use crate::integer_parse::{is_c_whitespace, signed_prefix, IntegerBase, ParseStatus};
    use proptest::prelude::*;

    #[test]
    fn signed_fields_preserve_prefixes_and_explicit_trailing_whitespace() {
        let mut cursor = Cursor::new(b" \t-010 \n: 0x10tail");
        assert_eq!(cursor.integer(IntegerBase::Auto), Ok(-8));
        assert_eq!(cursor.offset(), 6);
        cursor.skip_whitespace();
        assert_eq!(cursor.offset(), 8);
        assert_eq!(cursor.literal(b":"), Ok(()));
        assert_eq!(cursor.integer(IntegerBase::Auto), Ok(16));
        assert_eq!(cursor.remaining(), b"tail");
    }

    #[test]
    fn missing_or_overflowing_fields_never_reuse_a_value_or_offset() {
        for (bytes, expected) in [
            (b"".as_slice(), FieldError::NoMatch),
            (b" \t\n", FieldError::NoMatch),
            (b"+", FieldError::NoMatch),
            (b"9223372036854775808", FieldError::Overflow),
            (b"-9223372036854775809", FieldError::Overflow),
        ] {
            let mut cursor = Cursor::new(bytes);
            assert_eq!(cursor.integer(IntegerBase::Auto), Err(expected));
            assert_eq!(cursor.offset(), 0);
        }
        let mut cursor = Cursor::new(b"18446744073709551616");
        assert_eq!(cursor.hex(None), Err(FieldError::Overflow));
        assert_eq!(cursor.offset(), 0);
    }

    #[test]
    fn hexadecimal_width_excludes_leading_whitespace() {
        let mut cursor = Cursor::new(b" \t0041D800");
        assert_eq!(cursor.hex(Some(4)), Ok(0x41));
        assert_eq!(cursor.offset(), 6);
        assert_eq!(cursor.hex(Some(4)), Ok(0xd800));
        assert!(cursor.remaining().is_empty());
        let mut cursor = Cursor::new(b" +0x41!");
        assert_eq!(cursor.hex(None), Ok(0x41));
        assert_eq!(cursor.remaining(), b"!");
    }

    #[test]
    fn words_are_borrowed_and_width_limited_without_implicit_suffix_checks() {
        let mut cursor = Cursor::new(b" \t/Example put");
        assert_eq!(cursor.word(Some(4)), Ok(b"/Exa".as_slice()));
        assert_eq!(cursor.remaining(), b"mple put");
        assert_eq!(cursor.literal(b" put"), Err(FieldError::NoMatch));
        assert_eq!(cursor.offset(), 6);
        let mut cursor = Cursor::new(b" \n");
        assert_eq!(cursor.word(None), Err(FieldError::NoMatch));
        assert_eq!(cursor.offset(), 0);
    }

    #[test]
    fn spans_reject_both_interval_overruns_and_negative_or_large_indices() {
        assert_eq!(checked_span(255, 1, 256), Some(255..256));
        assert_eq!(checked_span(255, 2, 256), None);
        assert_eq!(checked_span(-1, 1, 256), None);
        assert_eq!(checked_span(256, 1, 256), None);
        assert_eq!(checked_span(256, 0, 256), Some(256..256));
        assert_eq!(checked_span(i64::MAX, i64::MAX, 256), None);
        assert_eq!(checked_span(0, -1, 256), None);
        assert_eq!(checked_inclusive_span(0, 1, 0, 256), None);
        assert_eq!(checked_inclusive_span(255, 1, 2, 256), None);
        assert_eq!(
            checked_inclusive_span(0, i64::MAX, i64::MAX, 256),
            Some(0..1)
        );
        assert_eq!(checked_inclusive_span(0, i64::MIN, i64::MAX, 256), None);
    }

    #[test]
    fn sequence_capacity_includes_the_terminator_and_failure_is_atomic() {
        let mut output = [0x7f; 256];
        let bytes = b"0041 ".repeat(65);
        assert_eq!(
            copy_without_spaces(&bytes, &mut output),
            Err(FieldError::Overflow)
        );
        assert_eq!(output, [0x7f; 256]);
        assert_eq!(copy_without_spaces(b"0041 0042 ", &mut output), Ok(8));
        assert_eq!(&output[..9], b"00410042\0");
        let mut exact = [0x7f; 256];
        assert_eq!(copy_without_spaces(&[b'A'; 255], &mut exact), Ok(255));
        assert_eq!(exact[255], 0);
        assert_eq!(copy_without_spaces(b"", &mut []), Err(FieldError::Overflow));
    }

    proptest! {
        #[test]
        fn arbitrary_integer_fields_keep_offsets_inside_the_input(
            bytes in prop::collection::vec(any::<u8>(), 0..512),
            base_index in 0u8..3,
        ) {
            let base = match base_index {
                0 => IntegerBase::Auto,
                1 => IntegerBase::Decimal,
                _ => IntegerBase::Hex,
            };
            let parsed = signed_prefix(&bytes, base);
            let mut cursor = Cursor::new(&bytes);
            let result = cursor.integer(base);
            match parsed.status {
                ParseStatus::InRange => {
                    prop_assert_eq!(result, Ok(parsed.value));
                    prop_assert_eq!(cursor.offset(), parsed.consumed);
                }
                ParseStatus::NoDigits => {
                    prop_assert_eq!(result, Err(FieldError::NoMatch));
                    prop_assert_eq!(cursor.offset(), 0);
                }
                ParseStatus::Overflow => {
                    prop_assert_eq!(result, Err(FieldError::Overflow));
                    prop_assert_eq!(cursor.offset(), 0);
                }
            }
            prop_assert!(cursor.offset() <= bytes.len());
        }

        #[test]
        fn representable_long_fields_preserve_values_and_consumption(
            value in any::<i64>(),
            leading in prop::collection::vec(prop::sample::select(b" \t\n\r\x0b\x0c".to_vec()), 0..16),
        ) {
            let mut bytes = leading;
            bytes.extend_from_slice(value.to_string().as_bytes());
            let end = bytes.len();
            bytes.extend_from_slice(b" _tail");
            let mut cursor = Cursor::new(&bytes);
            prop_assert_eq!(cursor.integer(IntegerBase::Decimal), Ok(value));
            prop_assert_eq!(cursor.offset(), end);
            cursor.skip_whitespace();
            prop_assert_eq!(cursor.peek(), Some(b'_'));
            prop_assert_eq!(cursor.offset(), end + 1);
        }

        #[test]
        fn four_digit_hex_fields_have_exact_width_and_value(
            value in any::<u16>(),
            leading in prop::collection::vec(prop::sample::select(b" \t\n\r\x0b\x0c".to_vec()), 0..16),
        ) {
            let start = leading.len();
            let mut bytes = leading;
            bytes.extend_from_slice(format!("{value:04X}F").as_bytes());
            let mut cursor = Cursor::new(&bytes);
            prop_assert_eq!(cursor.hex(Some(4)), Ok(u64::from(value)));
            prop_assert_eq!(cursor.offset(), start + 4);
            prop_assert_eq!(cursor.remaining(), b"F");
        }

        #[test]
        fn arbitrary_word_widths_never_copy_or_cross_the_input_boundary(
            bytes in prop::collection::vec(any::<u8>(), 0..512),
            width in 0usize..600,
        ) {
            let leading = bytes.iter().take_while(|&&byte| is_c_whitespace(byte)).count();
            let length = bytes[leading..].iter().take(width).take_while(|&&byte| !is_c_whitespace(byte)).count();
            let mut cursor = Cursor::new(&bytes);
            let result = cursor.word(Some(width));
            if length == 0 {
                prop_assert_eq!(result, Err(FieldError::NoMatch));
                prop_assert_eq!(cursor.offset(), 0);
            } else {
                prop_assert_eq!(result, Ok(&bytes[leading..leading + length]));
                prop_assert_eq!(cursor.offset(), leading + length);
            }
            prop_assert!(cursor.offset() <= bytes.len());
        }

        #[test]
        fn arbitrary_spans_are_wholly_contained_or_rejected(
            start in any::<i64>(),
            count in any::<i64>(),
            slots in 0usize..1024,
        ) {
            let expected = if start >= 0 && count >= 0 {
                let end = i128::from(start) + i128::from(count);
                (end <= slots as i128).then_some(start as usize..end as usize)
            } else {
                None
            };
            prop_assert_eq!(checked_span(start, count, slots), expected);
        }

        #[test]
        fn inclusive_ranges_never_reverse_overflow_or_exceed_capacity(
            start in any::<i64>(),
            first in any::<i64>(),
            last in any::<i64>(),
            slots in 0usize..1024,
        ) {
            let count = i128::from(last) - i128::from(first) + 1;
            let end = i128::from(start) + count;
            let expected = if start >= 0 && first <= last && end <= slots as i128 {
                Some(start as usize..end as usize)
            } else {
                None
            };
            prop_assert_eq!(checked_inclusive_span(start, first, last, slots), expected);
        }

        #[test]
        fn normalized_sequences_preserve_bytes_and_reject_before_writing(
            input in prop::collection::vec(prop::sample::select(b"0123456789ABCDEF ".to_vec()), 0..600),
            capacity in 0usize..300,
        ) {
            let expected = input.iter().copied().filter(|&byte| byte != b' ').collect::<Vec<_>>();
            let mut output = vec![0x7f; capacity];
            let result = copy_without_spaces(&input, &mut output);
            if expected.len() < capacity {
                prop_assert_eq!(result, Ok(expected.len()));
                prop_assert_eq!(&output[..expected.len()], expected.as_slice());
                prop_assert_eq!(output[expected.len()], 0);
                prop_assert!(output[expected.len() + 1..].iter().all(|&byte| byte == 0x7f));
            } else {
                prop_assert_eq!(result, Err(FieldError::Overflow));
                prop_assert!(output.iter().all(|&byte| byte == 0x7f));
            }
        }
    }
}
