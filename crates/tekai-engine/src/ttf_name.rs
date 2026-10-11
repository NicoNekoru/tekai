//! Checked ownership and output planning for OpenType `name` tables.
//!
//! Record offsets are relative to the header's actual storage offset. Only the
//! union of referenced strings is retained, including format-1 language tags.
//! https://learn.microsoft.com/en-us/typography/opentype/spec/name

#![warn(clippy::all)]

use std::cmp::Ordering;
use std::ffi::{CStr, CString};
use std::io::Read;
use std::ops::Range;

const HEADER_BYTES: usize = 6;
const RECORD_BYTES: usize = 12;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum NameError {
    TableExtent,
    Version,
    Truncated,
    Read,
    StorageOffset,
    StringSpan,
    Utf16Length,
    Allocation,
    OutputLength,
    OutputOffset,
}

impl NameError {
    pub(crate) fn message(self) -> &'static CStr {
        match self {
            Self::TableExtent => c"name: table extent exceeds font file",
            Self::Version => c"name: unsupported table version",
            Self::Truncated => c"name: truncated table",
            Self::Read => c"name: cannot read table",
            Self::StorageOffset => c"name: string storage overlaps records or exceeds table",
            Self::StringSpan => c"name: string span exceeds table storage",
            Self::Utf16Length => c"name: UTF-16 string has an odd byte length",
            Self::Allocation => c"name: cannot allocate table",
            Self::OutputLength => c"name: subset string exceeds unsigned 16-bit length",
            Self::OutputOffset => c"name: subset storage offset exceeds unsigned 16-bit capacity",
        }
    }
}

pub(crate) fn checked_table_extent(
    offset: u32,
    length: u32,
    file_length: u64,
) -> Result<Range<u64>, NameError> {
    let start = u64::from(offset);
    let end = start + u64::from(length);
    if length < HEADER_BYTES as u32 || end > file_length {
        return Err(NameError::TableExtent);
    }
    Ok(start..end)
}

#[derive(Clone, Debug)]
struct NameRecord {
    platform: u16,
    encoding: u16,
    language: u16,
    name: u16,
    span: Range<usize>,
}

impl NameRecord {
    fn utf16(&self) -> bool {
        self.platform == 0 || self.platform == 3 && !matches!(self.encoding, 3..=5)
    }

    fn tagged(&self) -> bool {
        matches!(self.name, 1 | 3 | 4 | 6)
            && (self.platform == 1 && self.encoding == 0
                || self.platform == 3 && matches!(self.encoding, 0 | 1))
    }
}

#[derive(Debug)]
pub(crate) struct NameTable {
    version: u16,
    records: Vec<NameRecord>,
    language_tags: Vec<Range<usize>>,
    strings: Vec<u8>,
}

struct TableReader<R> {
    reader: R,
    length: u64,
    at: u64,
}

impl<R: Read> TableReader<R> {
    fn exact(&mut self, bytes: &mut [u8]) -> Result<(), NameError> {
        if bytes.len() as u64 > self.length - self.at {
            return Err(NameError::Truncated);
        }
        self.reader.read_exact(bytes).map_err(|error| {
            if error.kind() == std::io::ErrorKind::UnexpectedEof {
                NameError::Truncated
            } else {
                NameError::Read
            }
        })?;
        self.at += bytes.len() as u64;
        Ok(())
    }

    fn word(&mut self) -> Result<u16, NameError> {
        let mut bytes = [0; 2];
        self.exact(&mut bytes)?;
        Ok(u16::from_be_bytes(bytes))
    }

    fn skip_to(&mut self, end: u64) -> Result<(), NameError> {
        if end < self.at || end > self.length {
            return Err(NameError::StringSpan);
        }
        let mut scratch = [0; 4096];
        while self.at < end {
            let count = (end - self.at).min(scratch.len() as u64) as usize;
            self.exact(&mut scratch[..count])?;
        }
        Ok(())
    }
}

fn string_span(offset: u16, length: u16, storage_length: u64) -> Result<Range<usize>, NameError> {
    let start = usize::from(offset);
    let end = start + usize::from(length);
    if end as u64 > storage_length {
        return Err(NameError::StringSpan);
    }
    Ok(start..end)
}

fn reserve<T>(items: &mut Vec<T>, count: usize) -> Result<(), NameError> {
    items
        .try_reserve_exact(count)
        .map_err(|_| NameError::Allocation)
}

fn union_spans(mut spans: Vec<Range<usize>>) -> Vec<Range<usize>> {
    spans.retain(|span| !span.is_empty());
    spans.sort_unstable_by_key(|span| (span.start, span.end));
    let mut count = 0;
    for index in 0..spans.len() {
        if count > 0 && spans[index].start <= spans[count - 1].end {
            spans[count - 1].end = spans[count - 1].end.max(spans[index].end);
        } else {
            spans.swap(count, index);
            count += 1;
        }
    }
    spans.truncate(count);
    spans
}

fn compact_span(
    span: &Range<usize>,
    regions: &[(Range<usize>, usize)],
) -> Result<Range<usize>, NameError> {
    if span.is_empty() {
        return Ok(0..0);
    }
    let index = regions.partition_point(|(region, _)| region.end <= span.start);
    let (region, start) = regions.get(index).ok_or(NameError::StringSpan)?;
    if span.start < region.start || span.end > region.end {
        return Err(NameError::StringSpan);
    }
    Ok(start + span.start - region.start..start + span.end - region.start)
}

/// Read from the start of the table after checking its directory extent.
/// Offsets and lengths are u16, so referenced storage can cover at most 131070
/// bytes regardless of padding or the directory's u32 table length.
pub(crate) fn read_table(reader: impl Read, length: u64) -> Result<NameTable, NameError> {
    let mut reader = TableReader {
        reader,
        length,
        at: 0,
    };
    let version = reader.word()?;
    if version > 1 {
        return Err(NameError::Version);
    }
    let count = usize::from(reader.word()?);
    let storage_offset = u64::from(reader.word()?);
    let records_end = HEADER_BYTES + count * RECORD_BYTES;
    if records_end as u64 > storage_offset || storage_offset > length {
        return Err(NameError::StorageOffset);
    }
    let storage_length = length - storage_offset;
    let mut records = Vec::new();
    reserve(&mut records, count)?;
    for _ in 0..count {
        let platform = reader.word()?;
        let encoding = reader.word()?;
        let language = reader.word()?;
        let name = reader.word()?;
        let length = reader.word()?;
        let offset = reader.word()?;
        let record = NameRecord {
            platform,
            encoding,
            language,
            name,
            span: string_span(offset, length, storage_length)?,
        };
        if record.utf16() && length % 2 != 0 {
            return Err(NameError::Utf16Length);
        }
        records.push(record);
    }
    let mut language_tags = Vec::new();
    if version == 1 {
        if reader.at + 2 > storage_offset {
            return Err(NameError::StorageOffset);
        }
        let count = usize::from(reader.word()?);
        if reader.at + count as u64 * 4 > storage_offset {
            return Err(NameError::StorageOffset);
        }
        reserve(&mut language_tags, count)?;
        for _ in 0..count {
            let length = reader.word()?;
            let offset = reader.word()?;
            if length % 2 != 0 {
                return Err(NameError::Utf16Length);
            }
            language_tags.push(string_span(offset, length, storage_length)?);
        }
    }
    let mut spans = Vec::new();
    reserve(&mut spans, records.len() + language_tags.len())?;
    spans.extend(records.iter().map(|record| record.span.clone()));
    spans.extend(language_tags.iter().cloned());
    let spans = union_spans(spans);
    let byte_count: usize = spans.iter().map(|span| span.len()).sum();
    let mut strings = Vec::new();
    reserve(&mut strings, byte_count)?;
    let mut regions = Vec::new();
    reserve(&mut regions, spans.len())?;
    for span in spans {
        reader.skip_to(storage_offset + span.start as u64)?;
        let start = strings.len();
        strings.resize(start + span.len(), 0);
        reader.exact(&mut strings[start..])?;
        regions.push((span, start));
    }
    for record in &mut records {
        record.span = compact_span(&record.span, &regions)?;
    }
    for span in &mut language_tags {
        *span = compact_span(span, &regions)?;
    }
    Ok(NameTable {
        version,
        records,
        language_tags,
        strings,
    })
}

fn strip_byte(byte: u8) -> bool {
    matches!(
        byte,
        b'(' | b')'
            | b'<'
            | b'>'
            | b'['
            | b']'
            | b'{'
            | b'}'
            | b'/'
            | b'%'
            | b' '
            | b'\t'
            | b'\n'
            | b'\r'
            | 0x0b
            | 0x0c
    )
}

impl NameTable {
    /// Keep the backend's Macintosh-first selection and legacy Windows low-byte
    /// spelling. UTF-16 lengths were checked during parsing before pair access.
    /// Unknown format-1 language tags are retained but cannot select a name.
    pub(crate) fn font_name(&self, windows_fallback: bool) -> Result<Option<CString>, NameError> {
        let usable_language = |record: &&NameRecord| {
            self.version != 1
                || record.language < 0x8000
                || usize::from(record.language - 0x8000) < self.language_tags.len()
        };
        let mac = self
            .records
            .iter()
            .filter(usable_language)
            .find(|record| record.platform == 1 && record.encoding == 0 && record.name == 6);
        let selected = mac.or_else(|| {
            windows_fallback
                .then(|| {
                    self.records.iter().filter(usable_language).find(|record| {
                        record.platform == 3 && matches!(record.encoding, 0 | 1) && record.name == 6
                    })
                })
                .flatten()
        });
        let Some(record) = selected else {
            return Ok(None);
        };
        let bytes = &self.strings[record.span.clone()];
        let mut name = Vec::new();
        reserve(
            &mut name,
            bytes.len() / if record.platform == 3 { 2 } else { 1 } + 1,
        )?;
        if record.platform == 3 {
            for byte in bytes
                .as_chunks::<2>()
                .0
                .iter()
                .map(|pair| pair[1])
                .take_while(|&byte| byte != 0)
            {
                if !strip_byte(byte) {
                    name.push(byte);
                }
            }
        } else {
            for &byte in bytes.iter().take_while(|&&byte| byte != 0) {
                if !strip_byte(byte) {
                    name.push(byte);
                }
            }
        }
        Ok(Some(CString::new(name).map_err(|_| NameError::StringSpan)?))
    }

    pub(crate) fn output(&self, subset_tag: Option<[u8; 6]>) -> Result<OutputTable<'_>, NameError> {
        let mut items = Vec::new();
        reserve(&mut items, self.records.len() + self.language_tags.len())?;
        for record in &self.records {
            let prefix = if subset_tag.is_some() && record.tagged() {
                if record.platform == 3 {
                    Prefix::Utf16
                } else {
                    Prefix::Bytes
                }
            } else {
                Prefix::None
            };
            items.push(OutputString {
                prefix,
                span: record.span.clone(),
                bytes: &self.strings[record.span.clone()],
            });
        }
        items.extend(self.language_tags.iter().map(|span| OutputString {
            prefix: Prefix::None,
            span: span.clone(),
            bytes: &self.strings[span.clone()],
        }));
        for item in &items {
            u16::try_from(item.len()).map_err(|_| NameError::OutputLength)?;
        }
        let tag = subset_tag.unwrap_or([0; 6]);
        let (chunks, offsets) = match ordinary_plan(&items) {
            Ok(plan) => plan,
            Err(NameError::OutputOffset) => compact_plan(&items, &self.strings, &tag)?,
            Err(error) => return Err(error),
        };
        let header_length = HEADER_BYTES
            + self.records.len() * RECORD_BYTES
            + if self.version == 1 {
                2 + self.language_tags.len() * 4
            } else {
                0
            };
        let storage_offset = u16::try_from(header_length).map_err(|_| NameError::OutputOffset)?;
        let record_count =
            u16::try_from(self.records.len()).map_err(|_| NameError::OutputOffset)?;
        let language_count =
            u16::try_from(self.language_tags.len()).map_err(|_| NameError::OutputOffset)?;
        let mut header = Vec::new();
        reserve(&mut header, header_length)?;
        header.extend_from_slice(&self.version.to_be_bytes());
        header.extend_from_slice(&record_count.to_be_bytes());
        header.extend_from_slice(&storage_offset.to_be_bytes());
        for (index, record) in self.records.iter().enumerate() {
            for word in [
                record.platform,
                record.encoding,
                record.language,
                record.name,
                items[index].len() as u16,
                offsets[index],
            ] {
                header.extend_from_slice(&word.to_be_bytes());
            }
        }
        if self.version == 1 {
            header.extend_from_slice(&language_count.to_be_bytes());
            for index in self.records.len()..items.len() {
                header.extend_from_slice(&(items[index].len() as u16).to_be_bytes());
                header.extend_from_slice(&offsets[index].to_be_bytes());
            }
        }
        Ok(OutputTable {
            header,
            chunks,
            tag,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Prefix {
    None,
    Bytes,
    Utf16,
}

impl Prefix {
    fn len(self) -> usize {
        match self {
            Self::None => 0,
            Self::Bytes => 7,
            Self::Utf16 => 14,
        }
    }

    fn byte(self, tag: &[u8; 6], index: usize) -> u8 {
        let index = if self == Self::Utf16 {
            if index.is_multiple_of(2) {
                return 0;
            }
            index / 2
        } else {
            index
        };
        tag.get(index).copied().unwrap_or(b'+')
    }
}

#[derive(Clone, Debug)]
struct OutputString<'a> {
    prefix: Prefix,
    span: Range<usize>,
    bytes: &'a [u8],
}

impl OutputString<'_> {
    fn len(&self) -> usize {
        self.prefix.len() + self.bytes.len()
    }

    fn bytes<'a>(&'a self, tag: &'a [u8; 6]) -> impl Iterator<Item = u8> + 'a {
        (0..self.prefix.len())
            .map(move |index| self.prefix.byte(tag, index))
            .chain(self.bytes.iter().copied())
    }
}

pub(crate) struct OutputTable<'a> {
    header: Vec<u8>,
    chunks: Vec<OutputString<'a>>,
    tag: [u8; 6],
}

impl OutputTable<'_> {
    pub(crate) fn emit(&self, mut byte: impl FnMut(u8)) {
        for &value in &self.header {
            byte(value);
        }
        for chunk in &self.chunks {
            for value in chunk.bytes(&self.tag) {
                byte(value);
            }
        }
    }
}

type StoragePlan<'a> = (Vec<OutputString<'a>>, Vec<u16>);

fn ordinary_plan<'a>(items: &[OutputString<'a>]) -> Result<StoragePlan<'a>, NameError> {
    let mut chunks = Vec::new();
    let mut offsets = Vec::new();
    reserve(&mut chunks, items.len())?;
    reserve(&mut offsets, items.len())?;
    let mut at = 0;
    for item in items {
        offsets.push(u16::try_from(at).map_err(|_| NameError::OutputOffset)?);
        at += item.len();
        chunks.push(item.clone());
    }
    Ok((chunks, offsets))
}

/// Keep overlapping unchanged strings shared. Tagged strings share exact
/// duplicates and prefixes. Sort blocks by their latest allowable end so a
/// large last string can extend beyond 65535 without narrowing any offset.
fn compact_plan<'a>(
    items: &[OutputString<'a>],
    strings: &'a [u8],
    tag: &[u8; 6],
) -> Result<StoragePlan<'a>, NameError> {
    compact_plan_with_work(items, strings, tag, &mut PlanWork::default())
}

#[derive(Default)]
struct PlanWork {
    #[cfg(test)]
    byte_comparisons: usize,
    #[cfg(test)]
    index_byte_comparisons: usize,
    #[cfg(test)]
    rank_visits: usize,
    #[cfg(test)]
    rmq_visits: usize,
    #[cfg(test)]
    raw_visits: usize,
}

struct NameComparison {
    order: Ordering,
    common: usize,
}

impl PlanWork {
    fn compare_byte(&mut self, left: u8, right: u8) -> Ordering {
        #[cfg(test)]
        {
            self.byte_comparisons += 1;
        }
        left.cmp(&right)
    }

    fn compare(
        &mut self,
        left: &OutputString<'_>,
        right: &OutputString<'_>,
        tag: &[u8; 6],
        index: Option<&SubstringIndex>,
    ) -> NameComparison {
        if left.prefix == right.prefix && left.span.start == right.span.start {
            return NameComparison {
                order: left.len().cmp(&right.len()),
                common: left.len().min(right.len()),
            };
        }
        let limit = left.len().min(right.len());
        let mut at = 0;
        // Once both generated prefixes end, compare the source substrings
        // through the exact suffix index instead of walking their bytes.
        while at < limit && (at < left.prefix.len() || at < right.prefix.len()) {
            let value = |item: &OutputString<'_>| {
                if at < item.prefix.len() {
                    item.prefix.byte(tag, at)
                } else {
                    item.bytes[at - item.prefix.len()]
                }
            };
            let order = self.compare_byte(value(left), value(right));
            if order != Ordering::Equal {
                return NameComparison { order, common: at };
            }
            at += 1;
        }
        if at == limit {
            return NameComparison {
                order: left.len().cmp(&right.len()),
                common: at,
            };
        }
        let left_start = left.span.start + at - left.prefix.len();
        let right_start = right.span.start + at - right.prefix.len();
        // No-index planning is restricted to one shared prefix and start,
        // which returned above. All other compact plans build this index.
        let index = index.expect("distinct compact names require the substring index");
        let available = (left.span.end - left_start).min(right.span.end - right_start);
        let common = index
            .common_prefix(left_start, right_start, self)
            .min(available);
        let order = if common == available {
            left.len().cmp(&right.len())
        } else {
            self.compare_byte(
                left.bytes[left_start + common - left.span.start],
                right.bytes[right_start + common - right.span.start],
            )
        };
        NameComparison {
            order,
            common: at + common,
        }
    }
}

/// Exact source-substring comparison for the compact fallback. Prefix doubling
/// uses stable integer counting passes, followed by Kasai LCP and segment-tree
/// range minima. Construction is O(S log S), comparisons O(14 + log S), and
/// storage O(S), for the at-most-131070-byte retained source store.
struct SubstringIndex {
    order: Vec<u32>,
    inverse: Vec<u32>,
    minima: Vec<u32>,
    leaves: usize,
}

fn zero_words(count: usize) -> Result<Vec<u32>, NameError> {
    let mut words = Vec::new();
    reserve(&mut words, count)?;
    words.resize(count, 0);
    Ok(words)
}

fn counting_pass(
    input: &[u32],
    output: &mut [u32],
    ranks: &[u32],
    shift: usize,
    counts: &mut [u32],
    work: &mut PlanWork,
) {
    counts.fill(0);
    let key = |position: u32| ranks.get(position as usize + shift).copied().unwrap_or(0) as usize;
    for &position in input {
        counts[key(position)] += 1;
    }
    let mut at = 0;
    for count in counts.iter_mut() {
        let length = *count;
        *count = at;
        at += length;
    }
    for &position in input {
        let count = &mut counts[key(position)];
        output[*count as usize] = position;
        *count += 1;
    }
    #[cfg(test)]
    {
        work.rank_visits += 2 * input.len() + 2 * counts.len();
    }
    #[cfg(not(test))]
    let _ = work;
}

impl SubstringIndex {
    fn new(bytes: &[u8], work: &mut PlanWork) -> Result<Self, NameError> {
        let length = bytes.len();
        if length > usize::from(u16::MAX) * 2 {
            return Err(NameError::StringSpan);
        }
        if length == 0 {
            return Ok(Self {
                order: Vec::new(),
                inverse: Vec::new(),
                minima: zero_words(2)?,
                leaves: 1,
            });
        }
        let mut order = zero_words(length)?;
        let mut scratch = zero_words(length)?;
        let mut ranks = zero_words(length)?;
        let mut next = zero_words(length)?;
        let mut counts = zero_words(length.max(256) + 1)?;
        for position in 0..length {
            order[position] = position as u32;
            ranks[position] = u32::from(bytes[position]) + 1;
        }
        let mut width = 1;
        let mut classes = 256;
        while width < length {
            counting_pass(
                &order,
                &mut scratch,
                &ranks,
                width,
                &mut counts[..=classes],
                work,
            );
            counting_pass(
                &scratch,
                &mut order,
                &ranks,
                0,
                &mut counts[..=classes],
                work,
            );
            let pair = |position: usize| {
                (
                    ranks[position],
                    ranks.get(position + width).copied().unwrap_or(0),
                )
            };
            let mut previous = None;
            classes = 0;
            for &position in &order {
                let position = position as usize;
                let current = pair(position);
                if previous != Some(current) {
                    classes += 1;
                    previous = Some(current);
                }
                next[position] = classes as u32;
            }
            #[cfg(test)]
            {
                work.rank_visits += length;
            }
            std::mem::swap(&mut ranks, &mut next);
            if classes == length {
                break;
            }
            width *= 2;
        }
        drop(scratch);
        drop(counts);
        for (rank, &position) in order.iter().enumerate() {
            ranks[position as usize] = rank as u32;
        }
        next.fill(0);
        let mut common = 0;
        for position in 0..length {
            let rank = ranks[position] as usize;
            if rank == 0 {
                common = 0;
                continue;
            }
            let previous = order[rank - 1] as usize;
            while common < length - position && common < length - previous {
                #[cfg(test)]
                {
                    work.index_byte_comparisons += 1;
                }
                if bytes[position + common] != bytes[previous + common] {
                    break;
                }
                common += 1;
            }
            next[rank] = common as u32;
            common = common.saturating_sub(1);
        }
        let leaves = length.next_power_of_two();
        let mut minima = zero_words(leaves * 2)?;
        minima.fill(u32::MAX);
        minima[leaves..leaves + length].copy_from_slice(&next);
        for node in (1..leaves).rev() {
            minima[node] = minima[node * 2].min(minima[node * 2 + 1]);
        }
        Ok(Self {
            order,
            inverse: ranks,
            minima,
            leaves,
        })
    }

    fn common_prefix(&self, left: usize, right: usize, work: &mut PlanWork) -> usize {
        if left == right {
            return self.inverse.len() - left;
        }
        let left_rank = self.inverse[left] as usize;
        let right_rank = self.inverse[right] as usize;
        let mut start = left_rank.min(right_rank) + 1 + self.leaves;
        let mut end = left_rank.max(right_rank) + 1 + self.leaves;
        let mut common = u32::MAX;
        while start < end {
            #[cfg(test)]
            {
                work.rmq_visits += 1;
            }
            if !start.is_multiple_of(2) {
                common = common.min(self.minima[start]);
                start += 1;
            }
            if !end.is_multiple_of(2) {
                end -= 1;
                common = common.min(self.minima[end]);
            }
            start /= 2;
            end /= 2;
        }
        #[cfg(not(test))]
        let _ = work;
        common as usize
    }

    fn matching_suffixes(
        &self,
        pattern: &OutputString<'_>,
        strings: &[u8],
        tag: &[u8; 6],
        work: &mut PlanWork,
    ) -> Range<usize> {
        let mut boundary = |upper: bool| {
            let mut start = 0;
            let mut end = self.order.len();
            while start < end {
                let middle = start + (end - start) / 2;
                let position = self.order[middle] as usize;
                let suffix = OutputString {
                    prefix: Prefix::None,
                    span: position..strings.len(),
                    bytes: &strings[position..],
                };
                let compared = work.compare(&suffix, pattern, tag, Some(self));
                let order = if compared.common == pattern.len() {
                    Ordering::Equal
                } else {
                    compared.order
                };
                if order == Ordering::Less || upper && order == Ordering::Equal {
                    start = middle + 1;
                } else {
                    end = middle;
                }
            }
            start
        };
        boundary(false)..boundary(true)
    }
}

struct RawQuery {
    item: usize,
    length: usize,
    ranks: Range<usize>,
}

struct PriorityMinima {
    nodes: Vec<u64>,
    leaves: usize,
}

impl PriorityMinima {
    fn new(leaves: usize) -> Result<Self, NameError> {
        let mut nodes = Vec::new();
        reserve(&mut nodes, leaves * 2)?;
        nodes.resize(leaves * 2, u64::MAX);
        Ok(Self { nodes, leaves })
    }

    fn activate(&mut self, rank: usize, priority: u64, work: &mut PlanWork) {
        let mut node = self.leaves + rank;
        self.nodes[node] = priority;
        while node > 1 {
            node /= 2;
            self.nodes[node] = self.nodes[node * 2].min(self.nodes[node * 2 + 1]);
            #[cfg(test)]
            {
                work.raw_visits += 1;
            }
        }
        #[cfg(not(test))]
        let _ = work;
    }

    fn minimum(&self, ranks: &Range<usize>, work: &mut PlanWork) -> u64 {
        let mut start = self.leaves + ranks.start;
        let mut end = self.leaves + ranks.end;
        let mut priority = u64::MAX;
        while start < end {
            #[cfg(test)]
            {
                work.raw_visits += 1;
            }
            if !start.is_multiple_of(2) {
                priority = priority.min(self.nodes[start]);
                start += 1;
            }
            if !end.is_multiple_of(2) {
                end -= 1;
                priority = priority.min(self.nodes[end]);
            }
            start /= 2;
            end /= 2;
        }
        #[cfg(not(test))]
        let _ = work;
        priority
    }
}

const RAW_POSITION_BITS: u32 = 17;
const RAW_POSITION_MASK: u64 = (1 << RAW_POSITION_BITS) - 1;

struct RawStorage<'a> {
    spans: &'a [Range<usize>],
    latest_start: &'a [usize],
    bytes: &'a [u8],
}

impl RawStorage<'_> {
    fn priority(&self, position: usize) -> u64 {
        let block = self.spans.partition_point(|span| span.end <= position);
        let within = position - self.spans[block].start;
        let added = within.saturating_sub(self.latest_start[block]);
        // Each eligible relative start and added constraint fits u16. The
        // bounded source store needs seventeen position bits. Zero-increase
        // matches leave existing raw-block ordering constraints unchanged.
        ((added as u64) << (RAW_POSITION_BITS + 16))
            | ((within as u64) << RAW_POSITION_BITS)
            | position as u64
    }
}

/// Resolve tagged strings already present in unchanged raw blocks. Binary
/// searches find exact matching suffix-rank intervals. A descending length
/// sweep activates each source position once, provided a complete match fits
/// inside its raw block and its relative start fits u16. Rank-range minima
/// then prefer the smallest added raw-block start constraint, relative start,
/// and source position, without R*S byte scans. This is deterministic compact
/// packing, not a globally optimal packing solver.
/// Matching uses O(R log² S) indexed comparisons, with an O(S log S) offline
/// sweep. Together with name sorting the fallback costs
/// O(S log S + R log S (log R + log S)) and O(S + R) auxiliary storage.
fn raw_matches(
    items: &[OutputString<'_>],
    tagged: &[usize],
    storage: RawStorage<'_>,
    tag: &[u8; 6],
    index: &SubstringIndex,
    work: &mut PlanWork,
) -> Result<Vec<u32>, NameError> {
    let mut queries = Vec::new();
    reserve(&mut queries, tagged.len())?;
    for &item in tagged {
        let ranks = index.matching_suffixes(&items[item], storage.bytes, tag, work);
        if !ranks.is_empty() {
            queries.push(RawQuery {
                item,
                length: items[item].len(),
                ranks,
            });
        }
    }
    let mut matches = zero_words(items.len())?;
    matches.fill(u32::MAX);
    if queries.is_empty() {
        return Ok(matches);
    }
    queries.sort_unstable_by(|left, right| {
        right
            .length
            .cmp(&left.length)
            .then(left.item.cmp(&right.item))
    });
    let mut capacities = zero_words(storage.bytes.len())?;
    let mut positions = Vec::new();
    reserve(&mut positions, storage.bytes.len())?;
    for span in storage.spans {
        let end = span.end.min(span.start + usize::from(u16::MAX) + 1);
        for (offset, capacity) in capacities[span.start..end].iter_mut().enumerate() {
            let position = span.start + offset;
            *capacity = (span.end - position) as u32;
            positions.push(position as u32);
        }
    }
    positions.sort_unstable_by(|&left, &right| {
        capacities[right as usize]
            .cmp(&capacities[left as usize])
            .then(left.cmp(&right))
    });
    let mut minima = PriorityMinima::new(index.leaves)?;
    let mut active = 0;
    for query in queries {
        while active < positions.len()
            && capacities[positions[active] as usize] as usize >= query.length
        {
            let position = positions[active];
            minima.activate(
                index.inverse[position as usize] as usize,
                storage.priority(position as usize),
                work,
            );
            active += 1;
        }
        let priority = minima.minimum(&query.ranks, work);
        matches[query.item] = if priority == u64::MAX {
            u32::MAX
        } else {
            (priority & RAW_POSITION_MASK) as u32
        };
    }
    Ok(matches)
}

fn compact_plan_with_work<'a>(
    items: &[OutputString<'a>],
    strings: &'a [u8],
    tag: &[u8; 6],
    work: &mut PlanWork,
) -> Result<StoragePlan<'a>, NameError> {
    let mut raw = Vec::new();
    let mut tagged = Vec::new();
    reserve(&mut raw, items.len())?;
    reserve(&mut tagged, items.len())?;
    for (index, item) in items.iter().enumerate() {
        if item.prefix == Prefix::None {
            raw.push(item.span.clone());
        } else {
            tagged.push(index);
        }
    }
    let raw = union_spans(raw);
    let mut chunks = Vec::new();
    reserve(&mut chunks, raw.len() + tagged.len())?;
    chunks.extend(raw.iter().map(|span| OutputString {
        prefix: Prefix::None,
        span: span.clone(),
        bytes: &strings[span.clone()],
    }));
    let mut assignments = Vec::new();
    reserve(&mut assignments, items.len())?;
    assignments.resize(items.len(), (0_usize, 0_usize));
    let mut latest_start = Vec::new();
    reserve(&mut latest_start, raw.len() + tagged.len())?;
    latest_start.resize(raw.len(), 0_usize);
    for (index, item) in items
        .iter()
        .enumerate()
        .filter(|(_, item)| item.prefix == Prefix::None)
    {
        if item.span.is_empty() {
            assignments[index] = (usize::MAX, 0);
            continue;
        }
        let block = raw.partition_point(|span| span.end <= item.span.start);
        let within = item.span.start - raw[block].start;
        assignments[index] = (block, within);
        latest_start[block] = latest_start[block].max(within);
    }
    tagged.sort_unstable_by_key(|&index| {
        let item = &items[index];
        (item.prefix, item.span.start, item.span.end)
    });
    let mut unique = Vec::new();
    reserve(&mut unique, tagged.len())?;
    let mut representatives = Vec::new();
    reserve(&mut representatives, items.len())?;
    representatives.resize(items.len(), usize::MAX);
    let mut previous = None;
    for &index in &tagged {
        let item = &items[index];
        if previous.is_none_or(|previous: usize| {
            let previous = &items[previous];
            previous.prefix != item.prefix || previous.span != item.span
        }) {
            previous = Some(index);
            unique.push(index);
        }
        representatives[index] = previous.ok_or(NameError::StringSpan)?;
    }
    let shared_start = unique.first().is_none_or(|&first| {
        unique.iter().all(|&other| {
            items[first].prefix == items[other].prefix
                && items[first].span.start == items[other].span.start
        })
    });
    let suffix_index = if shared_start && (raw.is_empty() || unique.is_empty()) {
        None
    } else {
        Some(SubstringIndex::new(strings, work)?)
    };
    if !raw.is_empty() && !unique.is_empty() {
        let matches = raw_matches(
            items,
            &unique,
            RawStorage {
                spans: &raw,
                latest_start: &latest_start,
                bytes: strings,
            },
            tag,
            suffix_index
                .as_ref()
                .expect("raw sharing requires the substring index"),
            work,
        )?;
        unique.retain(|&item| {
            let position = matches[item];
            if position == u32::MAX {
                return true;
            }
            let position = position as usize;
            let block = raw.partition_point(|span| span.end <= position);
            let within = position - raw[block].start;
            assignments[item] = (block, within);
            latest_start[block] = latest_start[block].max(within);
            false
        });
    }
    unique.sort_unstable_by(|&left, &right| {
        work.compare(&items[left], &items[right], tag, suffix_index.as_ref())
            .order
            .then(left.cmp(&right))
    });
    let mut pending = 0;
    for index in 0..unique.len() {
        let item = &items[unique[index]];
        let prefix_of_next = unique.get(index + 1).is_some_and(|&next| {
            work.compare(item, &items[next], tag, suffix_index.as_ref())
                .common
                == item.len()
        });
        if prefix_of_next {
            continue;
        }
        let block = chunks.len();
        chunks.push(item.clone());
        latest_start.push(0);
        for &assigned in &unique[pending..=index] {
            assignments[assigned] = (block, 0);
        }
        pending = index + 1;
    }
    for index in tagged {
        assignments[index] = assignments[representatives[index]];
    }
    let mut order = Vec::new();
    reserve(&mut order, chunks.len())?;
    order.extend(0..chunks.len());
    order.sort_unstable_by(|&left, &right| {
        (chunks[left].len() - latest_start[left])
            .cmp(&(chunks[right].len() - latest_start[right]))
            .then(left.cmp(&right))
    });
    let mut starts = Vec::new();
    reserve(&mut starts, chunks.len())?;
    starts.resize(chunks.len(), 0_usize);
    let mut at = 0_usize;
    for &block in &order {
        if at + latest_start[block] > usize::from(u16::MAX) {
            return Err(NameError::OutputOffset);
        }
        starts[block] = at;
        at += chunks[block].len();
    }
    let mut offsets = Vec::new();
    reserve(&mut offsets, items.len())?;
    for (block, within) in assignments {
        let start = if block == usize::MAX {
            0
        } else {
            starts[block] + within
        };
        offsets.push(u16::try_from(start).map_err(|_| NameError::OutputOffset)?);
    }
    let mut ordered = Vec::new();
    reserve(&mut ordered, chunks.len())?;
    for block in order {
        ordered.push(chunks[block].clone());
    }
    Ok((ordered, offsets))
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn table(
        version: u16,
        records: &[[u16; 6]],
        tags: &[[u16; 2]],
        gap: usize,
        strings: &[u8],
    ) -> Vec<u8> {
        let storage = HEADER_BYTES
            + RECORD_BYTES * records.len()
            + if version == 1 { 2 + tags.len() * 4 } else { 0 }
            + gap;
        let mut bytes = Vec::new();
        for word in [version, records.len() as u16, storage as u16] {
            bytes.extend_from_slice(&word.to_be_bytes());
        }
        for record in records {
            for word in record {
                bytes.extend_from_slice(&word.to_be_bytes());
            }
        }
        if version == 1 {
            bytes.extend_from_slice(&(tags.len() as u16).to_be_bytes());
            for tag in tags {
                for word in tag {
                    bytes.extend_from_slice(&word.to_be_bytes());
                }
            }
        }
        bytes.resize(storage, 0xdd);
        bytes.extend_from_slice(strings);
        bytes
    }

    fn read(bytes: &[u8]) -> Result<NameTable, NameError> {
        read_table(bytes, bytes.len() as u64)
    }
    fn emitted(table: &OutputTable<'_>) -> Vec<u8> {
        let mut bytes = Vec::new();
        table.emit(|byte| bytes.push(byte));
        bytes
    }
    fn word(bytes: &[u8], at: usize) -> u16 {
        u16::from_be_bytes([bytes[at], bytes[at + 1]])
    }

    #[test]
    fn checks_storage_records_and_utf16_before_string_access() {
        assert_eq!(
            checked_table_extent(50, 5, 100),
            Err(NameError::TableExtent)
        );
        assert_eq!(
            checked_table_extent(u32::MAX, 6, u32::MAX as u64 + 6),
            Ok(u32::MAX as u64..u32::MAX as u64 + 6)
        );
        for (bytes, expected) in [
            (vec![0, 2, 0, 0, 0, 6], NameError::Version),
            (vec![0, 0, 0, 1, 0, 6], NameError::StorageOffset),
            (
                table(0, &[[1, 0, 0, 6, 2, 0]], &[], 0, b"A"),
                NameError::StringSpan,
            ),
            (
                table(0, &[[3, 1, 0, 6, 1, 0]], &[], 0, b"A"),
                NameError::Utf16Length,
            ),
            (table(1, &[], &[[1, 0]], 0, b"A"), NameError::Utf16Length),
            (vec![0, 1, 0, 0, 0, 6], NameError::StorageOffset),
        ] {
            assert!(matches!(read(&bytes), Err(error) if error == expected));
        }
        let mut bytes = table(0, &[[1, 0, 0, 6, 1, 0]], &[], 0, b"A");
        bytes[5] = 17;
        assert!(matches!(read(&bytes), Err(NameError::StorageOffset)));
    }

    #[test]
    fn actual_storage_offset_gaps_and_overlapping_spans_are_honored() {
        let bytes = table(
            0,
            &[[1, 0, 0, 6, 4, 100], [1, 0, 0, 4, 3, 101]],
            &[],
            19,
            &[vec![0xee; 100], b"Name".to_vec(), vec![0xcc; 8192]].concat(),
        );
        let parsed = read(&bytes).unwrap();
        assert_eq!(parsed.strings, b"Name");
        assert_eq!(parsed.font_name(true).unwrap().unwrap().to_bytes(), b"Name");
        assert_eq!(&parsed.strings[parsed.records[1].span.clone()], b"ame");
    }

    #[test]
    fn ordinary_output_keeps_subset_bytes_and_format1_language_tags() {
        let strings = [
            b"Font".as_slice(),
            &[0, b'F', 0, b'o', 0, b'n', 0, b't'],
            &[0, b'e', 0, b'n'],
        ]
        .concat();
        let source = table(
            1,
            &[[1, 0, 0x8000, 6, 4, 0], [3, 1, 0x8000, 4, 8, 4]],
            &[[4, 12]],
            9,
            &strings,
        );
        let parsed = read(&source).unwrap();
        let bytes = emitted(&parsed.output(Some(*b"ABCDEF")).unwrap());
        assert_eq!(word(&bytes, 0), 1);
        assert_eq!(word(&bytes, 4), 36);
        assert_eq!(word(&bytes, 14), 11);
        assert_eq!(word(&bytes, 16), 0);
        assert_eq!(word(&bytes, 26), 22);
        assert_eq!(word(&bytes, 28), 11);
        assert_eq!(word(&bytes, 30), 1);
        assert_eq!(word(&bytes, 32), 4);
        assert_eq!(word(&bytes, 34), 33);
        assert_eq!(&bytes[36..47], b"ABCDEF+Font");
        assert_eq!(
            &bytes[47..69],
            &[
                0, b'A', 0, b'B', 0, b'C', 0, b'D', 0, b'E', 0, b'F', 0, b'+', 0, b'F', 0, b'o', 0,
                b'n', 0, b't'
            ]
        );
        assert_eq!(&bytes[69..], &[0, b'e', 0, b'n']);
        assert_eq!(read(&bytes).unwrap().language_tags.len(), 1);
    }

    #[test]
    fn names_above_old_fixed_buffer_keep_legacy_sanitizing_and_selection() {
        let name = [vec![b'A'; 4096], b" /B\0ignored".to_vec()].concat();
        let source = table(0, &[[1, 0, 0, 6, name.len() as u16, 0]], &[], 0, &name);
        let parsed = read(&source).unwrap();
        assert_eq!(
            parsed.font_name(false).unwrap().unwrap().to_bytes(),
            &[vec![b'A'; 4096], b"B".to_vec()].concat()
        );
        let source = table(
            1,
            &[[1, 0, 0x8001, 6, 3, 0], [3, 1, 0, 6, 4, 3]],
            &[[0, 0]],
            0,
            b"Bad\0O\0K",
        );
        let parsed = read(&source).unwrap();
        assert!(parsed.font_name(false).unwrap().is_none());
        assert_eq!(parsed.font_name(true).unwrap().unwrap().to_bytes(), b"OK");
    }

    #[test]
    fn output_checks_expanded_lengths_and_preserves_large_shared_names() {
        let source = table(
            0,
            &[[1, 0, 0, 6, u16::MAX, 0]],
            &[],
            0,
            &vec![b'A'; usize::from(u16::MAX)],
        );
        assert!(matches!(
            read(&source).unwrap().output(Some(*b"ABCDEF")),
            Err(NameError::OutputLength)
        ));
        let source = table(0, &[[1, 0, 0, 6, 32000, 0]; 8], &[], 0, &vec![b'A'; 32000]);
        let parsed = read(&source).unwrap();
        let bytes = emitted(&parsed.output(Some(*b"ABCDEF")).unwrap());
        assert_eq!(bytes.len(), 6 + 8 * 12 + 32007);
        let roundtrip = read(&bytes).unwrap();
        assert_eq!(roundtrip.strings.len(), 32007);
        for record in roundtrip.records {
            assert_eq!(record.span, 0..32007);
        }
        let source = table(
            0,
            &[
                [1, 0, 0, 10, 60000, 0],
                [1, 0, 0, 11, 50000, 10000],
                [1, 0, 0, 12, 10000, 50000],
            ],
            &[],
            0,
            &vec![b'L'; 60000],
        );
        let parsed = read(&source).unwrap();
        let bytes = emitted(&parsed.output(None).unwrap());
        let roundtrip = read(&bytes).unwrap();
        assert_eq!(roundtrip.strings.len(), 60000);
        assert_eq!(roundtrip.records[1].span, 10000..60000);
    }

    #[test]
    fn reader_does_not_walk_unreferenced_table_padding() {
        struct Count<'a> {
            bytes: &'a [u8],
            read: &'a mut usize,
        }
        impl Read for Count<'_> {
            fn read(&mut self, out: &mut [u8]) -> std::io::Result<usize> {
                let count = out.len().min(self.bytes.len());
                out[..count].copy_from_slice(&self.bytes[..count]);
                self.bytes = &self.bytes[count..];
                *self.read += count;
                Ok(count)
            }
        }
        let bytes = table(0, &[[1, 0, 0, 6, 4, 0]; 2048], &[], 0, b"Name");
        let mut read = 0;
        let parsed = read_table(
            Count {
                bytes: &bytes,
                read: &mut read,
            },
            u64::from(u32::MAX),
        )
        .unwrap();
        assert_eq!(read, bytes.len());
        assert_eq!(parsed.strings.len(), 4);
        assert_eq!(parsed.records.len(), 2048);
    }

    #[test]
    fn final_string_may_extend_beyond_unsigned_offset_capacity() {
        let source = table(
            0,
            &[[1, 0, 0, 6, 1, 0], [1, 0, 0, 13, u16::MAX, 0]],
            &[],
            0,
            &vec![b'A'; usize::from(u16::MAX)],
        );
        let parsed = read(&source).unwrap();
        let bytes = emitted(&parsed.output(Some(*b"ABCDEF")).unwrap());
        assert_eq!(word(&bytes, 28), 8);
        assert_eq!(bytes.len(), 30 + 8 + usize::from(u16::MAX));
        let output = read(&bytes).unwrap();
        assert_eq!(output.records[1].span.len(), usize::from(u16::MAX));
    }

    #[test]
    fn thousands_of_shared_long_spans_need_no_byte_comparison_walks() {
        let strings = vec![b'A'; 60000];
        for varied_lengths in [false, true] {
            let items: Vec<_> = (0..4096)
                .map(|index| {
                    let length = if varied_lengths {
                        60000 - index % 1024
                    } else {
                        60000
                    };
                    OutputString {
                        prefix: Prefix::Bytes,
                        span: 0..length,
                        bytes: &strings[..length],
                    }
                })
                .collect();
            let mut work = PlanWork::default();
            let (chunks, offsets) =
                compact_plan_with_work(&items, &strings, b"ABCDEF", &mut work).unwrap();
            assert_eq!(work.byte_comparisons, 0);
            assert_eq!(chunks.len(), 1);
            assert_eq!(chunks[0].len(), 60007);
            assert!(offsets.iter().all(|&offset| offset == 0));
        }
    }

    #[test]
    fn different_start_long_shared_names_use_bounded_exact_index_work() {
        let count = 4096;
        let strings = vec![b'A'; 60000 + count - 1];
        let items: Vec<_> = (0..count)
            .map(|start| OutputString {
                prefix: Prefix::Bytes,
                span: start..start + 60000,
                bytes: &strings[start..start + 60000],
            })
            .collect();
        let mut work = PlanWork::default();
        let (chunks, offsets) =
            compact_plan_with_work(&items, &strings, b"ABCDEF", &mut work).unwrap();
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].len(), 60007);
        assert!(offsets.iter().all(|&offset| offset == 0));
        let levels = strings.len().next_power_of_two().ilog2() as usize;
        assert!(
            work.index_byte_comparisons <= strings.len() * 2,
            "{}",
            work.index_byte_comparisons
        );
        assert!(
            work.rank_visits <= (9 * strings.len() + 1024) * levels,
            "{}",
            work.rank_visits
        );
        assert!(
            work.byte_comparisons <= 14 * count * 64,
            "{}",
            work.byte_comparisons
        );
        assert!(
            work.rmq_visits <= (levels + 1) * count * 64,
            "{}",
            work.rmq_visits
        );
    }

    #[test]
    fn generated_utf16_prefix_may_match_source_string_bytes() {
        let tag = *b"ABCDEF";
        let prefix: Vec<_> = tag
            .iter()
            .copied()
            .chain(*b"+")
            .flat_map(|byte| [0, byte])
            .collect();
        let strings = [prefix.as_slice(), b"tail"].concat();
        let literal = OutputString {
            prefix: Prefix::None,
            span: 0..strings.len(),
            bytes: &strings,
        };
        let generated = OutputString {
            prefix: Prefix::Utf16,
            span: 14..strings.len(),
            bytes: &strings[14..],
        };
        let mut work = PlanWork::default();
        let index = SubstringIndex::new(&strings, &mut work).unwrap();
        let compared = work.compare(&literal, &generated, &tag, Some(&index));
        assert_eq!(compared.order, Ordering::Equal);
        assert_eq!(compared.common, strings.len());
        assert!(work.byte_comparisons <= 14);
    }

    #[test]
    fn tagged_value_already_in_raw_storage_keeps_full_u16_spans_representable() {
        let mut strings = vec![b'A'; usize::from(u16::MAX) * 2];
        strings[..7].copy_from_slice(b"ABCDEF+");
        let records = [
            [1, 0, 0, 1, 93, 7],
            [1, 0, 0, 13, u16::MAX, 0],
            [1, 0, 0, 14, u16::MAX, u16::MAX],
        ];
        let source = table(0, &records, &[], 0, &strings);
        let parsed = read(&source).unwrap();
        let output = emitted(&parsed.output(Some(*b"ABCDEF")).unwrap());
        assert_eq!(output.len(), 42 + strings.len());
        assert_eq!(word(&output, 14), 100);
        assert_eq!(word(&output, 16), 0);
        assert_eq!(word(&output, 28), 0);
        assert_eq!(word(&output, 40), u16::MAX);
        assert_eq!(&output[42..], &strings);
        let roundtrip = read(&output).unwrap();
        assert_eq!(
            &roundtrip.strings[roundtrip.records[0].span.clone()],
            &strings[..100]
        );
    }

    #[test]
    fn shared_occurrence_prefers_no_added_constraint_over_earlier_source_bytes() {
        let mut strings = vec![b'A'; 75010];
        strings[100..1100].fill(b'Y');
        strings[65000..65010].fill(b'Z');
        for start in [64000, 65010] {
            strings[start..start + 7].copy_from_slice(b"ABCDEF+");
        }
        let records = [
            [1, 0, 0, 1, 33, 7],
            [1, 0, 0, 13, 65000, 0],
            [1, 0, 0, 14, 10000, 65010],
            [1, 0, 1, 1, 10, 65000],
            [1, 0, 2, 1, 1000, 100],
        ];
        let source = table(0, &records, &[], 0, &strings);
        let parsed = read(&source).unwrap();
        let output = emitted(&parsed.output(Some(*b"ABCDEF")).unwrap());
        assert_eq!(output.len(), 66 + 76024);
        assert_eq!(word(&output, 16), 1024);
        assert_eq!(word(&output, 28), 11024);
        assert_eq!(word(&output, 40), 1024);
        assert_eq!(word(&output, 52), 0);
        assert_eq!(word(&output, 64), 17);
        let roundtrip = read(&output).unwrap();
        for (record, original) in roundtrip.records.iter().zip(records) {
            let original = &strings
                [usize::from(original[5])..usize::from(original[5]) + usize::from(original[4])];
            let expected = if record.name == 1 {
                [b"ABCDEF+".as_slice(), original].concat()
            } else {
                original.to_vec()
            };
            assert_eq!(&roundtrip.strings[record.span.clone()], &expected);
        }
    }

    #[test]
    fn thousands_of_tagged_raw_matches_use_indexed_length_sweep() {
        let count = 4096;
        let mut strings = vec![b'A'; usize::from(u16::MAX) * 2];
        strings[..7].copy_from_slice(b"ABCDEF+");
        let mut items = vec![
            OutputString {
                prefix: Prefix::None,
                span: 0..65535,
                bytes: &strings[..65535],
            },
            OutputString {
                prefix: Prefix::None,
                span: 65535..131070,
                bytes: &strings[65535..],
            },
        ];
        items.extend((0..count).map(|offset| OutputString {
            prefix: Prefix::Bytes,
            span: 7 + offset..100 + offset,
            bytes: &strings[7 + offset..100 + offset],
        }));
        let mut work = PlanWork::default();
        let (chunks, offsets) =
            compact_plan_with_work(&items, &strings, b"ABCDEF", &mut work).unwrap();
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].prefix, Prefix::None);
        assert_eq!(chunks[0].bytes, &strings);
        assert_eq!(offsets[0], 0);
        assert_eq!(offsets[1], u16::MAX);
        assert!(offsets[2..].iter().all(|&offset| offset == 0));
        let levels = strings.len().next_power_of_two().ilog2() as usize;
        assert!(
            work.byte_comparisons <= 15 * count * 2 * (levels + 1),
            "{}",
            work.byte_comparisons
        );
        assert!(
            work.raw_visits <= (usize::from(u16::MAX) + 1 + count) * (levels + 1),
            "{}",
            work.raw_visits
        );
        assert!(
            work.index_byte_comparisons <= strings.len() * 2,
            "{}",
            work.index_byte_comparisons
        );
    }

    proptest! {
        #[test]
        fn indexed_raw_reuse_matches_naive_complete_raw_windows(
            payload in prop::collection::vec(any::<u8>(), 0..128),
            tag in prop::array::uniform6(any::<u8>()),
            unicode in any::<bool>(),
            regions in prop::collection::vec((any::<u16>(), any::<u16>()), 0..16),
            picks in prop::collection::vec((any::<u16>(), any::<u16>()), 0..16),
            latest_seed in any::<u16>(),
        ) {
            let prefix = if unicode { Prefix::Utf16 } else { Prefix::Bytes };
            let prefix_bytes: Vec<_> = (0..prefix.len()).map(|position| prefix.byte(&tag, position)).collect();
            let strings = [prefix_bytes.as_slice(), payload.as_slice(), prefix_bytes.as_slice(), payload.as_slice()].concat();
            let mut raw = regions.into_iter().map(|(start, length)| {
                let start = usize::from(start) % (strings.len() + 1);
                let length = usize::from(length) % (strings.len() - start + 1);
                start..start + length
            }).collect::<Vec<_>>();
            raw.push(0..prefix.len() + payload.len());
            let raw = union_spans(raw);
            let latest_start: Vec<_> = raw.iter().enumerate().map(|(index, span)| {
                (usize::from(latest_seed) + index) % span.len()
            }).collect();
            let mut items = vec![OutputString { prefix, span: prefix.len()..prefix.len() + payload.len(), bytes: &strings[prefix.len()..prefix.len() + payload.len()] }];
            items.extend(picks.iter().map(|&(start, length)| {
                let start = usize::from(start) % (strings.len() + 1);
                let length = usize::from(length) % (strings.len() - start + 1);
                OutputString { prefix, span: start..start + length, bytes: &strings[start..start + length] }
            }));
            let mut work = PlanWork::default();
            let index = SubstringIndex::new(&strings, &mut work).unwrap();
            let tagged: Vec<_> = (0..items.len()).collect();
            let matches = raw_matches(&items, &tagged, RawStorage { spans: &raw, latest_start: &latest_start, bytes: &strings }, &tag, &index, &mut work).unwrap();
            for (item, position) in items.iter().zip(matches) {
                let expected: Vec<_> = item.bytes(&tag).collect();
                let mut preferred = None;
                for (block, span) in raw.iter().enumerate() {
                    for (offset, window) in strings[span.clone()].windows(expected.len()).enumerate() {
                        if window == expected {
                            let candidate = (offset.saturating_sub(latest_start[block]), offset, (span.start + offset) as u32);
                            if preferred.is_none_or(|previous| candidate < previous) { preferred = Some(candidate); }
                        }
                    }
                }
                prop_assert_eq!(position, preferred.map_or(u32::MAX, |(_, _, position)| position));
            }
        }

        #[test]
        fn exact_source_substring_comparisons_match_naive_bytes(
            bytes in prop::collection::vec(any::<u8>(), 0..192),
            left_start in any::<u16>(), right_start in any::<u16>(),
            left_length in any::<u16>(), right_length in any::<u16>(),
            left_prefix in 0_u8..3, right_prefix in 0_u8..3,
            tag in prop::array::uniform6(any::<u8>()),
        ) {
            let substring = |start: u16, length: u16, prefix: u8| {
                let start = usize::from(start) % (bytes.len() + 1);
                let length = usize::from(length) % (bytes.len() - start + 1);
                OutputString {
                    prefix: match prefix { 0 => Prefix::None, 1 => Prefix::Bytes, _ => Prefix::Utf16 },
                    span: start..start + length,
                    bytes: &bytes[start..start + length],
                }
            };
            let left = substring(left_start, left_length, left_prefix);
            let right = substring(right_start, right_length, right_prefix);
            let mut work = PlanWork::default();
            let index = SubstringIndex::new(&bytes, &mut work).unwrap();
            let comparison = work.compare(&left, &right, &tag, Some(&index));
            let expected_common = left.bytes(&tag).zip(right.bytes(&tag)).take_while(|(left, right)| left == right).count();
            prop_assert_eq!(comparison.order, left.bytes(&tag).cmp(right.bytes(&tag)));
            prop_assert_eq!(comparison.common, expected_common);
            prop_assert!(work.byte_comparisons <= 15);
            prop_assert!(work.index_byte_comparisons <= bytes.len() * 2);
        }

        #[test]
        fn suffix_index_lcp_matches_every_naive_short_suffix(
            bytes in prop::collection::vec(0_u8..8, 1..128),
            pairs in prop::collection::vec((any::<u16>(), any::<u16>()), 0..64),
        ) {
            let mut work = PlanWork::default();
            let index = SubstringIndex::new(&bytes, &mut work).unwrap();
            for (left, right) in pairs {
                let left = usize::from(left) % bytes.len();
                let right = usize::from(right) % bytes.len();
                let expected = bytes[left..].iter().zip(&bytes[right..]).take_while(|(left, right)| left == right).count();
                prop_assert_eq!(index.common_prefix(left, right, &mut work), expected);
            }
            prop_assert!(work.index_byte_comparisons <= bytes.len() * 2);
        }

        #[test]
        fn arbitrary_bytes_never_escape_declared_extent(bytes in prop::collection::vec(any::<u8>(), 0..2048), declared in 0_usize..2048) {
            let length = declared.min(bytes.len());
            if let Ok(table) = read_table(&bytes[..length], length as u64) {
                prop_assert!(table.strings.len() <= usize::from(u16::MAX) * 2);
                for record in &table.records { prop_assert!(record.span.end <= table.strings.len()); }
                for span in &table.language_tags { prop_assert!(span.end <= table.strings.len()); }
            }
        }

        #[test]
        fn checked_subset_plan_round_trips_source_strings(
            strings in prop::collection::vec(any::<u8>(), 0..2048),
            spans in prop::collection::vec((any::<u16>(), any::<u16>(), any::<bool>()), 0..64),
            tag in prop::array::uniform6(b'A'..=b'Z'),
        ) {
            let records: Vec<_> = spans.iter().map(|&(start, len, tagged)| {
                let start = usize::from(start) % (strings.len() + 1);
                let len = usize::from(len) % (strings.len() - start + 1);
                [1, 0, 0, if tagged { 6 } else { 10 }, len as u16, start as u16]
            }).collect();
            let source = table(0, &records, &[], 17, &strings);
            let parsed = read(&source).unwrap();
            let bytes = emitted(&parsed.output(Some(tag)).unwrap());
            let output = read(&bytes).unwrap();
            prop_assert_eq!(output.records.len(), records.len());
            for (record, original) in output.records.iter().zip(&records) {
                let input = &strings[usize::from(original[5])..usize::from(original[5]) + usize::from(original[4])];
                let expected = if original[3] == 6 { [tag.as_slice(), b"+", input].concat() } else { input.to_vec() };
                prop_assert_eq!(&output.strings[record.span.clone()], expected);
            }
        }

        #[test]
        fn format1_preserves_utf16_language_tags_and_windows_subset_bytes(
            text in prop::collection::vec(b'A'..=b'Z', 0..2048),
            language in prop::collection::vec(b'a'..=b'z', 0..32),
            gap in 0_usize..64,
            tag in prop::array::uniform6(b'A'..=b'Z'),
        ) {
            let name: Vec<_> = text.iter().flat_map(|&byte| [0, byte]).collect();
            let language: Vec<_> = language.iter().flat_map(|&byte| [0, byte]).collect();
            let source = table(1, &[[3, 1, 0x8000, 6, name.len() as u16, 0]],
                &[[language.len() as u16, name.len() as u16]], gap,
                &[name.as_slice(), language.as_slice()].concat());
            let parsed = read(&source).unwrap();
            let font_name = parsed.font_name(true).unwrap().unwrap();
            prop_assert_eq!(font_name.to_bytes(), &text);
            let bytes = emitted(&parsed.output(Some(tag)).unwrap());
            let output = read(&bytes).unwrap();
            prop_assert_eq!(&output.strings[output.language_tags[0].clone()], &language);
            let prefix: Vec<_> = tag.iter().copied().chain(*b"+").flat_map(|byte| [0, byte]).collect();
            prop_assert_eq!(&output.strings[output.records[0].span.clone()], [prefix, name].concat());
        }
    }
}
