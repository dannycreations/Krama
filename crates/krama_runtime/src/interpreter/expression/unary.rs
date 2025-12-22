use krama_core::{Error, ObjectKind, Span, UnaryOperator};

use crate::Interpreter;

impl<'ast> Interpreter<'ast> {
  pub fn eval_unary_expression(
    &self,
    operator: UnaryOperator,
    right: ObjectKind<'ast>,
    span: Span,
  ) -> Result<ObjectKind<'ast>, Error<'ast>> {
    right.unary_op(operator).map_err(|k| k.at(span))
  }
}
