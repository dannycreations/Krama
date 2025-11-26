use krama_core::ast::expression::{Expression, ExpressionKind, FunctionBody};
use krama_core::token::TokenKind;

use crate::parser::precedence::Precedence;
use crate::parser::{ParseError, Parser};

impl<'a, 'ast> Parser<'a, 'ast>
where
  'a: 'ast,
{
  pub(super) fn parse_fn_expression(&mut self) -> ParseError<'ast> {
    let start_span = self.current_token.span;
    self.advance();

    self.consume_token(TokenKind::LParen)?;

    let parameters = self.parse_fn_parameters()?;

    let kind = if self.current_token.kind == TokenKind::Colon {
      self.advance();
      Some(self.parse_type()?)
    } else {
      None
    };

    let body = if self.current_token.kind == TokenKind::Arrow {
      self.advance();
      let body_expr = self.parse_expression(Precedence::Lowest)?;
      FunctionBody::Expression(self.arena.alloc(body_expr))
    } else {
      let body_block = self.arena.alloc(self.parse_block_statement()?);
      FunctionBody::Block(body_block)
    };

    Ok(Expression {
      kind: ExpressionKind::Fn {
        parameters,
        body,
        kind,
      },
      span: start_span,
    })
  }
}
