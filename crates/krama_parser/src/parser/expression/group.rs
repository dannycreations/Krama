use bumpalo::collections::Vec as BumpVec;
use krama_core::ast::expression::{Expression, ExpressionKind, FunctionBody};
use krama_core::error::{Error, ErrorKind};
use krama_core::token::TokenKind;

use super::{ParseError, Parser, Precedence};

impl<'a, 'ast> Parser<'a, 'ast>
where
  'a: 'ast,
{
  pub(super) fn parse_paren_expression(&mut self) -> ParseError<'ast> {
    let start_span = self.current_token.span;
    self.advance();

    // Handle empty parameter list `()` for a function
    if self.current_token.kind == TokenKind::RParen {
      self.advance();

      if self.current_token.kind == TokenKind::Arrow {
        self.advance();
        let body = self.parse_expression(Precedence::Lowest)?;
        return Ok(Expression {
          kind: ExpressionKind::Fn {
            parameters: BumpVec::new_in(self.arena),
            body: FunctionBody::Expression(self.arena.alloc(body)),
            kind: None,
          },
          span: start_span,
        });
      }

      if self.current_token.kind == TokenKind::LBrace {
        let body = self.arena.alloc(self.parse_block_statement()?);
        return Ok(Expression {
          kind: ExpressionKind::Fn {
            parameters: BumpVec::new_in(self.arena),
            body: FunctionBody::Block(body),
            kind: None,
          },
          span: start_span,
        });
      }

      return Err(Error {
        span: start_span,
        kind: ErrorKind::SyntaxError(
          "Empty parenthesis are only valid for function expressions."
            .to_string(),
        ),
      });
    }

    // It's not `()`, so parse contents.
    let mut expressions = BumpVec::new_in(self.arena);
    expressions.push(self.parse_expression(Precedence::Lowest)?);

    while self.current_token.kind == TokenKind::Comma {
      self.advance();
      expressions.push(self.parse_expression(Precedence::Lowest)?);
    }

    if self.current_token.kind != TokenKind::RParen {
      return Err(Error {
        span: self.current_token.span,
        kind: ErrorKind::SyntaxError(
          "Expected ')' after expression or parameter list".to_string(),
        ),
      });
    }
    self.advance();

    // Now, check if it's a function or a grouped expression.
    let is_arrow_func = self.current_token.kind == TokenKind::Arrow;
    let is_block_func = self.current_token.kind == TokenKind::LBrace;

    if is_arrow_func || is_block_func {
      // It's a function. Validate that `expressions` is a valid parameter list.
      let parameters = self.parse_fn_parameters()?;

      let body = if is_arrow_func {
        self.advance();
        let body_expr = self.parse_expression(Precedence::Lowest)?;
        FunctionBody::Expression(self.arena.alloc(body_expr))
      } else {
        // is_block_func
        let body_block = self.arena.alloc(self.parse_block_statement()?);
        FunctionBody::Block(body_block)
      };

      let kind = if self.current_token.kind == TokenKind::Colon {
        self.advance();
        Some(self.parse_type()?)
      } else {
        None
      };

      return Ok(Expression {
        kind: ExpressionKind::Fn {
          parameters,
          body,
          kind,
        },
        span: start_span,
      });
    }

    // It's not a function. It must be a grouped expression.
    if expressions.len() == 1 {
      // This is a valid grouped expression
      Ok(expressions.pop().unwrap())
    } else {
      // e.g. `(1, 2)` which is not a function and not a single expression.
      Err(Error {
                span: start_span,
                kind: ErrorKind::SyntaxError("Invalid grouped expression. It should contain only one expression. Tuples are not supported.".to_string()),
            })
    }
  }
}
