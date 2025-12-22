use krama_core::{
  AssignmentOperator, Error, Expression, ExpressionKind, FunctionBody, Match,
  ObjectKind, Span, Type,
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
    // Handle "if let" style pattern matching: if (Ok(val) = result)
    if let ExpressionKind::Assignment {
      left,
      operator: AssignmentOperator::Assign,
      right,
    } = &condition.kind
    {
      if let ExpressionKind::Call {
        function,
        arguments,
      } = &left.kind
      {
        if let ExpressionKind::Identifier(name) = &function.kind {
          if *name == "Ok" || *name == "Err" {
            let right_val = self.eval_expression(right, None).await?;

            // Unwrap Return(Err) if present, as eval_expression wraps Err in Return
            // to support error propagation. But here we want to match on the Err itself.
            let effective_val = if let ObjectKind::Return(inner) = &right_val {
              if let ObjectKind::Err(_) = inner {
                inner
              } else {
                &right_val
              }
            } else {
              &right_val
            };

            let is_match = matches!(
              (*name, effective_val),
              ("Ok", ObjectKind::Ok(_)) | ("Err", ObjectKind::Err(_))
            );

            if is_match {
              // Extract the inner value
              let inner_val = match effective_val {
                ObjectKind::Ok(v) => v,
                ObjectKind::Err(v) => v,
                _ => unreachable!(),
              };

              // Bind to the variable if provided
              if arguments.len() == 1 {
                if let ExpressionKind::Identifier(bind_name) =
                  &arguments[0].kind
                {
                  // Create a new scope for the binding + then branch
                  let new_interpreter = self.new_enclosed();
                  // inner_val is &'ast ObjectKind<'ast>. We need to clone the object,
                  // not the reference.
                  new_interpreter.env_mut(condition.span)?.set(
                    bind_name,
                    (*inner_val).clone(),
                    false,
                    false,
                  );

                  return new_interpreter
                    .eval_expression(then_branch, kind)
                    .await;
                }
              }
              // If arguments don't match or it's just a check without binding
              return self.eval_expression(then_branch, kind).await;
            } else {
              // Pattern didn't match, go to else branch
              if let Some(else_branch) = else_branch {
                return self.eval_expression(else_branch, kind).await;
              } else {
                return Ok(ObjectKind::Void);
              }
            }
          }
        }
      }
    }

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
