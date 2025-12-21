use krama_core::{ErrorKind, Span, Statement, StatementKind, TokenKind};

use super::Parser;

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub fn parse_type_statement(
    &mut self,
    public: bool,
    start_span: Span,
  ) -> Result<Statement<'ast>, ErrorKind> {
    self.consume(TokenKind::Type)?;

    let name = self.parse_identifier()?;

    self.consume(TokenKind::Equal)?;

    let kind = self.parse_type()?;

    let end_span = kind.span;

    Ok(Statement::new(
      StatementKind::Type { public, name, kind },
      start_span.merge(&end_span),
    ))
  }
}
