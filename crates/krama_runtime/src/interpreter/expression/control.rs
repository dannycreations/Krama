use std::borrow::Cow;

use krama_core::{
  Error, Expression, FunctionBody, Match, ObjectKind, Span, Type,
};

use crate::Interpreter;

impl<'ast> Interpreter<'ast> {
  /// Evaluates an 'if' expression, including pattern matching support.
  pub async fn eval_if_expression(
    &self,
    condition: &Expression<'ast>,
    then_branch: &Expression<'ast>,
    else_branch: Option<&'ast Expression<'ast>>,
    kind: Option<&Type<'ast>>,
  ) -> Result<ObjectKind<'ast>, Error<'ast>> {
    // 1. Check if the condition is a pattern match (e.g., if (Ok(v) = expr)).
    if let Some(bindings) = self.try_match_assignment(condition).await? {
      let new_interpreter = self.new_enclosed();
      for (name, val) in bindings {
        new_interpreter
          .env_mut(condition.span)?
          .set(name, val, false, false);
      }
      return new_interpreter.eval_expression(then_branch, kind).await;
    }

    // 2. Fallback to normal truthy evaluation.
    let condition_val = self.eval_expression(condition, None).await?;
    if condition_val.is_truthy() {
      self.eval_expression(then_branch, kind).await
    } else if let Some(else_branch) = else_branch {
      self.eval_expression(else_branch, kind).await
    } else {
      Ok(ObjectKind::Void)
    }
  }

  /// Evaluates a 'match' expression by iterating through patterns in each arm.
  pub async fn eval_match_expression(
    &self,
    subject: &Expression<'ast>,
    arms: &[Match<'ast>],
    span: Span,
  ) -> Result<ObjectKind<'ast>, Error<'ast>> {
    let subject_val = self.eval_expression(subject, None).await?;

    // Unwrap Return(Err) for matching, as eval_expression automatically wraps errors in Return signals.
    let effective_val = if let ObjectKind::Return(inner) = &subject_val {
      if let ObjectKind::Err(_) = inner {
        inner
      } else {
        &subject_val
      }
    } else {
      &subject_val
    };

    // Iterate through each arm and its patterns.
    for arm in arms {
      for pattern in &arm.patterns {
        if let Some(bindings) = self
          .eval_match_pattern(effective_val, pattern, span)
          .await?
        {
          // 1. Prepare interpreter with bindings if necessary.
          let interpreter = if !bindings.is_empty() {
            let new_interp = self.new_enclosed();
            for (name, val) in bindings {
              new_interp.env_mut(span)?.set(name, val, false, false);
            }
            Cow::Owned(new_interp)
          } else {
            Cow::Borrowed(self)
          };

          // 2. Execute arm body.
          let result = match &arm.body {
            FunctionBody::Block(block) => {
              interpreter
                .eval_block_statement_with_new_scope(block)
                .await?
            }
            FunctionBody::Expression(expression) => {
              interpreter
                .new_enclosed()
                .eval_expression(expression, None)
                .await?
            }
          };

          // 3. Handle Return signals and control flow.
          if let ObjectKind::Return(_) = &result {
            return Ok(result);
          }

          // Break/Continue in a match expression resolve to Void to allow the outer loop to handle the signal.
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
