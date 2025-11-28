use bumpalo::collections::Vec as BumpVec;
use krama_core::{
  ast::{
    expression::{Expression, ExpressionKind, FunctionBody},
    precedence::Precedence,
    statement::Parameter,
  },
  error::{Error, ErrorKind},
  span::Span,
  token::TokenKind,
};

use crate::parser::{ParseError, Parser};

impl<'a, 'ast> Parser<'a, 'ast> {
  pub(super) fn parse_fn_expression(&mut self) -> ParseError<'ast> {
    let start_span = self.current_token.span;
    self.advance();

    self.consume_token(TokenKind::LParen)?;

    let parameters = self.parse_fn_parameters()?;

    let kind = if self.current_token.kind == TokenKind::Colon {
      self.advance();
      Some(self.parse_type()?)
    } else {
      None
    };

    let body = if self.current_token.kind == TokenKind::Arrow {
      self.advance();
      let body_expr = self.parse_expression(Precedence::Lowest)?;
      FunctionBody::Expression(self.arena.alloc(body_expr))
    } else {
      let body_block = self.arena.alloc(self.parse_block_statement()?);
      FunctionBody::Block(body_block)
    };

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
  ) -> Result<BumpVec<'ast, Parameter<'ast>>, Error> {
    let mut parameters = BumpVec::new_in(self.arena);
    if self.current_token.kind == TokenKind::RParen {
      self.advance();
      return Ok(parameters);
    }

    loop {
      let param_span_start = self.current_token.span;
      let name = if let TokenKind::Identifier(name) = self.current_token.kind {
        self.arena.alloc_str(name)
      } else {
        return Err(Error {
          span: self.current_token.span,
          kind: ErrorKind::SyntaxError("Expected parameter name".to_string()),
        });
      };
      self.advance();

      let kind = if self.current_token.kind == TokenKind::Colon {
        self.advance();
        Some(self.parse_type()?)
      } else {
        None
      };

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
      return Err(Error {
        span: self.current_token.span,
        kind: ErrorKind::SyntaxError(format!(
          "Expected {} after parameters",
          TokenKind::RParen
        )),
      });
    }
    self.advance();

    Ok(parameters)
  }

  pub(super) fn parse_fn_expr_with_params(
    &mut self,
    start_span: Span,
    parameters: BumpVec<'ast, Parameter<'ast>>,
  ) -> ParseError<'ast> {
    let body = if self.current_token.kind == TokenKind::Arrow {
      self.advance();
      let body_expr = self.parse_expression(Precedence::Lowest)?;
      FunctionBody::Expression(self.arena.alloc(body_expr))
    } else {
      let body_block = self.arena.alloc(self.parse_block_statement()?);
      FunctionBody::Block(body_block)
    };

    let kind = if self.current_token.kind == TokenKind::Colon {
      self.advance();
      Some(self.parse_type()?)
    } else {
      None
    };

    Ok(Expression::new(
      ExpressionKind::Fn {
        parameters,
        body,
        kind,
      },
      start_span,
    ))
  }
}
