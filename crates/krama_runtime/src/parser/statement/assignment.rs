use std::sync::Arc;

use krama_core::{
  ConstBinding, Destructure, ErrorKindResult, PrecedenceKind, Span, Statement,
  StatementKind, TokenKind,
};

use super::Parser;

impl<'a> Parser<'a> {
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
}
