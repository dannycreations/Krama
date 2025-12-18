use krama_core::{Token, TokenKind};

use super::Lexer;

impl<'a> Lexer<'a> {
  pub fn string(&mut self, start: usize) -> Token<'a> {
    let content_start = self.position;

    while let Some(c) = self.peek_byte() {
      match c {
        b'"' => break,
        b'\\' => {
          self.advance_byte();
          if self.peek_byte().is_some() {
            self.advance_byte();
          }
        }
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
