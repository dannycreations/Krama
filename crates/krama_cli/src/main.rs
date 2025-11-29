use anyhow::Result;
use clap::Parser;
use krama_cli::cmd::args::Args;

#[tokio::main]
async fn main() -> Result<()> {
  let args = Args::parse();
  args.execute().await
}
