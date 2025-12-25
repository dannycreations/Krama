use krama_core::{Expression, ExpressionKind, PrecedenceKind};

use super::{ParseResult, Parser};

impl<'a> Parser<'a> {
  pub fn parse_member_expression(&mut self, object: Expression) -> ParseResult {
    self.advance();
    let property = self.parse_expression(PrecedenceKind::Member)?;
    let span = object.span.merge(&property.span);
    Ok(Expression::new(
      ExpressionKind::Member {
        object: Box::new(object),
        property: Box::new(property),
      },
      span,
    ))
  }
}
