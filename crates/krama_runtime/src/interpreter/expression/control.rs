use krama_core::{
  Error, Expression, FunctionBody, MatchArm, Object, Span, Type,
};

use crate::Interpreter;

impl<'ast> Interpreter<'ast> {
  pub async fn eval_if_expression(
    &self,
    condition: &Expression<'ast>,
    then_branch: &Expression<'ast>,
    else_branch: Option<&'ast Expression<'ast>>,
    kind: Option<&Type<'ast>>,
  ) -> Result<Object<'ast>, Error<'ast>> {
    let condition = self.eval_expression(condition, None).await?;

    if bool::from(&condition) {
      self.eval_expression(then_branch, kind).await
    } else if let Some(else_branch) = else_branch {
      self.eval_expression(else_branch, kind).await
    } else {
      Ok(Object::Void)
    }
  }

  pub async fn eval_match_expression(
    &self,
    subject: &Expression<'ast>,
    arms: &[MatchArm<'ast>],
    span: Span,
  ) -> Result<Object<'ast>, Error<'ast>> {
    let subject = self.eval_expression(subject, None).await?;

    for arm in arms {
      for pattern in &arm.patterns {
        let matched = self.eval_match_pattern(&subject, pattern, span).await?;
        if matched {
          return match &arm.body {
            FunctionBody::Block(block) => {
              self.eval_block_statement_with_new_scope(block).await
            }
            FunctionBody::Expression(expression) => {
              let new_interpreter = self.new_enclosed();
              new_interpreter.eval_expression(expression, None).await
            }
          };
        }
      }
    }

    Ok(Object::Void)
  }
}
