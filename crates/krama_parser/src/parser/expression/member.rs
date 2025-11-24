use super::ParseError;
use super::Parser;
use super::Precedence;
use krama_core::ast::expression::Expression;
use krama_core::ast::expression::ExpressionKind;
use krama_core::span::Span;

impl<'a, 'ast> Parser<'a, 'ast>
where
  'a: 'ast,
{
  pub(super) fn parse_member_expression(
    &mut self,
    object: Expression<'ast>,
  ) -> ParseError<'ast> {
    self.advance();
    let property = self.parse_expression(Precedence::Member)?;
    let span = Span::new(object.span.start, property.span.end);
    Ok(Expression {
      kind: ExpressionKind::Member {
        object: self.arena.alloc(object),
        property: self.arena.alloc(property),
      },
      span,
    })
  }
}
