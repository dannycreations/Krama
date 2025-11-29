use bumpalo::collections::Vec as BumpVec;
use krama_core::{
  ast::{
    expression::{Expression, ExpressionKind},
    precedence::Precedence,
    statement::Parameter,
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
        let parameter = self.expression_to_parameter(expr)?;
        parameters.push(parameter);
      }
      self.parse_fn_expr_with_params(start_span, parameters)
    } else if expressions.len() == 1 {
      expressions.pop().ok_or_else(|| Error {
        span: start_span,
        kind: ErrorKind::SyntaxError(
          "Expected expression in parentheses".to_string(),
        ),
        file_path: None,
        source: None,
      })
    } else {
      Err(Error {
        span: start_span,
        kind: ErrorKind::SyntaxError(
          "Invalid grouped expression. To create a tuple, use square brackets `[]`"
            .to_string(),
        ),
        file_path: None,
        source: None,
      })
    }
  }

  fn expression_to_parameter(
    &self,
    expr: Expression<'ast>,
  ) -> Result<Parameter<'ast>, Error> {
    match expr.kind {
      ExpressionKind::Identifier(name) => Ok(Parameter {
        name,
        kind: None,
        default: None,
        span: expr.span,
      }),
      ExpressionKind::Assignment {
        left,
        operator,
        right,
      } => {
        if operator != krama_core::ast::operator::AssignmentOperator::Assign {
          return Err(Error {
            span: expr.span,
            kind: ErrorKind::SyntaxError(
              "Invalid expression in function parameters.".to_string(),
            ),
            file_path: None,
            source: None,
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
            file_path: None,
            source: None,
          });
        };
        Ok(Parameter {
          name,
          kind: None,
          default: Some(right),
          span: expr.span,
        })
      }
      _ => Err(Error {
        span: expr.span,
        kind: ErrorKind::SyntaxError(
          "Invalid expression in function parameters.".to_string(),
        ),
        file_path: None,
        source: None,
      }),
    }
  }
}
