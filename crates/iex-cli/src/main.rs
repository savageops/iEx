use std::ffi::OsString;

use anyhow::Result;
use clap::{Parser, Subcommand};

mod agent_output;
mod compat;
mod explain;
mod inspect;
mod search;

use compat::Invocation;
use explain::{run_explain_command, ExplainArgs};
use inspect::{run_inspect_command, InspectArgs};
use search::{run_matches_command, run_search_command, SearchArgs};

const TOP_LEVEL_AFTER_HELP: &str = "SCHEMA
  ix search EXPR is the canonical IX command surface
  bare text in ix search is a literal substring, so a|b means the bytes \"a|b\"
  regex syntax requires re:pattern; literal alternation uses lit:a || lit:b
  expr: lit:text | re:pattern | prefix:x | suffix:x | A && B | A || B
COMPAT TRANSLATOR
  top-level ix PATTERN [PATH]... accepts a narrow rg-shaped subset for agents
  supported: PATTERN, -e PATTERN, repeated -e, -F, -i, -j, -n, --json, --hidden
  accepted input lowers into canonical IX search; unsupported flags fail guided
  raw regex patterns containing && or || are ambiguous and rejected
  use ix search <expr> [PATH]... for native IX boolean expressions
AGENT OUTPUT
  search prints one ix.result.v1 JSON sentinel unless --json is used
  zero-match search is status:\"ok\" with matches:0, not an error
  matches prints hit records only, no terminal result sentinel
  inspect grouped output prints ix.inspect.* sentinels and ix.next.v1 hints
  inspect without file bounds uses a bounded first window
SNIPS
  ix error src
  ix -e timeout -e error src
  ix -F -i 'session timeout' logs
  ix search 'lit:fn' crates --json
  ix search 're:TODO|FIXME' .
  ix search 'lit:TODO || lit:FIXME' .
  ix matches 're:TODO|FIXME' .
  ix inspect src/main.rs
  ix inspect src/main.rs --range 40:80
  ix inspect --expr 'lit:SearchConfig' crates --context 2 --json
  ix explain 'lit:breach && lit:auth'";

#[derive(Parser, Debug)]
#[command(
    name = "ix",
    about = "IX v2 intelligent expression toolkit",
    after_help = TOP_LEVEL_AFTER_HELP
)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Command {
    #[command(about = "Hit records plus terminal result state")]
    Search(SearchArgs),
    #[command(about = "Hit records only, same search engine")]
    Matches(SearchArgs),
    #[command(about = "Read-only file windows and match context")]
    Inspect(InspectArgs),
    #[command(about = "Expression plan JSON")]
    Explain(ExplainArgs),
}

fn main() {
    if let Err(err) = run() {
        eprintln!(
            "{}",
            agent_output::render_error_v1("command_failed", &format!("{err:#}"), None)
        );
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let raw_args: Vec<OsString> = std::env::args_os().collect();
    match compat::route_invocation(raw_args)? {
        Invocation::Canonical(command) => dispatch_command(command),
        Invocation::Compat(args) => run_search_command(args),
    }
}

fn dispatch_command(command: Command) -> Result<()> {
    match command {
        Command::Search(args) => run_search_command(args),
        Command::Matches(args) => run_matches_command(args),
        Command::Inspect(args) => run_inspect_command(args),
        Command::Explain(args) => run_explain_command(args),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn top_level_help_names_command_contracts() {
        let mut command = Cli::command();
        let mut help = Vec::new();
        command
            .write_long_help(&mut help)
            .expect("top-level help should render");
        let help = String::from_utf8(help).expect("help should be UTF-8");

        assert!(help.contains("search"));
        assert!(help.contains("Hit records plus terminal result state"));
        assert!(help.contains("matches"));
        assert!(help.contains("Hit records only, same search engine"));
        assert!(help.contains("inspect"));
        assert!(help.contains("Read-only file windows and match context"));
        assert!(help.contains("explain"));
        assert!(help.contains("Expression plan JSON"));
        assert!(help.contains("ix search EXPR is the canonical IX command surface"));
        assert!(help.contains("bare text in ix search is a literal substring"));
        assert!(help.contains("regex syntax requires re:pattern"));
        assert!(help.contains("top-level ix PATTERN [PATH]... accepts a narrow rg-shaped subset"));
        assert!(help.contains("supported: PATTERN, -e PATTERN, repeated -e, -F, -i, -j"));
        assert!(help.contains("raw regex patterns containing && or || are ambiguous and rejected"));
        assert!(help.contains("expr: lit:text | re:pattern | prefix:x | suffix:x"));
        assert!(help.contains("search prints one ix.result.v1 JSON sentinel"));
        assert!(help.contains("zero-match search is status"));
        assert!(help.contains("matches prints hit records only"));
        assert!(help.contains("no terminal result sentinel"));
        assert!(help.contains("inspect grouped output prints ix.inspect.* sentinels"));
        assert!(help.contains("ix.next.v1 hints"));
        assert!(help.contains("inspect without file bounds uses a bounded first window"));
        assert!(!help.contains(&["ix", ".search", ".summary"].concat()));
        assert!(!help.contains(&["ix", ".search", ".timings_ms"].concat()));
        assert!(!help.contains(&["ix", ".search", ".slowest"].concat()));
        assert!(!help.contains(&["matches", "_found", "=0"].concat()));
        assert!(help.contains("ix inspect src/main.rs"));
        assert!(help.contains("ix inspect src/main.rs --range 40:80"));
        assert!(help.contains("ix explain 'lit:breach && lit:auth'"));
    }

    #[test]
    fn subcommand_help_names_surface_specific_contracts() {
        let search_help = subcommand_help("search");
        assert!(search_help.contains("canonical native IX expression surface"));
        assert!(search_help.contains("translator regex patterns containing && or ||"));
        assert!(search_help.contains("ix.result.v1 JSON sentinel"));
        assert!(search_help.contains("matches:0"));
        assert!(!search_help.contains(&["ix", ".search", ".summary"].concat()));
        assert!(!search_help.contains(&["ix", ".search", ".timings_ms"].concat()));
        assert!(!search_help.contains(&["ix", ".search", ".slowest"].concat()));
        assert!(!search_help.contains(&["matches", "_found", "=0"].concat()));

        let matches_help = subcommand_help("matches");
        assert!(matches_help.contains("canonical native IX expression surface"));
        assert!(matches_help.contains("ix matches emits hit records only"));

        let inspect_help = subcommand_help("inspect");
        assert!(inspect_help.contains("ix inspect is read-only"));
        assert!(inspect_help.contains("omitted file-window bounds default"));
        assert!(inspect_help.contains("limit-shaped reads continue with --start-line"));
        assert!(inspect_help.contains("range-shaped reads continue with --range"));

        let explain_help = subcommand_help("explain");
        assert!(explain_help.contains("same native IX expression parser"));
        assert!(explain_help.contains("structured ExpressionPlan JSON"));
    }

    fn subcommand_help(name: &str) -> String {
        let mut command = Cli::command();
        let subcommand = command
            .find_subcommand_mut(name)
            .expect("subcommand should exist");
        let mut help = Vec::new();
        subcommand
            .write_long_help(&mut help)
            .expect("subcommand help should render");
        String::from_utf8(help).expect("help should be UTF-8")
    }
}
