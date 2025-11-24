use crate::parser::precedence::Precedence;
use crate::parser::ParseError;
use crate::parser::Parser;
use bumpalo::collections::Vec as BumpVec;
use krama_core::ast::expression::Expression;
use krama_core::ast::expression::ExpressionKind;
use krama_core::token::TokenKind;

impl<'a, 'ast> Parser<'a, 'ast>
where
  'a: 'ast,
{
  pub(super) fn parse_collection_expression(&mut self) -> ParseError<'ast> {
    let start_span = self.current_token.as_ref().unwrap().span;
    self.consume_token(TokenKind::LBracket).unwrap();

    let mut elements = BumpVec::new_in(self.arena);

    // Check for empty collection
    if self
      .current_token
      .as_ref()
      .is_some_and(|t| t.kind == TokenKind::RBracket)
    {
      let end_span = self.current_token.as_ref().unwrap().span;
      self.advance();
      return Ok(Expression {
        kind: ExpressionKind::Collection { elements },
        span: start_span.merge(&end_span),
      });
    }

    // Parse expressions
    loop {
      elements.push(self.parse_expression(Precedence::Lowest)?);
      if self
        .current_token
        .as_ref()
        .is_some_and(|t| t.kind == TokenKind::RBracket)
      {
        break;
      }
      self.consume_token(TokenKind::Comma).unwrap();
      if self
        .current_token
        .as_ref()
        .is_some_and(|t| t.kind == TokenKind::RBracket)
      {
        // Allow trailing comma
        break;
      }
    }

    let end_span = self.current_token.as_ref().unwrap().span;
    self.consume_token(TokenKind::RBracket).unwrap();

    Ok(Expression {
      kind: ExpressionKind::Collection { elements },
      span: start_span.merge(&end_span),
    })
  }
}
