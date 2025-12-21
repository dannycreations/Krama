use anyhow::Result;
use clap::Parser as ClapParser;

mod repl;
mod run;
mod test;

pub use repl::*;
pub use run::*;
pub use test::*;

#[derive(ClapParser)]
#[clap(author, version, about, long_about = None)]
pub struct Command {
  #[clap(subcommand)]
  command: Option<CommandKind>,
}

impl Command {
  pub async fn execute(self) -> Result<()> {
    match self.command {
      Some(command) => command.execute().await,
      None => Repl.execute().await,
    }
  }
}

#[derive(ClapParser)]
enum CommandKind {
  Run(Run),
  Test(Test),
  Check(Run),
}

impl CommandKind {
  async fn execute(self) -> Result<()> {
    match self {
      CommandKind::Run(run) => run.execute(true).await,
      CommandKind::Test(test) => test.execute().await,
      CommandKind::Check(check) => check.execute(false).await,
    }
  }
}
