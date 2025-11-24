use anyhow::Result;
use bumpalo::Bump;
use clap::Parser as ClapParser;
use krama_cli::cmd::{self, repl::Repl};

#[derive(ClapParser)]
#[clap(author, version, about, long_about = None)]
struct Args {
  #[clap(subcommand)]
  command: Option<Command>,
}

#[derive(ClapParser)]
enum Command {
  Run(cmd::run::Run),
  Test(cmd::test::Test),
}

#[tokio::main]
async fn main() -> Result<()> {
  let args = Args::parse();
  let arena = Bump::new();

  match args.command {
    Some(Command::Run(run)) => run.execute(&arena).await?,
    Some(Command::Test(test)) => {
      let root_path = std::env::current_dir()?;
      let root_path = root_path.to_str().unwrap();
      test.execute(root_path).await?
    }
    None => Repl.execute(&arena).await?,
  }
  Ok(())
}
