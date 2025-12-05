use krama_core::{
  ast::expression::Expression,
  error::{Error, ErrorKind},
  object::Object,
  span::Span,
};

use super::Interpreter;

impl<'ast> Interpreter<'ast> {
  pub(super) async fn eval_identifier(
    &self,
    expression: &Expression<'ast>,
    name: &'ast str,
    span: Span<'ast>,
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
