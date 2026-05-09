use std::io::Write;

use anyhow::Result;
use clap::Args;
use iex_core::ExpressionPlan;

const EXPLAIN_AFTER_HELP: &str = "EXPRESSION CONTRACT
  ix explain uses the same native IX expression parser as ix search and ix matches
  bare text is literal; regex requires re:pattern; boolean composition uses && and ||
  output is the structured ExpressionPlan JSON used to inspect lowering before execution
SNIPS
  ix explain 'lit:timeout'
  ix explain 're:TODO|FIXME'
  ix explain 'lit:error && re:\\btimeout\\b'";

#[derive(Args, Debug)]
#[command(after_help = EXPLAIN_AFTER_HELP)]
pub struct ExplainArgs {
    #[arg(help = "Expression to parse into an IX plan")]
    pub expr: String,
}

pub fn run_explain_command(args: ExplainArgs) -> Result<()> {
    let plan = ExpressionPlan::parse(&args.expr)?;
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    serde_json::to_writer(&mut handle, &plan)?;
    handle.write_all(b"\n")?;
    Ok(())
}
