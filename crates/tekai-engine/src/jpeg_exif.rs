//! Optional JPEG resolution metadata, parsed without following raw TIFF pointers.

const EXIF_SIGNATURE: &[u8] = b"Exif\0\0";
const IFD_ENTRY_BYTES: usize = 12;

#[derive(Clone, Copy)]
enum Endian {
    Little,
    Big,
}

impl Endian {
    fn u16(self, bytes: &[u8], offset: usize) -> Option<u16> {
        let bytes: [u8; 2] = bytes.get(offset..offset.checked_add(2)?)?.try_into().ok()?;
        Some(match self {
            Self::Little => u16::from_le_bytes(bytes),
            Self::Big => u16::from_be_bytes(bytes),
        })
    }

    fn u32(self, bytes: &[u8], offset: usize) -> Option<u32> {
        let bytes: [u8; 4] = bytes.get(offset..offset.checked_add(4)?)?.try_into().ok()?;
        Some(match self {
            Self::Little => u32::from_le_bytes(bytes),
            Self::Big => u32::from_be_bytes(bytes),
        })
    }
}

pub(crate) fn app1_payload_length(segment_length: u16) -> Option<usize> {
    segment_length.checked_sub(2).map(usize::from)
}

/// Return a complete resolution pair, or ignore invalid optional metadata.
pub(crate) fn parse_exif_resolution(payload: &[u8]) -> Option<(i32, i32)> {
    let after_signature = payload.strip_prefix(EXIF_SIGNATURE)?;
    // Retain the old reader's acceptance of zero padding before the TIFF header.
    let start = after_signature.iter().position(|byte| *byte != 0)?;
    let tiff = after_signature.get(start..)?;
    let header = tiff.get(..8)?;
    let endian = match header.get(..2)? {
        b"II" => Endian::Little,
        b"MM" => Endian::Big,
        _ => return None,
    };
    if endian.u16(header, 2)? != 42 {
        return None;
    }
    let ifd = usize::try_from(endian.u32(header, 4)?).ok()?;
    let fields = usize::from(endian.u16(tiff, ifd)?);
    let entries_start = ifd.checked_add(2)?;
    let entries_end = entries_start.checked_add(fields.checked_mul(IFD_ENTRY_BYTES)?)?;
    let entries = tiff.get(entries_start..entries_end)?;

    let mut xres = 72;
    let mut yres = 72;
    let mut res_unit = 1.0;
    for entry in entries.chunks_exact(IFD_ENTRY_BYTES) {
        let tag = endian.u16(entry, 0)?;
        let field_type = endian.u16(entry, 2)?;
        let count = endian.u32(entry, 4)?;
        match tag {
            282 | 283 => {
                if field_type != 5 || count != 1 {
                    return None;
                }
                let offset = usize::try_from(endian.u32(entry, 8)?).ok()?;
                let rational = tiff.get(offset..offset.checked_add(8)?)?;
                // Preserve the previous integer quotient and final unit rounding.
                // checked_div also rejects zero and the signed MIN / -1 trap.
                let numerator = endian.u32(rational, 0)? as i32;
                let denominator = endian.u32(rational, 4)? as i32;
                let value = numerator.checked_div(denominator)?;
                if tag == 282 {
                    xres = value;
                } else {
                    yres = value;
                }
            }
            296 => {
                if field_type != 3 || count != 1 {
                    return None;
                }
                match endian.u16(entry, 8)? {
                    2 => res_unit = 1.0,
                    3 => res_unit = 2.54,
                    _ => {}
                }
            }
            _ => {}
        }
    }
    Some((
        (f64::from(xres) * res_unit) as i32,
        (f64::from(yres) * res_unit) as i32,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn put_u16(bytes: &mut Vec<u8>, endian: Endian, value: u16) {
        bytes.extend_from_slice(&match endian {
            Endian::Little => value.to_le_bytes(),
            Endian::Big => value.to_be_bytes(),
        });
    }

    fn put_u32(bytes: &mut Vec<u8>, endian: Endian, value: u32) {
        bytes.extend_from_slice(&match endian {
            Endian::Little => value.to_le_bytes(),
            Endian::Big => value.to_be_bytes(),
        });
    }

    fn set_u16(bytes: &mut [u8], offset: usize, endian: Endian, value: u16) {
        bytes[offset..offset + 2].copy_from_slice(&match endian {
            Endian::Little => value.to_le_bytes(),
            Endian::Big => value.to_be_bytes(),
        });
    }

    fn set_u32(bytes: &mut [u8], offset: usize, endian: Endian, value: u32) {
        bytes[offset..offset + 4].copy_from_slice(&match endian {
            Endian::Little => value.to_le_bytes(),
            Endian::Big => value.to_be_bytes(),
        });
    }

    fn header(endian: Endian, ifd: u32) -> Vec<u8> {
        let mut bytes = EXIF_SIGNATURE.to_vec();
        bytes.extend_from_slice(match endian {
            Endian::Little => b"II",
            Endian::Big => b"MM",
        });
        put_u16(&mut bytes, endian, 42);
        put_u32(&mut bytes, endian, ifd);
        bytes
    }

    fn resolution_payload(endian: Endian, unit: u16) -> Vec<u8> {
        let mut bytes = header(endian, 8);
        put_u16(&mut bytes, endian, 3);
        for (tag, field_type, value) in [(282, 5, 50), (283, 5, 58), (296, 3, u32::from(unit))] {
            put_u16(&mut bytes, endian, tag);
            put_u16(&mut bytes, endian, field_type);
            put_u32(&mut bytes, endian, 1);
            if field_type == 3 {
                put_u16(&mut bytes, endian, unit);
                put_u16(&mut bytes, endian, 0);
            } else {
                put_u32(&mut bytes, endian, value);
            }
        }
        put_u32(&mut bytes, endian, 0);
        put_u32(&mut bytes, endian, 144);
        put_u32(&mut bytes, endian, 1);
        put_u32(&mut bytes, endian, 72);
        put_u32(&mut bytes, endian, 1);
        bytes
    }

    #[test]
    fn app1_extent_checks_subtraction_and_stays_within_u16() {
        assert_eq!(app1_payload_length(0), None);
        assert_eq!(app1_payload_length(1), None);
        assert_eq!(app1_payload_length(2), Some(0));
        assert_eq!(app1_payload_length(8), Some(6));
        assert_eq!(app1_payload_length(u16::MAX), Some(65_533));
    }

    #[test]
    fn exact_signature_only_jpeg_is_invalid_optional_metadata() {
        let jpeg = b"\xff\xd8\xff\xe1\x00\x08Exif\0\0";
        assert_eq!(jpeg.len(), 12);
        let extent = app1_payload_length(u16::from_be_bytes([jpeg[4], jpeg[5]])).unwrap();
        assert_eq!(extent, 6);
        assert_eq!(parse_exif_resolution(&jpeg[6..6 + extent]), None);
    }

    #[test]
    fn little_and_big_endian_controls_and_every_truncation() {
        for endian in [Endian::Little, Endian::Big] {
            let bytes = resolution_payload(endian, 2);
            assert_eq!(bytes.len(), 72);
            assert_eq!(parse_exif_resolution(&bytes), Some((144, 72)));
            for length in 0..bytes.len() {
                assert_eq!(
                    parse_exif_resolution(&bytes[..length]),
                    None,
                    "prefix {length}"
                );
            }
            let mut padded = EXIF_SIGNATURE.to_vec();
            padded.extend_from_slice(&[0; 7]);
            padded.extend_from_slice(&bytes[EXIF_SIGNATURE.len()..]);
            assert_eq!(parse_exif_resolution(&padded), Some((144, 72)));
        }
    }

    #[test]
    fn positive_integer_quotients_and_unit_conversion_keep_legacy_rounding() {
        for endian in [Endian::Little, Endian::Big] {
            let mut bytes = resolution_payload(endian, 3);
            assert_eq!(parse_exif_resolution(&bytes), Some((365, 182)));
            set_u32(&mut bytes, 56, endian, 145);
            set_u32(&mut bytes, 60, endian, 2);
            set_u32(&mut bytes, 64, endian, 73);
            set_u32(&mut bytes, 68, endian, 2);
            assert_eq!(parse_exif_resolution(&bytes), Some((182, 91)));
            set_u16(&mut bytes, 48, endian, 2);
            assert_eq!(parse_exif_resolution(&bytes), Some((72, 36)));
        }
    }

    #[test]
    fn invalid_signature_zero_padding_and_short_headers_are_ignored() {
        for payload in [
            b"".as_slice(),
            b"Exif\0\0",
            b"Exif\0\0\0\0",
            b"Exif\0\0M",
            b"Exif\0\0I",
        ] {
            assert_eq!(parse_exif_resolution(payload), None);
        }
        assert_eq!(parse_exif_resolution(&vec![0; 65_533]), None);
        let mut zero_padding = EXIF_SIGNATURE.to_vec();
        zero_padding.resize(65_533, 0);
        assert_eq!(parse_exif_resolution(&zero_padding), None);
        for endian in [Endian::Little, Endian::Big] {
            let mut bytes = header(endian, 8);
            assert_eq!(parse_exif_resolution(&bytes), None);
            bytes[5] = b'X';
            assert_eq!(parse_exif_resolution(&bytes), None);
            bytes[5] = 0;
            set_u16(&mut bytes, 8, endian, 43);
            assert_eq!(parse_exif_resolution(&bytes), None);
        }
    }

    #[test]
    fn ifd_and_entry_count_boundaries_are_checked_before_traversal() {
        for endian in [Endian::Little, Endian::Big] {
            for offset in [0, 8, 9, 10, 72, 0x7fff_ffff, u32::MAX] {
                let bytes = header(endian, offset);
                assert_eq!(parse_exif_resolution(&bytes), None, "offset {offset}");
            }
            let mut bytes = header(endian, 8);
            put_u16(&mut bytes, endian, 1);
            assert_eq!(parse_exif_resolution(&bytes), None);
            set_u16(&mut bytes, 14, endian, u16::MAX);
            assert_eq!(parse_exif_resolution(&bytes), None);
            for remaining in 0..IFD_ENTRY_BYTES {
                let mut truncated = header(endian, 8);
                put_u16(&mut truncated, endian, 1);
                truncated.extend(std::iter::repeat_n(0, remaining));
                assert_eq!(
                    parse_exif_resolution(&truncated),
                    None,
                    "entry bytes {remaining}"
                );
            }
        }
    }

    #[test]
    fn rational_offsets_and_zero_or_overflowing_division_are_ignored() {
        for endian in [Endian::Little, Endian::Big] {
            for offset in [66, 67, 0x7fff_ffff, u32::MAX] {
                let mut bytes = resolution_payload(endian, 2);
                set_u32(&mut bytes, 24, endian, offset);
                assert_eq!(parse_exif_resolution(&bytes), None);
            }
            for remaining in 0..8 {
                let mut bytes = resolution_payload(endian, 2);
                set_u32(&mut bytes, 24, endian, 66 - remaining);
                assert_eq!(
                    parse_exif_resolution(&bytes),
                    None,
                    "rational bytes {remaining}"
                );
            }
            for numerator_offset in [56, 64] {
                let mut bytes = resolution_payload(endian, 2);
                set_u32(&mut bytes, numerator_offset + 4, endian, 0);
                assert_eq!(parse_exif_resolution(&bytes), None);
                set_u32(&mut bytes, numerator_offset, endian, 0x8000_0000);
                set_u32(&mut bytes, numerator_offset + 4, endian, 0xffff_ffff);
                assert_eq!(parse_exif_resolution(&bytes), None);
            }
        }
    }

    #[test]
    fn known_tag_types_and_counts_cannot_reuse_previous_values() {
        for endian in [Endian::Little, Endian::Big] {
            for offset in [18, 30] {
                for field_type in [1, 2, 3, 4, 7, 9, 10, u16::MAX] {
                    let mut bytes = resolution_payload(endian, 2);
                    set_u16(&mut bytes, offset, endian, field_type);
                    assert_eq!(parse_exif_resolution(&bytes), None);
                }
            }
            for field_type in [1, 2, 4, 5, 7, 9, 10, u16::MAX] {
                let mut bytes = resolution_payload(endian, 2);
                set_u16(&mut bytes, 42, endian, field_type);
                assert_eq!(parse_exif_resolution(&bytes), None);
            }
            for offset in [20, 32, 44] {
                for count in [0, 2, u32::MAX] {
                    let mut bytes = resolution_payload(endian, 2);
                    set_u32(&mut bytes, offset, endian, count);
                    assert_eq!(parse_exif_resolution(&bytes), None);
                }
            }
        }
    }

    #[test]
    fn absent_resolution_tags_default_independently_and_unknown_tags_are_skipped() {
        for endian in [Endian::Little, Endian::Big] {
            let mut empty = header(endian, 8);
            put_u16(&mut empty, endian, 0);
            assert_eq!(parse_exif_resolution(&empty), Some((72, 72)));
            let mut bytes = resolution_payload(endian, 2);
            set_u16(&mut bytes, 16, endian, 999);
            set_u32(&mut bytes, 24, endian, u32::MAX);
            assert_eq!(parse_exif_resolution(&bytes), Some((72, 72)));
            set_u16(&mut bytes, 16, endian, 282);
            set_u32(&mut bytes, 24, endian, 50);
            set_u16(&mut bytes, 28, endian, 999);
            assert_eq!(parse_exif_resolution(&bytes), Some((144, 72)));
            set_u16(&mut bytes, 16, endian, 999);
            set_u16(&mut bytes, 28, endian, 283);
            set_u32(&mut bytes, 64, endian, 96);
            assert_eq!(parse_exif_resolution(&bytes), Some((72, 96)));
        }
    }

    #[test]
    fn checked_scalar_reads_reject_unrepresentable_ranges() {
        for endian in [Endian::Little, Endian::Big] {
            assert_eq!(endian.u16(&[0; 8], usize::MAX), None);
            assert_eq!(endian.u32(&[0; 8], usize::MAX - 2), None);
            assert_eq!(endian.u16(&[0; 8], 7), None);
            assert_eq!(endian.u32(&[0; 8], 5), None);
        }
    }
}
