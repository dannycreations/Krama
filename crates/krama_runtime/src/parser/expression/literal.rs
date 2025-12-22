use krama_core::{
  ErrorKind, Expression, ExpressionKind, LiteralKind, TokenKind,
};

use super::{ParseResult, Parser};

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub fn parse_literal(&mut self) -> ParseResult<'a, 'ast> {
    let token = self.current_token.clone();
    self.advance();

    let literal = match token.kind {
      TokenKind::Integer(value) => {
        let value = value.replace('_', "").parse().map_err(|_| {
          ErrorKind::SyntaxError("Invalid integer literal".to_string())
        })?;
        LiteralKind::Integer(value)
      }
      TokenKind::Float(value) => {
        let value = value.replace('_', "").parse().map_err(|_| {
          ErrorKind::SyntaxError("Invalid float literal".to_string())
        })?;
        LiteralKind::Float(value)
      }
      TokenKind::String(value) => {
        LiteralKind::String(self.arena.alloc_str(value))
      }
      TokenKind::True => LiteralKind::Boolean(true),
      TokenKind::False => LiteralKind::Boolean(false),
      TokenKind::Null => LiteralKind::Null,
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

  pub fn parse_identifier_expression(&mut self) -> ParseResult<'a, 'ast> {
    let token = self.current_token.clone();
    let name = self.parse_identifier()?;
    Ok(Expression::new(
      ExpressionKind::Identifier(name),
      token.span,
    ))
  }
}
