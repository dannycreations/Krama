use krama_core::{
  ErrorKind, ErrorKindResult, Expression, ExpressionKind, FunctionBody,
  Parameter, PrecedenceKind, TokenKind, Type,
};

use super::{ParseResult, Parser};

impl<'a> Parser<'a> {
  /// Parses a function expression (`fn(...) {...}`).
  pub fn parse_fn_expression(&mut self) -> ParseResult {
    let start_span = self.current_token.span;
    self.consume(TokenKind::Fn)?;
    self.consume(TokenKind::LParen)?;

    let parameters = self.parse_fn_parameters()?;
    self.consume(TokenKind::RParen)?;

    let (body, kind) = self.parse_classic_fn_body_and_return_type()?;

    Ok(Expression::new(
      ExpressionKind::Fn {
        parameters,
        body,
        kind,
      },
      start_span,
    ))
  }

  /// Parses function parameters, handling optional types and default values.
  pub fn parse_fn_parameters(&mut self) -> ErrorKindResult<Vec<Parameter>> {
    let mut parameters = Vec::new();
    if self.current_token.kind == TokenKind::RParen {
      return Ok(parameters);
    }

    loop {
      let param_span_start = self.current_token.span;
      let name = self.parse_identifier()?;
      let kind = self.parse_optional_type()?;

      let default = if self.current_token.kind == TokenKind::Equal {
        self.advance();
        Some(Box::new(self.parse_expression(PrecedenceKind::Lowest)?))
      } else {
        None
      };

      parameters.push(Parameter {
        name,
        kind,
        default,
        span: param_span_start,
      });

      if self.current_token.kind != TokenKind::Comma {
        break;
      }
      self.advance();
    }

    Ok(parameters)
  }

  /// Parses the body and return type of an arrow function (`(...) : T => expr`).
  pub fn parse_arrow_fn_body_and_return_type(
    &mut self,
  ) -> ErrorKindResult<(FunctionBody, Option<Type>)> {
    let kind = self.parse_optional_type()?;

    self.consume(TokenKind::Arrow)?;
    if self.current_token.kind == TokenKind::LBrace {
      return Err(ErrorKind::SyntaxError(
        "Arrow functions cannot have a block body.".to_string(),
      ));
    }

    let body_expr = self.parse_expression(PrecedenceKind::Lowest)?;
    let body = FunctionBody::Expression(Box::new(body_expr));
    Ok((body, kind))
  }

  /// Parses the body and return type of a classic function (`fn(...) : T {...}`).
  pub fn parse_classic_fn_body_and_return_type(
    &mut self,
  ) -> ErrorKindResult<(FunctionBody, Option<Type>)> {
    let kind = self.parse_optional_type()?;

    if self.current_token.kind == TokenKind::Arrow {
      return Err(
        ErrorKind::SyntaxError(
          "`fn` functions cannot use `=>` syntax. Use a block body `{...}` instead.".to_string(),
        ),
      );
    }

    let body_block = Box::new(self.parse_block_statement()?);
    let body = FunctionBody::Block(body_block);
    Ok((body, kind))
  }
}
