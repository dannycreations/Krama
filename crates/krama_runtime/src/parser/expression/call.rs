use bumpalo::collections::Vec as BumpVec;
use krama_core::{
  ErrorKind, Expression, ExpressionKind, Precedence, TokenKind,
};

use super::{ParseResult, Parser};

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub fn parse_call_expression(
    &mut self,
    function: Expression<'ast>,
  ) -> ParseResult<'a, 'ast> {
    let token = self.current_token.clone();
    self.consume(TokenKind::LParen)?;

    let arguments =
      self.parse_comma_separated_expressions(TokenKind::RParen)?;
    let end_token = self.consume(TokenKind::RParen)?;
    let span = token.span.merge(&end_token.span);
    Ok(Expression::new(
      ExpressionKind::Call {
        function: self.arena.alloc(function),
        arguments,
      },
      span,
    ))
  }

  pub fn parse_comma_separated_expressions(
    &mut self,
    end_token: TokenKind,
  ) -> Result<BumpVec<'ast, Expression<'ast>>, ErrorKind> {
    let mut expressions = BumpVec::new_in(self.arena);

    if self.current_token.kind == end_token {
      return Ok(expressions);
    }

    loop {
      expressions.push(self.parse_expression(Precedence::Lowest)?);

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
