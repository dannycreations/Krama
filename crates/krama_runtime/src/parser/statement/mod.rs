mod assignment;
mod block;
mod control;
mod flow;
mod function;
mod public;
mod test;

use krama_core::{ErrorKind, Precedence, Statement, StatementKind, TokenKind};

use super::Parser;

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub fn parse_statement(&mut self) -> Result<Statement<'ast>, ErrorKind> {
    let token = self.current_token.clone();

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
      TokenKind::For => self.parse_for_statement(),
      _ => {
        let expression = self.parse_expression(Precedence::Lowest)?;
        let span = expression.span;
        let statement_kind = StatementKind::Expression {
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
