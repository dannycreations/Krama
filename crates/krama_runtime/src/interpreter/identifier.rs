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
    if let Some(distance) = self.locals.borrow().get(&expression.span) {
      if let Some(value) = self.get_at(*distance, name) {
        return Ok(value.clone());
      }
    }

    self.environment.borrow().get(name).map_or_else(
      || {
        Err((
          ErrorKind::ReferenceError(format!("'{}' is not defined", name)),
          span,
        ))
      },
      |v| Ok(v.clone()),
    )
  }
}
