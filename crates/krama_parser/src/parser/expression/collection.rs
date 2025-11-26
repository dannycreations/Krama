use bumpalo::collections::Vec as BumpVec;
use krama_core::ast::expression::{Expression, ExpressionKind};
use krama_core::token::TokenKind;

use crate::parser::precedence::Precedence;
use crate::parser::{ParseError, Parser};

impl<'a, 'ast> Parser<'a, 'ast>
where
  'a: 'ast,
{
  pub(super) fn parse_collection_expression(&mut self) -> ParseError<'ast> {
    let start_span = self.current_token.span;
    self.consume_token(TokenKind::LBracket).unwrap();

    let mut elements = BumpVec::new_in(self.arena);

    // Check for empty collection
    if self.current_token.kind == TokenKind::RBracket {
      let end_span = self.current_token.span;
      self.advance();
      return Ok(Expression {
        kind: ExpressionKind::Collection { elements },
        span: start_span.merge(&end_span),
      });
    }

    // Parse expressions
    loop {
      elements.push(self.parse_expression(Precedence::Lowest)?);
      if self.current_token.kind == TokenKind::RBracket {
        break;
      }
      self.consume_token(TokenKind::Comma).unwrap();
      if self.current_token.kind == TokenKind::RBracket {
        // Allow trailing comma
        break;
      }
    }

    let end_span = self.current_token.span;
    self.consume_token(TokenKind::RBracket).unwrap();

    Ok(Expression {
      kind: ExpressionKind::Collection { elements },
      span: start_span.merge(&end_span),
    })
  }
}
