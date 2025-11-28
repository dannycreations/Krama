use std::str;

use krama_core::token::{Token, TokenKind, KEYWORDS};

use super::Lexer;

impl<'a> Lexer<'a> {
  pub(super) fn identifier(&mut self, start: usize) -> Token<'a> {
    while let Some(c) = self.peek() {
      if c.is_alphanumeric() || c == '_' {
        self.advance();
      } else {
        break;
      }
    }

    let value = str::from_utf8(&self.input[start..self.position]).unwrap();

    let kind = KEYWORDS
      .get(value)
      .cloned()
      .unwrap_or(TokenKind::Identifier(value));

    Token::new(kind, self.span(start))
  }
}
