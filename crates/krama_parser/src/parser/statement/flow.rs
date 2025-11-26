use super::Parser;
use krama_core::ast::statement::{Statement, StatementKind};
use krama_core::error::Error;
use krama_core::token::TokenKind;

impl<'a, 'ast> Parser<'a, 'ast>
where
  'a: 'ast,
{
  pub(super) fn parse_return_statement(
    &mut self,
  ) -> Result<Statement<'ast>, Error> {
    let start_span = self.current_token.as_ref().unwrap().span;
    self.advance();

    let value = if self
      .current_token
      .as_ref()
      .is_some_and(|t| t.kind != TokenKind::Newline)
    {
      Some(self.parse_expression(super::super::precedence::Precedence::Lowest)?)
    } else {
      None
    };

    Ok(Statement {
      kind: StatementKind::Return {
        value: value.map(|v| self.arena.alloc(v) as &_),
      },
      span: start_span,
    })
  }

  pub(super) fn parse_break_statement(
    &mut self,
  ) -> Result<Statement<'ast>, Error> {
    let start_span = self.current_token.as_ref().unwrap().span;
    self.advance();
    Ok(Statement {
      kind: StatementKind::Break,
      span: start_span,
    })
  }

  pub(super) fn parse_continue_statement(
    &mut self,
  ) -> Result<Statement<'ast>, Error> {
    let start_span = self.current_token.as_ref().unwrap().span;
    self.advance();
    Ok(Statement {
      kind: StatementKind::Continue,
      span: start_span,
    })
  }
}
