use krama_core::{
  ast::expression::Expression, error::ErrorKind, object::Object, span::Span,
};

use super::Interpreter;

impl<'ast> Interpreter<'ast> {
  pub(super) async fn eval_identifier(
    &self,
    expression: &Expression<'ast>,
    name: &'ast str,
    span: Span<'ast>,
  ) -> Result<Object<'ast>, (ErrorKind, Span<'ast>)> {
    self.lookup_variable(expression, name).map_or_else(
      || {
        Err((
          ErrorKind::ReferenceError(format!("'{}' is not defined", name)),
          span,
        ))
      },
      Ok,
    )
  }

  pub(super) fn lookup_variable(
    &self,
    expression: &Expression<'ast>,
    name: &'ast str,
  ) -> Option<Object<'ast>> {
    if let Some(distance) = self.look_up_variable(expression) {
      if let Some(value) = self.get_at(distance, name) {
        return Some(value.clone());
      }
    }

    self.environment.borrow().get(name)
  }
}
