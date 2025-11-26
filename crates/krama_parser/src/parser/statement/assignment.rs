use bumpalo::collections::Vec as BumpVec;
use krama_core::ast::statement::{
  Binding, DestructuredIdentifier, Statement, StatementKind,
};
use krama_core::error::Error;
use krama_core::span::Span;
use krama_core::token::TokenKind;

use super::Parser;

impl<'a, 'ast> Parser<'a, 'ast>
where
  'a: 'ast,
{
  pub(super) fn parse_let_statement(
    &mut self,
  ) -> Result<Statement<'ast>, Error> {
    let start_span = self.current_token.span;
    self.advance();

    let name = self.parse_identifier()?;

    let kind = if self.current_token.kind == TokenKind::Colon {
      self.advance();
      Some(self.parse_type()?)
    } else {
      None
    };

    self.advance();

    let value =
      self.parse_expression(super::super::precedence::Precedence::Lowest)?;

    Ok(Statement {
      kind: StatementKind::Let {
        name,
        kind,
        value: self.arena.alloc(value),
      },
      span: start_span,
    })
  }

  pub(super) fn parse_const_statement(
    &mut self,
    public: bool,
    start_span: Span,
  ) -> Result<Statement<'ast>, Error> {
    self.advance();

    let binding = if self.current_token.kind == TokenKind::LBrace {
      self.advance();
      let items = self.parse_destructured_items()?;
      self.advance();
      Binding::Destructure(items)
    } else {
      let name = self.parse_identifier()?;
      if self.current_token.kind == TokenKind::Comma {
        self.advance();
        self.advance();
        let items = self.parse_destructured_items()?;
        self.advance();
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

    self.advance();

    let value =
      self.parse_expression(super::super::precedence::Precedence::Lowest)?;
    Ok(Statement {
      kind: StatementKind::Const {
        public,
        binding,
        kind,
        value: self.arena.alloc(value),
      },
      span: start_span,
    })
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
        self.advance();
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
      self.advance();
    }
    Ok(items)
  }
}
