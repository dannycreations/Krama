use std::process;

use anyhow::Result;
use bumpalo::{collections::String as BumpString, Bump};
use clap::Parser;
use krama_core::{error::ErrorKind, object::Object};
use krama_runtime::interpreter::Interpreter;
use tokio::{
  io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader},
  signal,
};

use crate::error::report_error;

#[derive(Parser)]
pub struct Repl;

impl Repl {
  pub async fn execute(&self) -> Result<()> {
    let arena = Bump::new();
    let mut reader = BufReader::new(io::stdin());
    let mut stdout = io::stdout();
    let mut line = String::new();
    let mut history = BumpString::new_in(&arena);
    let interpreter = Interpreter::new(&arena, Some("repl"));

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

                      history.push_str(&line);
                      let source = arena.alloc_str(&history);

                      match interpreter.check(source) {
                        Ok(_) => {
                            match interpreter.eval(source).await {
                                Ok(object) => {
                                    if !matches!(object, Object::Void) {
                                        println!("{}", object);
                                    }
                                }
                                Err((kind, span)) => {
                                    report_error("repl", source, span, kind);
                                }
                            }
                            history.clear();
                        }
                        Err((kind, span)) => {
                          if let ErrorKind::SyntaxError(msg) = &kind {
                            let is_unexpected_eof = msg.contains("Unexpected");
                            let is_missing_closer = msg.contains("Expected") && msg.contains("Eof");

                            if is_unexpected_eof || is_missing_closer {
                                continue;
                            }
                          }
                          report_error("repl", source, span, kind);
                          history.clear();
                        }
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
}
