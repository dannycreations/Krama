use std::{
  env,
  io::Error,
  path::{Path, PathBuf},
};

use anyhow::{anyhow, Context, Result};
use bumpalo::Bump;
use clap::Parser;
use krama_runtime::{interpreter::Interpreter, testing::TestResult};
use tokio::fs;
use walkdir::WalkDir;

use crate::error::report_error;

#[derive(Parser)]
pub struct Test {
  #[clap(default_value = "src")]
  pub path: String,
}

fn find_test_files(path: &Path) -> Result<Vec<PathBuf>, Error> {
  let mut test_files = Vec::new();
  for entry in WalkDir::new(path) {
    let entry = entry?;
    let path = entry.path();
    if path.is_file() && path.to_string_lossy().ends_with("_test.km") {
      test_files.push(path.to_path_buf());
    }
  }
  Ok(test_files)
}

impl Test {
  pub async fn execute(&self, arena: &mut Bump) -> Result<()> {
    let mut passed = 0;
    let mut failed = 0;

    let root_path = env::current_dir()?;
    let mut path_buf = root_path;
    path_buf.push(&self.path);
    let path = path_buf.as_path();

    let test_files = find_test_files(path)?;

    for path_buf in test_files {
      arena.reset();
      let path = path_buf.as_path();
      let content = fs::read_to_string(&path).await?;
      let path_str = path.to_str().context("path is not valid UTF-8")?;

      println!("Running tests in {}", path.display());

      let content_in_arena = arena.alloc_str(&content);
      let path_in_arena = arena.alloc_str(path_str);
      let interpreter = Interpreter::new(arena, Some(path_in_arena));
      let program = interpreter
        .parse_and_resolve(content_in_arena)
        .map_err(|(kind, span)| anyhow!("Error: {}, Span: {:?}", kind, span))?;
      let results = interpreter.run_tests(&program.statements).await;
      for result in results {
        match result {
          TestResult::Success(name) => {
            println!("  test {} ... ok", name);
            passed += 1;
          }
          TestResult::Failure(name, (kind, span)) => {
            println!("  '{}'... failed", name);
            report_error(path_str, &content, span, kind);
            failed += 1;
          }
        }
      }
    }

    println!("\nTest results: {} passed, {} failed", passed, failed);
    Ok(())
  }
}
