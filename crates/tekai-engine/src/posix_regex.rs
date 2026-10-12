//! Bounded byte-oriented ERE matching in the C locale.
//!
//! Language recognition uses a Thompson NFA. Captures are reconstructed only
//! after selecting the earliest, longest whole match. Reconstruction gives
//! each earlier subpattern the longest span compatible with the remainder,
//! including unparenthesized subpatterns, and retains the last repetition.
//! There is no recursive trial-and-backtrack matcher. Both recognition and
//! reconstruction debit one shared work budget; reconstruction memoizes NFA
//! interval queries instead of enumerating capture histories.
//! Byte-width bounds remove impossible splits. Unrequested subtrees are not
//! dissected, and fixed-width repetitions retain just their last occurrence.
//!
//! Contracts come from POSIX XBD section 9 and regcomp/regexec:
//! https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap09.html
//! https://pubs.opengroup.org/onlinepubs/9699919799/functions/regcomp.html
//! Native test oracles establish macOS choices for unspecified ERE syntax.
//! This module has no locale, native regex, or mutable-global runtime calls.
//!
//! Admission limits are 16 KiB patterns, 1 MiB text, 256 groups, 64 levels of
//! nesting, 4096 AST nodes, and 8192 NFA states. Interval memoization holds at
//! most 32768 entries. One match has a 32 Mi-unit budget, including scratch
//! initialization, sorting, instruction visits, and capture-selection work.
//! Intervals use the macOS RE_DUP_MAX of 255. Complexity and allocation errors
//! return diagnostics instead of an unbounded search or a native fallback.

#![warn(clippy::all)]

use std::collections::HashMap;
use std::fmt;
use std::ops::Range;

pub(crate) const MAX_PATTERN_BYTES: usize = 16 * 1024;
pub(crate) const MAX_TEXT_BYTES: usize = 1024 * 1024;
const MAX_NODES: usize = 4096;
const MAX_STATES: usize = 8192;
const MAX_GROUPS: usize = 256;
const MAX_DEPTH: usize = 64;
const MAX_REPEAT: usize = 255;
const MAX_INTERVALS: usize = 32768;
const MAX_WORK: usize = 32 * 1024 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RegexError {
    Invalid(&'static str),
    PatternLimit,
    TextLimit,
    CompileLimit,
    MatchLimit,
    Allocation,
}

impl fmt::Display for RegexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Invalid(message) => message,
            Self::PatternLimit => "regular expression exceeds the 16 KiB pattern limit",
            Self::TextLimit => "regular expression input exceeds the 1 MiB limit",
            Self::CompileLimit => "regular expression exceeds the compilation resource limit",
            Self::MatchLimit => "regular expression exceeds the matching resource limit",
            Self::Allocation => "unable to allocate regular expression storage",
        })
    }
}

impl std::error::Error for RegexError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Match {
    pub(crate) whole: Range<usize>,
    /// Slot zero is the whole match. Absent groups differ from empty ranges.
    pub(crate) captures: Vec<Option<Range<usize>>>,
}

#[derive(Clone, Copy, Debug)]
struct Bytes([u64; 4]);

impl Bytes {
    fn empty() -> Self {
        Self([0; 4])
    }

    fn insert(&mut self, byte: u8) {
        self.0[usize::from(byte) / 64] |= 1 << (byte % 64);
    }

    fn contains(self, byte: u8) -> bool {
        self.0[usize::from(byte) / 64] & (1 << (byte % 64)) != 0
    }

    fn union(&mut self, other: Self) {
        for (left, right) in self.0.iter_mut().zip(other.0) {
            *left |= right;
        }
    }

    fn finish(mut self, icase: bool, invert: bool) -> Self {
        if icase {
            for byte in b'A'..=b'Z' {
                if self.contains(byte) || self.contains(byte.to_ascii_lowercase()) {
                    self.insert(byte);
                    self.insert(byte.to_ascii_lowercase());
                }
            }
        }
        if invert {
            for word in &mut self.0 {
                *word = !*word;
            }
        }
        // The former interface accepts C strings, without REG_STARTEND.
        self.0[0] &= !1;
        self
    }
}

#[derive(Debug)]
enum Kind {
    Empty,
    Bytes(Bytes),
    Begin,
    End,
    WordBegin,
    WordEnd,
    Sequence(Vec<usize>),
    Alternate(Vec<usize>),
    Group(usize, usize),
    Repeat {
        child: usize,
        min: usize,
        max: Option<usize>,
    },
}

#[derive(Debug)]
struct Node {
    kind: Kind,
    groups: Range<usize>,
    width: Width,
}

/// Byte lengths, capped above every admitted input. Anchors consume no bytes.
#[derive(Clone, Copy, Debug)]
struct Width {
    min: usize,
    max: Option<usize>,
}

impl Width {
    const ZERO: Self = Self {
        min: 0,
        max: Some(0),
    };
    const ONE: Self = Self {
        min: 1,
        max: Some(1),
    };

    fn fixed(self) -> Option<usize> {
        self.max.filter(|&max| max == self.min)
    }

    fn admits(self, length: usize) -> bool {
        length >= self.min && self.max.is_none_or(|max| length <= max)
    }

    fn then(self, next: Self) -> Self {
        Self {
            min: self.min.saturating_add(next.min).min(MAX_TEXT_BYTES + 1),
            max: self
                .max
                .zip(next.max)
                .map(|(a, b)| a.saturating_add(b).min(MAX_TEXT_BYTES + 1)),
        }
    }

    fn repeat(self, min: usize, max: Option<usize>) -> Self {
        Self {
            min: self.min.saturating_mul(min).min(MAX_TEXT_BYTES + 1),
            max: match (max, self.max) {
                (Some(0), _) | (_, Some(0)) => Some(0),
                (Some(count), Some(width)) => {
                    Some(count.saturating_mul(width).min(MAX_TEXT_BYTES + 1))
                }
                _ => None,
            },
        }
    }

    fn splits(self, tail: Self, start: usize, end: usize) -> Option<Range<usize>> {
        let low = start
            .checked_add(self.min)?
            .max(tail.max.map_or(start, |max| end.saturating_sub(max)));
        let high = end
            .checked_sub(tail.min)?
            .min(self.max.map_or(end, |max| start.saturating_add(max)));
        (low <= high).then(|| low..high + 1)
    }
}

struct Parser<'a> {
    pattern: &'a [u8],
    at: usize,
    icase: bool,
    groups: usize,
    nodes: Vec<Node>,
}

impl Parser<'_> {
    fn peek(&self) -> Option<u8> {
        self.pattern.get(self.at).copied()
    }

    fn push(&mut self, kind: Kind, groups: Range<usize>) -> Result<usize, RegexError> {
        if self.nodes.len() >= MAX_NODES {
            return Err(RegexError::CompileLimit);
        }
        self.nodes
            .try_reserve(1)
            .map_err(|_| RegexError::Allocation)?;
        let id = self.nodes.len();
        let width = match &kind {
            Kind::Bytes(_) => Width::ONE,
            Kind::Group(_, child) => self.nodes[*child].width,
            Kind::Sequence(parts) => parts.iter().fold(Width::ZERO, |width, &part| {
                width.then(self.nodes[part].width)
            }),
            Kind::Alternate(parts) => Width {
                min: parts
                    .iter()
                    .map(|&p| self.nodes[p].width.min)
                    .min()
                    .expect("nonempty alternatives"),
                max: parts.iter().try_fold(0, |max, &part| {
                    self.nodes[part].width.max.map(|width| max.max(width))
                }),
            },
            Kind::Repeat { child, min, max } => self.nodes[*child].width.repeat(*min, *max),
            _ => Width::ZERO,
        };
        self.nodes.push(Node {
            kind,
            groups,
            width,
        });
        Ok(id)
    }

    fn alternation(&mut self, depth: usize, grouped: bool) -> Result<usize, RegexError> {
        let first_group = self.groups + 1;
        let first = self.sequence(depth, grouped)?;
        if self.peek() != Some(b'|') {
            return Ok(first);
        }
        if matches!(self.nodes[first].kind, Kind::Empty) {
            return Err(RegexError::Invalid("empty alternation operand"));
        }
        let mut branches = Vec::new();
        branches
            .try_reserve(1)
            .map_err(|_| RegexError::Allocation)?;
        branches.push(first);
        while self.peek() == Some(b'|') {
            self.at += 1;
            let branch = self.sequence(depth, grouped)?;
            if matches!(self.nodes[branch].kind, Kind::Empty) {
                return Err(RegexError::Invalid("empty alternation operand"));
            }
            branches
                .try_reserve(1)
                .map_err(|_| RegexError::Allocation)?;
            branches.push(branch);
        }
        self.push(Kind::Alternate(branches), first_group..self.groups + 1)
    }

    fn sequence(&mut self, depth: usize, grouped: bool) -> Result<usize, RegexError> {
        let first_group = self.groups + 1;
        let mut parts = Vec::new();
        while self
            .peek()
            .is_some_and(|b| b != b'|' && !(grouped && b == b')'))
        {
            let mut atom = self.atom(depth)?;
            let repetition = match self.peek() {
                Some(b'*') => {
                    self.at += 1;
                    Some((0, None))
                }
                Some(b'+') => {
                    self.at += 1;
                    Some((1, None))
                }
                Some(b'?') => {
                    self.at += 1;
                    Some((0, Some(1)))
                }
                Some(b'{')
                    if self
                        .pattern
                        .get(self.at + 1)
                        .is_some_and(u8::is_ascii_digit) =>
                {
                    let bounds = self.bounds()?;
                    Some(bounds)
                }
                _ => None,
            };
            if let Some((min, max)) = repetition {
                if matches!(
                    self.nodes[atom].kind,
                    Kind::Begin | Kind::End | Kind::WordBegin | Kind::WordEnd
                ) {
                    return Err(RegexError::Invalid("repetition operand is an anchor"));
                }
                atom = self.push(
                    Kind::Repeat {
                        child: atom,
                        min,
                        max,
                    },
                    self.nodes[atom].groups.clone(),
                )?;
                if self.peek().is_some_and(|b| matches!(b, b'*' | b'+' | b'?'))
                    || (self.peek() == Some(b'{')
                        && self
                            .pattern
                            .get(self.at + 1)
                            .is_some_and(u8::is_ascii_digit))
                {
                    return Err(RegexError::Invalid("multiple repetition operators"));
                }
            }
            parts.try_reserve(1).map_err(|_| RegexError::Allocation)?;
            parts.push(atom);
        }
        match parts.len() {
            0 => self.push(Kind::Empty, first_group..first_group),
            1 => Ok(parts[0]),
            _ => self.push(Kind::Sequence(parts), first_group..self.groups + 1),
        }
    }

    fn atom(&mut self, depth: usize) -> Result<usize, RegexError> {
        let byte = self
            .peek()
            .ok_or(RegexError::Invalid("missing expression"))?;
        self.at += 1;
        let no_groups = self.groups + 1..self.groups + 1;
        match byte {
            b'(' => {
                if depth >= MAX_DEPTH || self.groups >= MAX_GROUPS {
                    return Err(RegexError::CompileLimit);
                }
                self.groups += 1;
                let group = self.groups;
                let child = self.alternation(depth + 1, true)?;
                if self.peek() != Some(b')') {
                    return Err(RegexError::Invalid("parentheses not balanced"));
                }
                self.at += 1;
                self.push(Kind::Group(group, child), group..self.groups + 1)
            }
            b'[' => {
                for (spelling, kind) in [
                    (b"[:<:]]".as_slice(), Kind::WordBegin),
                    (b"[:>:]]".as_slice(), Kind::WordEnd),
                ] {
                    if self.pattern[self.at..].starts_with(spelling) {
                        self.at += spelling.len();
                        return self.push(kind, no_groups);
                    }
                }
                let bytes = self.bracket()?;
                self.push(Kind::Bytes(bytes), no_groups)
            }
            b'.' => self.push(
                Kind::Bytes(Bytes([u64::MAX; 4]).finish(false, false)),
                no_groups,
            ),
            b'^' => self.push(Kind::Begin, no_groups),
            b'$' => self.push(Kind::End, no_groups),
            b'*' | b'+' | b'?' => Err(RegexError::Invalid("repetition has no operand")),
            b'{' if self.peek().is_some_and(|byte| byte.is_ascii_digit()) => {
                Err(RegexError::Invalid("repetition has no operand"))
            }
            b'\\' => {
                let escaped = self
                    .peek()
                    .ok_or(RegexError::Invalid("trailing backslash"))?;
                self.at += 1;
                self.literal(escaped, no_groups)
            }
            // At top level an unmatched ')' is an ordinary ERE character.
            _ => self.literal(byte, no_groups),
        }
    }

    fn literal(&mut self, byte: u8, groups: Range<usize>) -> Result<usize, RegexError> {
        let mut bytes = Bytes::empty();
        bytes.insert(byte);
        self.push(Kind::Bytes(bytes.finish(self.icase, false)), groups)
    }

    fn number(&mut self) -> Result<usize, RegexError> {
        let start = self.at;
        let mut value = 0usize;
        while let Some(byte) = self.peek().filter(u8::is_ascii_digit) {
            value = value
                .checked_mul(10)
                .and_then(|v| v.checked_add(usize::from(byte - b'0')))
                .filter(|v| *v <= MAX_REPEAT)
                .ok_or(RegexError::Invalid("repetition count exceeds 255"))?;
            self.at += 1;
        }
        if self.at == start {
            Err(RegexError::Invalid("invalid repetition count"))
        } else {
            Ok(value)
        }
    }

    fn bounds(&mut self) -> Result<(usize, Option<usize>), RegexError> {
        self.at += 1;
        let min = self.number()?;
        let max = if self.peek() == Some(b',') {
            self.at += 1;
            if self.peek() == Some(b'}') {
                None
            } else {
                Some(self.number()?)
            }
        } else {
            Some(min)
        };
        if self.peek() != Some(b'}') || max.is_some_and(|max| max < min) {
            return Err(RegexError::Invalid("invalid repetition bounds"));
        }
        self.at += 1;
        Ok((min, max))
    }

    fn bracket_element(&mut self) -> Result<(Bytes, Option<u8>), RegexError> {
        let mut bytes = Bytes::empty();
        if self.peek() == Some(b'[')
            && self
                .pattern
                .get(self.at + 1)
                .is_some_and(|b| matches!(b, b':' | b'.' | b'='))
        {
            let delimiter = self.pattern[self.at + 1];
            self.at += 2;
            let start = self.at;
            while !(self.peek() == Some(delimiter) && self.pattern.get(self.at + 1) == Some(&b']'))
            {
                if self.peek().is_none() {
                    return Err(RegexError::Invalid("invalid bracket expression"));
                }
                self.at += 1;
            }
            let name = &self.pattern[start..self.at];
            self.at += 2;
            if delimiter == b':' {
                for byte in 1u8..=255 {
                    let matched = match name {
                        b"alnum" => byte.is_ascii_alphanumeric(),
                        b"alpha" => byte.is_ascii_alphabetic(),
                        b"blank" => matches!(byte, b' ' | b'\t'),
                        b"cntrl" => byte.is_ascii_control(),
                        b"digit" => byte.is_ascii_digit(),
                        b"graph" => byte.is_ascii_graphic(),
                        b"lower" => byte.is_ascii_lowercase(),
                        b"print" => byte.is_ascii_graphic() || byte == b' ',
                        b"punct" => byte.is_ascii_punctuation(),
                        b"space" => matches!(byte, b' ' | b'\t' | b'\n' | b'\r' | 11 | 12),
                        b"upper" => byte.is_ascii_uppercase(),
                        b"xdigit" => byte.is_ascii_hexdigit(),
                        _ => return Err(RegexError::Invalid("invalid character class")),
                    };
                    if matched {
                        bytes.insert(byte);
                    }
                }
                return Ok((bytes, None));
            }
            let byte =
                collating_byte(name).ok_or(RegexError::Invalid("invalid collating element"))?;
            bytes.insert(byte);
            return Ok((bytes, (delimiter == b'.').then_some(byte)));
        }
        let byte = self
            .peek()
            .ok_or(RegexError::Invalid("brackets not balanced"))?;
        self.at += 1;
        bytes.insert(byte);
        Ok((bytes, Some(byte)))
    }

    fn bracket(&mut self) -> Result<Bytes, RegexError> {
        let invert = self.peek() == Some(b'^');
        if invert {
            self.at += 1;
        }
        let mut bytes = Bytes::empty();
        let mut elements = 0;
        while self.peek() != Some(b']') || elements == 0 {
            if self.peek().is_none() {
                return Err(RegexError::Invalid("brackets not balanced"));
            }
            let (element, endpoint) = self.bracket_element()?;
            if self.peek() == Some(b'-') && self.pattern.get(self.at + 1) != Some(&b']') {
                self.at += 1;
                let (_, last) = self.bracket_element()?;
                let (Some(first), Some(last)) = (endpoint, last) else {
                    return Err(RegexError::Invalid("invalid character range"));
                };
                if first > last {
                    return Err(RegexError::Invalid("invalid character range"));
                }
                for byte in first..=last {
                    bytes.insert(byte);
                }
                if self.peek() == Some(b'-') && self.pattern.get(self.at + 1) != Some(&b']') {
                    return Err(RegexError::Invalid("overlapping character ranges"));
                }
            } else {
                bytes.union(element);
            }
            elements += 1;
        }
        self.at += 1;
        Ok(bytes.finish(self.icase, invert))
    }
}

fn collating_byte(name: &[u8]) -> Option<u8> {
    if name.len() == 1 {
        return Some(name[0]);
    }
    Some(match name {
        b"alert" => 7,
        b"backspace" => 8,
        b"tab" => 9,
        b"newline" => 10,
        b"vertical-tab" => 11,
        b"form-feed" => 12,
        b"carriage-return" => 13,
        b"space" => b' ',
        b"exclamation-mark" => b'!',
        b"quotation-mark" => b'"',
        b"number-sign" => b'#',
        b"dollar-sign" => b'$',
        b"percent-sign" => b'%',
        b"ampersand" => b'&',
        b"apostrophe" => b'\'',
        b"left-parenthesis" => b'(',
        b"right-parenthesis" => b')',
        b"asterisk" => b'*',
        b"plus-sign" => b'+',
        b"comma" => b',',
        b"hyphen-minus" => b'-',
        b"period" => b'.',
        b"slash" => b'/',
        b"colon" => b':',
        b"semicolon" => b';',
        b"less-than-sign" => b'<',
        b"equals-sign" => b'=',
        b"greater-than-sign" => b'>',
        b"question-mark" => b'?',
        b"commercial-at" => b'@',
        b"left-square-bracket" => b'[',
        b"backslash" => b'\\',
        b"right-square-bracket" => b']',
        b"circumflex" => b'^',
        b"underscore" => b'_',
        b"grave-accent" => b'`',
        b"left-curly-bracket" => b'{',
        b"vertical-line" => b'|',
        b"right-curly-bracket" => b'}',
        b"tilde" => b'~',
        b"zero" => b'0',
        b"one" => b'1',
        b"two" => b'2',
        b"three" => b'3',
        b"four" => b'4',
        b"five" => b'5',
        b"six" => b'6',
        b"seven" => b'7',
        b"eight" => b'8',
        b"nine" => b'9',
        b"NUL" => 0,
        b"SOH" => 1,
        b"STX" => 2,
        b"ETX" => 3,
        b"EOT" => 4,
        b"ENQ" => 5,
        b"ACK" => 6,
        b"SO" => 14,
        b"SI" => 15,
        b"DLE" => 16,
        b"DC1" => 17,
        b"DC2" => 18,
        b"DC3" => 19,
        b"DC4" => 20,
        b"NAK" => 21,
        b"SYN" => 22,
        b"ETB" => 23,
        b"CAN" => 24,
        b"EM" => 25,
        b"SUB" => 26,
        b"ESC" => 27,
        b"IS4" => 28,
        b"IS3" => 29,
        b"IS2" => 30,
        b"IS1" => 31,
        b"DEL" => 127,
        _ => return None,
    })
}

#[derive(Clone, Copy, Debug)]
enum Inst {
    Jump(usize),
    Split(usize, usize),
    Bytes(Bytes, usize),
    Begin(usize),
    End(usize),
    WordBegin(usize),
    WordEnd(usize),
}

#[derive(Clone, Copy, Debug)]
struct Span {
    start: usize,
    end: usize,
}

#[derive(Clone, Copy, Debug)]
struct Step {
    child: Span,
    tail: usize,
}

#[derive(Debug)]
enum Plan {
    None,
    Sequence {
        spans: Vec<Span>,
        tails: Vec<Width>,
    },
    Repeat {
        steps: Vec<Step>,
        loop_at: Option<usize>,
    },
}

struct Builder<'a> {
    nodes: &'a [Node],
    code: Vec<Inst>,
    spans: Vec<Option<Span>>,
    plans: Vec<Plan>,
}

impl Builder<'_> {
    fn emit(&mut self, instruction: Inst) -> Result<usize, RegexError> {
        if self.code.len() >= MAX_STATES {
            return Err(RegexError::CompileLimit);
        }
        self.code
            .try_reserve(1)
            .map_err(|_| RegexError::Allocation)?;
        let pc = self.code.len();
        self.code.push(instruction);
        Ok(pc)
    }

    fn build(&mut self, id: usize) -> Result<Span, RegexError> {
        let start = self.emit(Inst::Jump(0))?;
        let end = self.emit(Inst::Jump(0))?;
        let span = Span { start, end };
        let mut plan = Plan::None;
        let first = match &self.nodes[id].kind {
            Kind::Empty => end,
            Kind::Bytes(bytes) => self.emit(Inst::Bytes(*bytes, end))?,
            Kind::Begin => self.emit(Inst::Begin(end))?,
            Kind::End => self.emit(Inst::End(end))?,
            Kind::WordBegin => self.emit(Inst::WordBegin(end))?,
            Kind::WordEnd => self.emit(Inst::WordEnd(end))?,
            Kind::Group(_, child) => {
                let child = self.build(*child)?;
                self.code[child.end] = Inst::Jump(end);
                child.start
            }
            Kind::Sequence(parts) => {
                let mut spans = Vec::new();
                spans
                    .try_reserve_exact(parts.len())
                    .map_err(|_| RegexError::Allocation)?;
                for &part in parts {
                    spans.push(self.build(part)?);
                }
                for index in 0..spans.len() {
                    self.code[spans[index].end] =
                        Inst::Jump(spans.get(index + 1).map_or(end, |s| s.start));
                }
                let first = spans[0].start;
                let mut tails = Vec::new();
                tails
                    .try_reserve_exact(parts.len() + 1)
                    .map_err(|_| RegexError::Allocation)?;
                tails.resize(parts.len() + 1, Width::ZERO);
                for index in (0..parts.len()).rev() {
                    tails[index] = self.nodes[parts[index]].width.then(tails[index + 1]);
                }
                plan = Plan::Sequence { spans, tails };
                first
            }
            Kind::Alternate(parts) => {
                let mut first = None;
                for &part in parts {
                    let child = self.build(part)?;
                    self.code[child.end] = Inst::Jump(end);
                    first = Some(match first {
                        None => child.start,
                        Some(previous) => self.emit(Inst::Split(previous, child.start))?,
                    });
                }
                first.expect("alternations have operands")
            }
            Kind::Repeat { child, min, max } => {
                let count = max.unwrap_or(*min + 1);
                let mut steps = Vec::new();
                steps
                    .try_reserve_exact(count)
                    .map_err(|_| RegexError::Allocation)?;
                for _ in 0..count {
                    steps.push(Step {
                        child: self.build(*child)?,
                        tail: 0,
                    });
                }
                let mut tail = end;
                for index in (0..count).rev() {
                    if max.is_none() && index == *min {
                        let split = self.emit(Inst::Split(steps[index].child.start, tail))?;
                        self.code[steps[index].child.end] = Inst::Jump(split);
                        steps[index].tail = split;
                        tail = split;
                    } else {
                        self.code[steps[index].child.end] = Inst::Jump(tail);
                        steps[index].tail = tail;
                        tail = if index >= *min {
                            self.emit(Inst::Split(steps[index].child.start, tail))?
                        } else {
                            steps[index].child.start
                        };
                    }
                }
                plan = Plan::Repeat {
                    steps,
                    loop_at: max.is_none().then_some(*min),
                };
                tail
            }
        };
        self.code[start] = Inst::Jump(first);
        if self.spans[id].is_none() {
            self.spans[id] = Some(span);
            self.plans[id] = plan;
        }
        Ok(span)
    }
}

#[derive(Debug)]
pub(crate) struct Regex {
    nodes: Vec<Node>,
    code: Vec<Inst>,
    spans: Vec<Option<Span>>,
    plans: Vec<Plan>,
    root: usize,
    groups: usize,
}

impl Regex {
    pub(crate) fn compile(pattern: &[u8], icase: bool) -> Result<Self, RegexError> {
        if pattern.len() > MAX_PATTERN_BYTES {
            return Err(RegexError::PatternLimit);
        }
        if pattern.is_empty() {
            return Err(RegexError::Invalid("empty expression"));
        }
        if pattern.contains(&0) {
            return Err(RegexError::Invalid("NUL in expression"));
        }
        let mut parser = Parser {
            pattern,
            at: 0,
            icase,
            groups: 0,
            nodes: Vec::new(),
        };
        let root = parser.alternation(0, false)?;
        let mut spans = Vec::new();
        spans
            .try_reserve_exact(parser.nodes.len())
            .map_err(|_| RegexError::Allocation)?;
        spans.resize(parser.nodes.len(), None);
        let mut plans = Vec::new();
        plans
            .try_reserve_exact(parser.nodes.len())
            .map_err(|_| RegexError::Allocation)?;
        plans.resize_with(parser.nodes.len(), || Plan::None);
        let mut builder = Builder {
            nodes: &parser.nodes,
            code: Vec::new(),
            spans,
            plans,
        };
        builder.build(root)?;
        let Builder {
            code, spans, plans, ..
        } = builder;
        Ok(Self {
            nodes: parser.nodes,
            code,
            spans,
            plans,
            root,
            groups: parser.groups,
        })
    }

    pub(crate) fn capture_count(&self) -> usize {
        self.groups + 1
    }

    pub(crate) fn captures(
        &self,
        text: &[u8],
        requested: usize,
    ) -> Result<Option<Match>, RegexError> {
        self.captures_with_budget(text, requested, MAX_WORK)
    }

    fn captures_with_budget(
        &self,
        text: &[u8],
        requested: usize,
        work: usize,
    ) -> Result<Option<Match>, RegexError> {
        self.captures_measured(text, requested, work)
            .map(|(matched, _)| matched)
    }

    /// Count the same charged operations used by the production admission
    /// budget. The CLI still uses MAX_WORK through captures, not a caller-
    /// supplied limit. This hook makes bounded-growth regressions testable.
    pub(crate) fn captures_measured(
        &self,
        text: &[u8],
        requested: usize,
        work: usize,
    ) -> Result<(Option<Match>, usize), RegexError> {
        if text.len() > MAX_TEXT_BYTES {
            return Err(RegexError::TextLimit);
        }
        if text.contains(&0) {
            return Err(RegexError::Invalid("NUL in matching input"));
        }
        let mut matcher = Matcher {
            regex: self,
            text,
            remaining: work,
            intervals: HashMap::new(),
            seen: Vec::new(),
            epoch: 0,
            stack: Vec::new(),
        };
        matcher.charge(self.code.len())?;
        matcher
            .seen
            .try_reserve_exact(self.code.len())
            .map_err(|_| RegexError::Allocation)?;
        matcher.seen.resize(self.code.len(), 0);
        matcher
            .stack
            .try_reserve_exact(self.code.len() * 2)
            .map_err(|_| RegexError::Allocation)?;
        let Some(whole) = matcher.search()? else {
            return Ok((None, work - matcher.remaining));
        };
        let slots = requested.min(self.capture_count());
        let mut captures = Vec::new();
        captures
            .try_reserve_exact(slots)
            .map_err(|_| RegexError::Allocation)?;
        captures.resize(slots, None);
        if slots > 0 {
            captures[0] = Some(whole.clone());
            if slots > 1 {
                matcher.dissect(self.root, whole.start, whole.end, &mut captures)?;
            }
        }
        Ok((Some(Match { whole, captures }), work - matcher.remaining))
    }
}

struct Matcher<'a> {
    regex: &'a Regex,
    text: &'a [u8],
    remaining: usize,
    intervals: HashMap<(usize, usize, usize, usize), bool>,
    seen: Vec<u32>,
    epoch: u32,
    stack: Vec<usize>,
}

// Each thread records an instruction counter and its whole-match start.
type Threads = Vec<(usize, usize)>;

impl Matcher<'_> {
    fn work(&mut self) -> Result<(), RegexError> {
        self.charge(1)
    }

    fn charge(&mut self, amount: usize) -> Result<(), RegexError> {
        self.remaining = self
            .remaining
            .checked_sub(amount)
            .ok_or(RegexError::MatchLimit)?;
        Ok(())
    }

    fn frontier(
        &mut self,
        mut seeds: Threads,
        at: usize,
        accept: usize,
    ) -> Result<(Threads, Option<usize>), RegexError> {
        // Scratch initialization is charged once. Generation marks avoid
        // zeroing the entire NFA for each byte and each interval query.
        self.charge(seeds.len() * (seeds.len().checked_ilog2().unwrap_or(0) as usize + 1))?;
        seeds.sort_unstable_by_key(|&(_, start)| start);
        if self.epoch == u32::MAX {
            self.charge(self.seen.len())?;
            self.seen.fill(0);
            self.epoch = 0;
        }
        self.epoch += 1;
        let mut active = Vec::new();
        let mut stack = std::mem::take(&mut self.stack);
        active
            .try_reserve_exact(self.regex.code.len())
            .map_err(|_| RegexError::Allocation)?;
        let mut found = None;
        for (seed, start) in seeds {
            stack.push(seed);
            while let Some(pc) = stack.pop() {
                self.work()?;
                if self.seen[pc] == self.epoch {
                    continue;
                }
                self.seen[pc] = self.epoch;
                if pc == accept {
                    found.get_or_insert(start);
                    continue;
                }
                match self.regex.code[pc] {
                    Inst::Jump(next) => stack.push(next),
                    Inst::Split(first, second) => {
                        stack.push(second);
                        stack.push(first);
                    }
                    Inst::Begin(next) if at == 0 => stack.push(next),
                    Inst::End(next) if at == self.text.len() => stack.push(next),
                    Inst::WordBegin(next)
                        if self.word_at(at)
                            && !at.checked_sub(1).is_some_and(|before| self.word_at(before)) =>
                    {
                        stack.push(next)
                    }
                    Inst::WordEnd(next)
                        if !self.word_at(at)
                            && at.checked_sub(1).is_some_and(|before| self.word_at(before)) =>
                    {
                        stack.push(next)
                    }
                    Inst::Bytes(_, _) => active.push((pc, start)),
                    Inst::Begin(_) | Inst::End(_) | Inst::WordBegin(_) | Inst::WordEnd(_) => {}
                }
            }
        }
        self.stack = stack;
        Ok((active, found))
    }

    fn advance(&mut self, active: Threads, byte: u8) -> Result<Threads, RegexError> {
        let mut next = Vec::new();
        next.try_reserve_exact(active.len())
            .map_err(|_| RegexError::Allocation)?;
        for (pc, start) in active {
            self.work()?;
            if let Inst::Bytes(bytes, successor) = self.regex.code[pc] {
                if bytes.contains(byte) {
                    next.push((successor, start));
                }
            }
        }
        Ok(next)
    }

    fn word_at(&self, at: usize) -> bool {
        self.text
            .get(at)
            .is_some_and(|b| b.is_ascii_alphanumeric() || *b == b'_')
    }

    fn search(&mut self) -> Result<Option<Range<usize>>, RegexError> {
        let span = self.regex.spans[self.regex.root].expect("root is compiled");
        let mut seeds = Vec::new();
        let mut best: Option<Range<usize>> = None;
        for at in 0..=self.text.len() {
            if best.is_none() {
                seeds.try_reserve(1).map_err(|_| RegexError::Allocation)?;
                seeds.push((span.start, at));
            }
            let (active, matched) = self.frontier(seeds, at, span.end)?;
            if let Some(start) = matched {
                if best.as_ref().is_none_or(|old| start <= old.start) {
                    best = Some(start..at);
                }
            }
            if let Some(&byte) = self.text.get(at) {
                seeds = self.advance(active, byte)?;
                if let Some(best) = &best {
                    seeds.retain(|&(_, start)| start <= best.start);
                }
                if best.is_some() && seeds.is_empty() {
                    break;
                }
            } else {
                break;
            }
        }
        Ok(best)
    }

    fn between(&mut self, span: Span, start: usize, end: usize) -> Result<bool, RegexError> {
        let key = (span.start, span.end, start, end);
        if let Some(&matched) = self.intervals.get(&key) {
            return Ok(matched);
        }
        if self.intervals.len() >= MAX_INTERVALS {
            return Err(RegexError::MatchLimit);
        }
        let mut seeds = Vec::new();
        seeds.try_reserve(1).map_err(|_| RegexError::Allocation)?;
        seeds.push((span.start, start));
        let mut matched = false;
        for at in start..=end {
            let (active, found) = self.frontier(seeds, at, span.end)?;
            if at == end {
                matched = found.is_some();
                break;
            }
            seeds = self.advance(active, self.text[at])?;
            if seeds.is_empty() {
                break;
            }
        }
        self.intervals
            .try_reserve(1)
            .map_err(|_| RegexError::Allocation)?;
        self.intervals.insert(key, matched);
        Ok(matched)
    }

    fn node_matches(&mut self, id: usize, start: usize, end: usize) -> Result<bool, RegexError> {
        self.work()?;
        if !self.regex.nodes[id].width.admits(end - start) {
            return Ok(false);
        }
        self.between(
            self.regex.spans[id].expect("visited child was compiled"),
            start,
            end,
        )
    }

    fn dissect(
        &mut self,
        id: usize,
        start: usize,
        end: usize,
        captures: &mut [Option<Range<usize>>],
    ) -> Result<(), RegexError> {
        let node = &self.regex.nodes[id];
        if node.groups.is_empty() || node.groups.start >= captures.len() {
            // This interval is already known to match. A subtree without any
            // requested capture cannot change the observable result.
            return Ok(());
        }
        self.work()?;
        self.charge(node.groups.len())?;
        for group in node.groups.clone() {
            if let Some(slot) = captures.get_mut(group) {
                *slot = None;
            }
        }
        match &node.kind {
            Kind::Group(group, child) => {
                if let Some(slot) = captures.get_mut(*group) {
                    *slot = Some(start..end);
                }
                self.dissect(*child, start, end, captures)?;
            }
            Kind::Alternate(parts) => {
                for &part in parts {
                    if self.node_matches(part, start, end)? {
                        self.dissect(part, start, end, captures)?;
                        return Ok(());
                    }
                }
                return Err(RegexError::Invalid("capture reconstruction failed"));
            }
            Kind::Sequence(parts) => {
                let Plan::Sequence { spans, tails } = &self.regex.plans[id] else {
                    unreachable!()
                };
                let accept = self.regex.spans[id].expect("sequence compiled").end;
                let mut at = start;
                for (index, &part) in parts.iter().enumerate() {
                    if self.regex.nodes[part].groups.start >= captures.len() {
                        // Group numbers increase from left to right. The
                        // remaining sequence has no requested capture slots.
                        break;
                    }
                    let width = self.regex.nodes[part].width;
                    if let Some(length) = width.fixed() {
                        // There is one possible boundary. The sequence is
                        // known to match, so suffix re-recognition is needless.
                        let split = at + length;
                        self.dissect(part, at, split, captures)?;
                        at = split;
                        continue;
                    }
                    let mut selected = None;
                    let splits = width
                        .splits(tails[index + 1], at, end)
                        .ok_or(RegexError::Invalid("capture reconstruction failed"))?;
                    for split in splits.rev() {
                        self.work()?;
                        if self.node_matches(part, at, split)?
                            && (index + 1 == parts.len() && split == end
                                || index + 1 < parts.len()
                                    && self.between(
                                        Span {
                                            start: spans[index + 1].start,
                                            end: accept,
                                        },
                                        split,
                                        end,
                                    )?)
                        {
                            selected = Some(split);
                            break;
                        }
                    }
                    let split =
                        selected.ok_or(RegexError::Invalid("capture reconstruction failed"))?;
                    self.dissect(part, at, split, captures)?;
                    at = split;
                }
            }
            Kind::Repeat { child, min, max } => {
                let width = self.regex.nodes[*child].width;
                if let Some(length) = width.fixed() {
                    if length == 0 {
                        if *max != Some(0) {
                            self.dissect(*child, start, end, captures)?;
                        }
                    } else {
                        // A positive fixed width determines every occurrence
                        // uniquely. Only the last occurrence can contribute
                        // returned captures, so earlier occurrences need no
                        // reconstruction or quadratic suffix recognition.
                        if start < end {
                            self.dissect(*child, end - length, end, captures)?;
                        }
                    }
                    return Ok(());
                }
                let Plan::Repeat { steps, loop_at } = &self.regex.plans[id] else {
                    unreachable!()
                };
                let accept = self.regex.spans[id].expect("repeat compiled").end;
                let mut at = start;
                let mut index = 0;
                let mut iterations = 0;
                while index < steps.len() {
                    if at == end
                        && index >= *min
                        && (iterations > 0 || !self.node_matches(*child, at, at)?)
                    {
                        break;
                    }
                    let step = steps[index];
                    let mut selected = None;
                    let tail = width.repeat(
                        min.saturating_sub(iterations + 1),
                        max.map(|max| max - iterations - 1),
                    );
                    let splits = width
                        .splits(tail, at, end)
                        .ok_or(RegexError::Invalid("capture reconstruction failed"))?;
                    for split in splits.rev() {
                        self.work()?;
                        if self.node_matches(*child, at, split)?
                            && self.between(
                                Span {
                                    start: step.tail,
                                    end: accept,
                                },
                                split,
                                end,
                            )?
                        {
                            selected = Some(split);
                            break;
                        }
                    }
                    let split =
                        selected.ok_or(RegexError::Invalid("capture reconstruction failed"))?;
                    self.dissect(*child, at, split, captures)?;
                    iterations += 1;
                    let progressed = split > at;
                    at = split;
                    if Some(index) == *loop_at {
                        if !progressed {
                            break;
                        }
                    } else {
                        index += 1;
                    }
                }
                if at != end {
                    return Err(RegexError::Invalid("capture reconstruction failed"));
                }
            }
            Kind::Empty
            | Kind::Bytes(_)
            | Kind::Begin
            | Kind::End
            | Kind::WordBegin
            | Kind::WordEnd => {}
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn found(pattern: &[u8], text: &[u8]) -> Match {
        Regex::compile(pattern, false)
            .unwrap()
            .captures(text, usize::MAX)
            .unwrap()
            .unwrap()
    }

    #[test]
    fn whole_match_is_earliest_then_longest() {
        assert_eq!(found(b"a|aa", b"zaazaa").whole, 1..3);
        assert_eq!(
            found(b"(wee|week)(knights|night)", b"weeknights").whole,
            0..10
        );
        assert_eq!(found(b"a*(a*)", b"aa").captures, [Some(0..2), Some(2..2)]);
        assert_eq!(
            found(b"(a|aa)(a?)", b"aa").captures,
            [Some(0..2), Some(0..2), Some(2..2)]
        );
    }

    #[test]
    fn last_repetition_resets_unselected_descendants() {
        assert_eq!(
            found(b"((a)|(b))*", b"ab").captures,
            [Some(0..2), Some(1..2), None, Some(1..2)]
        );
        assert_eq!(found(b"(a?){2}", b"a").captures, [Some(0..1), Some(1..1)]);
        assert_eq!(found(b"(a?){1,2}", b"a").captures, [Some(0..1), Some(0..1)]);
        assert_eq!(found(b"(a?)*", b"").captures, [Some(0..0), Some(0..0)]);
        assert_eq!(found(b"(a?){0,2}", b"").captures, [Some(0..0), Some(0..0)]);
        assert_eq!(
            found(b"(())?b", b"b").captures,
            [Some(0..1), Some(0..0), Some(0..0)]
        );
        assert_eq!(found(b"(a*)+", b"aaa").captures, [Some(0..3), Some(0..3)]);
    }

    #[test]
    fn nullable_optional_nested_group_shrink_prefers_empty_participation() {
        // POSIX XBD 9.1 ranks an empty capture above nonparticipation.
        assert_eq!(
            found(b"(())?b", b"b").captures,
            [Some(0..1), Some(0..0), Some(0..0)]
        );
    }

    #[test]
    fn nullable_bounded_nested_group_shrink_prefers_empty_participation() {
        assert_eq!(
            found(b"(()){0,2}", b"").captures,
            [Some(0..0), Some(0..0), Some(0..0)]
        );
    }

    #[test]
    fn nullable_alternation_search_shrink_keeps_the_earliest_match() {
        assert_eq!(found(b"c.(()|a)(.)?", b"ccba").whole, 0..3);
    }

    #[test]
    fn optional_bounded_search_shrink_keeps_the_earliest_match() {
        assert_eq!(found(b"[^a]([ab])?.([ab]){0,2}", b"bcaaa").whole, 0..4);
    }

    #[test]
    fn icase_optional_alternation_shrink_preserves_participating_capture() {
        let actual = Regex::compile(b"^(((b)?b|(.)))$", true)
            .unwrap()
            .captures(b"Bb", usize::MAX)
            .unwrap()
            .unwrap();
        assert_eq!(
            actual.captures,
            [Some(0..2), Some(0..2), Some(0..2), Some(0..1), None]
        );
    }

    // An independent AST recognizer exhausts split points. It has no NFA
    // instructions, compiler plans, or capture reconstruction calls. Inputs
    // used by the properties are deliberately small enough for this model.
    struct LanguageModel<'a> {
        regex: &'a Regex,
        text: &'a [u8],
        memo: HashMap<(usize, usize, usize), bool>,
    }

    impl LanguageModel<'_> {
        fn matched(&mut self, node: usize, start: usize, end: usize) -> bool {
            if let Some(&result) = self.memo.get(&(node, start, end)) {
                return result;
            }
            let result = match &self.regex.nodes[node].kind {
                Kind::Empty => start == end,
                Kind::Bytes(bytes) => end == start + 1 && bytes.contains(self.text[start]),
                Kind::Begin => start == end && start == 0,
                Kind::End => start == end && start == self.text.len(),
                Kind::WordBegin | Kind::WordEnd => unreachable!("not generated"),
                Kind::Group(_, child) => self.matched(*child, start, end),
                Kind::Alternate(parts) => parts.iter().any(|&p| self.matched(p, start, end)),
                Kind::Sequence(parts) => self.sequence(parts, start, end),
                Kind::Repeat { child, min, max } => {
                    let mut memo = HashMap::new();
                    self.repeated(*child, *min, *max, start, end, 0, &mut memo)
                }
            };
            self.memo.insert((node, start, end), result);
            result
        }

        fn sequence(&mut self, parts: &[usize], start: usize, end: usize) -> bool {
            match parts.split_first() {
                None => start == end,
                Some((&part, tail)) => (start..=end).any(|split| {
                    self.matched(part, start, split) && self.sequence(tail, split, end)
                }),
            }
        }

        #[allow(clippy::too_many_arguments)]
        fn repeated(
            &mut self,
            child: usize,
            min: usize,
            max: Option<usize>,
            start: usize,
            end: usize,
            count: usize,
            memo: &mut HashMap<(usize, usize), bool>,
        ) -> bool {
            if let Some(&result) = memo.get(&(start, count)) {
                return result;
            }
            // Null iterations cannot add a word to the recognized language
            // after the lower bound is met. This also terminates star loops.
            let result = start == end && count >= min
                || !max.is_some_and(|max| count == max)
                    && (start..=end).any(|split| {
                        (split > start || count < min)
                            && self.matched(child, start, split)
                            && self.repeated(child, min, max, split, end, count + 1, memo)
                    });
            memo.insert((start, count), result);
            result
        }

        fn search(&mut self) -> Option<Range<usize>> {
            for start in 0..=self.text.len() {
                for end in (start..=self.text.len()).rev() {
                    if self.matched(self.regex.root, start, end) {
                        return Some(start..end);
                    }
                }
            }
            None
        }

        fn captures(&mut self, node: usize, start: usize, end: usize) -> Vec<Option<Range<usize>>> {
            let mut captures = vec![None; self.regex.capture_count()];
            // POSIX A.9.1 describes recursive longest-compatible selection.
            // Select splits through the AST recognizer, without consulting
            // production plans, instruction spans, or reconstruction helpers.
            match &self.regex.nodes[node].kind {
                Kind::Group(group, child) => {
                    captures = self.captures(*child, start, end);
                    captures[*group] = Some(start..end);
                }
                Kind::Alternate(parts) => {
                    let child = parts
                        .iter()
                        .copied()
                        .find(|&p| self.matched(p, start, end))
                        .unwrap();
                    captures = self.captures(child, start, end);
                }
                Kind::Sequence(parts) => {
                    let mut at = start;
                    for (index, &part) in parts.iter().enumerate() {
                        let split = (at..=end)
                            .rev()
                            .find(|&split| {
                                self.matched(part, at, split)
                                    && self.sequence(&parts[index + 1..], split, end)
                            })
                            .unwrap();
                        for (slot, value) in captures.iter_mut().zip(self.captures(part, at, split))
                        {
                            if value.is_some() {
                                *slot = value;
                            }
                        }
                        at = split;
                    }
                }
                Kind::Repeat { child, min, max } => {
                    let mut at = start;
                    let mut count = 0;
                    while !max.is_some_and(|max| count == max) {
                        if at == end
                            && count >= *min
                            && (count > 0 || !self.matched(*child, at, at))
                        {
                            break;
                        }
                        let next_min = min.saturating_sub(count + 1);
                        let next_max = max.map(|max| max - count - 1);
                        let split = (at..=end)
                            .rev()
                            .find(|&split| {
                                self.matched(*child, at, split)
                                    && self.repeated(
                                        *child,
                                        next_min,
                                        next_max,
                                        split,
                                        end,
                                        0,
                                        &mut HashMap::new(),
                                    )
                            })
                            .unwrap();
                        captures = self.captures(*child, at, split);
                        count += 1;
                        at = split;
                    }
                    assert_eq!(at, end);
                }
                _ => {}
            }
            captures
        }

        fn validate_groups(
            &mut self,
            node: usize,
            parent: &Option<Range<usize>>,
            captures: &[Option<Range<usize>>],
        ) {
            match &self.regex.nodes[node].kind {
                Kind::Group(group, child) => {
                    let capture = &captures[*group];
                    if let Some(span) = capture {
                        let enclosing = parent.as_ref().expect("matched child needs parent");
                        assert!(enclosing.start <= span.start && span.end <= enclosing.end);
                        assert!(self.matched(*child, span.start, span.end));
                    }
                    self.validate_groups(*child, capture, captures);
                }
                Kind::Sequence(parts) | Kind::Alternate(parts) => {
                    for &part in parts {
                        self.validate_groups(part, parent, captures);
                    }
                }
                Kind::Repeat { child, .. } => self.validate_groups(*child, parent, captures),
                _ => {}
            }
        }
    }

    #[test]
    fn empty_unmatched_and_unrequested_captures_differ() {
        assert_eq!(
            found(b"(a)?()", b"").captures,
            [Some(0..0), None, Some(0..0)]
        );
        let regex = Regex::compile(b"((a))", false).unwrap();
        assert_eq!(regex.capture_count(), 3);
        assert!(regex
            .captures(b"a", 0)
            .unwrap()
            .unwrap()
            .captures
            .is_empty());
        assert_eq!(
            regex.captures(b"a", 1).unwrap().unwrap().captures,
            [Some(0..1)]
        );
    }

    #[test]
    fn anchors_and_dot_have_no_newline_flag() {
        assert!(Regex::compile(b"^a$", false)
            .unwrap()
            .captures(b"a\n", 1)
            .unwrap()
            .is_none());
        assert_eq!(found(b"^a.$", b"a\n").whole, 0..2);
        assert_eq!(found(b"[[:<:]]word[[:>:]]", b"a word! words").whole, 2..6);
    }

    #[test]
    fn macos_ere_extensions_preserve_literal_escapes_and_top_level_parentheses() {
        assert_eq!(found(b"a)b", b"a)b").whole, 0..3);
        assert_eq!(found(b"\\1\\w", b"1w").whole, 0..2);
        assert_eq!(found(b"a{word}", b"a{word}").whole, 0..7);
        assert_eq!(found(b"[]-]+", b"-]-").whole, 0..3);
        assert_eq!(found(b"[\\]+", b"\\\\").whole, 0..2);
        assert_eq!(found(b"[--@]", b"/").whole, 0..1);
    }

    #[test]
    fn byte_matching_and_ascii_casefold_do_not_decode_unicode() {
        assert_eq!(
            found(&[0x80, b'.', 0xff], &[b'z', 0x80, 0xfe, 0xff]).whole,
            1..4
        );
        let regex = Regex::compile(b"([[:lower:]]+)[^A]", true).unwrap();
        assert_eq!(
            regex.captures(b"ABx", 2).unwrap().unwrap().captures,
            [Some(0..3), Some(0..2)]
        );
        assert!(Regex::compile(&[0x80], true)
            .unwrap()
            .captures(&[0x81], 1)
            .unwrap()
            .is_none());
    }

    #[test]
    fn compile_and_match_admission_limits_are_ordinary_errors() {
        assert_eq!(
            Regex::compile(&vec![b'a'; MAX_PATTERN_BYTES + 1], false).unwrap_err(),
            RegexError::PatternLimit
        );
        assert_eq!(
            Regex::compile(b"(((a{255}){255}){255})", false).unwrap_err(),
            RegexError::CompileLimit
        );
        let deep = format!(
            "{}a{}",
            "(".repeat(MAX_DEPTH + 1),
            ")".repeat(MAX_DEPTH + 1)
        );
        assert_eq!(
            Regex::compile(deep.as_bytes(), false).unwrap_err(),
            RegexError::CompileLimit
        );
        let regex = Regex::compile(b"a", false).unwrap();
        assert_eq!(
            regex
                .captures(&vec![b'a'; MAX_TEXT_BYTES + 1], 1)
                .unwrap_err(),
            RegexError::TextLimit
        );
        assert_eq!(
            regex.captures_with_budget(b"a", 1, 0).unwrap_err(),
            RegexError::MatchLimit
        );
    }

    #[test]
    fn ambiguous_failure_uses_bounded_nfa_work_not_backtracking() {
        for pattern in [b"(a|aa)*b".as_slice(), b"(a*)*b".as_slice()] {
            let regex = Regex::compile(pattern, false).unwrap();
            assert!(regex
                .captures_with_budget(&vec![b'a'; 2000], 10, 200_000)
                .unwrap()
                .is_none());
        }
    }

    #[test]
    fn simple_capture_workloads_remain_linear_through_the_text_limit() {
        for (pattern, requested) in [
            (b"(a*)".as_slice(), usize::MAX),
            (b"((a)*)".as_slice(), usize::MAX),
            (b"(a*)a*".as_slice(), usize::MAX),
            (b"(b)?a*".as_slice(), usize::MAX),
            (b"((a)*)".as_slice(), 0),
            (b"((a)*)".as_slice(), 1),
        ] {
            let regex = Regex::compile(pattern, false).unwrap();
            let mut previous = None;
            for length in [40_000, 256 * 1024, MAX_TEXT_BYTES] {
                let text = vec![b'a'; length];
                let (matched, work) = regex
                    .captures_measured(&text, requested, MAX_WORK)
                    .unwrap_or_else(|error| {
                        panic!("{pattern:?}, requested={requested}, length={length}: {error}")
                    });
                let matched = matched.unwrap();
                assert_eq!(matched.whole, 0..length);
                assert_eq!(matched.captures.len(), requested.min(regex.capture_count()));
                if requested > 1 {
                    if pattern == b"(b)?a*" {
                        assert_eq!(matched.captures[1], None);
                    } else {
                        assert_eq!(matched.captures[1], Some(0..length));
                    }
                    if pattern == b"((a)*)" {
                        assert_eq!(matched.captures[2], Some(length - 1..length));
                    }
                }
                eprintln!(
                    "regex work {pattern:?} requested={requested} length={length} work={work}"
                );
                assert!(
                    work <= 30 * length + 1024,
                    "{pattern:?}: work={work}, length={length}"
                );
                if let Some((old_length, old_work)) = previous {
                    assert!(work <= old_work * length / old_length + 1024,
                        "superlinear work for {pattern:?}: {old_length}/{old_work} -> {length}/{work}");
                }
                previous = Some((length, work));
            }
        }
    }

    fn patterns() -> impl Strategy<Value = String> {
        prop::sample::select(vec![
            "a",
            "b",
            "c",
            ".",
            "[ab]",
            "[^a]",
            "[a-c]",
            "[[:alpha:]]",
            "[[:lower:]]",
            "()",
        ])
        .prop_map(str::to_string)
        .prop_recursive(3, 24, 3, |inner| {
            prop_oneof![
                inner.clone().prop_map(|p| format!("({p})")),
                prop::collection::vec(inner.clone(), 2..4).prop_map(|p| p.concat()),
                prop::collection::vec(inner.clone(), 2..4)
                    .prop_map(|p| format!("({})", p.join("|"))),
                (
                    inner,
                    prop::sample::select(vec!["?", "*", "+", "{0,2}", "{1,3}"])
                )
                    .prop_map(|(p, repeat)| format!("({p}){repeat}")),
            ]
        })
    }

    #[cfg(target_os = "macos")]
    mod native {
        use super::*;
        use std::ffi::CString;

        extern "C" {
            fn regcomp_l(
                regex: *mut libc::regex_t,
                pattern: *const libc::c_char,
                flags: libc::c_int,
                locale: libc::locale_t,
            ) -> libc::c_int;
        }

        struct Locale(libc::locale_t);

        impl Drop for Locale {
            fn drop(&mut self) {
                unsafe { libc::freelocale(self.0) };
            }
        }

        struct Compiled(libc::regex_t);

        impl Drop for Compiled {
            fn drop(&mut self) {
                unsafe { libc::regfree(&mut self.0) };
            }
        }

        fn oracle(
            pattern: &[u8],
            text: &[u8],
            icase: bool,
        ) -> Result<Option<Vec<Option<Range<usize>>>>, i32> {
            let locale = Locale(unsafe {
                libc::newlocale(libc::LC_ALL_MASK, c"C".as_ptr(), std::ptr::null_mut())
            });
            assert!(!locale.0.is_null());
            let pattern = CString::new(pattern).unwrap();
            let text = CString::new(text).unwrap();
            let mut regex = unsafe { std::mem::zeroed::<libc::regex_t>() };
            let flags = libc::REG_EXTENDED | if icase { libc::REG_ICASE } else { 0 };
            let code = unsafe { regcomp_l(&mut regex, pattern.as_ptr(), flags, locale.0) };
            if code != 0 {
                return Err(code);
            }
            let regex = Compiled(regex);
            let mut captures = vec![
                libc::regmatch_t {
                    rm_so: -1,
                    rm_eo: -1
                };
                MAX_GROUPS + 1
            ];
            let code = unsafe {
                libc::regexec(
                    &regex.0,
                    text.as_ptr(),
                    captures.len(),
                    captures.as_mut_ptr(),
                    0,
                )
            };
            if code == libc::REG_NOMATCH {
                return Ok(None);
            }
            assert_eq!(code, 0);
            // Drop the compiled form before its explicitly owned C locale.
            drop(regex);
            Ok(Some(
                captures
                    .into_iter()
                    .map(|c| (c.rm_so >= 0).then_some(c.rm_so as usize..c.rm_eo as usize))
                    .collect(),
            ))
        }

        fn check(pattern: &[u8], text: &[u8], icase: bool) {
            let expected = oracle(pattern, text, icase).unwrap();
            let actual = Regex::compile(pattern, icase)
                .unwrap()
                .captures(text, usize::MAX)
                .unwrap()
                .map(|mut m| {
                    m.captures.resize(MAX_GROUPS + 1, None);
                    m.captures
                });
            assert_eq!(
                actual, expected,
                "pattern={pattern:?} text={text:?} icase={icase}"
            );
        }

        #[test]
        fn macos_native_defects_have_explicit_observed_or_corrected_outcomes() {
            // These are contradictions with XBD 9.1 and regexec rules 3/4,
            // not unspecified syntax extensions. Keep the exact native result
            // visible, separately from spec-correct production assertions.
            for (pattern, text, icase, observed_bad, posix_correct) in [
                (
                    "(a?){0,2}",
                    "",
                    false,
                    vec![Some(0..0), None],
                    vec![Some(0..0), Some(0..0)],
                ),
                (
                    "(()){0,2}",
                    "",
                    false,
                    vec![Some(0..0), None, None],
                    vec![Some(0..0), Some(0..0), Some(0..0)],
                ),
                (
                    "((a)|(b))*",
                    "ab",
                    true,
                    vec![Some(0..2), Some(1..2), Some(0..2), Some(1..2)],
                    vec![Some(0..2), Some(1..2), None, Some(1..2)],
                ),
                (
                    "c.(()|a)(.)?",
                    "ccba",
                    false,
                    vec![Some(1..4), Some(3..4), None, None],
                    vec![Some(0..3), Some(2..2), Some(2..2), Some(2..3)],
                ),
                (
                    "[^a]([ab])?.([ab]){0,2}",
                    "bcaaa",
                    false,
                    vec![Some(1..5), Some(2..3), Some(4..5)],
                    vec![Some(0..4), None, Some(3..4)],
                ),
                (
                    "^(((b)?b|(.)))$",
                    "Bb",
                    true,
                    vec![Some(0..2), Some(0..2), Some(0..2), None, None],
                    vec![Some(0..2), Some(0..2), Some(0..2), Some(0..1), None],
                ),
            ] {
                let actual_native = oracle(pattern.as_bytes(), text.as_bytes(), icase)
                    .unwrap()
                    .unwrap();
                let observed = &actual_native[..observed_bad.len()];
                assert!(
                    observed == observed_bad || observed == posix_correct,
                    "uncharacterized native outcome for {pattern:?}, text={text:?}, icase={icase}: {observed:?}"
                );
                assert!(actual_native[observed_bad.len()..]
                    .iter()
                    .all(Option::is_none));
                if observed == posix_correct {
                    eprintln!("native regex now returns the POSIX-correct result for {pattern:?}");
                }
                let actual = Regex::compile(pattern.as_bytes(), icase)
                    .unwrap()
                    .captures(text.as_bytes(), usize::MAX)
                    .unwrap()
                    .unwrap();
                assert_eq!(actual.captures, posix_correct);
            }
            check(b"(())?b", b"b", false);
        }

        #[test]
        fn nested_alternation_repetition_and_empty_captures_match_native() {
            for (pattern, text) in [
                ("a*(a*)", "aa"),
                ("(a|aa)*", "aa"),
                ("a|(a)", "a"),
                ("((a?)*)", "a"),
                ("(a*)+", "aaa"),
                ("(a?){2}", "a"),
                ("(a?){1,2}", "a"),
                ("(a|ab)(bc|c)", "abc"),
                ("((ab|a)*)(b?)", "ab"),
                ("(a)?()", ""),
                ("[[:<:]]word[[:>:]]", "word!"),
                ("[[.tab.]]", "\t"),
                ("[[.zero.]]", "0"),
                ("[[=a=]]", "A"),
                ("a)b", "a)b"),
                ("\\1\\w", "1w"),
                ("a{word}", "a{word}"),
                ("[]-]+", "-]-"),
                ("[\\]+", "\\\\"),
                ("[--@]", "/"),
            ] {
                for icase in [false, true] {
                    check(pattern.as_bytes(), text.as_bytes(), icase);
                }
            }
            check(
                &[b'(', 0x80, b'|', 0xff, b')', b'+'],
                &[0x80, 0xff, 0x80],
                true,
            );
        }

        #[test]
        fn invalid_patterns_have_native_error_controls() {
            for pattern in [
                "",
                "[",
                "(",
                "*a",
                "{1}",
                "a**",
                "a{2,1}",
                "a{256}",
                "a{1",
                "[z-a]",
                "[[:bogus:]]",
                "a|",
                "|a",
                "abc\\",
                "a{2}*",
            ] {
                assert!(
                    Regex::compile(pattern.as_bytes(), false).is_err(),
                    "{pattern}"
                );
                assert!(
                    oracle(pattern.as_bytes(), b"abc", false).is_err(),
                    "{pattern}"
                );
            }
        }

        fn capture_patterns() -> impl Strategy<Value = String> {
            // Native regexec has known defects in quantified group capture
            // reporting, including an ICASE optional group in alternation.
            // This domain has no quantified captured groups. Unmarked atom
            // repetition, nested groups, alternatives, and empty captures
            // remain. Spec properties cover quantified captured groups below.
            prop::sample::select(vec![
                "a",
                "b",
                "c",
                ".",
                "[ab]",
                "[^a]",
                "[a-c]",
                "[[:alpha:]]",
                "[[:lower:]]",
                "()",
                "a*",
                "b?",
                "c+",
                "[ab]{0,2}",
            ])
            .prop_map(str::to_string)
            .prop_recursive(3, 24, 3, |inner| {
                prop_oneof![
                    inner.clone().prop_map(|p| format!("({p})")),
                    prop::collection::vec(inner.clone(), 2..4).prop_map(|p| p.concat()),
                    prop::collection::vec(inner, 2..4).prop_map(|p| format!("({})", p.join("|"))),
                ]
            })
        }

        proptest! {
            #[test]
            fn generated_ere_whole_matches_match_c_locale_native(
                pattern in patterns(),
                text in prop::collection::vec(prop::sample::select(vec![b'a', b'b', b'c', b'A', b'B', b'\n', 0x80, 0xff]), 0..20),
                icase in any::<bool>(),
            ) {
                // Full anchoring isolates native language recognition from
                // the characterized earliest-start search defects above.
                let pattern = format!("^({pattern})$");
                let expected = oracle(pattern.as_bytes(), &text, icase).unwrap()
                    .map(|m| m[0].clone().unwrap());
                let actual = Regex::compile(pattern.as_bytes(), icase).unwrap()
                    .captures(&text, 1).unwrap().map(|m| m.whole);
                prop_assert_eq!(actual, expected, "pattern={} text={:?} icase={}", pattern, text, icase);
            }

            #[test]
            fn generated_capture_vectors_match_native_in_its_valid_capture_domain(
                pattern in capture_patterns(),
                text in prop::collection::vec(prop::sample::select(vec![b'a', b'b', b'c', b'A', b'B', b'\n', 0x80, 0xff]), 0..20),
                icase in any::<bool>(),
            ) {
                let pattern = format!("^({pattern})$");
                let expected = oracle(pattern.as_bytes(), &text, icase).unwrap();
                let actual = Regex::compile(pattern.as_bytes(), icase).unwrap()
                    .captures(&text, usize::MAX).unwrap().map(|mut m| {
                        m.captures.resize(MAX_GROUPS + 1, None);
                        m.captures
                    });
                prop_assert!(actual == expected, "pattern={} text={:?} icase={} actual={:?} native={:?}",
                    pattern, text, icase,
                    actual.as_ref().map(|m| &m[..pattern.matches('(').count() + 1]),
                    expected.as_ref().map(|m| &m[..pattern.matches('(').count() + 1]));
            }
        }
    }

    proptest! {
            #[test]
            fn unrestricted_search_and_captures_follow_the_independent_ast_model(
                pattern in patterns(),
                text in prop::collection::vec(prop::sample::select(vec![b'a', b'b', b'c', b'A', b'B', b'\n', 0x80, 0xff]), 0..8),
                icase in any::<bool>(),
            ) {
                let regex = Regex::compile(pattern.as_bytes(), icase).unwrap();
                let mut model = LanguageModel { regex: &regex, text: &text, memo: HashMap::new() };
                let expected = model.search();
                let actual = regex.captures(&text, usize::MAX).unwrap();
                prop_assert_eq!(actual.as_ref().map(|m| m.whole.clone()), expected,
                    "pattern={} text={:?} icase={}", pattern, text, icase);
                if let Some(actual) = actual {
                    let mut expected = model.captures(regex.root, actual.whole.start, actual.whole.end);
                    expected[0] = Some(actual.whole.clone());
                    prop_assert_eq!(&actual.captures, &expected,
                        "pattern={} text={:?} icase={}", pattern, text, icase);
                    model.validate_groups(regex.root, &Some(actual.whole), &actual.captures);
                }
            }

            #[test]
            fn repeated_nested_captures_follow_posix_last_occurrence_in_both_case_modes(
                letters in prop::collection::vec((any::<bool>(), any::<bool>()), 0..32),
                icase in any::<bool>(),
            ) {
                let text = letters.iter().map(|&(is_a, upper)| {
                    let byte = if is_a { b'a' } else { b'b' };
                    if upper && icase { byte.to_ascii_uppercase() } else { byte }
                }).collect::<Vec<_>>();
                let regex = Regex::compile(b"((a+)|(b+))*", icase).unwrap();
                let actual = regex.captures(&text, usize::MAX).unwrap().unwrap();
                let mut expected = vec![Some(0..text.len()), None, None, None];
                if let Some(&last) = text.last() {
                    let byte = last.to_ascii_lowercase();
                    let start = text.iter().rposition(|b| b.to_ascii_lowercase() != byte)
                        .map_or(0, |p| p + 1);
                    expected[1] = Some(start..text.len());
                    expected[if byte == b'a' { 2 } else { 3 }] = Some(start..text.len());
                }
                prop_assert_eq!(actual.captures, expected);
            }

            #[test]
            fn nullable_nested_intervals_follow_posix_empty_participation(
                min in 0usize..4,
                extra in 0usize..4,
                icase in any::<bool>(),
            ) {
                let max = min + extra;
                let pattern = format!("(()){{{min},{max}}}");
                let regex = Regex::compile(pattern.as_bytes(), icase).unwrap();
                let actual = regex.captures(b"", usize::MAX).unwrap().unwrap();
                let captured = (max > 0).then_some(0..0);
                prop_assert_eq!(actual.captures, vec![Some(0..0), captured.clone(), captured]);
            }
    }
}
