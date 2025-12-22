use krama_core::{
  Error, Expression, FunctionBody, Match, ObjectKind, Span, Type,
};

use crate::Interpreter;

impl<'ast> Interpreter<'ast> {
  pub async fn eval_if_expression(
    &self,
    condition: &Expression<'ast>,
    then_branch: &Expression<'ast>,
    else_branch: Option<&'ast Expression<'ast>>,
    kind: Option<&Type<'ast>>,
  ) -> Result<ObjectKind<'ast>, Error<'ast>> {
    let condition = self.eval_expression(condition, None).await?;

    if condition.is_truthy() {
      self.eval_expression(then_branch, kind).await
    } else if let Some(else_branch) = else_branch {
      self.eval_expression(else_branch, kind).await
    } else {
      Ok(ObjectKind::Void)
    }
  }

  pub async fn eval_match_expression(
    &self,
    subject: &Expression<'ast>,
    arms: &[Match<'ast>],
    span: Span,
  ) -> Result<ObjectKind<'ast>, Error<'ast>> {
    let subject = self.eval_expression(subject, None).await?;

    for arm in arms {
      for pattern in &arm.patterns {
        if self.eval_match_pattern(&subject, pattern, span).await? {
          let result = match &arm.body {
            FunctionBody::Block(block) => {
              self.eval_block_statement_with_new_scope(block).await?
            }
            FunctionBody::Expression(expression) => {
              self
                .new_enclosed()
                .eval_expression(expression, None)
                .await?
            }
          };

          if matches!(result, ObjectKind::Break | ObjectKind::Continue) {
            return Ok(ObjectKind::Void);
          }

          return Ok(result);
        }
      }
    }

    Ok(ObjectKind::Void)
  }
}
