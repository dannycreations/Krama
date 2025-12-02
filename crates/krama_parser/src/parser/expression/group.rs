use bumpalo::collections::Vec as BumpVec;
use krama_core::{
  ast::{
    expression::{Expression, ExpressionKind},
    precedence::Precedence,
    statement::Parameter,
  },
  error::ErrorKind,
  token::TokenKind,
};

use crate::parser::{ParseResult, Parser};

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub(super) fn parse_paren_expression(&mut self) -> ParseResult<'a, 'ast> {
    let start_span = self.current_token.span.clone();
    self.advance();

    if self.current_token.kind == TokenKind::RParen {
      self.advance();
      return self.build_fn_expression(start_span, BumpVec::new_in(self.arena));
    }

    let mut expressions = BumpVec::new_in(self.arena);
    expressions.push(self.parse_expression(Precedence::Lowest)?);

    if self.current_token.kind == TokenKind::Comma {
      self.advance();

      // If there's a comma, it must be an arrow function.
      if self.current_token.kind != TokenKind::RParen {
        expressions.push(self.parse_expression(Precedence::Lowest)?);
        while self.current_token.kind == TokenKind::Comma {
          self.advance();
          expressions.push(self.parse_expression(Precedence::Lowest)?);
        }
      }
    }

    self.consume(TokenKind::RParen)?;

    if self.current_token.kind == TokenKind::Arrow {
      let mut parameters = BumpVec::new_in(self.arena);
      for expr in expressions {
        let parameter = self.expression_to_parameter(expr)?;
        parameters.push(parameter);
      }
      self.build_fn_expression(start_span, parameters)
    } else if expressions.len() == 1 {
      expressions.pop().ok_or_else(|| {
        ErrorKind::SyntaxError("Expected expression in parentheses".to_string())
      })
    } else {
      Err(ErrorKind::SyntaxError(
        "Invalid grouped expression. To create a tuple, use square brackets `[]`".to_string(),
      ))
    }
  }

  fn expression_to_parameter(
    &mut self,
    expr: Expression<'ast>,
  ) -> Result<Parameter<'ast>, ErrorKind> {
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
          return Err(ErrorKind::SyntaxError(
            "Invalid expression in function parameters.".to_string(),
          ));
        }

        let mut param_from_left =
          self.expression_to_parameter((*left).clone())?;
        if param_from_left.default.is_some() {
          return Err(ErrorKind::SyntaxError(
            "Cannot have nested default values".to_string(),
          ));
        }

        param_from_left.default = Some(right);
        param_from_left.span = expr.span;
        Ok(param_from_left)
      }
      ExpressionKind::Typed { expr: inner, kind } => {
        let mut param_from_inner =
          self.expression_to_parameter((*inner).clone())?;
        if param_from_inner.kind.is_some() {
          return Err(ErrorKind::SyntaxError(
            "Cannot have nested type hints".to_string(),
          ));
        }
        param_from_inner.kind = Some(kind);
        param_from_inner.span = expr.span;
        Ok(param_from_inner)
      }
      _ => Err(ErrorKind::SyntaxError(
        "Invalid expression in function parameters.".to_string(),
      )),
    }
  }
}
