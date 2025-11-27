use krama_core::{ast::statement::Statement, error::Error, token::TokenKind};

use super::{Parser, Precedence};

pub(super) mod assignment;
pub(super) mod block;
pub(super) mod control;
pub(super) mod flow;
pub(super) mod function;
pub(super) mod public;
pub(super) mod test;

impl<'a, 'ast> Parser<'a, 'ast> {
  pub(super) fn parse_statement(&mut self) -> Result<Statement<'ast>, Error> {
    let token = self.current_token;

    let statement = match token.kind {
      TokenKind::Pub => self.parse_pub_statement(),
      TokenKind::Const => self.parse_const_statement(false, token.span),
      TokenKind::Fn => self.parse_fn_statement(false, token.span),
      TokenKind::Let => self.parse_let_statement(),
      TokenKind::Return => self.parse_return_statement(),
      TokenKind::Break => self.parse_break_statement(),
      TokenKind::Continue => self.parse_continue_statement(),
      TokenKind::Test => self.parse_test_statement(),
      TokenKind::While => self.parse_while_statement(),
      _ => {
        let expression = self.parse_expression(Precedence::Lowest)?;
        let span = expression.span;
        let statement_kind =
          krama_core::ast::statement::StatementKind::Expression {
            expression: self.arena.alloc(expression),
          };
        Ok(Statement::new(statement_kind, span))
      }
    }?;

    if self.current_token.kind == TokenKind::Semicolon {
      self.advance();
    }

    Ok(statement)
  }
}
