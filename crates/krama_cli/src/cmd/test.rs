use std::{env, path::PathBuf};

use anyhow::{Context, Result};
use bumpalo::Bump;
use clap::Parser;
use krama_core::{error::ErrorKind, span::Span};
use krama_runtime::{interpreter::Interpreter, testing::TestResult};
use tokio::fs;
use walkdir::WalkDir;

use crate::error::report_error;

#[derive(Parser)]
pub struct Test {
  #[clap(default_value = "src")]
  pub path: String,
}

#[derive(Debug)]
struct Failure<'a> {
  name: String,
  path: &'a str,
  content: &'a str,
  kind: ErrorKind,
  span: Span<'a>,
}

#[derive(Debug)]
enum TestKind<'a> {
  Success(String),
  Failure(Failure<'a>),
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

impl Test {
  pub async fn execute(&self) -> Result<()> {
    let mut passed = 0;
    let mut failed = 0;

    let root_path = env::current_dir()?;
    let mut path_buf = root_path;
    path_buf.push(&self.path);

    let test_files = find_test_files(path_buf)?;

    for path_buf in test_files {
      let arena = Bump::new();
      let path = path_buf.as_path();
      let content = fs::read_to_string(&path).await?;
      let path_str = path.to_str().context("path is not valid UTF-8")?;

      println!("Running tests in {}", path.display());

      let content_in_arena = arena.alloc_str(&content);
      let path_in_arena = arena.alloc_str(path_str);
      let interpreter = Interpreter::new(&arena, Some(path_in_arena));

      let program = match interpreter.parse_and_resolve(content_in_arena) {
        Ok(program) => program,
        Err((kind, span)) => {
          report_error(path_str, &content, span, kind);
          failed += 1;
          continue;
        }
      };

      let results = interpreter.run_tests(&program.statements).await;

      let cli_results = results
        .into_iter()
        .map(|res| match res {
          TestResult::Success(name) => TestKind::Success(name),
          TestResult::Failure(name, (kind, span)) => {
            TestKind::Failure(Failure {
              name,
              path: path_str,
              content: &content,
              kind,
              span,
            })
          }
        })
        .collect::<Vec<_>>();

      for result in cli_results {
        match result {
          TestKind::Success(name) => {
            println!("  test {} ... ok", name);
            passed += 1;
          }
          TestKind::Failure(failure) => {
            println!("  '{}'... failed", failure.name);
            report_error(
              failure.path,
              failure.content,
              failure.span,
              failure.kind,
            );
            failed += 1;
          }
        }
      }
    }

    println!("\nTest results: {} passed, {} failed", passed, failed);
    Ok(())
  }
}
