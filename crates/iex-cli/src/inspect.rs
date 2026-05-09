use std::{
    collections::{BTreeMap, BTreeSet},
    io::Write,
    path::PathBuf,
};

use anyhow::{bail, Context, Result};
use clap::{Args, ValueEnum};
use iex_core::{
    inspect_window, run_search, ExpressionPlan, InspectLine, InspectWindowReport,
    InspectWindowRequest, SearchConfig,
};
use serde_json::json;

use crate::agent_output::{json_string, normalize_display_path, render_next_v1};

const DEFAULT_FILE_WINDOW_LIMIT: usize = 240;

const INSPECT_AFTER_HELP: &str = "CONTRACT
  ix inspect is read-only: no mutation, replacement, shell delegation, or sed delegation
  file-window mode reads bounded UTF-8 line windows from explicit PATH arguments
  omitted file-window bounds default to a 240-line first window; eof=true means whole small file
  match-context mode uses --expr and the same search engine as ix search
AGENT OUTPUT
  grouped output emits ix.inspect.* sentinels with request/eof metadata and ix.next.v1 argv hints
  --format records preserves path:line:text output for pipe consumers
  --json / --format json emits the structured report contract
CONTINUATION
  limit-shaped reads continue with --start-line next --limit N
  range-shaped reads continue with --range next:next+span-1
  eof=true suppresses continuation and carries total_lines when the file horizon is known
SNIPS
  ix inspect src/main.rs
  ix inspect src/main.rs --total-count 40
  ix inspect src/main.rs --skip 120 --limit 30
  ix inspect src/main.rs --range 40:80
  ix inspect --expr 'lit:SearchConfig' crates --context 2 --json";

#[derive(Args, Debug, Clone)]
#[command(after_help = INSPECT_AFTER_HELP)]
pub struct InspectArgs {
    #[arg(value_name = "PATH", num_args = 1.., help = "Files or search roots")]
    pub paths: Vec<PathBuf>,

    #[arg(long = "total-count", alias = "head", help = "Emit first N lines")]
    pub total_count: Option<usize>,

    #[arg(long, help = "Skip N lines before emitting")]
    pub skip: Option<usize>,

    #[arg(long, help = "Emit at most N lines")]
    pub limit: Option<usize>,

    #[arg(long = "start-line", help = "Inclusive 1-based start line")]
    pub start_line: Option<usize>,

    #[arg(long = "end-line", help = "Inclusive 1-based end line")]
    pub end_line: Option<usize>,

    #[arg(long, value_parser = parse_line_range, help = "Inclusive START:END line range")]
    pub range: Option<LineRangeArg>,

    #[arg(
        long,
        help = "Allow explicit full/tail file output beyond the default window"
    )]
    pub all: bool,

    #[arg(long, help = "Emit JSON")]
    pub json: bool,

    #[arg(
        long = "format",
        value_enum,
        default_value_t = InspectOutputFormat::Grouped,
        help = "Output format: grouped, records, or json"
    )]
    pub format: InspectOutputFormat,

    #[arg(
        long,
        value_name = "EXPR",
        help = "IX expression for match-context mode"
    )]
    pub expr: Option<String>,

    #[arg(
        short = 'C',
        long = "context",
        help = "Lines before and after each match"
    )]
    pub context: Option<usize>,

    #[arg(short = 'B', long = "before-context", help = "Lines before each match")]
    pub before_context: Option<usize>,

    #[arg(short = 'A', long = "after-context", help = "Lines after each match")]
    pub after_context: Option<usize>,

    #[arg(long)]
    pub hidden: bool,

    #[arg(long)]
    pub follow_symlinks: bool,

    #[arg(short, long)]
    pub threads: Option<usize>,

    #[arg(long)]
    pub max_hits: Option<usize>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LineRangeArg {
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum InspectOutputFormat {
    Grouped,
    Records,
    Json,
}

pub fn run_inspect_command(args: InspectArgs) -> Result<()> {
    if args.expr.is_some() {
        run_match_context(args)
    } else {
        run_file_windows(args)
    }
}

fn run_file_windows(args: InspectArgs) -> Result<()> {
    if args.has_context_flags() {
        bail!("match context requires --expr so IX search owns hit discovery");
    }

    let reports: Vec<InspectWindowReport> = args
        .paths
        .iter()
        .map(|path| inspect_window(&window_request_for_path(&args, path.clone())?))
        .collect::<Result<Vec<_>>>()?;

    match args.output_format() {
        InspectOutputFormat::Json => {
            let stdout = std::io::stdout();
            let mut handle = stdout.lock();
            serde_json::to_writer(&mut handle, &json!({ "reports": reports }))?;
            handle.write_all(b"\n")?;
            return Ok(());
        }
        InspectOutputFormat::Grouped => {
            for report in &reports {
                print_grouped_window_report(report);
            }
        }
        InspectOutputFormat::Records => {
            for report in &reports {
                print_window_records(report);
            }
        }
    }
    Ok(())
}

fn run_match_context(args: InspectArgs) -> Result<()> {
    if args.range.is_some()
        || args.total_count.is_some()
        || args.skip.is_some()
        || args.limit.is_some()
        || args.start_line.is_some()
        || args.end_line.is_some()
        || args.all
    {
        bail!("match-context inspection accepts --expr with --context/--before-context/--after-context, not file-window bounds");
    }

    let expr = args
        .expr
        .clone()
        .expect("expr mode is checked before context inspection");
    let plan = ExpressionPlan::parse(&expr)?;
    let mut config = SearchConfig::from_roots(args.paths.clone(), plan);
    config.include_hidden = args.hidden;
    config.follow_symlinks = args.follow_symlinks;
    config.max_hits = args.max_hits;
    config.threads = args.threads;
    config.collect_hits = true;

    let report = run_search(&config).context("inspect context search failed")?;
    let before = args.before_context.or(args.context).unwrap_or(0);
    let after = args.after_context.or(args.context).unwrap_or(0);
    let context = build_context_reports(&report.hits, before, after)?;

    match args.output_format() {
        InspectOutputFormat::Json => {
            let stdout = std::io::stdout();
            let mut handle = stdout.lock();
            serde_json::to_writer(
                &mut handle,
                &json!({
                    "expression": report.expression,
                    "reports": context.iter().map(ContextFileReport::to_json).collect::<Vec<_>>()
                }),
            )?;
            handle.write_all(b"\n")?;
            return Ok(());
        }
        InspectOutputFormat::Grouped => {
            for file in &context {
                print_grouped_context_report(file);
            }
        }
        InspectOutputFormat::Records => {
            for file in &context {
                print_context_records(file);
            }
        }
    }
    Ok(())
}

fn print_grouped_window_report(report: &InspectWindowReport) {
    println!("{}", grouped_window_header(report));
    if report.lines.is_empty() {
        return;
    }

    print_grouped_lines(
        report
            .lines
            .iter()
            .map(|line| (line.line, None, line.text.as_str())),
    );
    if let Some(argv) = next_window_argv(report) {
        println!("{}", render_next_v1(&argv));
    }
}

fn grouped_window_header(report: &InspectWindowReport) -> String {
    let mut header = format!(
        "== ix.inspect.file path={} request={} ",
        json_string(&normalize_display_path(&report.path)),
        requested_window_label(report)
    );
    if let (Some(start), Some(end)) = (
        report.lines.first().map(|line| line.line),
        report.lines.last().map(|line| line.line),
    ) {
        header.push_str(&format!("range={start}:{end} "));
    }
    header.push_str(&format!(
        "emitted={} eof={}",
        report.total_emitted_lines, report.eof
    ));
    if let Some(total_lines) = report.total_lines {
        header.push_str(&format!(" total_lines={total_lines}"));
    }
    header.push_str(" ==");
    header
}

fn requested_window_label(report: &InspectWindowReport) -> String {
    if let Some(end_line) = report.requested.end_line {
        return format!("{}:{end_line}", report.requested.start_line);
    }
    if let Some(limit) = report.requested.limit {
        return format!("{}:+{limit}", report.requested.start_line);
    }
    format!("{}:*", report.requested.start_line)
}

fn print_grouped_context_report(report: &ContextFileReport) {
    if report.lines.is_empty() {
        println!(
            "== ix.inspect.context path={} emitted=0 ==",
            json_string(&normalize_display_path(&report.path))
        );
        return;
    }

    println!(
        "== ix.inspect.context path={} emitted={} ==",
        json_string(&normalize_display_path(&report.path)),
        report.lines.len()
    );
    print_grouped_lines(
        report
            .lines
            .iter()
            .map(|line| (line.line, Some(line.role), line.text.as_str())),
    );
}

fn print_grouped_lines<'a>(lines: impl Iterator<Item = (usize, Option<&'a str>, &'a str)>) {
    let lines: Vec<_> = lines.collect();
    let width = lines
        .iter()
        .map(|(line, _, _)| line.to_string().len())
        .max()
        .unwrap_or(1);
    for (line, role, text) in lines {
        if let Some(role) = role {
            println!("{line:>width$} {role:<7} | {text}");
        } else {
            println!("{line:>width$} | {text}");
        }
    }
}

fn next_window_argv(report: &InspectWindowReport) -> Option<Vec<String>> {
    if report.lines.is_empty() || report.requested.allow_full || report.eof {
        return None;
    }
    let last_line = report.lines.last()?.line;
    if let Some(end_line) = report.requested.end_line {
        let span = end_line.saturating_sub(report.requested.start_line) + 1;
        if report.total_emitted_lines < span {
            return None;
        }
        return Some(vec![
            "ix".to_owned(),
            "inspect".to_owned(),
            normalize_display_path(&report.path),
            "--range".to_owned(),
            format!("{}:{}", last_line + 1, last_line + span),
        ]);
    }
    let limit = report.requested.limit?;
    if limit == 0 || report.total_emitted_lines < limit {
        return None;
    }
    Some(vec![
        "ix".to_owned(),
        "inspect".to_owned(),
        report.path.clone(),
        "--start-line".to_owned(),
        (last_line + 1).to_string(),
        "--limit".to_owned(),
        limit.to_string(),
    ])
}

fn print_window_records(report: &InspectWindowReport) {
    for line in &report.lines {
        println!("{}:{}:{}", report.path, line.line, line.text);
    }
}

fn print_context_records(report: &ContextFileReport) {
    for line in &report.lines {
        println!("{}:{}:{}:{}", report.path, line.line, line.role, line.text);
    }
}

fn window_request_for_path(args: &InspectArgs, path: PathBuf) -> Result<InspectWindowRequest> {
    if args.range.is_some()
        && (args.total_count.is_some()
            || args.skip.is_some()
            || args.limit.is_some()
            || args.start_line.is_some()
            || args.end_line.is_some()
            || args.all)
    {
        bail!("--range cannot be combined with other file-window bounds");
    }
    if args.total_count.is_some()
        && (args.skip.is_some()
            || args.limit.is_some()
            || args.start_line.is_some()
            || args.end_line.is_some()
            || args.all)
    {
        bail!("--total-count cannot be combined with other file-window bounds");
    }

    if let Some(range) = args.range {
        return Ok(InspectWindowRequest::range(path, range.start, range.end));
    }
    if let Some(total_count) = args.total_count {
        return Ok(InspectWindowRequest::first(path, total_count));
    }

    Ok(InspectWindowRequest {
        path,
        start_line: args.start_line,
        end_line: args.end_line,
        skip: args.skip.unwrap_or(0),
        limit: implicit_or_explicit_limit(args),
        allow_full: args.all,
    })
}

fn implicit_or_explicit_limit(args: &InspectArgs) -> Option<usize> {
    if args.limit.is_some() || args.end_line.is_some() || args.all {
        args.limit
    } else {
        Some(DEFAULT_FILE_WINDOW_LIMIT)
    }
}

fn build_context_reports(
    hits: &[iex_core::SearchHit],
    before: usize,
    after: usize,
) -> Result<Vec<ContextFileReport>> {
    let mut by_path: BTreeMap<String, BTreeSet<usize>> = BTreeMap::new();
    for hit in hits {
        by_path
            .entry(hit.path.clone())
            .or_default()
            .insert(hit.line);
    }

    let mut files = Vec::new();
    for (path, match_lines) in by_path {
        let ranges = merge_ranges(
            match_lines
                .iter()
                .map(|line| (line.saturating_sub(before).max(1), line + after))
                .collect(),
        );
        let mut lines = Vec::new();
        for (start, end) in ranges {
            let report = inspect_window(&InspectWindowRequest::range(&path, start, end))?;
            for line in report.lines {
                let role = if match_lines.contains(&line.line) {
                    "match"
                } else {
                    "context"
                };
                lines.push(ContextLine {
                    line: line.line,
                    text: line.text,
                    role,
                });
            }
        }
        files.push(ContextFileReport { path, lines });
    }
    Ok(files)
}

fn merge_ranges(mut ranges: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    ranges.sort_unstable();
    let mut merged: Vec<(usize, usize)> = Vec::new();
    for (start, end) in ranges {
        let Some(last) = merged.last_mut() else {
            merged.push((start, end));
            continue;
        };
        if start <= last.1 + 1 {
            last.1 = last.1.max(end);
        } else {
            merged.push((start, end));
        }
    }
    merged
}

fn parse_line_range(value: &str) -> Result<LineRangeArg, String> {
    let Some((start, end)) = value.split_once(':') else {
        return Err("range must use START:END".to_owned());
    };
    let start = start
        .parse::<usize>()
        .map_err(|_| "range start must be a positive integer".to_owned())?;
    let end = end
        .parse::<usize>()
        .map_err(|_| "range end must be a positive integer".to_owned())?;
    if start == 0 || end == 0 {
        return Err("range lines must be greater than zero".to_owned());
    }
    if start > end {
        return Err("range start must be less than or equal to end".to_owned());
    }
    Ok(LineRangeArg { start, end })
}

impl InspectArgs {
    fn output_format(&self) -> InspectOutputFormat {
        if self.json {
            InspectOutputFormat::Json
        } else {
            self.format
        }
    }

    fn has_context_flags(&self) -> bool {
        self.context.is_some() || self.before_context.is_some() || self.after_context.is_some()
    }
}

struct ContextFileReport {
    path: String,
    lines: Vec<ContextLine>,
}

impl ContextFileReport {
    fn to_json(&self) -> serde_json::Value {
        json!({
            "path": self.path,
            "lines": self.lines.iter().map(ContextLine::to_json).collect::<Vec<_>>()
        })
    }
}

struct ContextLine {
    line: usize,
    text: String,
    role: &'static str,
}

impl ContextLine {
    fn to_json(&self) -> serde_json::Value {
        json!({
            "line": self.line,
            "text": self.text,
            "role": self.role
        })
    }
}

#[allow(dead_code)]
fn _assert_core_line_type_is_owned(_: &InspectLine) {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Cli;
    use clap::Parser;

    #[test]
    fn parses_range_argument() {
        let range = parse_line_range("3:9").unwrap();

        assert_eq!(range, LineRangeArg { start: 3, end: 9 });
    }

    #[test]
    fn rejects_inverted_range_argument() {
        let error = parse_line_range("9:3").unwrap_err();

        assert!(error.contains("less than or equal"));
    }

    #[test]
    fn routes_inspect_as_canonical_command() {
        let cli = Cli::try_parse_from(["ix", "inspect", "Cargo.toml", "--total-count", "2"])
            .expect("inspect command should parse");

        assert!(matches!(cli.command, crate::Command::Inspect(_)));
    }

    #[test]
    fn inspect_defaults_to_grouped_output() {
        let cli = Cli::try_parse_from(["ix", "inspect", "Cargo.toml", "--total-count", "2"])
            .expect("inspect command should parse");

        let crate::Command::Inspect(args) = cli.command else {
            panic!("inspect command should parse to inspect args");
        };
        assert_eq!(args.output_format(), InspectOutputFormat::Grouped);
    }

    #[test]
    fn inspect_accepts_record_format_for_compatibility() {
        let cli = Cli::try_parse_from([
            "ix",
            "inspect",
            "Cargo.toml",
            "--total-count",
            "2",
            "--format",
            "records",
        ])
        .expect("inspect command should parse");

        let crate::Command::Inspect(args) = cli.command else {
            panic!("inspect command should parse to inspect args");
        };
        assert_eq!(args.output_format(), InspectOutputFormat::Records);
    }

    #[test]
    fn inspect_json_flag_selects_json_format() {
        let cli = Cli::try_parse_from([
            "ix",
            "inspect",
            "Cargo.toml",
            "--total-count",
            "2",
            "--json",
        ])
        .expect("inspect command should parse");

        let crate::Command::Inspect(args) = cli.command else {
            panic!("inspect command should parse to inspect args");
        };
        assert_eq!(args.output_format(), InspectOutputFormat::Json);
    }

    #[test]
    fn inspect_rejects_mutation_flags_at_cli_boundary() {
        let err = Cli::try_parse_from(["ix", "inspect", "Cargo.toml", "--write"])
            .expect_err("inspect must not expose a mutation-shaped grammar");

        assert!(err.to_string().contains("unexpected argument"));
    }

    #[test]
    fn window_request_lowers_total_count() {
        let args = InspectArgs {
            paths: vec![PathBuf::from("Cargo.toml")],
            total_count: Some(5),
            skip: None,
            limit: None,
            start_line: None,
            end_line: None,
            range: None,
            all: false,
            json: false,
            format: InspectOutputFormat::Grouped,
            expr: None,
            context: None,
            before_context: None,
            after_context: None,
            hidden: false,
            follow_symlinks: false,
            threads: None,
            max_hits: None,
        };

        let request = window_request_for_path(&args, PathBuf::from("Cargo.toml")).unwrap();

        assert_eq!(request.start_line, Some(1));
        assert_eq!(request.limit, Some(5));
    }

    #[test]
    fn window_request_defaults_unbounded_reads_to_first_window() {
        let cli = Cli::try_parse_from(["ix", "inspect", "Cargo.toml"])
            .expect("inspect command should parse without explicit bounds");

        let crate::Command::Inspect(args) = cli.command else {
            panic!("inspect command should parse to inspect args");
        };
        let request = window_request_for_path(&args, PathBuf::from("Cargo.toml")).unwrap();

        assert_eq!(request.start_line, None);
        assert_eq!(request.limit, Some(DEFAULT_FILE_WINDOW_LIMIT));
        assert!(!request.allow_full);
    }

    #[test]
    fn window_request_defaults_start_line_to_safe_window() {
        let cli = Cli::try_parse_from(["ix", "inspect", "Cargo.toml", "--start-line", "40"])
            .expect("inspect command should parse with start-line only");

        let crate::Command::Inspect(args) = cli.command else {
            panic!("inspect command should parse to inspect args");
        };
        let request = window_request_for_path(&args, PathBuf::from("Cargo.toml")).unwrap();

        assert_eq!(request.start_line, Some(40));
        assert_eq!(request.limit, Some(DEFAULT_FILE_WINDOW_LIMIT));
        assert!(!request.allow_full);
    }

    #[test]
    fn window_request_preserves_all_as_explicit_full_read() {
        let cli = Cli::try_parse_from(["ix", "inspect", "Cargo.toml", "--all"])
            .expect("inspect command should parse with explicit all");

        let crate::Command::Inspect(args) = cli.command else {
            panic!("inspect command should parse to inspect args");
        };
        let request = window_request_for_path(&args, PathBuf::from("Cargo.toml")).unwrap();

        assert_eq!(request.limit, None);
        assert!(request.allow_full);
    }

    #[test]
    fn next_window_argv_advances_limited_windows() {
        let report = InspectWindowReport {
            path: "src/main.rs".to_owned(),
            requested: iex_core::InspectWindowBounds {
                start_line: 1,
                end_line: None,
                skip: 0,
                limit: Some(2),
                allow_full: false,
            },
            total_emitted_lines: 2,
            eof: false,
            total_lines: None,
            lines: vec![
                InspectLine {
                    line: 1,
                    text: "one".to_owned(),
                },
                InspectLine {
                    line: 2,
                    text: "two".to_owned(),
                },
            ],
        };

        assert_eq!(
            next_window_argv(&report),
            Some(vec![
                "ix".to_owned(),
                "inspect".to_owned(),
                "src/main.rs".to_owned(),
                "--start-line".to_owned(),
                "3".to_owned(),
                "--limit".to_owned(),
                "2".to_owned()
            ])
        );
    }

    #[test]
    fn next_window_argv_skips_partial_windows() {
        let report = InspectWindowReport {
            path: "src/main.rs".to_owned(),
            requested: iex_core::InspectWindowBounds {
                start_line: 1,
                end_line: None,
                skip: 0,
                limit: Some(4),
                allow_full: false,
            },
            total_emitted_lines: 2,
            eof: false,
            total_lines: None,
            lines: vec![
                InspectLine {
                    line: 1,
                    text: "one".to_owned(),
                },
                InspectLine {
                    line: 2,
                    text: "two".to_owned(),
                },
            ],
        };

        assert_eq!(next_window_argv(&report), None);
    }

    #[test]
    fn next_window_argv_skips_exact_full_window_at_eof() {
        let report = InspectWindowReport {
            path: "src/main.rs".to_owned(),
            requested: iex_core::InspectWindowBounds {
                start_line: 1,
                end_line: Some(2),
                skip: 0,
                limit: None,
                allow_full: false,
            },
            total_emitted_lines: 2,
            eof: true,
            total_lines: Some(2),
            lines: vec![
                InspectLine {
                    line: 1,
                    text: "one".to_owned(),
                },
                InspectLine {
                    line: 2,
                    text: "two".to_owned(),
                },
            ],
        };

        assert_eq!(next_window_argv(&report), None);
    }

    #[test]
    fn grouped_window_header_reports_empty_beyond_eof_horizon() {
        let report = InspectWindowReport {
            path: "docs\\note.md".to_owned(),
            requested: iex_core::InspectWindowBounds {
                start_line: 3600,
                end_line: Some(3700),
                skip: 0,
                limit: None,
                allow_full: false,
            },
            total_emitted_lines: 0,
            eof: true,
            total_lines: Some(3520),
            lines: Vec::new(),
        };

        let header = grouped_window_header(&report);

        assert!(header.contains("path=\"docs/note.md\""));
        assert!(header.contains("request=3600:3700"));
        assert!(header.contains("emitted=0"));
        assert!(header.contains("eof=true"));
        assert!(header.contains("total_lines=3520"));
    }

    #[test]
    fn merge_ranges_coalesces_overlapping_context() {
        assert_eq!(
            merge_ranges(vec![(1, 3), (3, 6), (9, 10)]),
            vec![(1, 6), (9, 10)]
        );
    }
}
