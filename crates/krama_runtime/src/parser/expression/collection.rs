use krama_core::{Expression, ExpressionKind, PrecedenceKind, TokenKind};

use super::{ParseResult, Parser};

impl<'a> Parser<'a> {
  pub fn parse_collection_expression(&mut self) -> ParseResult {
    let start_span = self.current_token.span;
    let elements = self.parse_delimited(
      TokenKind::LBracket,
      TokenKind::RBracket,
      TokenKind::Comma,
      |p| p.parse_expression(PrecedenceKind::Lowest),
    )?;
    let span = start_span.merge(&self.current_token.span);
    Ok(Expression::new(
      ExpressionKind::Collection { elements },
      span,
    ))
  }
}
