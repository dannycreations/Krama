use bumpalo::collections::Vec as BumpVec;
use krama_core::{
  ast::{
    precedence::Precedence,
    statement::{Binding, DestructuredIdentifier, Statement, StatementKind},
  },
  error::Error,
  span::Span,
  token::TokenKind,
};

use super::Parser;

impl<'a, 'ast> Parser<'a, 'ast> {
  pub(super) fn parse_let_statement(
    &mut self,
  ) -> Result<Statement<'ast>, Error> {
    let start_span = self.consume_token_and_get(TokenKind::Let)?.span;
    let name = self.parse_identifier()?;

    let kind = if self.current_token.kind == TokenKind::Colon {
      self.advance();
      Some(self.parse_type()?)
    } else {
      None
    };

    self.consume_token(TokenKind::Equal)?;

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
    start_span: Span,
  ) -> Result<Statement<'ast>, Error> {
    self.consume_token(TokenKind::Const)?;

    let binding = if self.current_token.kind == TokenKind::LBrace {
      self.consume_token(TokenKind::LBrace)?;
      let items = self.parse_destructured_items()?;
      self.consume_token(TokenKind::RBrace)?;
      Binding::Destructure(items)
    } else {
      let name = self.parse_identifier()?;
      if self.current_token.kind == TokenKind::Comma {
        self.consume_token(TokenKind::Comma)?;
        self.consume_token(TokenKind::LBrace)?;
        let items = self.parse_destructured_items()?;
        self.consume_token(TokenKind::RBrace)?;
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

    self.consume_token(TokenKind::Equal)?;

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
  ) -> Result<BumpVec<'ast, DestructuredIdentifier<'ast>>, Error> {
    let mut items = BumpVec::new_in(self.arena);
    if self.current_token.kind == TokenKind::RBrace {
      return Ok(items);
    }
    loop {
      let name = self.parse_identifier()?;

      let alias = if self.current_token.kind == TokenKind::As {
        self.consume_token(TokenKind::As)?;
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
      self.consume_token(TokenKind::Comma)?;
    }
    Ok(items)
  }
}
