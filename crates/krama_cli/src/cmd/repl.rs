use std::process;

use anyhow::Result;
use bumpalo::{collections::String as BumpString, Bump};
use clap::Parser;
use krama_core::{
  error::{report_error, ErrorKind},
  object::Object,
};
use krama_runtime::interpreter::Interpreter;
use tokio::{
  io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader},
  signal,
};

#[derive(Parser)]
pub struct Repl;

impl Repl {
  pub async fn execute(&self) -> Result<()> {
    let arena = Bump::new();
    let interpreter = Interpreter::new(&arena, Some("repl"));
    let mut reader = BufReader::new(io::stdin());
    let mut stdout = io::stdout();
    let mut line = String::new();
    let mut history = BumpString::new_in(&arena);

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
                      if Self::process_line(&interpreter, &arena, &mut history, &line).await.is_err() {
                          break;
                      }
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

  async fn process_line<'a>(
    interpreter: &Interpreter<'a>,
    arena: &'a Bump,
    history: &mut BumpString<'a>,
    line: &str,
  ) -> Result<()> {
    history.push_str(line);
    let source = arena.alloc_str(history.as_str());

    match interpreter.check(source) {
      Ok(_) => {
        match interpreter.eval(source).await {
          Ok(object) => {
            if !matches!(object, Object::Void) {
              println!("{}", object);
            }
          }
          Err(error) => {
            report_error(error);
          }
        }
        history.clear();
      }
      Err(error) => {
        if let ErrorKind::SyntaxError(msg) = &error.kind {
          if msg.contains("Unexpected end of file")
            || msg.ends_with("but got Eof")
          {
            return Ok(());
          }
        }
        report_error(error);
        history.clear();
      }
    }
    Ok(())
  }
}
