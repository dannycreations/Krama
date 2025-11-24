use super::Parser;
use bumpalo::collections::Vec as BumpVec;
use krama_core::ast::statement::{
  Binding, DestructuredIdentifier, Statement, StatementKind,
};
use krama_core::error::Error;
use krama_core::error::ErrorKind;
use krama_core::span::Span;
use krama_core::token::TokenKind;

impl<'a, 'ast> Parser<'a, 'ast>
where
  'a: 'ast,
{
  pub(super) fn parse_let_statement(
    &mut self,
  ) -> Result<Statement<'ast>, Error> {
    let start_span = self.current_token.as_ref().unwrap().span;
    self.advance();

    let name = if let Some(krama_core::token::Token {
      kind: krama_core::token::TokenKind::Identifier(name),
      ..
    }) = self.current_token.as_ref()
    {
      self.arena.alloc_str(name)
    } else {
      return Err(Error {
        span: self.current_token.as_ref().unwrap().span,
        kind: ErrorKind::ParserError("Expected identifier after 'let'"),
      });
    };
    self.advance();

    let kind = if self
      .current_token
      .as_ref()
      .is_some_and(|t| t.kind == TokenKind::Colon)
    {
      self.advance();
      Some(self.parse_type()?)
    } else {
      None
    };

    self.advance();

    let value =
      self.parse_expression(super::super::precedence::Precedence::Lowest)?;

    Ok(Statement {
      kind: StatementKind::Let { name, kind, value },
      span: start_span,
    })
  }

  pub(super) fn parse_const_statement(
    &mut self,
    public: bool,
    start_span: Span,
  ) -> Result<Statement<'ast>, Error> {
    self.advance();

    let binding = if self
      .current_token
      .as_ref()
      .is_some_and(|t| t.kind == krama_core::token::TokenKind::LBrace)
    {
      self.advance();
      let items = self.parse_destructured_items()?;
      self.advance();
      Binding::Destructure(items)
    } else {
      let name = if let Some(krama_core::token::Token {
        kind: krama_core::token::TokenKind::Identifier(name),
        ..
      }) = self.current_token.as_ref()
      {
        self.arena.alloc_str(name)
      } else {
        return Err(Error {
          span: start_span,
          kind: ErrorKind::ParserError("Expected identifier after 'const'"),
        });
      };
      self.advance();
      if self
        .current_token
        .as_ref()
        .is_some_and(|t| t.kind == krama_core::token::TokenKind::Comma)
      {
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

    let kind = if self
      .current_token
      .as_ref()
      .is_some_and(|t| t.kind == TokenKind::Colon)
    {
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
        value,
      },
      span: start_span,
    })
  }

  pub(super) fn parse_destructured_items(
    &mut self,
  ) -> Result<BumpVec<'ast, DestructuredIdentifier<'ast>>, Error> {
    let mut items = BumpVec::new_in(self.arena);
    if self
      .current_token
      .as_ref()
      .is_some_and(|t| t.kind == krama_core::token::TokenKind::RBrace)
    {
      return Ok(items);
    }
    loop {
      let name = if let Some(krama_core::token::Token {
        kind: krama_core::token::TokenKind::Identifier(name),
        ..
      }) = self.current_token.as_ref()
      {
        self.arena.alloc_str(name)
      } else {
        return Err(Error {
          span: self.current_token.as_ref().unwrap().span,
          kind: ErrorKind::ParserError("Expected identifier in destructuring"),
        });
      };
      self.advance();

      let alias = if self
        .current_token
        .as_ref()
        .is_some_and(|t| t.kind == krama_core::token::TokenKind::As)
      {
        self.advance();
        let alias_name = if let Some(krama_core::token::Token {
          kind: krama_core::token::TokenKind::Identifier(name),
          ..
        }) = self.current_token.as_ref()
        {
          *name
        } else {
          return Err(Error {
            span: self.current_token.as_ref().unwrap().span,
            kind: ErrorKind::ParserError("Expected identifier after 'as'"),
          });
        };
        self.advance();
        Some(self.arena.alloc_str(alias_name))
      } else {
        None
      };

      items.push(DestructuredIdentifier {
        name,
        alias: alias.map(|s| s as &str),
      });

      if !self
        .current_token
        .as_ref()
        .is_some_and(|t| t.kind == krama_core::token::TokenKind::Comma)
      {
        break;
      }
      self.advance();
    }
    Ok(items)
  }
}
