use bumpalo::collections::Vec as BumpVec;
use krama_core::{
  ConstBinding, Destructure, ErrorKind, PrecedenceKind, Span, Statement,
  StatementKind, TokenKind,
};

use super::Parser;

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub fn parse_let_statement(&mut self) -> Result<Statement<'ast>, ErrorKind> {
    let start_span = self.consume(TokenKind::Let)?.span;
    let name = self.parse_identifier()?;

    let kind = self.parse_optional_type()?;

    self.consume(TokenKind::Equal)?;

    let value = self.parse_expression(PrecedenceKind::Lowest)?;

    Ok(Statement::new(
      StatementKind::Let {
        name,
        kind,
        value: self.arena.alloc(value),
      },
      start_span,
    ))
  }

  pub fn parse_const_statement(
    &mut self,
    public: bool,
    start_span: Span,
  ) -> Result<Statement<'ast>, ErrorKind> {
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
        value: self.arena.alloc(value),
      },
      start_span,
    ))
  }

  fn parse_binding(&mut self) -> Result<ConstBinding<'ast>, ErrorKind> {
    if self.current_token.kind == TokenKind::LBrace {
      self.consume(TokenKind::LBrace)?;
      let items = self.parse_destructured_items()?;
      self.consume(TokenKind::RBrace)?;
      Ok(ConstBinding::Destructure(items))
    } else {
      let alias = self.parse_identifier()?;
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

  fn parse_destructured_items(
    &mut self,
  ) -> Result<BumpVec<'ast, Destructure<'ast>>, ErrorKind> {
    let mut items = BumpVec::new_in(self.arena);
    if self.current_token.kind == TokenKind::RBrace {
      return Ok(items);
    }
    loop {
      let name = self.parse_identifier()?;

      let alias = if self.current_token.kind == TokenKind::As {
        self.consume(TokenKind::As)?;
        let alias_name = self.parse_identifier()?;
        Some(self.arena.alloc_str(alias_name))
      } else {
        None
      };

      items.push(Destructure {
        name,
        alias: alias.map(|s| s as &str),
      });

      if self.current_token.kind != TokenKind::Comma {
        break;
      }
      self.consume(TokenKind::Comma)?;
    }
    Ok(items)
  }
}
