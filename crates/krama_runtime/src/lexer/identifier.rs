use krama_core::{Token, TokenKind, KEYWORDS};

use super::Lexer;

impl<'a> Lexer<'a> {
  pub fn identifier(&mut self, start: usize) -> Token<'a> {
    while let Some(c) = self.peek_byte() {
      if c.is_ascii_alphanumeric() || c == b'_' {
        self.advance_byte();
      } else {
        break;
      }
    }

    let value = self.slice(start, self.position);

    let kind = KEYWORDS
      .get(value)
      .cloned()
      .unwrap_or(TokenKind::Identifier(value));

    Token::new(kind, self.span(start))
  }
}
