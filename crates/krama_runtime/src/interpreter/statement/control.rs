use std::sync::Arc;

use krama_core::{Expression, Object, ObjectResult};

use crate::interpreter::Interpreter;

impl Interpreter {
  pub async fn eval_return_statement(
    &self,
    value_expr: Option<&Expression>,
  ) -> ObjectResult {
    let value = match value_expr {
      Some(expr) => self.eval_expression(expr, None).await?,
      None => Object::Void,
    };
    Ok(Object::Return(Arc::new(value)))
  }
}
