use krama_core::{Error, ObjectKind, Span, UnaryOperator};

use crate::Interpreter;

impl Interpreter {
  /// Evaluates a unary operation by delegating to ObjectKind.
  pub fn eval_unary_expression(
    &self,
    operator: UnaryOperator,
    right: ObjectKind,
    span: Span,
  ) -> Result<ObjectKind, Error> {
    // Unary operations are pure value transformations handled by krama_core.
    right.unary_op(operator).map_err(|k| k.at(span))
  }
}
