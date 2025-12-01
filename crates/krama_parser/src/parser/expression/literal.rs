use krama_core::{
  ast::{
    expression::{Expression, ExpressionKind},
    literal::Literal,
  },
  error::ErrorKind,
  token::TokenKind,
};

use crate::parser::{ParseResult, Parser};

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub(super) fn parse_literal(&mut self) -> ParseResult<'a, 'ast> {
    let token = self.current_token.clone();
    self.advance();

    let literal = match token.kind {
      TokenKind::Integer(value) => {
        let value = value.replace('_', "").parse().map_err(|_| {
          ErrorKind::SyntaxError("Invalid integer literal".to_string())
        })?;
        Literal::Integer(value)
      }
      TokenKind::Float(value) => {
        let value = value.replace('_', "").parse().map_err(|_| {
          ErrorKind::SyntaxError("Invalid float literal".to_string())
        })?;
        Literal::Float(value)
      }
      TokenKind::String(value) => Literal::String(self.arena.alloc_str(value)),
      TokenKind::True => Literal::Boolean(true),
      TokenKind::False => Literal::Boolean(false),
      TokenKind::Null => Literal::Null,
      _ => {
        return Err(ErrorKind::SyntaxError(format!(
          "Unexpected token for literal: {}",
          token.kind
        )))
      }
    };

    Ok(Expression::new(
      ExpressionKind::Literal(literal),
      token.span,
    ))
  }

  pub(super) fn parse_identifier_expression(
    &mut self,
  ) -> ParseResult<'a, 'ast> {
    let token = self.current_token.clone();
    let name = self.parse_identifier()?;
    Ok(Expression::new(
      ExpressionKind::Identifier(name),
      token.span,
    ))
  }
}
