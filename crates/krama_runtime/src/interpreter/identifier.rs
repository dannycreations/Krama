use krama_core::{Error, ErrorKind, Expression, Object, Span};

use super::Interpreter;

impl<'ast> Interpreter<'ast> {
  pub async fn eval_identifier(
    &self,
    expression: &Expression<'ast>,
    name: &'ast str,
    span: Span,
  ) -> Result<Object<'ast>, Error<'ast>> {
    if let Some(distance) = self.get_resolved_distance(expression) {
      if let Some(value) = self.get_at(distance, name) {
        return Ok(value.clone());
      }
    }

    self.environment.borrow().get(name).map_or_else(
      || {
        Err(Error::new(
          ErrorKind::ReferenceError(format!("'{}' is not defined", name)),
          span,
        ))
      },
      Ok,
    )
  }
}
