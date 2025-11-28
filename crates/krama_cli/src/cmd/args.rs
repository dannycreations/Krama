use anyhow::Result;
use bumpalo::Bump;
use clap::Parser as ClapParser;

use super::{repl::Repl, run::Run, test::Test};

#[derive(ClapParser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
  #[clap(subcommand)]
  command: Option<Command>,
}

impl Args {
  pub async fn execute(self, arena: &mut Bump) -> Result<()> {
    match self.command {
      Some(command) => command.execute(arena).await,
      None => Repl.execute(arena).await,
    }
  }
}

#[derive(ClapParser)]
enum Command {
  Run(Run),
  Test(Test),
}

impl Command {
  async fn execute(self, arena: &mut Bump) -> Result<()> {
    match self {
      Command::Run(run) => run.execute(arena).await,
      Command::Test(test) => test.execute(arena).await,
    }
  }
}
