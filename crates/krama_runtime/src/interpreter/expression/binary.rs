use futures::try_join;
use krama_core::{BinaryOperator, Error, Expression, ObjectKind, Span};

use crate::Interpreter;

impl Interpreter {
  /// Evaluates a binary expression, handling short-circuiting for logical operators.
  pub async fn eval_binary_expression(
    &self,
    left: &Expression,
    operator: BinaryOperator,
    right: &Expression,
    span: Span,
  ) -> Result<ObjectKind, Error> {
    // 1. Handle short-circuiting for logical operators (OR/AND).
    if matches!(
      operator,
      BinaryOperator::LogicalOr | BinaryOperator::LogicalAnd
    ) {
      let left_val = self.eval_expression(left, None).await?;

      // Propagate control flow signals (Return/Break/Continue) immediately.
      if left_val.is_control_signal() {
        return Ok(left_val);
      }

      let is_truthy = left_val.is_truthy();
      if (operator == BinaryOperator::LogicalOr && is_truthy)
        || (operator == BinaryOperator::LogicalAnd && !is_truthy)
      {
        return Ok(left_val);
      }
      return self.eval_expression(right, None).await;
    }

    // 2. Eager evaluation for all other binary operators.
    let (l, r) = try_join!(
      self.eval_expression(left, None),
      self.eval_expression(right, None)
    )?;

    // Delegate to core ObjectKind logic.
    l.binary_op(operator, &r).map_err(|k| k.at(span))
  }
}
