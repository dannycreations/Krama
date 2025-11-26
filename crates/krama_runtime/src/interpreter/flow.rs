use bumpalo::collections::Vec as BumpVec;
use krama_core::ast::expression::{
  Expression, FunctionBody, MatchArm, MatchPattern,
};
use krama_core::error::Error;
use krama_core::object::Object;

use super::Interpreter;

impl<'ast> Interpreter<'ast> {
  pub(super) async fn eval_match_expression(
    &self,
    subject: &Expression<'ast>,
    arms: &BumpVec<'ast, MatchArm<'ast>>,
    span: krama_core::span::Span,
  ) -> Result<Object<'ast>, Error> {
    let subject = self.eval_expression(subject, None).await?;

    for arm in arms {
      for pattern in &arm.patterns {
        let is_match = self.eval_match_pattern(&subject, pattern, span).await?;

        if is_match {
          let result = match &arm.body {
            FunctionBody::Block(block) => {
              self.eval_block_statement(block).await
            }
            FunctionBody::Expression(expr) => {
              Ok(self.eval_expression(expr, None).await?)
            }
          };

          return match result {
            Ok(Object::Break) => Ok(Object::Void),
            other => other,
          };
        }
      }
    }

    Ok(Object::Void)
  }

  async fn eval_match_pattern(
    &self,
    subject: &Object<'ast>,
    pattern: &MatchPattern<'ast>,
    _span: krama_core::span::Span,
  ) -> Result<bool, Error> {
    match pattern {
      MatchPattern::Else => Ok(true),
      MatchPattern::Expression(expr) => {
        let pattern_val = self.eval_expression(expr, None).await?;
        Ok(subject == &pattern_val)
      }
      MatchPattern::Range(start_expr, end_expr) => {
        let start = self.eval_expression(start_expr, None).await?;
        let end = self.eval_expression(end_expr, None).await?;

        match (subject, start, end) {
          (
            Object::Integer(s),
            Object::Integer(start_val),
            Object::Integer(end_val),
          ) => Ok(*s >= start_val && *s <= end_val),
          (
            Object::String(s),
            Object::String(start_val),
            Object::String(end_val),
          ) => {
            if s.len() == 1 && start_val.len() == 1 && end_val.len() == 1 {
              let s_char = s.chars().next().unwrap();
              let start_char = start_val.chars().next().unwrap();
              let end_char = end_val.chars().next().unwrap();
              Ok(s_char >= start_char && s_char <= end_char)
            } else {
              Ok(false)
            }
          }
          _ => Ok(false),
        }
      }
    }
  }
}
