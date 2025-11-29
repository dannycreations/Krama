use krama_core::{
  ast::statement::Statement, error::ErrorKind, span::Span, token::TokenKind,
};

use crate::parser::Parser;

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub(super) fn parse_pub_statement(
    &mut self,
  ) -> Result<Statement<'ast>, (ErrorKind, Span<'a>)> {
    let start_span = self.current_token.span.clone();
    self.advance();
    match self.current_token.kind {
      TokenKind::Const => self.parse_const_statement(true, start_span),
      TokenKind::Fn => self.parse_fn_statement(true, start_span),
      _ => Err((
        ErrorKind::SyntaxError(
          "Expected 'const' or 'fn' after 'pub'".to_string(),
        ),
        start_span,
      )),
    }
  }
}
