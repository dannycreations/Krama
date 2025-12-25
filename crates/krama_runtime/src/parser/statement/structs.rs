use krama_core::{
  ErrorKindResult, FunctionBody, PrecedenceKind, Span, Statement,
  StatementKind, StructField, StructMethod, TokenKind,
};

use crate::parser::Parser;

impl<'a> Parser<'a> {
  /// Parses a struct definition (`struct Name { ... }`).
  pub fn parse_struct_statement(
    &mut self,
    public: bool,
    start_span: Span,
  ) -> ErrorKindResult<Statement> {
    self.consume(TokenKind::Struct)?;
    let name = self.parse_identifier()?.into();
    self.consume(TokenKind::LBrace)?;

    let mut fields = Vec::new();
    let mut methods = Vec::new();

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

  /// Parses a single field within a struct.
  fn parse_struct_field(
    &mut self,
    public: bool,
  ) -> ErrorKindResult<StructField> {
    let start_span = self.current_token.span;
    let name = self.parse_identifier()?.into();
    self.consume(TokenKind::Colon)?;
    let kind = self.parse_type()?;

    let default = if self.current_token.kind == TokenKind::Equal {
      self.advance();
      Some(Box::new(self.parse_expression(PrecedenceKind::Lowest)?))
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

  /// Parses a method definition within a struct.
  fn parse_struct_method(
    &mut self,
    public: bool,
  ) -> ErrorKindResult<StructMethod> {
    let start_span = self.current_token.span;
    self.consume(TokenKind::Fn)?;
    let name = self.parse_identifier()?.into();
    self.consume(TokenKind::LParen)?;

    // Reuse parse_fn_parameters for consistency and to reduce duplication.
    let parameters = self.parse_fn_parameters()?;
    self.consume(TokenKind::RParen)?;

    let (body, kind) = self.parse_classic_fn_body_and_return_type()?;

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
