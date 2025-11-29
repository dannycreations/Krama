use bumpalo::collections::Vec as BumpVec;
use krama_core::{
  ast::{
    expression::{Expression, ExpressionKind},
    precedence::Precedence,
  },
  error::ErrorKind,
  span::Span,
  token::TokenKind,
};

use crate::parser::{ParseError, Parser};

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub(super) fn parse_call_expression(
    &mut self,
    function: Expression<'ast>,
  ) -> ParseError<'a, 'ast> {
    let token = self.current_token.clone();
    let arguments = self.parse_call_arguments()?;
    Ok(Expression::new(
      ExpressionKind::Call {
        function: self.arena.alloc(function),
        arguments,
      },
      token.span,
    ))
  }

  fn parse_call_arguments(
    &mut self,
  ) -> Result<BumpVec<'ast, Expression<'ast>>, (ErrorKind, Span<'a>)> {
    self.advance();
    let mut arguments = BumpVec::new_in(self.arena);
    if self.current_token.kind == TokenKind::RParen {
      self.advance();
      return Ok(arguments);
    }

    arguments.push(self.parse_expression(Precedence::Lowest)?);
    while self.current_token.kind == TokenKind::Comma {
      self.advance();
      arguments.push(self.parse_expression(Precedence::Lowest)?);
    }

    if self.current_token.kind != TokenKind::RParen {
      return Err((
        ErrorKind::SyntaxError(format!(
          "Expected {} after arguments",
          TokenKind::RParen
        )),
        self.current_token.span.clone(),
      ));
    }
    self.advance();

    Ok(arguments)
  }
}
