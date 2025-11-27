use std::str;

use krama_core::token::{Token, TokenKind};

use super::Lexer;

impl<'a> Lexer<'a> {
  pub(super) fn string(&mut self, start: usize) -> Token<'a> {
    let content_start = self.position;

    while let Some(c) = self.peek() {
      if c == '\\' {
        // If we see a backslash, consume it and the next character,
        // treating it as an escape sequence.
        self.advance();
        self.advance();
      } else if c == '"' {
        // This is the end of the string.
        break;
      } else {
        self.advance();
      }
    }

    if self.peek() != Some('"') {
      return Token::new(TokenKind::Unknown, self.span(start));
    }

    let content_end = self.position;
    let value =
      str::from_utf8(&self.input[content_start..content_end]).unwrap();

    self.advance();

    Token::new(TokenKind::String(value), self.span(start))
  }
}
