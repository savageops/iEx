use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind};
use anyhow::{anyhow, Result};
use memchr::{memchr, memchr2, memmem};
use regex::{
    bytes::{Regex as BytesRegex, RegexBuilder as BytesRegexBuilder},
    Regex,
};
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LogicMode {
    All,
    Any,
}

#[derive(Debug, Clone)]
enum Predicate {
    Literal {
        text: String,
        bytes: Vec<u8>,
        finder: memmem::Finder<'static>,
    },
    Prefix {
        text: String,
        bytes: Vec<u8>,
    },
    Suffix {
        text: String,
        bytes: Vec<u8>,
    },
    Regex {
        text: Regex,
        bytes: BytesRegex,
        fast_path: Option<RegexFastPath>,
    },
}

#[derive(Debug, Clone)]
enum RegexFastPath {
    PlainLiteral {
        needle_len: usize,
        finder: memmem::Finder<'static>,
        #[allow(dead_code)]
        reject_fast: Option<RejectFastGate>,
    },
    AsciiCaseFoldLiteral {
        searcher: AsciiCaseFoldSearcher,
        #[allow(dead_code)]
        reject_fast: Option<RejectFastGate>,
    },
    WordBoundaryLiteral {
        literal: Vec<u8>,
        finder: memmem::Finder<'static>,
    },
    AsciiCaseFoldWordBoundaryLiteral {
        searcher: AsciiCaseFoldSearcher,
    },
    LiteralAlternates {
        automaton: AhoCorasick,
        max_literal_len: usize,
        #[allow(dead_code)]
        reject_fast: Option<RejectFastGate>,
    },
}

#[derive(Debug, Clone)]
struct AsciiCaseFoldSearcher {
    needle: Vec<u8>,
    shift: [usize; 256],
    anchor: Option<LiteralAnchorPlan>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LiteralAnchorPlan {
    offset: usize,
    byte: u8,
}

const REJECT_FAST_MIN_LITERAL_LEN: usize = 6;
const REJECT_FAST_QUICK_PREFIX_LEN: usize = 3;
const LITERAL_ANCHOR_MIN_LEN: usize = 4;
const LITERAL_ANCHOR_MIN_DISTINCT_BYTES: usize = 3;
const LITERAL_ANCHOR_STRONG_RANK_MAX: u16 = 96;

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct RejectFastGate {
    min_literal_len: usize,
    prefix_len: usize,
    quick_automaton: AhoCorasick,
}

#[allow(dead_code)]
impl RejectFastGate {
    fn from_literals<'a, I>(literals: I, case_insensitive: bool) -> Option<Self>
    where
        I: IntoIterator<Item = &'a [u8]>,
    {
        let literals: Vec<Vec<u8>> = literals.into_iter().map(|literal| literal.to_vec()).collect();
        if literals.is_empty()
            || literals
                .iter()
                .any(|literal| literal.len() < REJECT_FAST_MIN_LITERAL_LEN)
        {
            return None;
        }

        let min_literal_len = literals.iter().map(Vec::len).min()?;
        let prefix_len = min_literal_len.min(REJECT_FAST_QUICK_PREFIX_LEN);
        let mut prefixes: Vec<Vec<u8>> = Vec::new();
        for literal in literals {
            let prefix = literal[..prefix_len].to_vec();
            if !prefixes.iter().any(|existing| existing == &prefix) {
                prefixes.push(prefix);
            }
        }
        let quick_automaton = AhoCorasickBuilder::new()
            .match_kind(MatchKind::LeftmostFirst)
            .ascii_case_insensitive(case_insensitive)
            .build(prefixes.iter().map(|prefix| prefix.as_slice()))
            .ok()?;

        Some(Self {
            min_literal_len,
            prefix_len,
            quick_automaton,
        })
    }

    fn could_match_anywhere(&self, haystack: &[u8]) -> bool {
        if haystack.len() < self.min_literal_len {
            return false;
        }
        self.quick_automaton.find(haystack).is_some()
    }

    fn could_match_with_start_in_range(&self, haystack: &[u8], start: usize, end: usize) -> bool {
        if start >= end || haystack.len() < self.min_literal_len {
            return false;
        }

        let bounded_end = end.min(haystack.len());
        let overlap = self.prefix_len.saturating_sub(1);
        let slice_start = start.saturating_sub(overlap);
        let slice_end = haystack.len().min(bounded_end.saturating_add(overlap));
        self.quick_automaton
            .find_iter(&haystack[slice_start..slice_end])
            .map(|m| slice_start + m.start())
            .any(|absolute_start| absolute_start >= start && absolute_start < bounded_end)
    }
}

impl AsciiCaseFoldSearcher {
    fn new(needle: &[u8]) -> Option<Self> {
        if needle.is_empty() || !needle.is_ascii() {
            return None;
        }

        let folded: Vec<u8> = needle.iter().map(|byte| byte.to_ascii_lowercase()).collect();
        let mut shift = [folded.len(); 256];

        if folded.len() > 1 {
            for (idx, &byte) in folded.iter().enumerate().take(folded.len() - 1) {
                set_ascii_casefold_shift(&mut shift, byte, folded.len() - 1 - idx);
            }
        }

        let anchor = best_literal_anchor_plan(&folded);
        Some(Self {
            needle: folded,
            shift,
            anchor,
        })
    }

    fn needle_len(&self) -> usize {
        self.needle.len()
    }

    fn matches_at_anchor(
        &self,
        haystack: &[u8],
        start: usize,
        anchor: LiteralAnchorPlan,
    ) -> bool {
        let needle_len = self.needle.len();
        if start + needle_len > haystack.len()
            || !ascii_casefold_eq(haystack[start + anchor.offset], self.needle[anchor.offset])
        {
            return false;
        }

        for needle_idx in anchor.offset + 1..needle_len {
            if !ascii_casefold_eq(haystack[start + needle_idx], self.needle[needle_idx]) {
                return false;
            }
        }
        for needle_idx in (0..anchor.offset).rev() {
            if !ascii_casefold_eq(haystack[start + needle_idx], self.needle[needle_idx]) {
                return false;
            }
        }
        true
    }

    fn find_with_anchor(
        &self,
        haystack: &[u8],
        start: usize,
        anchor: LiteralAnchorPlan,
    ) -> Option<usize> {
        let needle_len = self.needle.len();
        let last_candidate_start = haystack.len().checked_sub(needle_len)?;
        let scan_end = last_candidate_start + anchor.offset + 1;
        let mut scan_offset = start + anchor.offset;
        let lower = anchor.byte;
        let upper = anchor.byte.to_ascii_uppercase();

        while scan_offset < scan_end {
            let relative = if lower == upper {
                memchr(lower, &haystack[scan_offset..scan_end])
            } else {
                memchr2(lower, upper, &haystack[scan_offset..scan_end])
            }?;
            let absolute = scan_offset + relative;
            let candidate_start = absolute - anchor.offset;
            if self.matches_at_anchor(haystack, candidate_start, anchor) {
                return Some(candidate_start);
            }
            scan_offset = absolute + 1;
        }
        None
    }

    fn find(&self, haystack: &[u8], start: usize) -> Option<usize> {
        let needle_len = self.needle.len();
        if needle_len == 0 || start >= haystack.len() || haystack.len() - start < needle_len {
            return None;
        }

        if needle_len == 1 {
            return haystack[start..]
                .iter()
                .position(|&byte| ascii_casefold_eq(byte, self.needle[0]))
                .map(|offset| start + offset);
        }

        if let Some(anchor) = self.anchor {
            return self.find_with_anchor(haystack, start, anchor);
        }

        let mut offset = start;
        while offset + needle_len <= haystack.len() {
            let mut needle_idx = needle_len - 1;
            while ascii_casefold_eq(haystack[offset + needle_idx], self.needle[needle_idx]) {
                if needle_idx == 0 {
                    return Some(offset);
                }
                needle_idx -= 1;
            }

            let skip = self.shift[haystack[offset + needle_len - 1] as usize].max(1);
            offset += skip;
        }

        None
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum PredicateDescriptor {
    Literal(String),
    Prefix(String),
    Suffix(String),
    Regex(String),
}

#[derive(Debug, Clone, Serialize)]
pub struct ExpressionPlan {
    pub source: String,
    pub mode: LogicMode,
    pub predicates: Vec<PredicateDescriptor>,
    #[serde(skip)]
    compiled: Vec<Predicate>,
}

impl ExpressionPlan {
    pub fn parse(raw: &str) -> Result<Self> {
        let source = raw.trim();
        if source.is_empty() {
            return Err(anyhow!("expression cannot be empty"));
        }

        let (mode, tokens): (LogicMode, Vec<&str>) = if source.contains("||") {
            (
                LogicMode::Any,
                source
                    .split("||")
                    .map(str::trim)
                    .filter(|t| !t.is_empty())
                    .collect(),
            )
        } else if source.contains("&&") {
            (
                LogicMode::All,
                source
                    .split("&&")
                    .map(str::trim)
                    .filter(|t| !t.is_empty())
                    .collect(),
            )
        } else {
            (LogicMode::All, vec![source])
        };

        if tokens.is_empty() {
            return Err(anyhow!("expression did not contain any valid tokens"));
        }

        let mut compiled = Vec::with_capacity(tokens.len());
        let mut descriptors = Vec::with_capacity(tokens.len());
        for token in tokens {
            let (predicate, descriptor) = parse_token(token)?;
            compiled.push(predicate);
            descriptors.push(descriptor);
        }

        Ok(Self {
            source: source.to_owned(),
            mode,
            predicates: descriptors,
            compiled,
        })
    }

    pub fn matches(&self, haystack: &str) -> bool {
        match self.mode {
            LogicMode::All => self.compiled.iter().all(|p| predicate_matches(p, haystack)),
            LogicMode::Any => self.compiled.iter().any(|p| predicate_matches(p, haystack)),
        }
    }

    pub fn supports_byte_mode(&self) -> bool {
        true
    }

    pub fn matches_bytes(&self, haystack: &[u8]) -> bool {
        match self.mode {
            LogicMode::All => self
                .compiled
                .iter()
                .all(|p| predicate_matches_bytes(p, haystack)),
            LogicMode::Any => self
                .compiled
                .iter()
                .any(|p| predicate_matches_bytes(p, haystack)),
        }
    }

    pub fn first_match_column(&self, haystack: &str) -> Option<usize> {
        let mut columns: Vec<usize> = self
            .compiled
            .iter()
            .filter_map(|p| predicate_column(p, haystack))
            .collect();

        if columns.is_empty() {
            None
        } else {
            columns.sort_unstable();
            columns.into_iter().next().map(|col| col + 1)
        }
    }

    pub fn first_match_column_bytes(&self, haystack: &[u8]) -> Option<usize> {
        let mut columns: Vec<usize> = self
            .compiled
            .iter()
            .filter_map(|p| predicate_column_bytes(p, haystack))
            .collect();

        if columns.is_empty() {
            None
        } else {
            columns.sort_unstable();
            columns.into_iter().next().map(|col| col + 1)
        }
    }

    pub fn fast_match_count_no_hits(&self, haystack: &str) -> Option<usize> {
        self.fast_match_count_no_hits_bytes(haystack.as_bytes())
    }

    pub fn fast_match_count_no_hits_bytes(&self, haystack: &[u8]) -> Option<usize> {
        match self.compiled.as_slice() {
            [Predicate::Regex {
                bytes, fast_path, ..
            }] => Some(match fast_path {
                Some(RegexFastPath::PlainLiteral { finder, .. }) => {
                    finder.find_iter(haystack).count()
                }
                Some(RegexFastPath::AsciiCaseFoldLiteral { searcher, .. }) => {
                    count_casefold_literal_occurrences_bytes(haystack, searcher)
                }
                Some(RegexFastPath::WordBoundaryLiteral { literal, finder }) => {
                    count_word_boundary_literal_occurrences_bytes(haystack, literal, finder)
                }
                Some(RegexFastPath::AsciiCaseFoldWordBoundaryLiteral { searcher }) => {
                    count_casefold_word_boundary_literal_occurrences_bytes(haystack, searcher)
                }
                Some(RegexFastPath::LiteralAlternates { automaton, .. }) => {
                    count_alternate_literal_occurrences_bytes(haystack, automaton)
                }
                None => bytes.find_iter(haystack).count(),
            }),
            [Predicate::Literal { bytes, finder, .. }] => {
                if bytes.is_empty() {
                    return Some(0);
                }
                Some(finder.find_iter(haystack).count())
            }
            _ => None,
        }
    }

    pub fn fast_match_count_no_hits_bytes_in_range(
        &self,
        haystack: &[u8],
        start: usize,
        end: usize,
    ) -> Option<usize> {
        let bounded_end = end.min(haystack.len());
        if start >= bounded_end {
            return Some(0);
        }

        match self.compiled.as_slice() {
            [Predicate::Regex { fast_path, .. }] => match fast_path {
                Some(RegexFastPath::PlainLiteral { needle_len, finder, .. }) => {
                    Some(count_literal_occurrences_bytes_in_range(
                        haystack,
                        finder,
                        *needle_len,
                        start,
                        bounded_end,
                    ))
                }
                Some(RegexFastPath::AsciiCaseFoldLiteral { searcher, .. }) => {
                    Some(count_casefold_literal_occurrences_bytes_in_range(
                        haystack,
                        searcher,
                        start,
                        bounded_end,
                    ))
                }
                Some(RegexFastPath::WordBoundaryLiteral { literal, finder }) => {
                    Some(count_word_boundary_literal_occurrences_bytes_in_range(
                        haystack,
                        literal,
                        finder,
                        start,
                        bounded_end,
                    ))
                }
                Some(RegexFastPath::AsciiCaseFoldWordBoundaryLiteral { searcher }) => {
                    Some(count_casefold_word_boundary_literal_occurrences_bytes_in_range(
                    haystack,
                    searcher,
                    start,
                    bounded_end,
                ))
                }
                Some(RegexFastPath::LiteralAlternates {
                    automaton,
                    max_literal_len,
                    ..
                }) => Some(count_alternate_literal_occurrences_bytes_in_range(
                        haystack,
                        automaton,
                        *max_literal_len,
                        start,
                        bounded_end,
                    )),
                None => None,
            },
            [Predicate::Literal { bytes, finder, .. }] => {
                Some(count_literal_occurrences_bytes_in_range(
                    haystack,
                    finder,
                    bytes.len(),
                    start,
                    bounded_end,
                ))
            }
            _ => None,
        }
    }

}

fn parse_token(token: &str) -> Result<(Predicate, PredicateDescriptor)> {
    if let Some(value) = token.strip_prefix("re:") {
        let text =
            Regex::new(value).map_err(|err| anyhow!("invalid regex token '{value}': {err}"))?;
        let mut bytes_builder = BytesRegexBuilder::new(value);
        bytes_builder.unicode(false);
        let bytes = bytes_builder
            .build()
            .map_err(|err| anyhow!("invalid regex token '{value}': {err}"))?;
        let fast_path = classify_regex_fast_path(value);
        return Ok((
            Predicate::Regex {
                text,
                bytes,
                fast_path,
            },
            PredicateDescriptor::Regex(value.to_owned()),
        ));
    }

    if let Some(value) = token.strip_prefix("prefix:") {
        if value.is_empty() {
            return Err(anyhow!("prefix token cannot be empty"));
        }
        return Ok((
            Predicate::Prefix {
                text: value.to_owned(),
                bytes: value.as_bytes().to_vec(),
            },
            PredicateDescriptor::Prefix(value.to_owned()),
        ));
    }

    if let Some(value) = token.strip_prefix("suffix:") {
        if value.is_empty() {
            return Err(anyhow!("suffix token cannot be empty"));
        }
        return Ok((
            Predicate::Suffix {
                text: value.to_owned(),
                bytes: value.as_bytes().to_vec(),
            },
            PredicateDescriptor::Suffix(value.to_owned()),
        ));
    }

    let value = token.strip_prefix("lit:").unwrap_or(token);
    if value.is_empty() {
        return Err(anyhow!("literal token cannot be empty"));
    }

    Ok((
        Predicate::Literal {
            text: value.to_owned(),
            bytes: value.as_bytes().to_vec(),
            finder: owned_finder(value.as_bytes()),
        },
        PredicateDescriptor::Literal(value.to_owned()),
    ))
}

fn predicate_matches(predicate: &Predicate, haystack: &str) -> bool {
    match predicate {
        Predicate::Literal { text, .. } => haystack.contains(text),
        Predicate::Prefix { text, .. } => haystack.starts_with(text),
        Predicate::Suffix { text, .. } => haystack.ends_with(text),
        Predicate::Regex { text, .. } => text.is_match(haystack),
    }
}

fn predicate_column(predicate: &Predicate, haystack: &str) -> Option<usize> {
    match predicate {
        Predicate::Literal { text, .. } => haystack.find(text),
        Predicate::Prefix { text, .. } => haystack.starts_with(text).then_some(0),
        Predicate::Suffix { text, .. } => haystack
            .ends_with(text)
            .then(|| haystack.len().saturating_sub(text.len())),
        Predicate::Regex { text, .. } => text.find(haystack).map(|m| m.start()),
    }
}

fn predicate_matches_bytes(predicate: &Predicate, haystack: &[u8]) -> bool {
    match predicate {
        Predicate::Literal { bytes, .. } => memmem::find(haystack, bytes).is_some(),
        Predicate::Prefix { bytes, .. } => haystack.starts_with(bytes),
        Predicate::Suffix { bytes, .. } => haystack.ends_with(bytes),
        Predicate::Regex {
            bytes, fast_path, ..
        } => match fast_path {
            Some(RegexFastPath::PlainLiteral { finder, .. }) => finder.find(haystack).is_some(),
            Some(RegexFastPath::AsciiCaseFoldLiteral { searcher, .. }) => {
                first_casefold_literal_column_bytes(haystack, searcher).is_some()
            }
            Some(RegexFastPath::WordBoundaryLiteral { literal, finder }) => {
                first_word_boundary_literal_column_bytes(haystack, literal, finder).is_some()
            }
            Some(RegexFastPath::AsciiCaseFoldWordBoundaryLiteral { searcher }) => {
                first_casefold_word_boundary_literal_column_bytes(haystack, searcher).is_some()
            }
            Some(RegexFastPath::LiteralAlternates { automaton, .. }) => {
                first_alternate_literal_match(haystack, automaton, 0).is_some()
            }
            None => bytes.is_match(haystack),
        },
    }
}

fn predicate_column_bytes(predicate: &Predicate, haystack: &[u8]) -> Option<usize> {
    match predicate {
        Predicate::Literal { bytes, .. } => memmem::find(haystack, bytes),
        Predicate::Prefix { bytes, .. } => haystack.starts_with(bytes).then_some(0),
        Predicate::Suffix { bytes, .. } => haystack
            .ends_with(bytes)
            .then(|| haystack.len().saturating_sub(bytes.len())),
        Predicate::Regex {
            bytes, fast_path, ..
        } => match fast_path {
            Some(RegexFastPath::PlainLiteral { finder, .. }) => finder.find(haystack),
            Some(RegexFastPath::AsciiCaseFoldLiteral { searcher, .. }) => {
                first_casefold_literal_column_bytes(haystack, searcher)
            }
            Some(RegexFastPath::WordBoundaryLiteral { literal, finder }) => {
                first_word_boundary_literal_column_bytes(haystack, literal, finder)
            }
            Some(RegexFastPath::AsciiCaseFoldWordBoundaryLiteral { searcher }) => {
                first_casefold_word_boundary_literal_column_bytes(haystack, searcher)
            }
            Some(RegexFastPath::LiteralAlternates { automaton, .. }) => {
                first_alternate_literal_match(haystack, automaton, 0)
            }
            None => bytes.find(haystack).map(|m| m.start()),
        },
    }
}

fn classify_regex_fast_path(pattern: &str) -> Option<RegexFastPath> {
    let (case_insensitive, core_pattern) = split_case_insensitive_prefix(pattern);

    if let Some(literal) = parse_word_boundary_literal(core_pattern) {
        if case_insensitive {
            return Some(RegexFastPath::AsciiCaseFoldWordBoundaryLiteral {
                searcher: AsciiCaseFoldSearcher::new(literal.as_bytes())?,
            });
        }
        let literal = literal.into_bytes();
        return Some(RegexFastPath::WordBoundaryLiteral {
            finder: owned_finder(&literal),
            literal,
        });
    }

    if is_plain_ascii_literal(core_pattern) {
        let reject_fast =
            RejectFastGate::from_literals([core_pattern.as_bytes()], case_insensitive);
        if case_insensitive {
            return Some(RegexFastPath::AsciiCaseFoldLiteral {
                searcher: AsciiCaseFoldSearcher::new(core_pattern.as_bytes())?,
                reject_fast,
            });
        }
        return Some(RegexFastPath::PlainLiteral {
            needle_len: core_pattern.len(),
            finder: owned_finder(core_pattern.as_bytes()),
            reject_fast,
        });
    }

    let alternates = parse_literal_alternates(core_pattern)?;
    if alternates.len() < 2 {
        return None;
    }

    let literals: Vec<Vec<u8>> = alternates
        .into_iter()
        .map(|part| part.into_bytes())
        .collect();
    let max_literal_len = literals.iter().map(Vec::len).max()?;
    let reject_fast =
        RejectFastGate::from_literals(literals.iter().map(|part| part.as_slice()), case_insensitive);
    let automaton = build_literal_automaton(literals, case_insensitive)?;
    Some(RegexFastPath::LiteralAlternates {
        automaton,
        max_literal_len,
        reject_fast,
    })
}

fn split_case_insensitive_prefix(pattern: &str) -> (bool, &str) {
    if let Some(rest) = pattern.strip_prefix("(?i)") {
        return (true, rest);
    }
    (false, pattern)
}

fn parse_word_boundary_literal(pattern: &str) -> Option<String> {
    let literal = pattern.strip_prefix("\\b")?.strip_suffix("\\b")?;
    if literal.is_empty() || !is_plain_ascii_literal(literal) {
        return None;
    }
    Some(literal.to_owned())
}

fn parse_literal_alternates(pattern: &str) -> Option<Vec<String>> {
    let inner = pattern.strip_prefix('(')?.strip_suffix(')')?;
    let mut parts = Vec::new();
    for part in inner.split('|') {
        if part.is_empty() || !is_plain_ascii_literal(part) {
            return None;
        }
        parts.push(part.to_owned());
    }
    (parts.len() >= 2).then_some(parts)
}

fn is_plain_ascii_literal(value: &str) -> bool {
    if !value.is_ascii() {
        return false;
    }
    value.bytes().all(|byte| {
        !matches!(
            byte,
            b'\\'
                | b'.'
                | b'^'
                | b'$'
                | b'*'
                | b'+'
                | b'?'
                | b'('
                | b')'
                | b'['
                | b']'
                | b'{'
                | b'}'
                | b'|'
        )
    })
}

fn owned_finder(needle: &[u8]) -> memmem::Finder<'static> {
    memmem::Finder::new(needle).into_owned()
}

fn ascii_casefold_eq(haystack_byte: u8, folded_needle_byte: u8) -> bool {
    haystack_byte.is_ascii() && haystack_byte.to_ascii_lowercase() == folded_needle_byte
}

fn set_ascii_casefold_shift(shift: &mut [usize; 256], folded_byte: u8, distance: usize) {
    let lower = folded_byte.to_ascii_lowercase();
    let upper = folded_byte.to_ascii_uppercase();
    shift[lower as usize] = distance;
    shift[upper as usize] = distance;
}

fn best_literal_anchor_plan(needle: &[u8]) -> Option<LiteralAnchorPlan> {
    if needle.len() < LITERAL_ANCHOR_MIN_LEN {
        return None;
    }

    let mut counts = [0u8; 256];
    let mut distinct = 0usize;
    for &byte in needle {
        let slot = &mut counts[byte as usize];
        if *slot == 0 {
            distinct += 1;
        }
        *slot = slot.saturating_add(1);
    }
    if distinct < LITERAL_ANCHOR_MIN_DISTINCT_BYTES {
        return None;
    }

    let center = needle.len() / 2;
    let (_, plan) = needle
        .iter()
        .copied()
        .enumerate()
        .map(|(offset, byte)| {
            let duplicate_penalty = counts[byte as usize].saturating_sub(1) as u16;
            let rank = ascii_anchor_frequency_rank(byte) as u16;
            let poison_penalty = if is_poisonous_anchor_byte(byte) { 1u16 } else { 0u16 };
            let edge_penalty = if offset == 0 || offset + 1 == needle.len() {
                1u16
            } else {
                0u16
            };
            let adjacent_duplicate_penalty = if (offset > 0 && needle[offset - 1] == byte)
                || (offset + 1 < needle.len() && needle[offset + 1] == byte)
            {
                1u16
            } else {
                0u16
            };
            let center_distance = offset.abs_diff(center) as u16;
            (
                (
                    poison_penalty,
                    duplicate_penalty,
                    adjacent_duplicate_penalty,
                    rank,
                    edge_penalty,
                    center_distance,
                    offset,
                ),
                LiteralAnchorPlan { offset, byte },
            )
        })
        .min_by_key(|(key, _)| *key)?;

    let anchor_count = counts[plan.byte as usize] as u16;
    let anchor_rank = ascii_anchor_frequency_rank(plan.byte) as u16;
    if is_poisonous_anchor_byte(plan.byte) {
        return None;
    }
    if anchor_count > 1 && anchor_rank > LITERAL_ANCHOR_STRONG_RANK_MAX {
        return None;
    }
    Some(plan)
}

fn ascii_anchor_frequency_rank(byte: u8) -> u8 {
    if byte.is_ascii_whitespace() {
        return 255;
    }
    if byte == b'_' {
        return 245;
    }
    if byte.is_ascii_digit() {
        return 232;
    }
    if !byte.is_ascii_alphanumeric() {
        return 120;
    }

    match byte.to_ascii_lowercase() {
        b'z' => 4,
        b'q' => 8,
        b'j' => 12,
        b'x' => 16,
        b'k' => 20,
        b'v' => 28,
        b'b' => 36,
        b'p' => 44,
        b'g' => 52,
        b'w' => 60,
        b'y' => 68,
        b'f' => 76,
        b'm' => 84,
        b'c' => 92,
        b'u' => 100,
        b'l' => 108,
        b'd' => 116,
        b'r' => 124,
        b'h' => 132,
        b's' => 140,
        b'n' => 148,
        b'i' => 156,
        b'o' => 164,
        b'a' => 172,
        b't' => 180,
        b'e' => 188,
        _ => 200,
    }
}

fn is_poisonous_anchor_byte(byte: u8) -> bool {
    byte.is_ascii_whitespace() || byte == b'_' || byte == b'\0'
}

fn build_literal_automaton(literals: Vec<Vec<u8>>, case_insensitive: bool) -> Option<AhoCorasick> {
    AhoCorasickBuilder::new()
        .match_kind(MatchKind::LeftmostFirst)
        .ascii_case_insensitive(case_insensitive)
        .build(literals.iter().map(|part| part.as_slice()))
        .ok()
}

fn is_ascii_word_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn has_ascii_word_boundaries(haystack: &[u8], offset: usize, len: usize) -> bool {
    let left_ok = offset == 0 || !is_ascii_word_byte(haystack[offset - 1]);
    let end = offset + len;
    let right_ok = end == haystack.len() || !is_ascii_word_byte(haystack[end]);
    left_ok && right_ok
}

fn first_word_boundary_literal_column_bytes(
    haystack: &[u8],
    literal: &[u8],
    finder: &memmem::Finder<'static>,
) -> Option<usize> {
    if literal.is_empty() {
        return None;
    }
    finder
        .find_iter(haystack)
        .find(|&offset| has_ascii_word_boundaries(haystack, offset, literal.len()))
}

fn first_casefold_literal_column_bytes(
    haystack: &[u8],
    searcher: &AsciiCaseFoldSearcher,
) -> Option<usize> {
    searcher.find(haystack, 0)
}

fn first_casefold_word_boundary_literal_column_bytes(
    haystack: &[u8],
    searcher: &AsciiCaseFoldSearcher,
) -> Option<usize> {
    let literal_len = searcher.needle_len();
    if literal_len == 0 {
        return None;
    }
    let mut start = 0usize;
    while let Some(offset) = searcher.find(haystack, start) {
        if has_ascii_word_boundaries(haystack, offset, literal_len) {
            return Some(offset);
        }
        start = offset.saturating_add(1);
    }
    None
}

fn count_word_boundary_literal_occurrences_bytes(
    haystack: &[u8],
    literal: &[u8],
    finder: &memmem::Finder<'static>,
) -> usize {
    if literal.is_empty() {
        return 0;
    }
    finder
        .find_iter(haystack)
        .filter(|&offset| has_ascii_word_boundaries(haystack, offset, literal.len()))
        .count()
}

fn count_casefold_literal_occurrences_bytes(
    haystack: &[u8],
    searcher: &AsciiCaseFoldSearcher,
) -> usize {
    let mut count = 0usize;
    let mut start = 0usize;
    let needle_len = searcher.needle_len();
    while let Some(offset) = searcher.find(haystack, start) {
        count += 1;
        start = offset.saturating_add(needle_len.max(1));
    }
    count
}

fn count_casefold_word_boundary_literal_occurrences_bytes(
    haystack: &[u8],
    searcher: &AsciiCaseFoldSearcher,
) -> usize {
    let literal_len = searcher.needle_len();
    if literal_len == 0 {
        return 0;
    }
    let mut count = 0usize;
    let mut start = 0usize;
    while let Some(offset) = searcher.find(haystack, start) {
        if has_ascii_word_boundaries(haystack, offset, literal_len) {
            count += 1;
            start = offset.saturating_add(literal_len);
        } else {
            start = offset.saturating_add(1);
        }
    }
    count
}

fn count_literal_occurrences_bytes_in_range(
    haystack: &[u8],
    finder: &memmem::Finder<'static>,
    needle_len: usize,
    start: usize,
    end: usize,
) -> usize {
    if needle_len == 0 || start >= end {
        return 0;
    }

    let overlap = needle_len.saturating_sub(1);
    let slice_start = start.saturating_sub(overlap);
    let slice_end = haystack.len().min(end.saturating_add(overlap));

    finder
        .find_iter(&haystack[slice_start..slice_end])
        .map(|offset| slice_start + offset)
        .filter(|&absolute_start| absolute_start >= start && absolute_start < end)
        .count()
}

fn count_casefold_literal_occurrences_bytes_in_range(
    haystack: &[u8],
    searcher: &AsciiCaseFoldSearcher,
    start: usize,
    end: usize,
) -> usize {
    let literal_len = searcher.needle_len();
    if literal_len == 0 || start >= end {
        return 0;
    }

    let overlap = literal_len.saturating_sub(1);
    let slice_start = start.saturating_sub(overlap);
    let slice_end = haystack.len().min(end.saturating_add(overlap));
    let slice = &haystack[slice_start..slice_end];
    let mut count = 0usize;
    let mut local_start = 0usize;
    while let Some(relative_start) = searcher.find(slice, local_start) {
        let absolute_start = slice_start + relative_start;
        if absolute_start >= start && absolute_start < end {
            count += 1;
        }
        local_start = relative_start.saturating_add(literal_len);
    }
    count
}

fn count_word_boundary_literal_occurrences_bytes_in_range(
    haystack: &[u8],
    literal: &[u8],
    finder: &memmem::Finder<'static>,
    start: usize,
    end: usize,
) -> usize {
    if literal.is_empty() || start >= end {
        return 0;
    }

    let overlap = literal.len();
    let slice_start = start.saturating_sub(overlap);
    let slice_end = haystack.len().min(end.saturating_add(overlap));

    finder
        .find_iter(&haystack[slice_start..slice_end])
        .map(|offset| slice_start + offset)
        .filter(|&absolute_start| absolute_start >= start && absolute_start < end)
        .filter(|&absolute_start| {
            has_ascii_word_boundaries(haystack, absolute_start, literal.len())
        })
        .count()
}

fn count_casefold_word_boundary_literal_occurrences_bytes_in_range(
    haystack: &[u8],
    searcher: &AsciiCaseFoldSearcher,
    start: usize,
    end: usize,
) -> usize {
    let literal_len = searcher.needle_len();
    if literal_len == 0 || start >= end {
        return 0;
    }

    let overlap = literal_len;
    let slice_start = start.saturating_sub(overlap);
    let slice_end = haystack.len().min(end.saturating_add(overlap));
    let slice = &haystack[slice_start..slice_end];
    let mut count = 0usize;
    let mut local_start = 0usize;
    while let Some(relative_start) = searcher.find(slice, local_start) {
        let absolute_start = slice_start + relative_start;
        if absolute_start >= start
            && absolute_start < end
            && has_ascii_word_boundaries(haystack, absolute_start, literal_len)
        {
            count += 1;
            local_start = relative_start.saturating_add(literal_len);
        } else {
            local_start = relative_start.saturating_add(1);
        }
    }
    count
}

fn first_alternate_literal_match(
    haystack: &[u8],
    automaton: &AhoCorasick,
    start: usize,
) -> Option<usize> {
    if start >= haystack.len() {
        return None;
    }
    automaton
        .find(&haystack[start..])
        .map(|m| start + m.start())
}

fn count_alternate_literal_occurrences_bytes(haystack: &[u8], automaton: &AhoCorasick) -> usize {
    automaton.find_iter(haystack).count()
}

fn count_alternate_literal_occurrences_bytes_in_range(
    haystack: &[u8],
    automaton: &AhoCorasick,
    max_literal_len: usize,
    start: usize,
    end: usize,
) -> usize {
    if max_literal_len == 0 || start >= end {
        return 0;
    }

    let overlap = max_literal_len.saturating_sub(1);
    let slice_start = start.saturating_sub(overlap);
    let slice_end = haystack.len().min(end.saturating_add(overlap));

    automaton
        .find_iter(&haystack[slice_start..slice_end])
        .map(|m| slice_start + m.start())
        .filter(|&absolute_start| absolute_start >= start && absolute_start < end)
        .count()
}

#[cfg(test)]
mod tests {
    use super::{ExpressionPlan, Predicate, RegexFastPath};

    fn regex_fast_path(plan: &ExpressionPlan) -> Option<&RegexFastPath> {
        match plan.compiled.as_slice() {
            [Predicate::Regex { fast_path, .. }] => fast_path.as_ref(),
            _ => None,
        }
    }

    fn reject_fast_enabled(plan: &ExpressionPlan) -> bool {
        match regex_fast_path(plan) {
            Some(RegexFastPath::PlainLiteral { reject_fast, .. }) => reject_fast.is_some(),
            Some(RegexFastPath::AsciiCaseFoldLiteral { reject_fast, .. }) => {
                reject_fast.is_some()
            }
            Some(RegexFastPath::LiteralAlternates { reject_fast, .. }) => reject_fast.is_some(),
            _ => false,
        }
    }

    fn reject_fast_gate(plan: &ExpressionPlan) -> Option<&super::RejectFastGate> {
        match regex_fast_path(plan) {
            Some(RegexFastPath::PlainLiteral { reject_fast, .. }) => reject_fast.as_ref(),
            Some(RegexFastPath::AsciiCaseFoldLiteral { reject_fast, .. }) => {
                reject_fast.as_ref()
            }
            Some(RegexFastPath::LiteralAlternates { reject_fast, .. }) => reject_fast.as_ref(),
            _ => None,
        }
    }

    #[test]
    fn word_boundary_fast_path_counts_only_real_boundaries() {
        let plan = ExpressionPlan::parse(r"re:\bPM_RESUME\b").expect("plan should parse");
        let haystack = b"XPM_RESUME PM_RESUME PM_RESUME2 _PM_RESUME_";
        assert_eq!(plan.fast_match_count_no_hits_bytes(haystack), Some(1));
    }

    #[test]
    fn alternates_fast_path_matches_regex_semantics_for_prefix_alts() {
        let plan = ExpressionPlan::parse(r"re:(abc|ab)").expect("plan should parse");
        assert_eq!(
            plan.fast_match_count_no_hits_bytes(b"ab abc abcab"),
            Some(4)
        );
    }

    #[test]
    fn alternates_fast_path_respects_leftmost_first_order() {
        let plan = ExpressionPlan::parse(r"re:(a|aa)").expect("plan should parse");
        assert_eq!(plan.fast_match_count_no_hits_bytes(b"aa"), Some(2));
    }

    #[test]
    fn alternates_fast_path_supports_phrase_literals() {
        let plan =
            ExpressionPlan::parse(r"re:(Sherlock Holmes|John Watson)").expect("plan should parse");
        assert_eq!(
            plan.fast_match_count_no_hits_bytes(b"Sherlock Holmes + John Watson"),
            Some(2)
        );
    }

    #[test]
    fn case_insensitive_literal_fast_path_counts_ascii_matches() {
        let plan = ExpressionPlan::parse(r"re:(?i)sherlock holmes").expect("plan should parse");
        assert_eq!(
            plan.fast_match_count_no_hits_bytes(b"Sherlock Holmes sherlock HOLMES"),
            Some(2)
        );
    }

    #[test]
    fn case_insensitive_literal_fast_path_is_classified_for_ascii_literals() {
        let plan = ExpressionPlan::parse(r"re:(?i)sherlock holmes").expect("plan should parse");
        assert!(matches!(
            regex_fast_path(&plan),
            Some(RegexFastPath::AsciiCaseFoldLiteral { .. })
        ));
        assert!(reject_fast_enabled(&plan));
    }

    #[test]
    fn casefold_searcher_anchor_plan_prefers_informative_interior_byte() {
        let searcher =
            super::AsciiCaseFoldSearcher::new(b"Sherlock Holmes").expect("searcher should build");
        assert_eq!(
            searcher.anchor,
            Some(super::LiteralAnchorPlan {
                offset: 7,
                byte: b'k',
            })
        );
    }

    #[test]
    fn casefold_searcher_anchor_plan_is_deterministic() {
        let first =
            super::AsciiCaseFoldSearcher::new(b"Sherlock Holmes").expect("searcher should build");
        let second =
            super::AsciiCaseFoldSearcher::new(b"Sherlock Holmes").expect("searcher should build");
        assert_eq!(first.anchor, second.anchor);
    }

    #[test]
    fn casefold_searcher_anchor_plan_stays_disabled_for_short_literals() {
        let searcher = super::AsciiCaseFoldSearcher::new(b"abc").expect("searcher should build");
        assert_eq!(searcher.anchor, None);
    }

    #[test]
    fn casefold_searcher_anchor_plan_stays_disabled_for_low_information_needles() {
        let searcher = super::AsciiCaseFoldSearcher::new(b"AAAAAA").expect("searcher should build");
        assert_eq!(searcher.anchor, None);
    }

    #[test]
    fn casefold_searcher_anchor_path_skips_false_positive_anchor_hits() {
        let searcher =
            super::AsciiCaseFoldSearcher::new(b"Sherlock Holmes").expect("searcher should build");
        let haystack = b"XXXXXXXkzzzzSherlock Holmes";
        assert_eq!(searcher.find(haystack, 0), Some(12));
    }

    #[test]
    fn case_insensitive_word_boundary_fast_path_respects_boundaries() {
        let plan = ExpressionPlan::parse(r"re:(?i)\bpm_resume\b").expect("plan should parse");
        let haystack = b"xpm_resume PM_RESUME PM_RESUME2 _PM_RESUME_";
        assert_eq!(plan.fast_match_count_no_hits_bytes(haystack), Some(1));
    }

    #[test]
    fn case_insensitive_word_boundary_fast_path_is_classified_for_ascii_literals() {
        let plan = ExpressionPlan::parse(r"re:(?i)\bpm_resume\b").expect("plan should parse");
        assert!(matches!(
            regex_fast_path(&plan),
            Some(RegexFastPath::AsciiCaseFoldWordBoundaryLiteral { .. })
        ));
    }

    #[test]
    fn case_insensitive_fast_path_does_not_activate_for_non_ascii_literals() {
        let plan = ExpressionPlan::parse(r"re:(?i)Straße").expect("plan should parse");
        assert!(regex_fast_path(&plan).is_none());
    }

    #[test]
    fn reject_fast_does_not_activate_for_short_literals() {
        let plan = ExpressionPlan::parse(r"re:(?i)short").expect("plan should parse");
        assert!(!reject_fast_enabled(&plan));
    }

    #[test]
    fn alternates_fast_path_enables_reject_fast_for_long_ascii_literals() {
        let plan =
            ExpressionPlan::parse(r"re:(ERR_SYS|PME_TURN_OFF|LINK_REQ_RST|CFG_BME_EVT)")
                .expect("plan should parse");
        assert!(matches!(
            regex_fast_path(&plan),
            Some(RegexFastPath::LiteralAlternates { .. })
        ));
        assert!(reject_fast_enabled(&plan));
    }

    #[test]
    fn reject_fast_gate_accepts_real_casefold_prefix_window() {
        let plan = ExpressionPlan::parse(r"re:(?i)Sherlock Holmes").expect("plan should parse");
        let haystack = b"prefix SHERLOCK HOLMES suffix";
        let gate = reject_fast_gate(&plan).expect("reject-fast gate should exist");
        assert!(gate.could_match_anywhere(haystack));
    }

    #[test]
    fn reject_fast_gate_rejects_non_matching_alternates_text() {
        let plan = ExpressionPlan::parse(r"re:(ERR_SYS|PME_TURN_OFF|LINK_REQ_RST|CFG_BME_EVT)")
            .expect("plan should parse");
        let gate = reject_fast_gate(&plan).expect("reject-fast gate should exist");
        let haystack = b"kernel warnings without tracked prefixes";
        assert!(!gate.could_match_anywhere(haystack));
        assert!(!gate.could_match_with_start_in_range(haystack, 0, haystack.len()));
    }

    #[test]
    fn literal_count_uses_cached_finder() {
        let plan = ExpressionPlan::parse("lit:ERR").expect("plan should parse");
        assert_eq!(
            plan.fast_match_count_no_hits_bytes(b"ERR nope ERR"),
            Some(2)
        );
    }

    #[test]
    fn literal_range_count_matches_full_count() {
        let plan = ExpressionPlan::parse("lit:abc").expect("plan should parse");
        let haystack = b"xxabcabcxxabc";
        let ranged = [
            plan.fast_match_count_no_hits_bytes_in_range(haystack, 0, 4),
            plan.fast_match_count_no_hits_bytes_in_range(haystack, 4, 9),
            plan.fast_match_count_no_hits_bytes_in_range(haystack, 9, haystack.len()),
        ]
        .into_iter()
        .map(|count| count.expect("range count should use literal fast path"))
        .sum::<usize>();

        assert_eq!(
            ranged,
            plan.fast_match_count_no_hits_bytes(haystack)
                .unwrap_or_default()
        );
    }

    #[test]
    fn word_boundary_range_count_matches_full_count() {
        let plan = ExpressionPlan::parse(r"re:\bPM_RESUME\b").expect("plan should parse");
        let haystack = b"PM_RESUME xPM_RESUME PM_RESUME PM_RESUME2 PM_RESUME";
        let ranged = [
            plan.fast_match_count_no_hits_bytes_in_range(haystack, 0, 11),
            plan.fast_match_count_no_hits_bytes_in_range(haystack, 11, 33),
            plan.fast_match_count_no_hits_bytes_in_range(haystack, 33, haystack.len()),
        ]
        .into_iter()
        .map(|count| count.expect("range count should use word-boundary fast path"))
        .sum::<usize>();

        assert_eq!(
            ranged,
            plan.fast_match_count_no_hits_bytes(haystack)
                .unwrap_or_default()
        );
    }

    #[test]
    fn case_insensitive_literal_range_count_matches_full_count() {
        let plan = ExpressionPlan::parse(r"re:(?i)sherlock").expect("plan should parse");
        let haystack = b"xxSherLock sherLOCKxxSHERLOCK";
        let ranged = [
            plan.fast_match_count_no_hits_bytes_in_range(haystack, 0, 10),
            plan.fast_match_count_no_hits_bytes_in_range(haystack, 10, 22),
            plan.fast_match_count_no_hits_bytes_in_range(haystack, 22, haystack.len()),
        ]
        .into_iter()
        .map(|count| count.expect("range count should use case-insensitive literal fast path"))
        .sum::<usize>();

        assert_eq!(
            ranged,
            plan.fast_match_count_no_hits_bytes(haystack)
                .unwrap_or_default()
        );
    }

    #[test]
    fn case_insensitive_word_boundary_range_count_matches_full_count() {
        let plan = ExpressionPlan::parse(r"re:(?i)\bpm_resume\b").expect("plan should parse");
        let haystack = b"PM_RESUME xpm_resume PM_RESUME PM_RESUME2 pm_resume";
        let ranged = [
            plan.fast_match_count_no_hits_bytes_in_range(haystack, 0, 12),
            plan.fast_match_count_no_hits_bytes_in_range(haystack, 12, 34),
            plan.fast_match_count_no_hits_bytes_in_range(haystack, 34, haystack.len()),
        ]
        .into_iter()
        .map(|count| {
            count.expect("range count should use case-insensitive word-boundary fast path")
        })
        .sum::<usize>();

        assert_eq!(
            ranged,
            plan.fast_match_count_no_hits_bytes(haystack)
                .unwrap_or_default()
        );
    }

    #[test]
    fn alternates_range_count_matches_full_count() {
        let plan = ExpressionPlan::parse(r"re:(abc|wxyz)").expect("plan should parse");
        let haystack = b"abc---wxyz---abcwxyz---wxyz";
        let ranged = [
            plan.fast_match_count_no_hits_bytes_in_range(haystack, 0, 8),
            plan.fast_match_count_no_hits_bytes_in_range(haystack, 8, 18),
            plan.fast_match_count_no_hits_bytes_in_range(haystack, 18, haystack.len()),
        ]
        .into_iter()
        .map(|count| count.expect("range count should use alternates fast path"))
        .sum::<usize>();

        assert_eq!(
            ranged,
            plan.fast_match_count_no_hits_bytes(haystack)
                .unwrap_or_default()
        );
    }
}
