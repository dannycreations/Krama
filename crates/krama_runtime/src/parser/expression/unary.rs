use krama_core::{
  ErrorKind, Expression, ExpressionKind, PrecedenceKind, TokenKind,
  UnaryOperator, UpdateOperator,
};

use super::{ParseResult, Parser};

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  /// Parses a prefix unary expression (!, -, ~).
  pub fn parse_unary_expression(&mut self) -> ParseResult<'a, 'ast> {
    let token = self.current_token.clone();

    // Handle '+' as a no-op prefix by just parsing the expression.
    if token.kind == TokenKind::Plus {
      self.advance();
      return self.parse_expression(PrecedenceKind::Prefix);
    }

    if let Some(operator) = UnaryOperator::from_token(token.kind) {
      self.advance();
      let right = self.parse_expression(PrecedenceKind::Prefix)?;
      return Ok(Expression::new(
        ExpressionKind::Unary {
          operator,
          right: self.arena.alloc(right),
        },
        token.span,
      ));
    }

    Err(ErrorKind::SyntaxError("Invalid unary operator".to_string()))
  }

  /// Parses a prefix update expression (++x, --x).
  pub fn parse_prefix_update_expression(&mut self) -> ParseResult<'a, 'ast> {
    let token = self.current_token.clone();

    if let Some(operator) = UpdateOperator::from_token(token.kind) {
      self.advance();
      let argument = self.parse_expression(PrecedenceKind::Prefix)?;

      return Ok(Expression::new(
        ExpressionKind::Update {
          operator,
          argument: self.arena.alloc(argument),
          prefix: true,
        },
        token.span,
      ));
    }

    Err(ErrorKind::SyntaxError(
      "Invalid prefix operator".to_string(),
    ))
  }
}
