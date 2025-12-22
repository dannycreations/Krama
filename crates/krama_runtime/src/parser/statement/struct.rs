use bumpalo::collections::Vec as BumpVec;
use krama_core::{
  ErrorKind, FunctionBody, Parameter, PrecedenceKind, Span, Statement,
  StatementKind, StructField, StructMethod, TokenKind,
};

use crate::parser::Parser;

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub fn parse_struct_statement(
    &mut self,
    public: bool,
    start_span: Span,
  ) -> Result<Statement<'ast>, ErrorKind> {
    self.consume(TokenKind::Struct)?;
    let name = self.parse_identifier()?;
    self.consume(TokenKind::LBrace)?;

    let mut fields = BumpVec::new_in(self.arena);
    let mut methods = BumpVec::new_in(self.arena);

    while self.current_token.kind != TokenKind::RBrace {
      let is_pub = if self.current_token.kind == TokenKind::Pub {
        self.advance();
        true
      } else {
        false
      };

      if self.current_token.kind == TokenKind::Fn {
        methods.push(self.parse_struct_method(is_pub)?);
      } else {
        fields.push(self.parse_struct_field(is_pub)?);
      }

      if self.current_token.kind == TokenKind::RBrace {
        break;
      }
    }

    let end_span = self.consume(TokenKind::RBrace)?.span;

    Ok(Statement::new(
      StatementKind::Struct {
        public,
        name,
        fields,
        methods,
      },
      start_span.merge(&end_span),
    ))
  }

  fn parse_struct_field(
    &mut self,
    public: bool,
  ) -> Result<StructField<'ast>, ErrorKind> {
    let start_span = self.current_token.span;
    let name = self.parse_identifier()?;
    self.consume(TokenKind::Colon)?;
    let kind = self.parse_type()?;

    let default = if self.current_token.kind == TokenKind::Equal {
      self.advance();
      Some(
        &*self
          .arena
          .alloc(self.parse_expression(PrecedenceKind::Lowest)?),
      )
    } else {
      None
    };

    let mut end_span = kind.span;
    if let Some(default_val) = &default {
      end_span = default_val.span;
    }

    if self.current_token.kind == TokenKind::Comma {
      self.advance();
    }

    Ok(StructField {
      public,
      name,
      kind,
      default,
      span: start_span.merge(&end_span),
    })
  }

  fn parse_struct_method(
    &mut self,
    public: bool,
  ) -> Result<StructMethod<'ast>, ErrorKind> {
    let start_span = self.current_token.span;
    self.consume(TokenKind::Fn)?;
    let name = self.parse_identifier()?;
    self.consume(TokenKind::LParen)?;

    let mut parameters = BumpVec::new_in(self.arena);
    if self.current_token.kind != TokenKind::RParen {
      loop {
        let param_start = self.current_token.span;
        let param_name = self.parse_identifier()?;
        self.consume(TokenKind::Colon)?;
        let param_kind = self.parse_type()?;

        let mut param_span = param_start.merge(&param_kind.span);
        let default = if self.current_token.kind == TokenKind::Equal {
          self.advance();
          let expr = self.parse_expression(PrecedenceKind::Lowest)?;
          param_span = param_span.merge(&expr.span);
          Some(&*self.arena.alloc(expr))
        } else {
          None
        };

        parameters.push(Parameter {
          name: param_name,
          kind: Some(param_kind),
          default,
          span: param_span,
        });

        if self.current_token.kind == TokenKind::RParen {
          break;
        }
        self.consume(TokenKind::Comma)?;
        if self.current_token.kind == TokenKind::RParen {
          break;
        }
      }
    }
    self.consume(TokenKind::RParen)?;

    let kind = self.parse_optional_type()?;

    let body = if self.current_token.kind == TokenKind::LBrace {
      FunctionBody::Block(self.arena.alloc(self.parse_block_statement()?))
    } else {
      self.consume(TokenKind::Arrow)?;
      FunctionBody::Expression(
        self
          .arena
          .alloc(self.parse_expression(PrecedenceKind::Lowest)?),
      )
    };

    let end_span = match &body {
      FunctionBody::Block(b) => b.span,
      FunctionBody::Expression(e) => e.span,
    };

    Ok(StructMethod {
      public,
      name,
      parameters,
      body,
      kind,
      span: start_span.merge(&end_span),
    })
  }
}
