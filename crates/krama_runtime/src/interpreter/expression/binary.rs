use futures::try_join;
use krama_core::{BinaryOperator, Error, Expression, ObjectKind, Span};

use crate::Interpreter;

impl<'ast> Interpreter<'ast> {
  pub async fn eval_binary_expression(
    &self,
    left: &Expression<'ast>,
    operator: BinaryOperator,
    right: &Expression<'ast>,
    span: Span,
  ) -> Result<ObjectKind<'ast>, Error<'ast>> {
    // Handle short-circuiting for logical operators before evaluating the right side.
    if matches!(
      operator,
      BinaryOperator::LogicalOr | BinaryOperator::LogicalAnd
    ) {
      let left_val = self.eval_expression(left, None).await?;
      let is_truthy = left_val.is_truthy();
      if (operator == BinaryOperator::LogicalOr && is_truthy)
        || (operator == BinaryOperator::LogicalAnd && !is_truthy)
      {
        return Ok(left_val);
      }
      return self.eval_expression(right, None).await;
    }

    // Eager evaluation for all other binary operators.
    let (l, r) = try_join!(
      self.eval_expression(left, None),
      self.eval_expression(right, None)
    )?;

    // Delegate to core ObjectKind logic for type-specific operations.
    l.binary_op(operator, &r, self.arena)
      .map_err(|k| k.at(span))
  }
}
