use bumpalo::collections::Vec as BumpVec;
use krama_core::{
  ErrorKind, Expression, ExpressionKind, LiteralKind, PrecedenceKind,
  TokenKind, UnaryOperator, UpdateOperator,
};

use crate::parser::{ParseResult, Parser};

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  /// Parses an identifier expression.
  pub(crate) fn parse_identifier_expression(
    &mut self,
  ) -> ParseResult<'a, 'ast> {
    let token = self.current_token.clone();
    let name = self.parse_identifier()?;
    Ok(Expression::new(
      ExpressionKind::Identifier(self.arena.alloc_str(name)),
      token.span,
    ))
  }

  /// Parses a literal expression (Integer, Float, String, Boolean, Null).
  pub(crate) fn parse_literal(&mut self) -> ParseResult<'a, 'ast> {
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

  /// Parses a parenthesized expression or an arrow function.
  pub(crate) fn parse_paren_expression(&mut self) -> ParseResult<'a, 'ast> {
    let start_span = self.current_token.span;
    self.consume(TokenKind::LParen)?;

    if self.current_token.kind == TokenKind::RParen {
      self.consume(TokenKind::RParen)?;
      if self.current_token.kind == TokenKind::Arrow {
        let (body, kind) = self.parse_arrow_fn_body_and_return_type()?;
        return Ok(Expression::new(
          ExpressionKind::Fn {
            parameters: BumpVec::new_in(self.arena),
            body,
            kind,
          },
          start_span,
        ));
      } else {
        return Err(ErrorKind::SyntaxError(
          "Parenthesized expression cannot be empty. Use `null` for a null value."
            .to_string(),
        ));
      }
    }

    // Try parsing as an arrow function first by checking if it looks like parameters.
    let mut fn_parser = self.clone();
    if let Ok(parameters) = fn_parser.parse_fn_parameters() {
      if fn_parser.current_token.kind == TokenKind::RParen {
        fn_parser.consume(TokenKind::RParen)?;
        if fn_parser.current_token.kind == TokenKind::Arrow
          || fn_parser.current_token.kind == TokenKind::Colon
        {
          *self = fn_parser;
          let (body, kind) = self.parse_arrow_fn_body_and_return_type()?;
          return Ok(Expression::new(
            ExpressionKind::Fn {
              parameters,
              body,
              kind,
            },
            start_span,
          ));
        }
      }
    }

    let expression = self.parse_expression(PrecedenceKind::Lowest)?;
    self.consume(TokenKind::RParen)?;
    Ok(expression)
  }

  /// Parses a prefix unary expression (!, -, ~).
  pub(crate) fn parse_unary_expression(&mut self) -> ParseResult<'a, 'ast> {
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
  pub(crate) fn parse_prefix_update_expression(
    &mut self,
  ) -> ParseResult<'a, 'ast> {
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
