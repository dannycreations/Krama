use krama_core::token::{Token, TokenKind};

use super::Lexer;

impl<'a> Lexer<'a> {
  pub(super) fn string(&mut self, start: usize) -> Token<'a> {
    let mut escaped = false;
    let content_start = self.position;

    while let Some(c) = self.peek_byte() {
      if escaped {
        escaped = false;
        self.advance_byte();
        continue;
      }
      match c {
        b'\\' => {
          escaped = true;
          self.advance_byte();
        }
        b'"' => break,
        _ => {
          self.advance_byte();
        }
      }
    }

    if self.peek_byte() != Some(b'"') {
      return Token::new(TokenKind::Unknown, self.span(start));
    }

    let content_end = self.position;
    let value = self.slice(content_start, content_end);

    self.advance_byte();

    Token::new(TokenKind::String(value), self.span(start))
  }
}
