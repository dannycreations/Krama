use krama_core::{ErrorKind, ErrorKindResult, Statement, TokenKind};

use super::Parser;

impl<'a> Parser<'a> {
  pub fn parse_pub_statement(&mut self) -> ErrorKindResult<Statement> {
    let start_span = self.current_token.span;
    self.advance();
    match self.current_token.kind {
      TokenKind::Const => self.parse_const_statement(true, start_span),
      TokenKind::Fn => self.parse_fn_statement(true, start_span),
      TokenKind::Enum => self.parse_enum_statement(true, start_span),
      TokenKind::Struct => self.parse_struct_statement(true, start_span),
      _ => Err(ErrorKind::SyntaxError(
        "Expected 'const', 'fn', 'enum' or 'struct' after 'pub'".to_string(),
      )),
    }
  }
}
