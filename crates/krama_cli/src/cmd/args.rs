use anyhow::Result;
use clap::Parser as ClapParser;

use super::{repl::Repl, run::Run, test::Test};

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
  Run(Run),
  Test(Test),
  Check(Run),
}

impl Command {
  async fn execute(self) -> Result<()> {
    match self {
      Command::Run(run) => run.execute(true).await,
      Command::Test(test) => test.execute().await,
      Command::Check(check) => check.execute(false).await,
    }
  }
}
