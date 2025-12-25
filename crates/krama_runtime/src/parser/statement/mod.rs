mod assignment;
mod block;
mod control;
mod enums;
mod function;
mod iteration;
mod public;
mod structs;
mod test;
mod types;

use krama_core::{
  ErrorKind, PrecedenceKind, Statement, StatementKind, TokenKind,
};

use super::Parser;

impl<'a> Parser<'a> {
  pub fn parse_statement(&mut self) -> Result<Statement, ErrorKind> {
    let token = self.current_token.clone();

    let statement = match token.kind {
      TokenKind::Pub => self.parse_pub_statement(),
      TokenKind::Const => self.parse_const_statement(false, token.span),
      TokenKind::Fn => self.parse_fn_statement(false, token.span),
      TokenKind::Enum => self.parse_enum_statement(false, token.span),
      TokenKind::Struct => self.parse_struct_statement(false, token.span),
      TokenKind::Type => self.parse_type_statement(false, token.span),
      TokenKind::Let => self.parse_let_statement(),
      TokenKind::Return => self.parse_return_statement(),
      TokenKind::Break => self.parse_break_statement(),
      TokenKind::Continue => self.parse_continue_statement(),
      TokenKind::Test => self.parse_test_statement(),
      TokenKind::While => self.parse_while_statement(),
      TokenKind::For => self.parse_for_statement(),
      _ => {
        let expression = self.parse_expression(PrecedenceKind::Lowest)?;
        let span = expression.span;
        let statement_kind = StatementKind::Expression {
          expression: Box::new(expression),
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
