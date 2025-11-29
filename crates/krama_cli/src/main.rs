use anyhow::Result;
use bumpalo::Bump;
use clap::Parser;
use krama_cli::cmd::args::Args;

#[tokio::main]
async fn main() -> Result<()> {
  let args = Args::parse();
  let mut arena = Bump::new();

  args.execute(&mut arena).await
}
