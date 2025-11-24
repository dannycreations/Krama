use crate::error::report_error;
use anyhow::Result;
use bumpalo::Bump;
use clap::Parser;
use krama_core::object::Object;
use krama_runtime::interpreter::Interpreter;
use tokio::io;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;

#[derive(Parser)]
pub struct Repl;

impl Repl {
  pub async fn execute(&self, arena: &Bump) -> Result<()> {
    let interpreter = Interpreter::new(arena, None);
    let mut reader = BufReader::new(io::stdin());
    let mut stdout = io::stdout();

    loop {
      stdout.write_all(b">> ").await?;
      stdout.flush().await?;

      let mut line = String::new();
      match reader.read_line(&mut line).await {
        Ok(0) => {
          break;
        }
        Ok(_) => {
          let trimmed_line = line.trim_end();
          if trimmed_line.is_empty() {
            continue;
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
        Err(e) => {
          eprintln!("Error: {:?}", e);
          break;
        }
      }
    }
    Ok(())
  }
}
