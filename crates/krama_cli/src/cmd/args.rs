use anyhow::Result;
use bumpalo::Bump;
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
    let mut arena = Bump::new();
    match self.command {
      Some(command) => command.execute(&mut arena).await,
      None => Repl.execute(&mut arena).await,
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
  async fn execute(self, arena: &mut Bump) -> Result<()> {
    match self {
      Command::Check(check) => check.execute(arena).await,
      Command::Run(run) => run.execute(arena).await,
      Command::Test(test) => test.execute(arena).await,
    }
  }
}
