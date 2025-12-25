use krama_core::{
  ErrorKind, Expression, ExpressionKind, LiteralKind, PrecedenceKind,
  TokenKind, UnaryOperator, UpdateOperator,
};

use crate::parser::{ParseResult, Parser};

impl<'a> Parser<'a> {
  /// Parses an identifier expression.
  pub fn parse_identifier_expression(&mut self) -> ParseResult {
    let token = self.current_token.clone();
    let name = self.parse_identifier()?;
    Ok(Expression::new(
      ExpressionKind::Identifier(name.into()),
      token.span,
    ))
  }

  /// Parses a literal expression (Integer, Float, String, Boolean, Null).
  pub fn parse_literal(&mut self) -> ParseResult {
    let token = self.current_token.clone();
    self.advance();

    let literal = match token.kind {
      TokenKind::Integer(value) => {
        LiteralKind::Integer(value.replace('_', "").parse().map_err(|_| {
          ErrorKind::SyntaxError("Invalid integer literal".to_string())
        })?)
      }
      TokenKind::Float(value) => {
        LiteralKind::Float(value.replace('_', "").parse().map_err(|_| {
          ErrorKind::SyntaxError("Invalid float literal".to_string())
        })?)
      }
      TokenKind::String(value) => LiteralKind::String(value.clone()),
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
  pub fn parse_paren_expression(&mut self) -> ParseResult {
    let start_span = self.current_token.span;
    self.consume(TokenKind::LParen)?;

    if self.current_token.kind == TokenKind::RParen {
      self.consume(TokenKind::RParen)?;
      if self.current_token.kind == TokenKind::Arrow {
        let (body, kind) = self.parse_arrow_fn_body_and_return_type()?;
        return Ok(Expression::new(
          ExpressionKind::Fn {
            parameters: Vec::new(),
            body,
            kind,
          },
          start_span,
        ));
      }
      return Err(ErrorKind::SyntaxError("Empty parens".into()));
    }

    let mut fn_parser = self.clone();
    if let Ok(parameters) = fn_parser.parse_fn_parameters() {
      if fn_parser.current_token.kind == TokenKind::RParen {
        fn_parser.consume(TokenKind::RParen)?;
        if matches!(
          fn_parser.current_token.kind,
          TokenKind::Arrow | TokenKind::Colon
        ) {
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

    let expr = self.parse_expression(PrecedenceKind::Lowest)?;
    self.consume(TokenKind::RParen)?;
    Ok(expr)
  }

  /// Parses a prefix unary expression (!, -, ~).
  pub fn parse_unary_expression(&mut self) -> ParseResult {
    let token = self.current_token.clone();
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
          right: Box::new(right),
        },
        token.span,
      ));
    }
    Err(ErrorKind::SyntaxError("Invalid unary operator".to_string()))
  }

  /// Parses a prefix update expression (++x, --x).
  pub fn parse_prefix_update_expression(&mut self) -> ParseResult {
    let token = self.current_token.clone();
    if let Some(operator) = UpdateOperator::from_token(token.kind) {
      self.advance();
      let argument = self.parse_expression(PrecedenceKind::Prefix)?;
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
}
