//! Typed parsers for the backend's fixed integer and bounded-word grammars.
//!
//! Callers pass initialized bytes, without their trailing C terminator. These
//! parsers preserve the old assignment-count rules. Type1 encoding operations
//! and Type3 preambles do not require the literals after their final assignment.
//! Only the two numeric TrueType glyph-name forms require full consumption.

#![warn(clippy::all)]

use crate::backend_parse::{Cursor, FieldError};
use crate::integer_parse::{
    is_c_whitespace, signed_prefix, unsigned_prefix, IntegerBase, ParseStatus,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct EncodingAssignment<'a> {
    pub(crate) index: i32,
    pub(crate) name: &'a [u8],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct EncodingCopy {
    pub(crate) source: i32,
    pub(crate) target: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct EncodingInterval {
    pub(crate) source: i32,
    pub(crate) count: i32,
    pub(crate) target: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct GlyphPreamble {
    pub(crate) index: i32,
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) depth: i32,
    pub(crate) llx: i32,
    pub(crate) lly: i32,
    pub(crate) urx: i32,
    pub(crate) ury: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PlatformEncoding {
    pub(crate) platform: i16,
    pub(crate) encoding: i16,
    pub(crate) consumed: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum NumericGlyphName {
    Unicode(u32),
    Index(i32),
}

fn integer(cursor: &mut Cursor<'_>) -> Result<i32, FieldError> {
    i32::try_from(cursor.integer(IntegerBase::Auto)?).map_err(|_| FieldError::Overflow)
}

fn word<'a>(cursor: &mut Cursor<'a>) -> Result<&'a [u8], FieldError> {
    let bytes = cursor.word(Some(255))?;
    // A caller may pass a written Type1 span that contains a NUL before its end.
    // The old string scan stopped there, without inspecting unwritten capacity.
    let length = bytes
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(bytes.len());
    if length == 0 {
        Err(FieldError::NoMatch)
    } else {
        Ok(&bytes[..length])
    }
}

/// The former predefined-encoding `%255s` field. Width excludes leading space.
pub(crate) fn predefined_encoding(bytes: &[u8]) -> Result<&[u8], FieldError> {
    word(&mut Cursor::new(bytes))
}

/// The assignments in `dup %i%255s put`. The caller still finds ` put` before
/// advancing to the next operation, just as it did after the old scan returned 2.
pub(crate) fn encoding_assignment(bytes: &[u8]) -> Result<EncodingAssignment<'_>, FieldError> {
    let mut cursor = Cursor::new(bytes);
    cursor.literal(b"dup")?;
    let index = integer(&mut cursor)?;
    let name = word(&mut cursor)?;
    Ok(EncodingAssignment { index, name })
}

/// The assignments in `dup dup %i exch %i get put`. The first integer is the
/// target, and the second is the source. The final `get put` does not gate them.
pub(crate) fn encoding_copy(bytes: &[u8]) -> Result<EncodingCopy, FieldError> {
    let mut cursor = Cursor::new(bytes);
    cursor.literal(b"dup")?;
    cursor.skip_whitespace();
    cursor.literal(b"dup")?;
    let target = integer(&mut cursor)?;
    cursor.skip_whitespace();
    cursor.literal(b"exch")?;
    let source = integer(&mut cursor)?;
    Ok(EncodingCopy { source, target })
}

/// The assignments in `dup dup %i %i getinterval %i exch putinterval`. The
/// caller checks both array spans before copying and finds the advancing suffix.
pub(crate) fn encoding_interval(bytes: &[u8]) -> Result<EncodingInterval, FieldError> {
    let mut cursor = Cursor::new(bytes);
    cursor.literal(b"dup")?;
    cursor.skip_whitespace();
    cursor.literal(b"dup")?;
    let source = integer(&mut cursor)?;
    let count = integer(&mut cursor)?;
    cursor.skip_whitespace();
    cursor.literal(b"getinterval")?;
    let target = integer(&mut cursor)?;
    Ok(EncodingInterval {
        source,
        count,
        target,
    })
}

/// The former CharStrings `%i` presence probe. A suffix does not make a parsed
/// count absent, and representable negative values remain parsed values.
pub(crate) fn charstring_count(bytes: &[u8]) -> Result<i32, FieldError> {
    integer(&mut Cursor::new(bytes))
}

/// The eight `%i` assignments after `\pdfglyph` and its skipped delimiter byte.
/// The trailing `=` was never required by the caller's assignment-count check.
pub(crate) fn glyph_preamble(bytes: &[u8]) -> Result<GlyphPreamble, FieldError> {
    let mut cursor = Cursor::new(bytes);
    Ok(GlyphPreamble {
        index: integer(&mut cursor)?,
        width: integer(&mut cursor)?,
        height: integer(&mut cursor)?,
        depth: integer(&mut cursor)?,
        llx: integer(&mut cursor)?,
        lly: integer(&mut cursor)?,
        urx: integer(&mut cursor)?,
        ury: integer(&mut cursor)?,
    })
}

/// `PidEid=%i, %i %n` requires the comma immediately after the first field. The
/// reported offset includes trailing whitespace, without counting `%n` as a
/// third assignment. Reject narrowing instead of wrapping into unrelated IDs.
pub(crate) fn platform_encoding(bytes: &[u8]) -> Result<PlatformEncoding, FieldError> {
    let mut cursor = Cursor::new(bytes);
    cursor.literal(b"PidEid=")?;
    let platform = integer(&mut cursor)?;
    cursor.literal(b",")?;
    let encoding = integer(&mut cursor)?;
    cursor.skip_whitespace();
    Ok(PlatformEncoding {
        platform: i16::try_from(platform).map_err(|_| FieldError::Overflow)?,
        encoding: i16::try_from(encoding).map_err(|_| FieldError::Overflow)?,
        consumed: cursor.offset(),
    })
}

fn full_hex(bytes: &[u8]) -> Result<Option<u32>, FieldError> {
    let parsed = unsigned_prefix(bytes, IntegerBase::Hex);
    if parsed.status == ParseStatus::NoDigits || parsed.consumed != bytes.len() {
        return Ok(None);
    }
    if parsed.status == ParseStatus::Overflow {
        return Err(FieldError::Overflow);
    }
    let negative = bytes.iter().copied().find(|&byte| !is_c_whitespace(byte)) == Some(b'-');
    // `%X` accepts a sign and converts to unsigned int. Check the magnitude at
    // that width, then negate there, so `uni-FFFFFFFF` keeps its old value 1.
    let magnitude = if negative {
        parsed.value.wrapping_neg()
    } else {
        parsed.value
    };
    let magnitude = u32::try_from(magnitude).map_err(|_| FieldError::Overflow)?;
    Ok(Some(if negative {
        magnitude.wrapping_neg()
    } else {
        magnitude
    }))
}

fn full_index(bytes: &[u8]) -> Result<Option<i32>, FieldError> {
    let parsed = signed_prefix(bytes, IntegerBase::Auto);
    if parsed.status == ParseStatus::NoDigits || parsed.consumed != bytes.len() {
        return Ok(None);
    }
    if parsed.status == ParseStatus::Overflow {
        return Err(FieldError::Overflow);
    }
    i32::try_from(parsed.value)
        .map(Some)
        .map_err(|_| FieldError::Overflow)
}

/// Recognize the two numeric names only after the caller has checked real post
/// names. Incomplete prefixes or trailing bytes remain ordinary missing names.
pub(crate) fn numeric_glyph_name(bytes: &[u8]) -> Result<Option<NumericGlyphName>, FieldError> {
    let mut cursor = Cursor::new(bytes);
    if cursor.literal(b"uni").is_ok() {
        full_hex(cursor.remaining()).map(|value| value.map(NumericGlyphName::Unicode))
    } else if cursor.literal(b"index").is_ok() {
        full_index(cursor.remaining()).map(|value| value.map(NumericGlyphName::Index))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn terminated(bytes: &[u8]) -> Vec<u8> {
        assert!(!bytes.contains(&0));
        let mut result = bytes.to_vec();
        result.push(0);
        result
    }

    fn spelling(value: i32, base: u8) -> String {
        let sign = if value < 0 { "-" } else { "" };
        match base {
            0 => value.to_string(),
            1 => format!("{sign}0{:o}", value.unsigned_abs()),
            _ => format!("{sign}0x{:X}", value.unsigned_abs()),
        }
    }

    fn native_assignment(bytes: &[u8]) -> Option<(i32, Vec<u8>)> {
        let input = terminated(bytes);
        let mut index = 0i32;
        let mut name = [0u8; 256];
        // Input and format terminate, and the word field fits its 256-byte array.
        let assigned = unsafe {
            libc::sscanf(
                input.as_ptr().cast(),
                c"dup %i%255s put".as_ptr(),
                &raw mut index,
                name.as_mut_ptr(),
            )
        };
        if assigned == 2 {
            let end = name.iter().position(|&byte| byte == 0).unwrap();
            Some((index, name[..end].to_vec()))
        } else {
            None
        }
    }

    fn native_copy(bytes: &[u8]) -> Option<EncodingCopy> {
        let input = terminated(bytes);
        let mut target = 0i32;
        let mut source = 0i32;
        // Both integer targets have the exact type required by their fields.
        let assigned = unsafe {
            libc::sscanf(
                input.as_ptr().cast(),
                c"dup dup %i exch %i get put".as_ptr(),
                &raw mut target,
                &raw mut source,
            )
        };
        (assigned == 2).then_some(EncodingCopy { source, target })
    }

    fn native_interval(bytes: &[u8]) -> Option<EncodingInterval> {
        let input = terminated(bytes);
        let mut source = 0i32;
        let mut count = 0i32;
        let mut target = 0i32;
        // All three integer targets have the exact type required by their fields.
        let assigned = unsafe {
            libc::sscanf(
                input.as_ptr().cast(),
                c"dup dup %i %i getinterval %i exch putinterval".as_ptr(),
                &raw mut source,
                &raw mut count,
                &raw mut target,
            )
        };
        (assigned == 3).then_some(EncodingInterval {
            source,
            count,
            target,
        })
    }

    #[test]
    fn encoding_operations_keep_assignment_order_and_ignored_final_literals() {
        for bytes in [
            b"dup 010/A put".as_slice(),
            b"dup-0x10/A missing",
            b"dup\t+2\x0b/A",
            b"dupdup 1 exch 2 get put",
            b"dup dup 010exch0x20 ignored",
            b"dup dup 1 2 getinterval3 anything",
        ] {
            assert_eq!(
                encoding_assignment(bytes)
                    .ok()
                    .map(|parsed| (parsed.index, parsed.name.to_vec())),
                native_assignment(bytes)
            );
            assert_eq!(encoding_copy(bytes).ok(), native_copy(bytes));
            assert_eq!(encoding_interval(bytes).ok(), native_interval(bytes));
        }
        assert_eq!(
            encoding_assignment(b"dup 1/A"),
            Ok(EncodingAssignment {
                index: 1,
                name: b"/A",
            })
        );
        assert_eq!(
            encoding_copy(b"dup dup 1 exch 2"),
            Ok(EncodingCopy {
                source: 2,
                target: 1,
            })
        );
        assert_eq!(
            encoding_interval(b"dup dup 1 2 getinterval 3"),
            Ok(EncodingInterval {
                source: 1,
                count: 2,
                target: 3,
            })
        );
    }

    #[test]
    fn bounded_words_keep_whitespace_width_and_written_nul_semantics() {
        assert_eq!(
            predefined_encoding(b" \tStandardEncoding def"),
            Ok(b"StandardEncoding".as_slice())
        );
        assert_eq!(predefined_encoding(b" \t\n"), Err(FieldError::NoMatch));
        assert_eq!(
            predefined_encoding(b"\0unwritten"),
            Err(FieldError::NoMatch)
        );
        assert_eq!(
            predefined_encoding(b"StandardEncoding\0unwritten"),
            Ok(b"StandardEncoding".as_slice())
        );
        let long = [b'A'; 300];
        assert_eq!(predefined_encoding(&long), Ok(&long[..255]));
        let mut assignment = b"dup 7/".to_vec();
        assignment.extend_from_slice(&long);
        assert_eq!(encoding_assignment(&assignment).unwrap().name.len(), 255);
    }

    #[test]
    fn platform_ids_keep_comma_and_whitespace_offset_without_narrowing() {
        assert_eq!(
            platform_encoding(b"PidEid=03, 0x10 \t\nnext"),
            Ok(PlatformEncoding {
                platform: 3,
                encoding: 16,
                consumed: 18,
            })
        );
        assert_eq!(platform_encoding(b"PidEid=3 ,1"), Err(FieldError::NoMatch));
        assert_eq!(platform_encoding(b" PidEid=3,1"), Err(FieldError::NoMatch));
        assert_eq!(
            platform_encoding(b"PidEid=32768,1"),
            Err(FieldError::Overflow)
        );
        assert_eq!(
            platform_encoding(b"PidEid=3,-32769"),
            Err(FieldError::Overflow)
        );
        assert_eq!(platform_encoding(b"PidEid=3,"), Err(FieldError::NoMatch));
    }

    #[test]
    fn type3_preamble_has_eight_base_zero_fields_and_no_required_equals() {
        let expected = GlyphPreamble {
            index: 8,
            width: 16,
            height: -3,
            depth: 0,
            llx: -1,
            lly: -2,
            urx: 3,
            ury: 4,
        };
        assert_eq!(glyph_preamble(b"010 0x10 -3 0 -1 -2 3 4"), Ok(expected));
        assert_eq!(
            glyph_preamble(b"010 0x10 -3 0 -1 -2 3 4 anything"),
            Ok(expected)
        );
        assert_eq!(glyph_preamble(b"1 2 3 4 5 6 7"), Err(FieldError::NoMatch));
        assert_eq!(
            glyph_preamble(b"1 2 3 4 5 6 7 2147483648"),
            Err(FieldError::Overflow)
        );
    }

    #[test]
    fn charstring_probe_preserves_prefixes_and_checks_integer_width() {
        assert_eq!(charstring_count(b" \t010 dict"), Ok(8));
        assert_eq!(charstring_count(b"-2 tail"), Ok(-2));
        assert_eq!(charstring_count(b"dict"), Err(FieldError::NoMatch));
        assert_eq!(
            charstring_count(b"2147483648 dict"),
            Err(FieldError::Overflow)
        );
        assert_eq!(charstring_count(b"-2147483649"), Err(FieldError::Overflow));
        assert_eq!(
            charstring_count(b"9223372036854775808"),
            Err(FieldError::Overflow)
        );
        // Darwin's former scanner capped numeric input at 512 field bytes.
        // The Rust parser consumes the entire initialized numeric prefix, so
        // leading zeros cannot hide the value from a later literal or field.
        let mut long_count = vec![b'0'; 1024];
        long_count.extend_from_slice(b"7 dict");
        assert_eq!(charstring_count(&long_count), Ok(7));
        let mut long_copy = b"dup dup ".to_vec();
        long_copy.extend_from_slice(&[b'0'; 1024]);
        long_copy.extend_from_slice(b"7 exch 2 get put");
        assert_eq!(
            encoding_copy(&long_copy),
            Ok(EncodingCopy {
                source: 2,
                target: 7,
            })
        );
    }

    #[test]
    fn numeric_glyph_names_require_full_consumption_and_preserve_signed_hex() {
        for (name, expected) in [
            (b"uni0041".as_slice(), NumericGlyphName::Unicode(0x41)),
            (b"uni 0x41", NumericGlyphName::Unicode(0x41)),
            (b"uni-FFFFFFFF", NumericGlyphName::Unicode(1)),
            (b"uni-1", NumericGlyphName::Unicode(u32::MAX)),
            (b"index010", NumericGlyphName::Index(8)),
            (b"index-0x10", NumericGlyphName::Index(-16)),
        ] {
            assert_eq!(numeric_glyph_name(name), Ok(Some(expected)));
        }
        for bytes in [
            b"uni".as_slice(),
            b"uni0x",
            b"uni0041 ",
            b"uni0041tail",
            b"index",
            b"index09",
            b"index1tail",
            b"ordinary",
        ] {
            assert_eq!(numeric_glyph_name(bytes), Ok(None));
        }
        for bytes in [
            b"uni100000000".as_slice(),
            b"uni-100000000",
            b"uni10000000000000000",
            b"index2147483648",
            b"index-2147483649",
            b"index9223372036854775808",
        ] {
            assert_eq!(numeric_glyph_name(bytes), Err(FieldError::Overflow));
        }
        assert_eq!(
            numeric_glyph_name(b"index9223372036854775808tail"),
            Ok(None)
        );
    }

    proptest! {
        #[test]
        fn valid_encoding_operations_match_native_assignments(
            source in any::<i32>(),
            target in any::<i32>(),
            count in any::<i32>(),
            base in 0u8..3,
            whitespace in prop::sample::select(vec!["", " ", "\t", "\x0b", "\r\n"]),
            suffix in prop::sample::select(vec!["", " missing", " get put", " exch putinterval"]),
        ) {
            let source = spelling(source, base);
            let target = spelling(target, base);
            let count = spelling(count, base);
            let copy = format!("dup{whitespace}dup{whitespace}{target}{whitespace}exch{whitespace}{source}{suffix}");
            prop_assert_eq!(encoding_copy(copy.as_bytes()).ok(), native_copy(copy.as_bytes()));
            // Adjacent signed integer fields are legal with zero separating
            // whitespace, but two adjacent unsigned spellings form one field.
            let interval = format!("dup{whitespace}dup {source} {count}{whitespace}getinterval{whitespace}{target}{suffix}");
            prop_assert_eq!(encoding_interval(interval.as_bytes()).ok(), native_interval(interval.as_bytes()));
        }

        #[test]
        fn bounded_encoding_assignments_match_native(
            index in any::<i32>(),
            name in prop::collection::vec(prop::sample::select(b"/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_-".to_vec()), 1..400),
            whitespace in prop::sample::select(vec!["", " ", "\t", "\x0b", "\r\n"]),
        ) {
            let mut bytes = format!("dup{whitespace}{index}{whitespace}").into_bytes();
            bytes.push(b'/');
            bytes.extend_from_slice(&name);
            bytes.extend_from_slice(b" missing");
            let input = terminated(&bytes);
            let mut native_index = 0i32;
            let mut native_name = [0u8; 256];
            // Both targets match their fields, and the word has a bounded array.
            let assigned = unsafe {
                libc::sscanf(
                    input.as_ptr().cast(),
                    c"dup %i%255s put".as_ptr(),
                    &raw mut native_index,
                    native_name.as_mut_ptr(),
                )
            };
            let parsed = encoding_assignment(&bytes);
            if assigned == 2 {
                let end = native_name.iter().position(|&byte| byte == 0).unwrap();
                let parsed = parsed.unwrap();
                prop_assert_eq!(parsed.index, native_index);
                prop_assert_eq!(parsed.name, &native_name[..end]);
            } else {
                prop_assert_eq!(parsed, Err(FieldError::NoMatch));
            }
        }

        #[test]
        fn platform_ids_match_native_values_and_exact_consumption(
            platform in any::<i16>(),
            encoding in any::<i16>(),
            whitespace in prop::sample::select(vec!["", " ", "\t", "\x0b", "\r\n"]),
        ) {
            let bytes = format!("PidEid={platform},{whitespace}{encoding}{whitespace}<font.ttf");
            let input = terminated(bytes.as_bytes());
            let mut native_platform = 0i32;
            let mut native_encoding = 0i32;
            let mut native_consumed = -1i32;
            // `%n` writes an int offset but does not increase assigned fields.
            let assigned = unsafe {
                libc::sscanf(
                    input.as_ptr().cast(),
                    c"PidEid=%i, %i %n".as_ptr(),
                    &raw mut native_platform,
                    &raw mut native_encoding,
                    &raw mut native_consumed,
                )
            };
            prop_assert_eq!(assigned, 2);
            prop_assert_eq!(platform_encoding(bytes.as_bytes()), Ok(PlatformEncoding {
                platform,
                encoding,
                consumed: native_consumed as usize,
            }));
        }

        #[test]
        fn numeric_glyph_names_match_native_defined_integer_widths(
            unicode in any::<u32>(),
            index in any::<i32>(),
            whitespace in prop::sample::select(vec!["", " ", "\t", "\x0b", "\r\n"]),
        ) {
            let unicode_name = format!("uni{whitespace}{unicode:X}");
            let index_name = format!("index{whitespace}{index}");
            let unicode_input = terminated(unicode_name.as_bytes());
            let index_input = terminated(index_name.as_bytes());
            let mut native_unicode = 0u32;
            let mut native_index = 0i32;
            let mut unicode_consumed = -1i32;
            let mut index_consumed = -1i32;
            // The unsigned and signed targets match `%X` and `%i`, respectively.
            unsafe {
                prop_assert_eq!(libc::sscanf(unicode_input.as_ptr().cast(), c"uni%X%n".as_ptr(), &raw mut native_unicode, &raw mut unicode_consumed), 1);
                prop_assert_eq!(libc::sscanf(index_input.as_ptr().cast(), c"index%i%n".as_ptr(), &raw mut native_index, &raw mut index_consumed), 1);
            }
            prop_assert_eq!(unicode_consumed as usize, unicode_name.len());
            prop_assert_eq!(index_consumed as usize, index_name.len());
            prop_assert_eq!(numeric_glyph_name(unicode_name.as_bytes()), Ok(Some(NumericGlyphName::Unicode(native_unicode))));
            prop_assert_eq!(numeric_glyph_name(index_name.as_bytes()), Ok(Some(NumericGlyphName::Index(native_index))));
        }

        #[test]
        fn type3_preambles_match_native_fields_even_without_trailing_equals(
            fields in any::<[i32; 8]>(),
            base in 0u8..3,
            whitespace in prop::sample::select(vec![" ", "\t", "\x0b", "\r\n"]),
            suffix in prop::sample::select(vec!["", " =", " ignored"]),
        ) {
            let bytes = fields.map(|field| spelling(field, base)).join(whitespace) + suffix;
            let input = terminated(bytes.as_bytes());
            let mut native = [0i32; 8];
            let targets = native.as_mut_ptr();
            // Each target is a distinct initialized i32 in the eight-slot array.
            let assigned = unsafe {
                libc::sscanf(
                    input.as_ptr().cast(),
                    c"%i %i %i %i %i %i %i %i =".as_ptr(),
                    targets.add(0),
                    targets.add(1),
                    targets.add(2),
                    targets.add(3),
                    targets.add(4),
                    targets.add(5),
                    targets.add(6),
                    targets.add(7),
                )
            };
            prop_assert_eq!(assigned, 8);
            prop_assert_eq!(glyph_preamble(bytes.as_bytes()), Ok(GlyphPreamble {
                index: native[0],
                width: native[1],
                height: native[2],
                depth: native[3],
                llx: native[4],
                lly: native[5],
                urx: native[6],
                ury: native[7],
            }));
        }

        #[test]
        fn arbitrary_initialized_slices_never_panic_or_escape_word_and_offset_bounds(
            bytes in prop::collection::vec(any::<u8>(), 0..4096),
        ) {
            if let Ok(word) = predefined_encoding(&bytes) {
                prop_assert!(word.len() <= 255);
                prop_assert!(!word.is_empty());
                prop_assert!(!word.contains(&0));
                prop_assert!(word.as_ptr() >= bytes.as_ptr());
                prop_assert!(word.as_ptr().wrapping_add(word.len()) <= bytes.as_ptr().wrapping_add(bytes.len()));
            }
            if let Ok(assignment) = encoding_assignment(&bytes) {
                prop_assert!(assignment.name.len() <= 255);
                prop_assert!(!assignment.name.is_empty());
            }
            if let Ok(parsed) = platform_encoding(&bytes) {
                prop_assert!(parsed.consumed <= bytes.len());
            }
            let _ = encoding_copy(&bytes);
            let _ = encoding_interval(&bytes);
            let _ = charstring_count(&bytes);
            let _ = glyph_preamble(&bytes);
            let _ = numeric_glyph_name(&bytes);
        }
    }
}
