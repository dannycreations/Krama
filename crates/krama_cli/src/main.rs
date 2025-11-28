use anyhow::Result;
use bumpalo::Bump;
use clap::Parser as ClapParser;
use krama_cli::cmd::{repl::Repl, run::Run, test::Test};
use tokio::runtime::Runtime;

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

fn main() -> Result<()> {
  let rt = Runtime::new()?;
  rt.block_on(async {
    let args = Args::parse();
    let mut arena = Bump::new();

    let result = match args.command {
      Some(command) => command.execute(&mut arena).await,
      None => Repl.execute(&mut arena).await,
    };

    arena.reset();
    result
  })
}
