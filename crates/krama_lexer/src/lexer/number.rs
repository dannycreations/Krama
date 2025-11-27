use krama_core::token::{Token, TokenKind};

use super::Lexer;

impl<'a> Lexer<'a> {
  pub(super) fn number(&mut self, start: usize) -> Token<'a> {
    let mut is_float = false;
    let num_start = self.position - 1;

    while let Some(c) = self.peek() {
      if c == '_' || c.is_ascii_digit() {
        self.advance();
      } else if c == '.' && !is_float {
        // Check for `..` to avoid lexing `1..10` as `1.`, `.`, `10`
        if self.input.get(self.position + 1) == Some(&b'.') {
          // This is a range, so we stop parsing the number.
          break;
        }
        is_float = true;
        self.advance();
      } else if c == 'e' || c == 'E' {
        is_float = true;
        self.advance();
        if self.peek() == Some('-') || self.peek() == Some('+') {
          self.advance();
        }
      } else {
        break;
      }
    }

    let num_end = self.position;
    let num_slice = &self.input_str[num_start..num_end];

    let token_kind = if is_float {
      TokenKind::Float(num_slice)
    } else {
      TokenKind::Integer(num_slice)
    };

    Token::new(token_kind, self.span(start))
  }
}
