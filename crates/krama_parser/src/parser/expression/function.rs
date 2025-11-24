use crate::parser::precedence::Precedence;
use crate::parser::ParseError;
use crate::parser::Parser;
use krama_core::ast::expression::{Expression, ExpressionKind, FunctionBody};
use krama_core::token::TokenKind;

impl<'a, 'ast> Parser<'a, 'ast>
where
  'a: 'ast,
{
  pub(super) fn parse_fn_expression(&mut self) -> ParseError<'ast> {
    let start_span = self.current_token.as_ref().unwrap().span;
    self.advance();

    self.consume_token(TokenKind::LParen)?;

    let parameters = self.parse_fn_parameters()?;

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

    let body = if self
      .current_token
      .as_ref()
      .is_some_and(|t| t.kind == TokenKind::Arrow)
    {
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
