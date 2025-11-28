use bumpalo::collections::Vec as BumpVec;
use krama_core::{
  ast::{
    expression::ExpressionKind, precedence::Precedence, statement::Parameter,
  },
  error::{Error, ErrorKind},
  token::TokenKind,
};

use super::{ParseError, Parser};

impl<'a, 'ast> Parser<'a, 'ast> {
  pub(super) fn parse_paren_expression(&mut self) -> ParseError<'ast> {
    let start_span = self.current_token.span;
    self.advance();

    if self.current_token.kind == TokenKind::RParen {
      self.advance();
      return self
        .parse_fn_expr_with_params(start_span, BumpVec::new_in(self.arena));
    }

    let mut expressions = BumpVec::new_in(self.arena);
    expressions.push(self.parse_expression(Precedence::Lowest)?);

    while self.current_token.kind == TokenKind::Comma {
      self.advance();
      expressions.push(self.parse_expression(Precedence::Lowest)?);
    }

    self.consume_token(TokenKind::RParen)?;

    if self.current_token.kind == TokenKind::Arrow
      || self.current_token.kind == TokenKind::LBrace
    {
      let mut parameters = BumpVec::new_in(self.arena);
      for expr in expressions {
        match expr.kind {
          ExpressionKind::Identifier(name) => {
            parameters.push(Parameter {
              name,
              kind: None,
              default: None,
              span: expr.span,
            });
          }
          ExpressionKind::Assignment {
            left,
            operator,
            right,
          } => {
            if operator != krama_core::ast::operator::AssignmentOperator::Assign
            {
              return Err(Error {
                span: expr.span,
                kind: ErrorKind::SyntaxError(
                  "Invalid expression in function parameters.".to_string(),
                ),
              });
            }
            let name = if let ExpressionKind::Identifier(name) = left.kind {
              name
            } else {
              return Err(Error {
                span: left.span,
                kind: ErrorKind::SyntaxError(
                  "Expected identifier as parameter name".to_string(),
                ),
              });
            };
            parameters.push(Parameter {
              name,
              kind: None,
              default: Some(right),
              span: expr.span,
            });
          }
          _ => {
            return Err(Error {
              span: expr.span,
              kind: ErrorKind::SyntaxError(
                "Invalid expression in function parameters.".to_string(),
              ),
            });
          }
        };
      }
      self.parse_fn_expr_with_params(start_span, parameters)
    } else if expressions.len() == 1 {
      expressions.pop().ok_or_else(|| Error {
        span: start_span,
        kind: ErrorKind::SyntaxError(
          "Expected expression in parentheses".to_string(),
        ),
      })
    } else {
      Err(Error {
        span: start_span,
        kind: ErrorKind::SyntaxError(
          "Invalid grouped expression. It should contain only one expression. Tuples are not supported."
            .to_string(),
        ),
      })
    }
  }
}
