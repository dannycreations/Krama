use anyhow::Result;
use bumpalo::Bump;
use clap::Parser as ClapParser;
use krama_cli::cmd::{repl::Repl, run::Run, test::Test};

#[derive(ClapParser)]
#[clap(author, version, about, long_about = None)]
struct Args {
  #[clap(subcommand)]
  command: Option<Command>,
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

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
  let args = Args::parse();
  let mut arena = Bump::new();

  let result = match args.command {
    Some(command) => command.execute(&mut arena).await,
    None => Repl.execute(&mut arena).await,
  };

  arena.reset();
  result
}
