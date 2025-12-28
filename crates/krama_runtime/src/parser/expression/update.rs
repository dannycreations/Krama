use krama_core::{
  ErrorKind, Expression, ExpressionKind, Precedence, TokenKind, UnaryOperator,
  UpdateOperator,
};

use crate::{ParseResult, Parser};

impl<'a> Parser<'a> {
  pub fn parse_unary_expression(&mut self) -> ParseResult {
    let token = self.current_token.clone();
    if token.kind == TokenKind::Plus {
      self.advance();
      return self.parse_expression(Precedence::Prefix);
    }
    if let Some(operator) = UnaryOperator::from_token(token.kind) {
      self.advance();
      let right = self.parse_expression(Precedence::Prefix)?;
      return Ok(Expression::new(
        ExpressionKind::Unary {
          operator,
          right: Box::new(right),
        },
        token.span,
      ));
    }
    Err(ErrorKind::SyntaxError("Invalid unary operator".to_string()))
  }

  pub fn parse_prefix_update_expression(&mut self) -> ParseResult {
    let token = self.current_token.clone();
    if let Some(operator) = UpdateOperator::from_token(token.kind) {
      self.advance();
      let argument = self.parse_expression(Precedence::Prefix)?;
      return Ok(Expression::new(
        ExpressionKind::Update {
          operator,
          argument: Box::new(argument),
          prefix: true,
        },
        token.span,
      ));
    }
    Err(ErrorKind::SyntaxError(
      "Invalid prefix operator".to_string(),
    ))
  }

  pub fn parse_postfix_expression(&mut self, left: Expression) -> ParseResult {
    let token = self.current_token.clone();
    self.advance();
    let span = left.span.merge(&token.span);
    let kind = match token.kind {
      TokenKind::PlusPlus => ExpressionKind::Update {
        operator: UpdateOperator::Increment,
        argument: Box::new(left),
        prefix: false,
      },
      TokenKind::MinusMinus => ExpressionKind::Update {
        operator: UpdateOperator::Decrement,
        argument: Box::new(left),
        prefix: false,
      },
      TokenKind::Question => ExpressionKind::Try(Box::new(left)),
      _ => unreachable!(),
    };
    Ok(Expression::new(kind, span))
  }
}
