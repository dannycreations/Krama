use krama_core::{
  ErrorKindResult, Expression, ExpressionKind, PrecedenceKind, TokenKind,
};

use super::{ParseResult, Parser};

impl<'a> Parser<'a> {
  pub fn parse_call_expression(&mut self, function: Expression) -> ParseResult {
    let token = self.current_token.clone();
    self.consume(TokenKind::LParen)?;

    let arguments =
      self.parse_comma_separated_expressions(TokenKind::RParen)?;
    let end_token = self.consume(TokenKind::RParen)?;
    let span = token.span.merge(&end_token.span);
    Ok(Expression::new(
      ExpressionKind::Call {
        function: Box::new(function),
        arguments,
      },
      span,
    ))
  }

  pub fn parse_comma_separated_expressions(
    &mut self,
    end_token: TokenKind,
  ) -> ErrorKindResult<Vec<Expression>> {
    let mut expressions = Vec::new();

    if self.current_token.kind == end_token {
      return Ok(expressions);
    }

    loop {
      expressions.push(self.parse_expression(PrecedenceKind::Lowest)?);

      if self.current_token.kind == end_token {
        break;
      }

      self.consume(TokenKind::Comma)?;

      if self.current_token.kind == end_token {
        break;
      }
    }

    Ok(expressions)
  }
}
