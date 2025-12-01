use std::{
  env,
  path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use bumpalo::Bump;
use clap::Parser;
use krama_core::error::report_error;
use krama_runtime::{interpreter::Interpreter, testing::TestResult};
use tokio::fs;
use walkdir::WalkDir;

#[derive(Parser)]
pub struct Test {
  #[clap(default_value = "src")]
  pub path: String,
}

impl Test {
  pub async fn execute(&self) -> Result<()> {
    let mut total_passed = 0;
    let mut total_failed = 0;

    let root_path = env::current_dir()?;
    let mut path_buf = root_path;
    path_buf.push(&self.path);

    let test_files = Self::find_test_files(path_buf)?;

    for path_buf in test_files {
      println!("Running tests in {}", path_buf.display());
      let (passed, failed) = Self::run_tests_in_file(&path_buf).await?;
      total_passed += passed;
      total_failed += failed;
    }

    println!(
      "\nTest results: {} passed, {} failed",
      total_passed, total_failed
    );
    Ok(())
  }

  fn find_test_files(path: PathBuf) -> Result<Vec<PathBuf>> {
    let found_files = WalkDir::new(path)
      .into_iter()
      .filter_map(Result::ok)
      .filter(|e| {
        e.file_type().is_file()
          && e.path().to_string_lossy().ends_with("_test.km")
      })
      .map(|e| e.path().to_path_buf())
      .collect::<Vec<PathBuf>>();

    Ok(found_files)
  }

  async fn run_tests_in_file(path: &Path) -> Result<(usize, usize)> {
    let mut passed = 0;
    let mut failed = 0;

    let arena = Bump::new();
    let content = fs::read_to_string(path).await?;
    let path_str = path.to_str().context("path is not valid UTF-8")?;

    let content_in_arena = arena.alloc_str(&content);
    let path_in_arena = arena.alloc_str(path_str);
    let interpreter = Interpreter::new(&arena, Some(path_in_arena));

    let program = match interpreter.parse_and_resolve(content_in_arena) {
      Ok(program) => program,
      Err(error) => {
        report_error(path_str, &content, error);
        return Ok((0, 1));
      }
    };

    let results = interpreter.run_tests(&program.statements).await;

    for result in results {
      match result {
        TestResult::Success(name) => {
          println!("  test {} ... ok", name);
          passed += 1;
        }
        TestResult::Failure(name, error) => {
          println!("  '{}'... failed", name);
          report_error(path_str, &content, error);
          failed += 1;
        }
      }
    }

    Ok((passed, failed))
  }
}
