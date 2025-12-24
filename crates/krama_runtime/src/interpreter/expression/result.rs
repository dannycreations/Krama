use krama_core::{Expression, ObjectKind, Span};

use crate::interpreter::Interpreter;

impl<'ast> Interpreter<'ast> {
  /// Evaluates the postfix '?' operator (Try expression).
  pub async fn eval_result(
    &self,
    expr: &Expression<'ast>,
    _span: Span,
  ) -> Result<ObjectKind<'ast>, krama_core::Error<'ast>> {
    let val = self.eval_expression(expr, None).await?;

    // If result is Return(Err), unwrap it to Err to allow the '?' operator to catch it.
    if let ObjectKind::Return(inner) = &val {
      if inner.is_result_err() {
        return Ok((*inner).clone());
      }
    }

    Ok(val)
  }
}
