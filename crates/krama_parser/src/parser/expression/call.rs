use super::ParseError;
use super::Parser;
use super::Precedence;
use bumpalo::collections::Vec as BumpVec;
use krama_core::ast::expression::Expression;
use krama_core::ast::expression::ExpressionKind;
use krama_core::error::Error;
use krama_core::error::ErrorKind;
use krama_core::token::TokenKind;

impl<'a, 'ast> Parser<'a, 'ast>
where
  'a: 'ast,
{
  pub(super) fn parse_call_expression(
    &mut self,
    function: Expression<'ast>,
  ) -> ParseError<'ast> {
    let token = *self.current_token.as_ref().unwrap();
    let arguments = self.parse_call_arguments()?;
    Ok(Expression {
      kind: ExpressionKind::Call {
        function: self.arena.alloc(function),
        arguments,
      },
      span: token.span,
    })
  }

  fn parse_call_arguments(
    &mut self,
  ) -> Result<BumpVec<'ast, Expression<'ast>>, Error> {
    self.advance();
    let mut arguments = BumpVec::new_in(self.arena);
    if self
      .current_token
      .as_ref()
      .is_some_and(|t| t.kind == TokenKind::RParen)
    {
      self.advance();
      return Ok(arguments);
    }

    arguments.push(self.parse_expression(Precedence::Lowest)?);
    while self
      .current_token
      .as_ref()
      .is_some_and(|t| t.kind == TokenKind::Comma)
    {
      self.advance();
      arguments.push(self.parse_expression(Precedence::Lowest)?);
    }

    if !self
      .current_token
      .as_ref()
      .is_some_and(|t| t.kind == TokenKind::RParen)
    {
      return Err(Error {
        span: self.current_token.as_ref().unwrap().span,
        kind: ErrorKind::ParserError("Expected ')' after arguments"),
      });
    }
    self.advance();

    Ok(arguments)
  }
}
