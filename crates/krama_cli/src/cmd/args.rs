use anyhow::Result;
use clap::Parser as ClapParser;

use super::{check::Check, repl::Repl, run::Run, test::Test};

#[derive(ClapParser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
  #[clap(subcommand)]
  command: Option<Command>,
}

impl Args {
  pub async fn execute(self) -> Result<()> {
    match self.command {
      Some(command) => command.execute().await,
      None => Repl.execute().await,
    }
  }
}

#[derive(ClapParser)]
enum Command {
  Check(Check),
  Run(Run),
  Test(Test),
}

impl Command {
  async fn execute(self) -> Result<()> {
    match self {
      Command::Check(check) => check.execute().await,
      Command::Run(run) => run.execute().await,
      Command::Test(test) => test.execute().await,
    }
  }
}
