use krama_core::{
  AssignmentOperator, BinaryOperator, ErrorKind, Expression, ExpressionKind,
};

use super::{ParseResult, Parser};

impl<'a> Parser<'a> {
  /// Parses an infix expression (binary or assignment) using Pratt parsing.
  pub fn parse_infix_expression(&mut self, left: Expression) -> ParseResult {
    let precedence = self.current_precedence();
    let token = self.current_token.clone();

    if let Some(op) = AssignmentOperator::from_token(token.kind.clone()) {
      self.advance();
      let left_span = left.span;
      let right = self.parse_expression(precedence)?;
      return Ok(Expression::new(
        ExpressionKind::Assignment {
          left: Box::new(left),
          operator: op,
          right: Box::new(right),
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
          left: Box::new(left),
          operator: op,
          right: Box::new(right),
        },
        left_span.merge(&right_span),
      ));
    }

    Err(ErrorKind::SyntaxError("Invalid infix operator".into()))
  }
}
