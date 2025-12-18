use bumpalo::collections::Vec as BumpVec;
use krama_core::{
  ErrorKind, Expression, ExpressionKind, FunctionBody, Parameter, Precedence,
  TokenKind, Type,
};

use super::{ParseResult, Parser};

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub fn parse_fn_expression(&mut self) -> ParseResult<'a, 'ast> {
    let start_span = self.current_token.span.clone();
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

  pub fn parse_fn_parameters(
    &mut self,
  ) -> Result<BumpVec<'ast, Parameter<'ast>>, ErrorKind> {
    let mut parameters = BumpVec::new_in(self.arena);
    if self.current_token.kind == TokenKind::RParen {
      return Ok(parameters);
    }

    loop {
      let param_span_start = self.current_token.span.clone();
      let name = if let TokenKind::Identifier(name) = self.current_token.kind {
        self.arena.alloc_str(name)
      } else {
        return Err(ErrorKind::SyntaxError(
          "Expected parameter name".to_string(),
        ));
      };
      self.advance();

      let kind = self.parse_optional_type()?;

      let default = if self.current_token.kind == TokenKind::Equal {
        self.advance();
        Some(&*self.arena.alloc(self.parse_expression(Precedence::Lowest)?))
      } else {
        None
      };

      let span = param_span_start;
      parameters.push(Parameter {
        name,
        kind,
        default,
        span,
      });

      if self.current_token.kind != TokenKind::Comma {
        break;
      }
      self.advance();
    }

    Ok(parameters)
  }

  pub fn parse_arrow_fn_body_and_return_type(
    &mut self,
  ) -> Result<(FunctionBody<'ast>, Option<Type<'ast>>), ErrorKind> {
    let kind = self.parse_optional_type()?;

    self.consume(TokenKind::Arrow)?;
    if self.current_token.kind == TokenKind::LBrace {
      return Err(ErrorKind::SyntaxError(
        "Arrow functions cannot have a block body.".to_string(),
      ));
    }

    let body_expr = self.parse_expression(Precedence::Lowest)?;
    let body = FunctionBody::Expression(self.arena.alloc(body_expr));
    Ok((body, kind))
  }

  fn parse_classic_fn_body_and_return_type(
    &mut self,
  ) -> Result<(FunctionBody<'ast>, Option<Type<'ast>>), ErrorKind> {
    let kind = self.parse_optional_type()?;

    if self.current_token.kind == TokenKind::Arrow {
      return Err(
        ErrorKind::SyntaxError(
          "`fn` functions cannot use `=>` syntax. Use a block body `{...}` instead.".to_string(),
        ),
      );
    }

    let body_block = self.arena.alloc(self.parse_block_statement()?);
    let body = FunctionBody::Block(body_block);
    Ok((body, kind))
  }
}
