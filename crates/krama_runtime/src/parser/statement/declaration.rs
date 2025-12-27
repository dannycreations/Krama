use std::sync::Arc;

use krama_core::{
  ConstBinding, Destructure, EnumVariant, ErrorKind, ErrorKindResult,
  FunctionBody, PrecedenceKind, Span, Statement, StatementKind, StructField,
  StructMethod, TokenKind,
};

use crate::Parser;

impl<'a> Parser<'a> {
  pub fn parse_pub_statement(&mut self) -> ErrorKindResult<Statement> {
    let start_span = self.current_token.span;
    self.advance();
    match self.current_token.kind {
      TokenKind::Const => self.parse_const_statement(true, start_span),
      TokenKind::Fn => self.parse_fn_statement(true, start_span),
      TokenKind::Enum => self.parse_enum_statement(true, start_span),
      TokenKind::Struct => self.parse_struct_statement(true, start_span),
      _ => Err(ErrorKind::SyntaxError(
        "Expected 'const', 'fn', 'enum' or 'struct' after 'pub'".to_string(),
      )),
    }
  }

  pub fn parse_let_statement(&mut self) -> ErrorKindResult<Statement> {
    let start_span = self.consume(TokenKind::Let)?.span;
    let name = self.parse_identifier()?;
    let kind = self.parse_optional_type()?;
    self.consume(TokenKind::Equal)?;
    let value = self.parse_expression(PrecedenceKind::Lowest)?;
    Ok(Statement::new(
      StatementKind::Let {
        name: name.into(),
        kind,
        value: Box::new(value),
      },
      start_span,
    ))
  }

  pub fn parse_const_statement(
    &mut self,
    public: bool,
    start_span: Span,
  ) -> ErrorKindResult<Statement> {
    self.consume(TokenKind::Const)?;
    let binding = self.parse_binding()?;
    let kind = self.parse_optional_type()?;
    self.consume(TokenKind::Equal)?;
    let value = self.parse_expression(PrecedenceKind::Lowest)?;
    Ok(Statement::new(
      StatementKind::Const {
        public,
        binding,
        kind,
        value: Box::new(value),
      },
      start_span,
    ))
  }

  fn parse_binding(&mut self) -> ErrorKindResult<ConstBinding> {
    if self.current_token.kind == TokenKind::LBrace {
      self.consume(TokenKind::LBrace)?;
      let items = self.parse_destructured_items()?;
      self.consume(TokenKind::RBrace)?;
      Ok(ConstBinding::Destructure(items))
    } else {
      let alias: Arc<str> = self.parse_identifier()?.into();
      if self.current_token.kind == TokenKind::Comma {
        self.consume(TokenKind::Comma)?;
        self.consume(TokenKind::LBrace)?;
        let items = self.parse_destructured_items()?;
        self.consume(TokenKind::RBrace)?;
        Ok(ConstBinding::ModuleAndDestructure { alias, items })
      } else {
        Ok(ConstBinding::Identifier(alias))
      }
    }
  }

  fn parse_destructured_items(&mut self) -> ErrorKindResult<Vec<Destructure>> {
    let mut items = Vec::new();
    if self.current_token.kind == TokenKind::RBrace {
      return Ok(items);
    }
    loop {
      let name = self.parse_identifier()?.into();
      let alias = if self.current_token.kind == TokenKind::As {
        self.advance();
        Some(self.parse_identifier()?.into())
      } else {
        None
      };
      items.push(Destructure { name, alias });
      if self.current_token.kind != TokenKind::Comma {
        break;
      }
      self.advance();
    }
    Ok(items)
  }

  pub fn parse_enum_statement(
    &mut self,
    public: bool,
    start_span: Span,
  ) -> ErrorKindResult<Statement> {
    self.consume(TokenKind::Enum)?;
    let name = self.parse_identifier()?.into();
    self.consume(TokenKind::LBrace)?;
    let mut variants = Vec::new();
    while self.current_token.kind != TokenKind::RBrace {
      let variant_span = self.current_token.span;
      let variant_name = self.parse_identifier()?.into();
      let fields = if self.current_token.kind == TokenKind::LParen {
        self.advance();
        let mut fields = Vec::new();
        if self.current_token.kind != TokenKind::RParen {
          loop {
            fields.push(self.parse_type()?);
            if self.current_token.kind == TokenKind::RParen {
              break;
            }
            self.consume(TokenKind::Comma)?;
          }
        }
        self.consume(TokenKind::RParen)?;
        Some(fields)
      } else {
        None
      };
      let end_variant_span = self.current_token.span;
      variants.push(EnumVariant {
        name: variant_name,
        fields,
        span: variant_span.merge(&end_variant_span),
      });
      if self.current_token.kind == TokenKind::Comma {
        self.advance();
      }
    }
    let end_span = self.consume(TokenKind::RBrace)?.span;
    Ok(Statement::new(
      StatementKind::Enum {
        public,
        name,
        variants,
      },
      start_span.merge(&end_span),
    ))
  }

  pub fn parse_fn_statement(
    &mut self,
    public: bool,
    start_span: Span,
  ) -> ErrorKindResult<Statement> {
    self.advance();
    let name = self.parse_identifier()?.into();
    self.consume(TokenKind::LParen)?;
    let parameters = self.parse_fn_parameters()?;
    self.consume(TokenKind::RParen)?;
    let (body, kind) = self.parse_classic_fn_body_and_return_type()?;
    Ok(Statement::new(
      StatementKind::Fn {
        public,
        name,
        parameters,
        body,
        kind,
      },
      start_span,
    ))
  }

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

  fn parse_struct_method(
    &mut self,
    public: bool,
  ) -> ErrorKindResult<StructMethod> {
    let start_span = self.current_token.span;
    self.consume(TokenKind::Fn)?;
    let name = self.parse_identifier()?.into();
    self.consume(TokenKind::LParen)?;
    let mut instance = false;
    let mut parameters = Vec::new();
    if self.current_token.kind == TokenKind::This {
      instance = true;
      self.advance();
      if self.current_token.kind == TokenKind::Comma {
        self.advance();
      }
    }
    parameters.extend(self.parse_fn_parameters()?);
    self.consume(TokenKind::RParen)?;
    let (body, kind) = self.parse_classic_fn_body_and_return_type()?;
    let end_span = match &body {
      FunctionBody::Block(b) => b.span,
      FunctionBody::Expression(e) => e.span,
    };
    Ok(StructMethod {
      public,
      instance,
      name,
      parameters,
      body,
      kind,
      span: start_span.merge(&end_span),
    })
  }

  pub fn parse_type_statement(
    &mut self,
    public: bool,
    start_span: Span,
  ) -> ErrorKindResult<Statement> {
    self.consume(TokenKind::Type)?;
    let name = self.parse_identifier()?.into();
    self.consume(TokenKind::Equal)?;
    let kind = self.parse_type()?;
    let end_span = kind.span;
    Ok(Statement::new(
      StatementKind::Type { public, name, kind },
      start_span.merge(&end_span),
    ))
  }
}
