use krama_core::{Expression, ExpressionKind, TokenKind};

use super::{ParseResult, Parser};

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub fn parse_collection_expression(&mut self) -> ParseResult<'a, 'ast> {
    let start_span = self.consume(TokenKind::LBracket)?.span;

    let elements =
      self.parse_comma_separated_expressions(TokenKind::RBracket)?;

    let end_span = self.consume(TokenKind::RBracket)?.span;

    Ok(Expression::new(
      ExpressionKind::Collection { elements },
      start_span.merge(&end_span),
    ))
  }
}
