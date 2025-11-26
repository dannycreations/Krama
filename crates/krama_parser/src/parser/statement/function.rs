use bumpalo::collections::Vec as BumpVec;
use krama_core::{
  ast::statement::{Parameter, Statement, StatementKind},
  error::{Error, ErrorKind},
  span::Span,
  token::TokenKind,
};

use super::Parser;

impl<'a, 'ast> Parser<'a, 'ast>
where
  'a: 'ast,
{
  pub(super) fn parse_fn_statement(
    &mut self,
    public: bool,
    start_span: Span,
  ) -> Result<Statement<'ast>, Error> {
    self.advance();
    let name = if let TokenKind::Identifier(name) = self.current_token.kind {
      self.arena.alloc_str(name)
    } else {
      return Err(Error {
        span: start_span,
        kind: ErrorKind::SyntaxError(
          "Expected function name after 'fn'".to_string(),
        ),
      });
    };
    self.advance();
    self.advance();
    let parameters = self.parse_fn_parameters()?;
    let kind = if self.current_token.kind == TokenKind::Colon {
      self.advance();
      Some(self.parse_type()?)
    } else {
      None
    };

    let body = self.parse_block_statement()?;
    Ok(Statement {
      kind: StatementKind::Fn {
        public,
        name,
        parameters,
        body: self.arena.alloc(body),
        kind,
      },
      span: start_span,
    })
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
