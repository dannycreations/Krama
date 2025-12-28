use std::{
  env,
  path::{Path, PathBuf},
  process,
};

use anyhow::{Context, Result};
use clap::{Parser as ClapParser, Subcommand};
use krama_core::{ErrorKind, Object};
use krama_runtime::{Interpreter, TestResult};
use tokio::{
  fs::read_to_string as tokio_read_to_string,
  io::{stdin, stdout, AsyncBufReadExt, AsyncWriteExt, BufReader},
  select, signal,
};
use walkdir::WalkDir;

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

#[derive(Subcommand)]
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

#[derive(ClapParser)]
pub struct Run {
  #[clap(default_value = "src/main.km")]
  pub file: String,
}

impl Run {
  pub async fn execute(&self, eval: bool) -> Result<()> {
    let interpreter = Interpreter::new(Some(self.file.clone()));
    let content = tokio_read_to_string(&self.file)
      .await
      .with_context(|| format!("Failed to read file: {}", &self.file))?;

    let result = if eval {
      interpreter.eval(&content).await.map(|_| ())
    } else {
      interpreter.check(&content).map(|_| ())
    };

    if let Err(error) = result {
      error.report();
    }
    Ok(())
  }
}

#[derive(ClapParser)]
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

    let content = tokio_read_to_string(path).await?;
    let path_str = path.to_str().context("path is not valid UTF-8")?;

    let interpreter = Interpreter::new(Some(path_str.to_string()));

    let statements = match interpreter.parse_and_check(&content) {
      Ok(statements) => statements,
      Err(error) => {
        error.report();
        return Ok((0, 1));
      }
    };

    let results = interpreter.run_tests(&statements).await;

    for result in results {
      match result {
        TestResult::Success(name) => {
          println!("  test {} ... ok", name);
          passed += 1;
        }
        TestResult::Failure(name, error) => {
          println!("  '{}'... failed", name);
          error.report();
          failed += 1;
        }
      }
    }

    Ok((passed, failed))
  }
}

pub struct Repl;

impl Repl {
  pub async fn execute(&self) -> Result<()> {
    let interpreter = Interpreter::new(Some("repl".to_string()));
    let mut reader = BufReader::new(stdin());
    let mut stdout = stdout();
    let mut line = String::new();
    let mut history = String::new();

    loop {
      let prompt = if history.is_empty() { "> " } else { "... " };
      stdout.write_all(prompt.as_bytes()).await?;
      stdout.flush().await?;
      line.clear();

      select! {
          _ = signal::ctrl_c() => {
              process::exit(0);
          }
          result = reader.read_line(&mut line) => {
              match result {
                  Ok(0) => break,
                  Ok(_) => {
                      if line.trim() == "exit" {
                          break;
                      }
                      Self::process_line(&interpreter, &mut history, &line).await?;
                  }
                  Err(e) => {
                      eprintln!("Error: {:?}", e);
                      break;
                  }
              }
          }
      }
    }
    Ok(())
  }

  async fn process_line(
    interpreter: &Interpreter,
    history: &mut String,
    line: &str,
  ) -> Result<()> {
    history.push_str(line);
    let source = history.as_str();

    if let Err(error) = interpreter.check(source) {
      if let ErrorKind::SyntaxError(msg) = &error.kind {
        if msg.contains("Unexpected end of file")
          || msg.ends_with("but got Eof")
        {
          return Ok(());
        }
      }
      error.report();
      history.clear();
    } else {
      match interpreter.eval(source).await {
        Ok(object) if !matches!(object, Object::Void) => {
          println!("{}", object)
        }
        Err(error) => error.report(),
        _ => {}
      };
      history.clear();
    }
    Ok(())
  }
}
