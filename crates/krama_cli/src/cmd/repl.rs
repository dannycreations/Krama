use std::process;

use anyhow::Result;
use clap::Parser;
use krama_core::{ErrorKind, ObjectKind};
use krama_runtime::Interpreter;
use tokio::{
  io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader},
  signal,
};

#[derive(Parser)]
pub struct Repl;

impl Repl {
  pub async fn execute(&self) -> Result<()> {
    let interpreter = Interpreter::new(Some("repl".to_string()));
    let mut reader = BufReader::new(io::stdin());
    let mut stdout = io::stdout();
    let mut line = String::new();
    let mut history = String::new();

    loop {
      let prompt = if history.is_empty() { "> " } else { "... " };
      stdout.write_all(prompt.as_bytes()).await?;
      stdout.flush().await?;
      line.clear();

      tokio::select! {
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
        Ok(object) if !matches!(object, ObjectKind::Void) => {
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
