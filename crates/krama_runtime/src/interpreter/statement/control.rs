use std::sync::Arc;

use krama_core::{ObjectKind, ObjectResult};

use crate::interpreter::Interpreter;

impl Interpreter {
  pub async fn eval_return_statement(
    &self,
    value_expr: Option<&krama_core::Expression>,
  ) -> ObjectResult {
    let value = match value_expr {
      Some(expr) => self.eval_expression(expr, None).await?,
      None => ObjectKind::Void,
    };
    Ok(ObjectKind::Return(Arc::new(value)))
  }
}
