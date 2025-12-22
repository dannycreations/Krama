use krama_core::{Expression, ExpressionKind, PrecedenceKind};

use super::{ParseResult, Parser};

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub fn parse_member_expression(
    &mut self,
    object: Expression<'ast>,
  ) -> ParseResult<'a, 'ast> {
    self.advance();
    let property = self.parse_expression(PrecedenceKind::Member)?;
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
