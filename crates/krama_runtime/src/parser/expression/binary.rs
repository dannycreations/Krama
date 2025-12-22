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

    // Consolidate operator mapping using the core methods to reduce duplication.
    if let Some(op) = AssignmentOperator::from_token(token.kind) {
      self.advance();
      let right = self.parse_expression(precedence)?;
      return Ok(Expression::new(
        ExpressionKind::Assignment {
          left: self.arena.alloc(left),
          operator: op,
          right: self.arena.alloc(right),
        },
        token.span,
      ));
    }

    if let Some(op) = BinaryOperator::from_token(token.kind) {
      self.advance();
      let right = self.parse_expression(precedence)?;
      let span = left.span.merge(&right.span);
      return Ok(Expression::new(
        ExpressionKind::Binary {
          left: self.arena.alloc(left),
          operator: op,
          right: self.arena.alloc(right),
        },
        span,
      ));
    }

    // This should theoretically not be reached if the precedence table is correct.
    Err(ErrorKind::SyntaxError("Invalid infix operator".to_string()))
  }
}
