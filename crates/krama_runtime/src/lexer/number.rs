use krama_core::{Token, TokenKind};

use super::Lexer;

impl<'a> Lexer<'a> {
  pub fn number(&mut self, start: usize) -> Token<'a> {
    let mut is_float = false;
    let num_start = self.position - 1;

    while let Some(c) = self.peek_byte() {
      match c {
        b'_' => {
          self.advance_byte();
        }
        b'0'..=b'9' => {
          self.advance_byte();
        }
        b'.' => {
          if is_float {
            break;
          }
          if self.peek_byte_nth(1) == Some(b'.') {
            break;
          }
          is_float = true;
          self.advance_byte();
        }
        b'e' | b'E' => {
          is_float = true;
          self.advance_byte();
          if self.peek_byte() == Some(b'-') || self.peek_byte() == Some(b'+') {
            self.advance_byte();
          }
        }
        _ => break,
      }
    }

    let num_end = self.position;
    let num_slice = self.slice(num_start, num_end);

    let token_kind = if is_float {
      TokenKind::Float(num_slice)
    } else {
      TokenKind::Integer(num_slice)
    };

    Token::new(token_kind, self.span(start))
  }
}
