use bumpalo::collections::Vec as BumpVec;
use krama_core::{
  ast::{
    expression::{Expression, ExpressionKind, FunctionBody},
    statement::Parameter,
  },
  error::{Error, ErrorKind},
  token::TokenKind,
};

use crate::parser::{precedence::Precedence, ParseError, Parser};

impl<'a, 'ast> Parser<'a, 'ast>
where
  'a: 'ast,
{
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

      let span = param_span_start;
      parameters.push(Parameter { name, kind, span });

      if self.current_token.kind != TokenKind::Comma {
        break;
      }
      self.advance();
    }

    if self.current_token.kind != TokenKind::RParen {
      return Err(Error {
        span: self.current_token.span,
        kind: ErrorKind::SyntaxError(
          "Expected ')' after parameters".to_string(),
        ),
      });
    }
    self.advance();

    Ok(parameters)
  }
}
