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

    // The '?' operator unwraps Return(Err(e)) to Err(e).
    // This allows it to "catch" implicit error propagation and continue execution.
    // If it's a normal Return(Ok(v)), it remains Return(Ok(v)) which will eventually
    // be propagated or unwrapped by the calling function.
    if let ObjectKind::Return(inner) = &val {
      if inner.is_result_err() {
        return Ok((*inner).clone());
      }
    }

    Ok(val)
  }
}
