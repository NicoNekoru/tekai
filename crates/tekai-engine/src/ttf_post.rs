//! Checked ownership for TrueType `post` names.
//!
//! Pascal strings are scanned once and only referenced strings are retained.
//! Glyph lookup then uses the original unsigned 16-bit name index directly.
//! https://learn.microsoft.com/en-us/typography/opentype/spec/post

#![warn(clippy::all)]

use std::collections::{BTreeMap, BTreeSet};
use std::ffi::{CStr, CString};
use std::io::Read;

pub(crate) const STANDARD_NAMES: usize = 258;
const CUSTOM_CAPACITY: usize = u16::MAX as usize + 1 - STANDARD_NAMES;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PostError {
    TableExtent,
    Truncated,
    Read,
    GlyphCount,
    NameIndex,
    InteriorNul,
    Allocation,
    OutputCount,
    NameLength,
}

impl PostError {
    pub(crate) fn message(self) -> &'static CStr {
        match self {
            Self::TableExtent => c"post: table extent exceeds font file",
            Self::Truncated => c"post: truncated glyph names",
            Self::Read => c"post: cannot read glyph names",
            Self::GlyphCount => c"post: glyph name count exceeds font glyph count",
            Self::NameIndex => c"post: glyph name index has no Pascal string",
            Self::InteriorNul => c"post: glyph name contains an embedded NUL",
            Self::Allocation => c"post: cannot allocate glyph names",
            Self::OutputCount => c"post: output glyph count exceeds unsigned 16-bit capacity",
            Self::NameLength => c"post: output glyph name exceeds Pascal-string capacity",
        }
    }
}

/// Validate the directory's extent before a table-sized allocation or read.
pub(crate) fn checked_table_extent(
    offset: u32,
    length: u32,
    file_length: u64,
) -> Result<std::ops::Range<u64>, PostError> {
    let start = u64::from(offset);
    let end = start + u64::from(length);
    if length < 32 || end > file_length {
        return Err(PostError::TableExtent);
    }
    Ok(start..end)
}

struct TableReader<R> {
    reader: R,
    remaining: u64,
}

impl<R: Read> TableReader<R> {
    fn exact(&mut self, bytes: &mut [u8]) -> Result<(), PostError> {
        if bytes.len() as u64 > self.remaining {
            return Err(PostError::Truncated);
        }
        self.reader.read_exact(bytes).map_err(|error| {
            if error.kind() == std::io::ErrorKind::UnexpectedEof {
                PostError::Truncated
            } else {
                PostError::Read
            }
        })?;
        self.remaining -= bytes.len() as u64;
        Ok(())
    }

    fn word(&mut self) -> Result<u16, PostError> {
        let mut bytes = [0; 2];
        self.exact(&mut bytes)?;
        Ok(u16::from_be_bytes(bytes))
    }
}

pub(crate) struct PostNames {
    indices: Vec<u16>,
    custom: Vec<Option<CString>>,
}

impl PostNames {
    pub(crate) fn indices(&self) -> &[u16] {
        &self.indices
    }

    pub(crate) fn custom(&self, index: u16) -> Option<&CStr> {
        self.custom
            .get(usize::from(index).checked_sub(STANDARD_NAMES)?)?
            .as_deref()
    }
}

/// Read the format-2 body after its 32-byte header. Extents and minimum string
/// cardinality are checked before reserving storage. Names beyond the largest
/// referenced index cannot affect any glyph and are neither read nor retained.
pub(crate) fn read_format2(
    reader: impl Read,
    body_length: u64,
    glyph_count: u16,
) -> Result<PostNames, PostError> {
    let mut reader = TableReader {
        reader,
        remaining: body_length,
    };
    let count = usize::from(reader.word()?);
    if count > usize::from(glyph_count) {
        return Err(PostError::GlyphCount);
    }
    if count as u64 * 2 > reader.remaining {
        return Err(PostError::Truncated);
    }
    let mut indices = Vec::new();
    indices
        .try_reserve_exact(count)
        .map_err(|_| PostError::Allocation)?;
    for _ in 0..count {
        indices.push(reader.word()?);
    }
    let custom_count = indices.iter().copied().max().map_or(0, |index| {
        (usize::from(index) + 1).saturating_sub(STANDARD_NAMES)
    });
    if custom_count as u64 > reader.remaining {
        return Err(PostError::NameIndex);
    }
    let mut needed = Vec::new();
    needed
        .try_reserve_exact(custom_count)
        .map_err(|_| PostError::Allocation)?;
    needed.resize(custom_count, false);
    for &index in &indices {
        if let Some(index) = usize::from(index).checked_sub(STANDARD_NAMES) {
            needed[index] = true;
        }
    }
    let mut custom = Vec::new();
    custom
        .try_reserve_exact(custom_count)
        .map_err(|_| PostError::Allocation)?;
    let mut scratch = [0; u8::MAX as usize];
    for retain in needed {
        let mut length = [0];
        reader.exact(&mut length)?;
        let name = &mut scratch[..usize::from(length[0])];
        reader.exact(name)?;
        let owned = if retain {
            if name.contains(&0) {
                return Err(PostError::InteriorNul);
            }
            let mut bytes = Vec::new();
            bytes
                .try_reserve_exact(name.len() + 1)
                .map_err(|_| PostError::Allocation)?;
            bytes.extend_from_slice(name);
            Some(CString::new(bytes).map_err(|_| PostError::InteriorNul)?)
        } else {
            None
        };
        custom.push(owned);
    }
    Ok(PostNames { indices, custom })
}

#[derive(Clone, Copy)]
pub(crate) struct OutputGlyphName<'a> {
    pub(crate) index: u16,
    pub(crate) name: &'a CStr,
    pub(crate) ambiguous: bool,
}

pub(crate) struct OutputNames<'a> {
    pub(crate) indices: Vec<u16>,
    pub(crate) custom: Vec<&'a CStr>,
}

/// Plan format-2 names without changing source metadata or copying name bytes.
/// Ordinary output keeps the legacy per-glyph custom-name ordering exactly.
/// When that would exceed format-2 capacity, ordered maps deduplicate names
/// with collision-independent O(glyphs * log(glyphs)) work.
pub(crate) fn output_names<'a>(
    glyphs: &[OutputGlyphName<'a>],
) -> Result<OutputNames<'a>, PostError> {
    if glyphs.len() > usize::from(u16::MAX) {
        return Err(PostError::OutputCount);
    }
    let custom_count = glyphs
        .iter()
        .filter(|glyph| usize::from(glyph.index) >= STANDARD_NAMES || glyph.ambiguous)
        .count();
    if custom_count <= CUSTOM_CAPACITY {
        let mut output = OutputNames {
            indices: Vec::new(),
            custom: Vec::new(),
        };
        output
            .indices
            .try_reserve_exact(glyphs.len())
            .map_err(|_| PostError::Allocation)?;
        output
            .custom
            .try_reserve_exact(custom_count)
            .map_err(|_| PostError::Allocation)?;
        for glyph in glyphs {
            if glyph.name.to_bytes().len() > usize::from(u8::MAX) {
                return Err(PostError::NameLength);
            }
            let index = if usize::from(glyph.index) >= STANDARD_NAMES || glyph.ambiguous {
                let index = u16::try_from(STANDARD_NAMES + output.custom.len())
                    .map_err(|_| PostError::NameIndex)?;
                output.custom.push(glyph.name);
                index
            } else {
                glyph.index
            };
            output.indices.push(index);
        }
        return Ok(output);
    }
    let required: BTreeSet<&[u8]> = glyphs
        .iter()
        .filter(|glyph| usize::from(glyph.index) >= STANDARD_NAMES)
        .map(|glyph| glyph.name.to_bytes())
        .collect();
    let mut ambiguity_budget = CUSTOM_CAPACITY
        .checked_sub(required.len())
        .ok_or(PostError::NameIndex)?;
    let mut assigned = BTreeMap::new();
    let mut output = OutputNames {
        indices: Vec::new(),
        custom: Vec::new(),
    };
    output
        .indices
        .try_reserve_exact(glyphs.len())
        .map_err(|_| PostError::Allocation)?;
    for glyph in glyphs {
        let bytes = glyph.name.to_bytes();
        if bytes.len() > usize::from(u8::MAX) {
            return Err(PostError::NameLength);
        }
        let mandatory = usize::from(glyph.index) >= STANDARD_NAMES;
        let index = if mandatory || glyph.ambiguous {
            if let Some(&index) = assigned.get(bytes) {
                index
            } else if !mandatory && !required.contains(bytes) && ambiguity_budget == 0 {
                // A full valid format-2 source can already use every custom
                // slot. Its standard ambiguous index denotes the identical
                // name, so keep that index instead of overflowing or dropping
                // all PostScript names.
                glyph.index
            } else {
                if !mandatory && !required.contains(bytes) {
                    ambiguity_budget -= 1;
                }
                let index = u16::try_from(STANDARD_NAMES + output.custom.len())
                    .map_err(|_| PostError::NameIndex)?;
                output
                    .custom
                    .try_reserve(1)
                    .map_err(|_| PostError::Allocation)?;
                output.custom.push(glyph.name);
                assigned.insert(bytes, index);
                index
            }
        } else {
            glyph.index
        };
        output.indices.push(index);
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn body(indices: &[u16], names: &[Vec<u8>]) -> Vec<u8> {
        let mut bytes = (indices.len() as u16).to_be_bytes().to_vec();
        for &index in indices {
            bytes.extend_from_slice(&index.to_be_bytes());
        }
        for name in names {
            bytes.push(name.len() as u8);
            bytes.extend_from_slice(name);
        }
        bytes
    }

    #[test]
    fn extent_and_pascal_boundaries_fail_before_unchecked_access() {
        assert_eq!(
            checked_table_extent(20, 31, 100),
            Err(PostError::TableExtent)
        );
        assert_eq!(
            checked_table_extent(u32::MAX, 32, u32::MAX as u64),
            Err(PostError::TableExtent)
        );
        assert_eq!(
            checked_table_extent(u32::MAX, 32, u32::MAX as u64 + 32),
            Ok(u32::MAX as u64..u32::MAX as u64 + 32)
        );
        for (bytes, length, expected) in [
            (vec![0], 1, PostError::Truncated),
            (vec![0, 2], 2, PostError::GlyphCount),
            (vec![0, 1, 1], 3, PostError::Truncated),
            (vec![0, 1, 1, 2], 4, PostError::NameIndex),
            (vec![0, 1, 1, 2, 3, b'A'], 6, PostError::Truncated),
            (vec![0, 1, 1, 2, 2, b'A', 0], 7, PostError::InteriorNul),
        ] {
            assert!(
                matches!(read_format2(bytes.as_slice(), length, 1), Err(error) if error == expected)
            );
        }
        let bytes = body(&[258, 258], &[vec![b'a'; 255]]);
        let names = read_format2(bytes.as_slice(), bytes.len() as u64, 2).unwrap();
        assert_eq!(names.custom(258).unwrap().to_bytes().len(), 255);
        assert_eq!(names.custom.iter().filter(|name| name.is_some()).count(), 1);
    }

    #[test]
    fn repeated_last_name_reads_one_prefix_and_retains_one_string() {
        struct CountReads<'a> {
            bytes: &'a [u8],
            consumed: &'a mut usize,
            calls: &'a mut usize,
        }
        impl Read for CountReads<'_> {
            fn read(&mut self, bytes: &mut [u8]) -> std::io::Result<usize> {
                *self.calls += 1;
                let count = bytes.len().min(self.bytes.len());
                bytes[..count].copy_from_slice(&self.bytes[..count]);
                self.bytes = &self.bytes[count..];
                *self.consumed += count;
                Ok(count)
            }
        }
        let count = 4096;
        let indices = vec![(STANDARD_NAMES + count - 1) as u16; count];
        let bytes = body(&indices, &vec![b"last".to_vec(); count]);
        let mut consumed = 0;
        let mut calls = 0;
        let names = read_format2(
            CountReads {
                bytes: &bytes,
                consumed: &mut consumed,
                calls: &mut calls,
            },
            bytes.len() as u64,
            count as u16,
        )
        .unwrap();
        assert_eq!(consumed, bytes.len());
        assert_eq!(calls, 1 + count * 3);
        assert_eq!(names.custom.iter().filter(|name| name.is_some()).count(), 1);
        for &index in names.indices() {
            assert_eq!(names.custom(index).unwrap().to_bytes(), b"last");
        }
    }

    #[test]
    fn output_custom_indices_cross_byte_boundary_and_preserve_ordinary_order() {
        let names: Vec<_> = (0..300)
            .map(|index| CString::new(format!("glyph{index}")).unwrap())
            .collect();
        let mut glyphs: Vec<_> = names
            .iter()
            .map(|name| OutputGlyphName {
                index: 258,
                name,
                ambiguous: false,
            })
            .collect();
        glyphs.push(glyphs[0]);
        let output = output_names(&glyphs).unwrap();
        assert_eq!(output.custom.len(), 301);
        assert_eq!(output.indices[255], 513);
        assert_eq!(output.indices[256], 514);
        assert_eq!(output.indices[299], 557);
        assert_eq!(output.indices[300], 558);
        assert_eq!(output.custom[0], output.custom[300]);
    }

    #[test]
    fn full_unsigned_custom_range_preserves_standard_ambiguous_names() {
        let names: Vec<_> = (0..CUSTOM_CAPACITY)
            .map(|index| CString::new(format!("g{index}")).unwrap())
            .collect();
        let mut glyphs = vec![OutputGlyphName {
            index: 257,
            name: c"dmacron",
            ambiguous: true,
        }];
        glyphs.extend(names.iter().map(|name| OutputGlyphName {
            index: 258,
            name,
            ambiguous: false,
        }));
        let output = output_names(&glyphs).unwrap();
        assert_eq!(output.indices[0], 257);
        assert_eq!(output.indices.last(), Some(&u16::MAX));
        assert_eq!(output.custom.len(), CUSTOM_CAPACITY);
    }

    proptest! {
        #[test]
        fn indexed_pascal_names_match_source_without_repeated_string_walks(
            names in prop::collection::vec(prop::collection::vec(1_u8..=255, 0..=255), 1..32),
            picks in prop::collection::vec(any::<u16>(), 0..128),
        ) {
            let indices: Vec<_> = picks.iter().map(|&pick| {
                if pick % 3 == 0 { pick % STANDARD_NAMES as u16 }
                else { STANDARD_NAMES as u16 + pick % names.len() as u16 }
            }).collect();
            let bytes = body(&indices, &names);
            let parsed = read_format2(bytes.as_slice(), bytes.len() as u64, indices.len() as u16).unwrap();
            prop_assert_eq!(parsed.indices(), &indices);
            let referenced: BTreeSet<_> = indices.iter().copied().filter(|&index| usize::from(index) >= STANDARD_NAMES).collect();
            prop_assert_eq!(parsed.custom.iter().filter(|name| name.is_some()).count(), referenced.len());
            for &index in &indices {
                if usize::from(index) >= STANDARD_NAMES {
                    prop_assert_eq!(parsed.custom(index).unwrap().to_bytes(), &names[usize::from(index) - STANDARD_NAMES]);
                } else {
                    prop_assert!(parsed.custom(index).is_none());
                }
            }
        }

        #[test]
        fn arbitrary_truncated_tables_never_read_outside_declared_body(
            bytes in prop::collection::vec(any::<u8>(), 0..1024),
            length in 0_usize..1024,
            count in any::<u16>(),
        ) {
            let declared = length.min(bytes.len());
            let parsed = read_format2(&bytes[..declared], declared as u64, count);
            if let Ok(parsed) = parsed {
                prop_assert!(parsed.indices().len() <= usize::from(count));
                prop_assert!(parsed.custom.len() <= declared);
                for &index in parsed.indices() {
                    prop_assert!(usize::from(index) < STANDARD_NAMES || parsed.custom(index).is_some());
                }
            }
        }

        #[test]
        fn output_plan_round_trips_original_name_bytes(
            names in prop::collection::vec("[A-Za-z0-9_.]{0,63}", 1..128),
            picks in prop::collection::vec(any::<usize>(), 0..256),
        ) {
            let owned: Vec<_> = names.iter().map(|name| CString::new(name.as_bytes()).unwrap()).collect();
            let glyphs: Vec<_> = picks.iter().map(|&pick| OutputGlyphName { index: 258, name: &owned[pick % owned.len()], ambiguous: false }).collect();
            let output = output_names(&glyphs).unwrap();
            for (glyph, &index) in glyphs.iter().zip(&output.indices) {
                prop_assert_eq!(output.custom[usize::from(index) - STANDARD_NAMES].to_bytes(), glyph.name.to_bytes());
            }
            prop_assert_eq!(output.custom.len(), glyphs.len());
            prop_assert_eq!(output.indices.len(), glyphs.len());
        }
    }
}
