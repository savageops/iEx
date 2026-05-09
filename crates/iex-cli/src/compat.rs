//! Rg-shaped argv normalization for agents that call IX through ripgrep-shaped
//! muscle memory.
//!
//! Boundary contract:
//! - canonical IX CLI parsing runs first;
//! - the accepted compatibility subset is intentionally narrow;
//! - accepted input lowers into canonical [`SearchArgs`];
//! - unsupported input fails with guided syntax instead of emulating ripgrep;
//! - this module must not own file scanning, search execution, or report output.
//!
//! New capability belongs in the canonical command/search owners first. This
//! layer may only translate argv into that capability after it exists.

use std::{ffi::OsString, path::PathBuf};

use anyhow::{bail, Result};
use clap::{error::ErrorKind, CommandFactory, Parser};
use regex::escape as regex_escape;

use crate::{search::SearchArgs, Cli, Command};

pub enum Invocation {
    Canonical(Command),
    Compat(SearchArgs),
}

#[derive(Parser, Debug)]
#[command(
    name = "ix",
    no_binary_name = true,
    disable_help_flag = true,
    disable_help_subcommand = true,
    disable_version_flag = true
)]
struct CompatSearchArgs {
    #[arg(short = 'e', long = "regexp", value_name = "PATTERN")]
    regexps: Vec<String>,

    #[arg(short = 'F', long = "fixed-strings")]
    fixed_strings: bool,

    #[arg(short = 'i', long = "ignore-case")]
    ignore_case: bool,

    #[arg(short = 'j', long = "threads")]
    threads: Option<usize>,

    #[arg(short = 'n', long = "line-number")]
    line_number: bool,

    #[arg(long)]
    json: bool,

    #[arg(long)]
    hidden: bool,

    #[arg(value_name = "PATTERN_OR_PATH", num_args = 0..)]
    positionals: Vec<OsString>,
}

pub fn route_invocation(raw_args: Vec<OsString>) -> Result<Invocation> {
    match Cli::try_parse_from(raw_args.clone()) {
        Ok(cli) => Ok(Invocation::Canonical(cli.command)),
        Err(err) => {
            if should_preserve_canonical_parse(&raw_args) {
                err.exit();
            }
            let compat = try_parse_compat_args(&raw_args[1..])?;
            Ok(Invocation::Compat(compat.into_search_args()?))
        }
    }
}

fn should_preserve_canonical_parse(raw_args: &[OsString]) -> bool {
    let Some(first) = raw_args.get(1) else {
        return true;
    };

    matches!(
        first.to_string_lossy().as_ref(),
        "-h" | "--help" | "-V" | "--version" | "help"
    ) || Cli::command()
        .get_subcommands()
        .any(|subcommand| subcommand.get_name() == first.to_string_lossy())
}

fn try_parse_compat_args(raw_args: &[OsString]) -> Result<CompatSearchArgs> {
    CompatSearchArgs::try_parse_from(raw_args.iter().cloned()).map_err(|err| {
        let message = format_compat_parse_error(raw_args, err.kind());
        anyhow::anyhow!(message)
    })
}

fn format_compat_parse_error(raw_args: &[OsString], kind: ErrorKind) -> String {
    if let Some(flag) = first_unsupported_compat_flag(raw_args) {
        let supported = "`ix PATTERN [PATH]...`, `-e/--regexp`, `-F/--fixed-strings`, `-i/--ignore-case`, `-j/--threads`, `-n/--line-number`, `--json`, and `--hidden`";
        return format!(
            "rg-shaped compatibility translator does not support `{flag}`. Supported subset: {supported}. Use canonical `ix search <expr> [PATH]...` for native IX syntax."
        );
    }

    if matches!(
        kind,
        ErrorKind::MissingRequiredArgument | ErrorKind::TooFewValues
    ) || compat_patterns_missing(raw_args)
    {
        return "rg-shaped compatibility translator expects `ix PATTERN [PATH]...` or `ix -e <PATTERN> [PATH]...`.".to_owned();
    }

    let supported = "`ix PATTERN [PATH]...`, `-e/--regexp`, `-F/--fixed-strings`, `-i/--ignore-case`, `-j/--threads`, `-n/--line-number`, `--json`, and `--hidden`";
    format!(
        "rg-shaped compatibility translator could not classify this search request. Supported subset: {supported}. Use canonical `ix search <expr> [PATH]...` for native IX syntax."
    )
}

fn compat_patterns_missing(raw_args: &[OsString]) -> bool {
    let mut expects_value = false;
    for arg in raw_args {
        let text = arg.to_string_lossy();
        if expects_value {
            expects_value = false;
            continue;
        }
        if text == "-e" || text == "--regexp" || text == "-j" || text == "--threads" {
            expects_value = true;
            continue;
        }
        if !text.starts_with('-') {
            return false;
        }
    }
    true
}

fn first_unsupported_compat_flag(raw_args: &[OsString]) -> Option<String> {
    let mut expects_value = false;
    for arg in raw_args {
        let text = arg.to_string_lossy();
        if expects_value {
            expects_value = false;
            continue;
        }

        let flag = text.as_ref();
        if flag == "--" {
            return None;
        }
        if flag == "-e" || flag == "--regexp" || flag == "-j" || flag == "--threads" {
            expects_value = true;
            continue;
        }
        if matches!(
            flag,
            "-F" | "--fixed-strings"
                | "-i"
                | "--ignore-case"
                | "-n"
                | "--line-number"
                | "--json"
                | "--hidden"
        ) {
            continue;
        }
        if flag.starts_with("-j") && flag.len() > 2 {
            continue;
        }
        if flag.starts_with("--threads=") {
            continue;
        }
        if flag.starts_with("-e") && flag.len() > 2 {
            continue;
        }
        if flag.starts_with("--regexp=") {
            continue;
        }
        if flag.starts_with('-') {
            return Some(flag.to_owned());
        }
    }
    None
}

impl CompatSearchArgs {
    fn into_search_args(self) -> Result<SearchArgs> {
        let _ = self.line_number;

        let (patterns, path_args): (Vec<String>, Vec<OsString>) = if self.regexps.is_empty() {
            let (pattern, paths) = self.positionals.split_first().ok_or_else(|| {
                anyhow::anyhow!(
                    "rg-shaped compatibility translator expects `ix PATTERN [PATH]...` or `ix -e <PATTERN> [PATH]...`."
                )
            })?;
            (vec![os_string_to_string(pattern)?], paths.to_vec())
        } else {
            (self.regexps, self.positionals)
        };

        let expr = lower_compat_expression(&patterns, self.fixed_strings, self.ignore_case)?;
        let paths = if path_args.is_empty() {
            vec![PathBuf::from(".")]
        } else {
            path_args.into_iter().map(PathBuf::from).collect()
        };

        Ok(SearchArgs {
            expr,
            paths,
            hidden: self.hidden,
            follow_symlinks: false,
            json: self.json,
            stats_only: false,
            max_hits: None,
            threads: self.threads,
            emit_report: None,
        })
    }
}

fn os_string_to_string(value: &OsString) -> Result<String> {
    value.clone().into_string().map_err(|_| {
        anyhow::anyhow!("rg-shaped compatibility translator requires UTF-8 search patterns")
    })
}

fn lower_compat_expression(
    patterns: &[String],
    fixed_strings: bool,
    ignore_case: bool,
) -> Result<String> {
    if patterns.is_empty() {
        bail!("rg-shaped compatibility translator requires at least one search pattern");
    }

    if patterns.iter().any(|pattern| pattern.trim().is_empty()) {
        bail!("rg-shaped compatibility translator does not accept empty search patterns");
    }

    if patterns.len() == 1 && looks_like_explicit_iex_expression(&patterns[0]) {
        if fixed_strings || ignore_case {
            bail!(
                "native IX expressions cannot be combined with `-F` or `-i` in rg-shaped translator mode. Use canonical `ix search <expr> [PATH]...` instead."
            );
        }
        return Ok(patterns[0].trim().to_owned());
    }

    if !fixed_strings
        && patterns
            .iter()
            .any(|pattern| contains_ix_boolean_operator(pattern))
    {
        bail!(
            "regex patterns containing `&&` or `||` are ambiguous with native IX boolean operators. Use `ix search <expr> [PATH]...` for this pattern."
        );
    }

    let lowered: Vec<String> = patterns
        .iter()
        .map(|pattern| lower_compat_pattern(pattern, fixed_strings, ignore_case))
        .collect();
    Ok(lowered.join(" || "))
}

fn lower_compat_pattern(pattern: &str, fixed_strings: bool, ignore_case: bool) -> String {
    if fixed_strings {
        if ignore_case || contains_ix_boolean_operator(pattern) {
            let prefix = if ignore_case { "(?i)" } else { "" };
            return format!("re:{prefix}{}", escape_literal_for_ix_regex(pattern));
        }
        return format!("lit:{pattern}");
    }

    if ignore_case {
        return format!("re:(?i){pattern}");
    }

    format!("re:{pattern}")
}

fn escape_literal_for_ix_regex(pattern: &str) -> String {
    let mut escaped = String::new();
    let mut literal_run = String::new();

    for ch in pattern.chars() {
        if ch == '&' {
            if !literal_run.is_empty() {
                escaped.push_str(&regex_escape(&literal_run));
                literal_run.clear();
            }
            escaped.push_str(r"\x26");
        } else {
            literal_run.push(ch);
        }
    }

    if !literal_run.is_empty() {
        escaped.push_str(&regex_escape(&literal_run));
    }

    escaped
}

fn looks_like_explicit_iex_expression(pattern: &str) -> bool {
    let trimmed = pattern.trim();
    if trimmed.contains("||") {
        return trimmed
            .split("||")
            .map(str::trim)
            .all(is_explicit_iex_predicate);
    }
    if trimmed.contains("&&") {
        return trimmed
            .split("&&")
            .map(str::trim)
            .all(is_explicit_iex_predicate);
    }
    is_explicit_iex_predicate(trimmed)
}

fn is_explicit_iex_predicate(token: &str) -> bool {
    token.starts_with("lit:")
        || token.starts_with("re:")
        || token.starts_with("prefix:")
        || token.starts_with("suffix:")
}

fn contains_ix_boolean_operator(pattern: &str) -> bool {
    pattern.contains("&&") || pattern.contains("||")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compat_module_stays_translation_only() {
        let source = include_str!("compat.rs");
        let forbidden_tokens = [
            ["use ", "iex_core"].concat(),
            ["iex_core", "::"].concat(),
            ["run_", "search("].concat(),
            ["run_", "search_prepared"].concat(),
            ["Search", "Config"].concat(),
            ["Search", "Report"].concat(),
            ["print_", "search_summary"].concat(),
            ["std::", "fs"].concat(),
        ];

        for token in forbidden_tokens {
            assert!(
                !source.contains(&token),
                "compat layer must stay argv normalization only; found forbidden token `{token}`"
            );
        }
    }

    #[test]
    fn canonical_search_keeps_bare_expression_untranslated() {
        let invocation = route_invocation(vec![
            OsString::from("ix"),
            OsString::from("search"),
            OsString::from("a|b"),
            OsString::from("src"),
        ])
        .expect("canonical parse should succeed before compat routing");

        match invocation {
            Invocation::Canonical(Command::Search(args)) => {
                assert_eq!(args.expr, "a|b");
                assert_eq!(args.paths, vec![PathBuf::from("src")]);
            }
            _ => panic!("canonical search should bypass compatibility translator"),
        }
    }

    #[test]
    fn compat_bare_pattern_lowers_to_regex() {
        let compat = try_parse_compat_args(&[OsString::from("a|b"), OsString::from("src")])
            .expect("compat parse should succeed");
        let search = compat
            .into_search_args()
            .expect("compat lowering should succeed");

        assert_eq!(search.expr, "re:a|b");
        assert_eq!(search.paths, vec![PathBuf::from("src")]);
    }

    #[test]
    fn compat_preserves_native_expression_when_bare() {
        let compat = try_parse_compat_args(&[OsString::from("lit:timeout"), OsString::from("src")])
            .expect("compat parse should succeed");
        let search = compat
            .into_search_args()
            .expect("native expression should pass through");

        assert_eq!(search.expr, "lit:timeout");
    }

    #[test]
    fn compat_preserves_explicit_native_boolean_expression() {
        let compat = try_parse_compat_args(&[
            OsString::from("lit:IX && lit:Rust"),
            OsString::from("README.md"),
        ])
        .expect("compat parse should succeed");
        let search = compat
            .into_search_args()
            .expect("explicit native expression should pass through");

        assert_eq!(search.expr, "lit:IX && lit:Rust");
        assert_eq!(search.paths, vec![PathBuf::from("README.md")]);
    }

    #[test]
    fn compat_rejects_ambiguous_regex_boolean_operators() {
        let compat = try_parse_compat_args(&[
            OsString::from("-e"),
            OsString::from("IX&&Rust"),
            OsString::from("README.md"),
        ])
        .expect("compat parse should succeed");
        let error = compat
            .into_search_args()
            .expect_err("ambiguous rg-shaped regex should fail guided");
        let message = format!("{error:#}");

        assert!(message.contains("ambiguous with native IX boolean operators"));
        assert!(message.contains("ix search <expr> [PATH]..."));
    }

    #[test]
    fn compat_rejects_ambiguous_regex_alternation_operator_shape() {
        let compat = try_parse_compat_args(&[
            OsString::from("-e"),
            OsString::from("a||b"),
            OsString::from("README.md"),
        ])
        .expect("compat parse should succeed");
        let error = compat
            .into_search_args()
            .expect_err("ambiguous rg-shaped regex should fail guided");
        let message = format!("{error:#}");

        assert!(message.contains("ambiguous with native IX boolean operators"));
    }

    #[test]
    fn compat_fixed_string_boolean_operators_lower_to_safe_regex() {
        let compat = try_parse_compat_args(&[
            OsString::from("-F"),
            OsString::from("IX&&Rust"),
            OsString::from("README.md"),
        ])
        .expect("compat parse should succeed");
        let search = compat
            .into_search_args()
            .expect("fixed string boolean operators should lower safely");

        assert_eq!(search.expr, r"re:IX\x26\x26Rust");
        assert_eq!(search.paths, vec![PathBuf::from("README.md")]);
    }

    #[test]
    fn compat_lowering_supports_repeatable_regexp_flags() {
        let compat = try_parse_compat_args(&[
            OsString::from("-e"),
            OsString::from("timeout"),
            OsString::from("-e"),
            OsString::from("error"),
            OsString::from("src"),
        ])
        .expect("compat parse should succeed");
        let search = compat
            .into_search_args()
            .expect("compat lowering should succeed");

        assert_eq!(search.expr, "re:timeout || re:error");
        assert_eq!(search.paths, vec![PathBuf::from("src")]);
    }

    #[test]
    fn compat_lowering_supports_fixed_string_ignore_case() {
        let compat = try_parse_compat_args(&[
            OsString::from("-F"),
            OsString::from("-i"),
            OsString::from("Timeout.*"),
        ])
        .expect("compat parse should succeed");
        let search = compat
            .into_search_args()
            .expect("compat lowering should succeed");

        assert_eq!(search.expr, r"re:(?i)Timeout\.\*");
        assert_eq!(search.paths, vec![PathBuf::from(".")]);
    }

    #[test]
    fn compat_accepts_json_hidden_threads_and_line_number() {
        let compat = try_parse_compat_args(&[
            OsString::from("--json"),
            OsString::from("--hidden"),
            OsString::from("-n"),
            OsString::from("-j"),
            OsString::from("4"),
            OsString::from("timeout"),
        ])
        .expect("compat parse should succeed");
        let search = compat
            .into_search_args()
            .expect("compat lowering should succeed");

        assert!(search.json);
        assert!(search.hidden);
        assert_eq!(search.threads, Some(4));
        assert_eq!(search.expr, "re:timeout");
    }

    #[test]
    fn compat_reports_guided_unsupported_flags() {
        let error = try_parse_compat_args(&[OsString::from("--files")])
            .expect_err("unsupported flags should fail");
        let message = format!("{error:#}");

        assert!(message.contains("`--files`"));
        assert!(message.contains("Supported subset"));
        assert!(message.contains("ix search <expr> [PATH]..."));
    }

    #[test]
    fn compat_detects_known_subcommands_without_hardcoding() {
        assert!(should_preserve_canonical_parse(&[
            OsString::from("ix"),
            OsString::from("search"),
        ]));
        assert!(should_preserve_canonical_parse(&[
            OsString::from("ix"),
            OsString::from("inspect"),
        ]));
        assert!(!should_preserve_canonical_parse(&[
            OsString::from("ix"),
            OsString::from("timeout"),
        ]));
    }
}
