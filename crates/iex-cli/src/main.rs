use std::ffi::OsString;

use anyhow::Result;
use clap::{Parser, Subcommand};

mod compat;
mod explain;
mod inspect;
mod search;

use compat::Invocation;
use explain::{run_explain_command, ExplainArgs};
use inspect::{run_inspect_command, InspectArgs};
use search::{run_matches_command, run_search_command, SearchArgs};

#[derive(Parser, Debug)]
#[command(
    name = "ix",
    about = "IX v2 intelligent expression toolkit",
    after_help = "SCHEMA\n  expr: lit:text | re:pattern | prefix:x | suffix:x | A && B | A || B\nSNIPS\n  ix error src\n  ix search 'lit:fn' crates --json\n  ix matches 're:TODO|FIXME' .\n  ix inspect src/main.rs --range 40:80\n  ix inspect --expr 'lit:SearchConfig' crates --context 2 --json"
)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Command {
    #[command(about = "Full search report and summary")]
    Search(SearchArgs),
    #[command(about = "Hit records only, same search engine")]
    Matches(SearchArgs),
    #[command(about = "Read-only file windows and match context")]
    Inspect(InspectArgs),
    #[command(about = "Expression plan JSON")]
    Explain(ExplainArgs),
}

fn main() -> Result<()> {
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
        assert!(help.contains("Full search report and summary"));
        assert!(help.contains("matches"));
        assert!(help.contains("Hit records only, same search engine"));
        assert!(help.contains("inspect"));
        assert!(help.contains("Read-only file windows and match context"));
        assert!(help.contains("explain"));
        assert!(help.contains("Expression plan JSON"));
        assert!(help.contains("expr: lit:text | re:pattern | prefix:x | suffix:x"));
        assert!(help.contains("ix inspect src/main.rs --range 40:80"));
    }
}
