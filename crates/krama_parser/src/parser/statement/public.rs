use krama_core::{
  ast::statement::Statement,
  error::{Error, ErrorKind},
  token::TokenKind,
};

use super::Parser;

impl<'a, 'ast> Parser<'a, 'ast> {
  pub(super) fn parse_pub_statement(
    &mut self,
  ) -> Result<Statement<'ast>, Error> {
    let start_span = self.current_token.span;
    self.advance();
    match self.current_token.kind {
      TokenKind::Const => self.parse_const_statement(true, start_span),
      TokenKind::Fn => self.parse_fn_statement(true, start_span),
      _ => Err(Error {
        span: start_span,
        kind: ErrorKind::SyntaxError(
          "Expected 'const' or 'fn' after 'pub'".to_string(),
        ),
      }),
    }
  }
}
