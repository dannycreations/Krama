use krama_core::{
  AssignmentOperator, Expression, ExpressionKind, ObjectKind, ObjectResult,
  Span, StatementBlock,
};

use crate::interpreter::Interpreter;

impl Interpreter {
  pub async fn eval_while_statement(
    &self,
    condition: &Expression,
    body: &StatementBlock,
  ) -> ObjectResult {
    loop {
      if let Some(bindings) = self.try_match_assignment(condition).await? {
        for (name, val) in bindings {
          self.stack.write().define(name.clone(), val, false, false);
        }
      } else {
        if let ExpressionKind::Assignment {
          left,
          operator: AssignmentOperator::Assign,
          ..
        } = &condition.kind
        {
          if let ExpressionKind::Call { function, .. } = &left.kind {
            if let ExpressionKind::Identifier(name) = &function.kind {
              if name.as_ref() == "Ok" || name.as_ref() == "Err" {
                break;
              }
            }
          }
        }
        if !self.eval_expression(condition, None).await?.is_truthy() {
          break;
        }
      }

      let result = self.eval_block_statement(body).await?;
      if let Some(ctrl) = self.handle_loop_control(result) {
        return Ok(ctrl);
      }
    }
    Ok(ObjectKind::Void)
  }

  pub async fn eval_for_statement(
    &self,
    binding: &krama_core::ForBinding,
    iterable: &Expression,
    body: &StatementBlock,
    span: Span,
  ) -> ObjectResult {
    let iterable_val = self.eval_expression(iterable, None).await?;
    let elements =
      self.collect_iterable_elements(&iterable_val, binding, span)?;
    for element in elements {
      {
        let mut stack = self.stack.write();
        stack.push("for_loop_iter".into(), None);
      }

      self.assign_for_binding(binding, element, span)?;
      let result = self.eval_block_statement(body).await;

      self.stack.write().pop();

      let result = result?;
      if let Some(ctrl) = self.handle_loop_control(result) {
        return Ok(ctrl);
      }
    }
    Ok(ObjectKind::Void)
  }
}
