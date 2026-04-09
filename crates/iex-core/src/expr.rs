use anyhow::{anyhow, Result};
use memchr::memmem;
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
    WordBoundaryLiteral {
        literal: Vec<u8>,
        finder: memmem::Finder<'static>,
    },
    LiteralAlternates {
        literals: Vec<Vec<u8>>,
        finders: Vec<memmem::Finder<'static>>,
    },
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
                Some(RegexFastPath::WordBoundaryLiteral { literal, finder }) => {
                    count_word_boundary_literal_occurrences_bytes(haystack, literal, finder)
                }
                Some(RegexFastPath::LiteralAlternates { literals, finders }) => {
                    count_alternate_literal_occurrences_bytes(haystack, literals, finders)
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
            Some(RegexFastPath::WordBoundaryLiteral { literal, finder }) => {
                first_word_boundary_literal_column_bytes(haystack, literal, finder).is_some()
            }
            Some(RegexFastPath::LiteralAlternates { literals, finders }) => {
                first_alternate_literal_match(haystack, literals, finders, 0).is_some()
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
            Some(RegexFastPath::WordBoundaryLiteral { literal, finder }) => {
                first_word_boundary_literal_column_bytes(haystack, literal, finder)
            }
            Some(RegexFastPath::LiteralAlternates { literals, finders }) => {
                first_alternate_literal_match(haystack, literals, finders, 0)
                    .map(|(offset, _)| offset)
            }
            None => bytes.find(haystack).map(|m| m.start()),
        },
    }
}

fn classify_regex_fast_path(pattern: &str) -> Option<RegexFastPath> {
    if let Some(literal) = parse_word_boundary_literal(pattern) {
        let bytes = literal.into_bytes();
        return Some(RegexFastPath::WordBoundaryLiteral {
            finder: owned_finder(&bytes),
            literal: bytes,
        });
    }

    let alternates = parse_literal_alternates(pattern)?;
    let literals: Vec<Vec<u8>> = alternates
        .into_iter()
        .map(|part| part.into_bytes())
        .collect();
    if literals.len() < 2 {
        return None;
    }
    let finders = literals
        .iter()
        .map(|literal| owned_finder(literal))
        .collect();
    Some(RegexFastPath::LiteralAlternates { literals, finders })
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
        if part.is_empty() || !is_ascii_identifier_literal(part) {
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

fn is_ascii_identifier_literal(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
}

fn owned_finder(needle: &[u8]) -> memmem::Finder<'static> {
    memmem::Finder::new(needle).into_owned()
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

fn first_alternate_literal_match(
    haystack: &[u8],
    literals: &[Vec<u8>],
    finders: &[memmem::Finder<'static>],
    start: usize,
) -> Option<(usize, usize)> {
    let mut best: Option<(usize, usize)> = None;
    for (index, finder) in finders.iter().enumerate() {
        if literals[index].is_empty() {
            continue;
        }
        let Some(relative) = finder.find(&haystack[start..]) else {
            continue;
        };
        let absolute = start + relative;
        match best {
            None => best = Some((absolute, index)),
            Some((best_offset, best_index)) => {
                if absolute < best_offset || (absolute == best_offset && index < best_index) {
                    best = Some((absolute, index));
                }
            }
        }
    }
    best
}

fn count_alternate_literal_occurrences_bytes(
    haystack: &[u8],
    literals: &[Vec<u8>],
    finders: &[memmem::Finder<'static>],
) -> usize {
    let mut count = 0usize;
    let mut cursor = 0usize;
    while cursor < haystack.len() {
        let Some((offset, index)) =
            first_alternate_literal_match(haystack, literals, finders, cursor)
        else {
            break;
        };
        count += 1;
        let advance = literals[index].len().max(1);
        cursor = offset.saturating_add(advance);
    }
    count
}

#[cfg(test)]
mod tests {
    use super::ExpressionPlan;

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
    fn literal_count_uses_cached_finder() {
        let plan = ExpressionPlan::parse("lit:ERR").expect("plan should parse");
        assert_eq!(
            plan.fast_match_count_no_hits_bytes(b"ERR nope ERR"),
            Some(2)
        );
    }
}
