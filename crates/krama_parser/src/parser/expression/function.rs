use bumpalo::collections::Vec as BumpVec;
use krama_core::{
  ast::{
    expression::{Expression, ExpressionKind, FunctionBody},
    precedence::Precedence,
    statement::Parameter,
    types::Type,
  },
  error::ErrorKind,
  span::Span,
  token::TokenKind,
};

use crate::parser::{ParseResult, Parser};

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub(super) fn parse_fn_expression(&mut self) -> ParseResult<'a, 'ast> {
    let start_span = self.current_token.span.clone();
    self.advance();

    self.consume(TokenKind::LParen)?;

    let parameters = self.parse_fn_parameters()?;
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

  pub(crate) fn parse_fn_parameters(
    &mut self,
  ) -> Result<BumpVec<'ast, Parameter<'ast>>, ErrorKind> {
    let mut parameters = BumpVec::new_in(self.arena);
    if self.current_token.kind == TokenKind::RParen {
      self.advance();
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
        Some(self.arena.alloc(self.parse_expression(Precedence::Lowest)?))
      } else {
        None
      };

      let span = param_span_start;
      parameters.push(Parameter {
        name,
        kind,
        default: default.map(|expr| &*expr),
        span,
      });

      if self.current_token.kind != TokenKind::Comma {
        break;
      }
      self.advance();
    }

    if self.current_token.kind != TokenKind::RParen {
      return Err(ErrorKind::SyntaxError(format!(
        "Expected {} after parameters",
        TokenKind::RParen
      )));
    }
    self.advance();

    Ok(parameters)
  }

  pub(super) fn build_fn_expression(
    &mut self,
    start_span: Span<'a>,
    parameters: BumpVec<'ast, Parameter<'ast>>,
  ) -> ParseResult<'a, 'ast> {
    let (body, kind) = self.parse_arrow_fn_body_and_return_type()?;

    Ok(Expression::new(
      ExpressionKind::Fn {
        parameters,
        body,
        kind,
      },
      start_span,
    ))
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

  fn parse_arrow_fn_body_and_return_type(
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
}
