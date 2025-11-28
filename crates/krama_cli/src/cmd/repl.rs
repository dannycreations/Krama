use std::process;

use anyhow::Result;
use bumpalo::Bump;
use clap::Parser;
use krama_core::object::Object;
use krama_runtime::interpreter::Interpreter;
use tokio::{
  io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader},
  signal,
};

use crate::error::report_error;

#[derive(Parser)]
pub struct Repl;

async fn evaluate_line<'a>(
  interpreter: &Interpreter<'a>,
  arena: &'a Bump,
  line: &str,
) {
  let trimmed_line = line.trim();
  if trimmed_line.is_empty() {
    return;
  }

  let line_in_arena = arena.alloc_str(trimmed_line);

  match interpreter.eval(line_in_arena).await {
    Ok(object) => {
      if !matches!(object, Object::Void) {
        println!("{}", object);
      }
    }
    Err(err) => report_error("repl", trimmed_line, err),
  };
}

impl Repl {
  pub async fn execute(&self, arena: &mut Bump) -> Result<()> {
    let interpreter = Interpreter::new(arena, None);
    let mut reader = BufReader::new(io::stdin());
    let mut stdout = io::stdout();
    let mut line = String::new();

    loop {
      stdout.write_all(b">> ").await?;
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
                      evaluate_line(&interpreter, arena, &line).await;
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
}
