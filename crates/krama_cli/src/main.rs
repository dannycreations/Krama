use anyhow::Result;
use bumpalo::Bump;
use clap::Parser as ClapParser;
use krama_cli::cmd::{repl::Repl, CommandExecutor};

#[derive(ClapParser)]
#[clap(author, version, about, long_about = None)]
struct Args {
  #[clap(subcommand)]
  command: Option<Command>,
}

#[derive(ClapParser)]
enum Command {
  Run(krama_cli::cmd::run::Run),
  Test(krama_cli::cmd::test::Test),
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
  let args = Args::parse();
  let arena = Bump::new();

  let command: Box<dyn CommandExecutor> = match args.command {
    Some(Command::Run(run)) => Box::new(run),
    Some(Command::Test(test)) => Box::new(test),
    None => Box::new(Repl),
  };

  command.execute(&arena).await
}
