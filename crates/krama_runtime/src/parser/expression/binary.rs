use krama_core::{
  AssignmentOperator, BinaryOperator, ErrorKind, Expression, ExpressionKind,
};

use super::{ParseResult, Parser};

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  /// Parses an infix expression (binary or assignment) using Pratt parsing.
  pub fn parse_infix_expression(
    &mut self,
    left: Expression<'ast>,
  ) -> ParseResult<'a, 'ast> {
    let precedence = self.current_precedence();
    let token = self.current_token.clone();

    if let Some(op) = AssignmentOperator::from_token(token.kind) {
      self.advance();
      let left_span = left.span;
      let right = self.parse_expression(precedence)?;
      return Ok(Expression::new(
        ExpressionKind::Assignment {
          left: self.arena.alloc(left),
          operator: op,
          right: self.arena.alloc(right),
        },
        token.span.merge(&left_span),
      ));
    }

    if let Some(op) = BinaryOperator::from_token(token.kind) {
      self.advance();
      let left_span = left.span;
      let right = self.parse_expression(precedence)?;
      let right_span = right.span;
      return Ok(Expression::new(
        ExpressionKind::Binary {
          left: self.arena.alloc(left),
          operator: op,
          right: self.arena.alloc(right),
        },
        left_span.merge(&right_span),
      ));
    }

    Err(ErrorKind::SyntaxError("Invalid infix operator".into()))
  }
}
