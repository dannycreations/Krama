use bumpalo::collections::Vec as BumpVec;
use krama_core::{
  ast::{
    precedence::Precedence,
    statement::{Binding, DestructuredIdentifier, Statement, StatementKind},
  },
  error::ErrorKind,
  span::Span,
  token::TokenKind,
};

use crate::parser::Parser;

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub(super) fn parse_let_statement(
    &mut self,
  ) -> Result<Statement<'ast>, (ErrorKind, Span<'a>)> {
    let start_span = self.consume(TokenKind::Let)?.span;
    let name = self.parse_identifier()?;

    let kind = if self.current_token.kind == TokenKind::Colon {
      self.advance();
      Some(self.parse_type()?)
    } else {
      None
    };

    self.consume(TokenKind::Equal)?;

    let value = self.parse_expression(Precedence::Lowest)?;

    Ok(Statement::new(
      StatementKind::Let {
        name,
        kind,
        value: self.arena.alloc(value),
      },
      start_span,
    ))
  }

  pub(super) fn parse_const_statement(
    &mut self,
    public: bool,
    start_span: Span<'a>,
  ) -> Result<Statement<'ast>, (ErrorKind, Span<'a>)> {
    self.consume(TokenKind::Const)?;

    let binding = if self.current_token.kind == TokenKind::LBrace {
      self.consume(TokenKind::LBrace)?;
      let items = self.parse_destructured_items()?;
      self.consume(TokenKind::RBrace)?;
      Binding::Destructure(items)
    } else {
      let name = self.parse_identifier()?;
      if self.current_token.kind == TokenKind::Comma {
        self.consume(TokenKind::Comma)?;
        self.consume(TokenKind::LBrace)?;
        let items = self.parse_destructured_items()?;
        self.consume(TokenKind::RBrace)?;
        Binding::ModuleAndDestructure {
          module_alias: name,
          items,
        }
      } else {
        Binding::Identifier(name)
      }
    };

    let kind = if self.current_token.kind == TokenKind::Colon {
      self.advance();
      Some(self.parse_type()?)
    } else {
      None
    };

    self.consume(TokenKind::Equal)?;

    let value = self.parse_expression(Precedence::Lowest)?;
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

  pub(super) fn parse_destructured_items(
    &mut self,
  ) -> Result<BumpVec<'ast, DestructuredIdentifier<'ast>>, (ErrorKind, Span<'a>)>
  {
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

      items.push(DestructuredIdentifier {
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
