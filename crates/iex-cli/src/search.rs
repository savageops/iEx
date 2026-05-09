use std::{fs, io::Write, path::PathBuf};

use anyhow::{Context, Result};
use clap::Args;
use iex_core::{run_search, ExpressionPlan, SearchConfig, SearchReport};

use crate::agent_output::render_search_result_v1;

const SEARCH_AFTER_HELP: &str = "EXPRESSION CONTRACT
  ix search EXPR and ix matches EXPR use the canonical native IX expression surface
  bare text is a literal substring: ix search 'a|b' searches for the bytes a|b
  regex alternation requires re:pattern: ix search 're:a|b' .
  literal alternation uses ||: ix search 'lit:a || lit:b' .
  top-level ix PATTERN [PATH]... is an rg-shaped translator into this surface
  translator regex patterns containing && or || are rejected as ambiguous
AGENT OUTPUT
  ix search emits hit records followed by one ix.result.v1 JSON sentinel
  zero-match search is status:\"ok\" with matches:0, not an error
  ix matches emits hit records only, no terminal result sentinel
  --json emits the structured SearchReport contract";

#[derive(Args, Debug, Clone)]
#[command(after_help = SEARCH_AFTER_HELP)]
pub struct SearchArgs {
    #[arg(help = "IX expression; bare text is literal, regex requires re:pattern")]
    pub expr: String,

    #[arg(
        value_name = "PATH",
        num_args = 0..,
        default_value = ".",
        help = "Files or directories to scan"
    )]
    pub paths: Vec<PathBuf>,

    #[arg(long)]
    pub hidden: bool,

    #[arg(long)]
    pub follow_symlinks: bool,

    #[arg(long)]
    pub json: bool,

    #[arg(long)]
    pub stats_only: bool,

    #[arg(long)]
    pub max_hits: Option<usize>,

    #[arg(short, long)]
    pub threads: Option<usize>,

    #[arg(long)]
    pub emit_report: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SearchRenderMode {
    Search,
    Matches,
}

pub fn run_search_command(args: SearchArgs) -> Result<()> {
    run_search_with_mode(args, SearchRenderMode::Search)
}

pub fn run_matches_command(args: SearchArgs) -> Result<()> {
    run_search_with_mode(args, SearchRenderMode::Matches)
}

fn run_search_with_mode(args: SearchArgs, mode: SearchRenderMode) -> Result<()> {
    let plan = ExpressionPlan::parse(&args.expr)?;
    let mut config = SearchConfig::from_roots(args.paths.clone(), plan.clone());
    config.include_hidden = args.hidden;
    config.follow_symlinks = args.follow_symlinks;
    config.max_hits = args.max_hits;
    config.threads = args.threads;
    config.collect_hits = !args.stats_only;

    let report = run_search(&config).context("search failed")?;

    if let Some(report_path) = args.emit_report {
        let json = serde_json::to_string_pretty(&report)?;
        if let Some(parent) = report_path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("failed to create report directory {}", parent.display())
            })?;
        }
        fs::write(&report_path, json)
            .with_context(|| format!("failed to write report to {}", report_path.display()))?;
    }

    if args.json {
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        serde_json::to_writer(&mut handle, &report)?;
        handle.write_all(b"\n")?;
        return Ok(());
    }

    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    write_search_text_output(&mut handle, &report, mode, args.stats_only)?;

    Ok(())
}

fn write_search_text_output<W: Write>(
    writer: &mut W,
    report: &SearchReport,
    mode: SearchRenderMode,
    stats_only: bool,
) -> Result<()> {
    if !stats_only {
        for hit in &report.hits {
            writeln!(
                writer,
                "{}:{}:{}:{}",
                hit.path, hit.line, hit.column, hit.preview
            )?;
        }
    }

    if mode == SearchRenderMode::Search {
        writeln!(writer, "{}", render_search_result_v1(report))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use iex_core::{
        stats::{SearchStats, SlowFileStat},
        SearchHit,
    };

    use super::*;

    #[test]
    fn search_text_output_emits_exactly_one_result_v1() {
        let report = sample_report(1);
        let mut output = Vec::new();

        write_search_text_output(&mut output, &report, SearchRenderMode::Search, false).unwrap();

        let output = String::from_utf8(output).unwrap();
        assert_eq!(output.matches("ix.result.v1").count(), 1);
        assert!(!output.contains(&["ix", ".search", ".summary"].concat()));
        assert!(!output.contains(&["ix", ".search", ".timings_ms"].concat()));
        assert!(!output.contains(&["ix", ".search", ".slowest"].concat()));
    }

    #[test]
    fn matches_text_output_emits_no_result_sentinel() {
        let report = sample_report(1);
        let mut output = Vec::new();

        write_search_text_output(&mut output, &report, SearchRenderMode::Matches, false).unwrap();

        let output = String::from_utf8(output).unwrap();
        assert_eq!(output.matches("ix.result.v1").count(), 0);
        assert!(output.contains("src/main.rs:7:3:needle"));
    }

    #[test]
    fn stats_only_search_still_emits_terminal_state() {
        let report = sample_report(0);
        let mut output = Vec::new();

        write_search_text_output(&mut output, &report, SearchRenderMode::Search, true).unwrap();

        let output = String::from_utf8(output).unwrap();
        assert_eq!(output.lines().count(), 1);
        assert!(output.contains(r#""status":"ok""#));
        assert!(output.contains(r#""matches":0"#));
    }

    fn sample_report(matches: usize) -> SearchReport {
        let mut stats = SearchStats::default();
        stats.matches_found = matches;
        stats.files_discovered = 1;
        stats.files_scanned = 1;
        stats.bytes_scanned = 64;
        stats.slowest_files.push(SlowFileStat {
            path: "src\\main.rs".to_owned(),
            duration_ms: 0.4,
            bytes: 64,
            linux_dominant_target: false,
        });

        SearchReport {
            expression: "lit:needle".to_owned(),
            hits: vec![SearchHit {
                path: "src/main.rs".to_owned(),
                line: 7,
                column: 3,
                preview: "needle".to_owned(),
            }],
            stats,
        }
    }
}
