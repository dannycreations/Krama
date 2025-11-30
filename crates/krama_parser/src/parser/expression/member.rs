use krama_core::ast::{
  expression::{Expression, ExpressionKind},
  precedence::Precedence,
};

use crate::parser::{ParseResult, Parser};

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub(super) fn parse_member_expression(
    &mut self,
    object: Expression<'ast>,
  ) -> ParseResult<'a, 'ast> {
    self.advance();
    let property = self.parse_expression(Precedence::Member)?;
    let span = object.span.merge(&property.span);
    Ok(Expression::new(
      ExpressionKind::Member {
        object: self.arena.alloc(object),
        property: self.arena.alloc(property),
      },
      span,
    ))
  }
}
