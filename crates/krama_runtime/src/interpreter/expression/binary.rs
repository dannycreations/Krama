use futures::try_join;
use krama_core::{BinaryOperator, Error, Expression, ObjectKind, Span};

use crate::Interpreter;

impl<'ast> Interpreter<'ast> {
  /// Evaluates a binary expression, handling short-circuiting for logical operators.
  pub async fn eval_binary_expression(
    &self,
    left: &Expression<'ast>,
    operator: BinaryOperator,
    right: &Expression<'ast>,
    span: Span,
  ) -> Result<ObjectKind<'ast>, Error<'ast>> {
    // 1. Handle short-circuiting for logical operators (OR/AND).
    // These must NOT evaluate the right side if the left side determines the result.
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

    // 2. Eager evaluation for all other binary operators (Arithmetic, Comparison, Bitwise).
    // We use try_join! to run both evaluations concurrently where possible (though typically sequential in this interpreter).
    let (l, r) = try_join!(
      self.eval_expression(left, None),
      self.eval_expression(right, None)
    )?;

    // Delegate to core ObjectKind logic for type-specific operations.
    l.binary_op(operator, &r, self.arena)
      .map_err(|k| k.at(span))
  }
}
