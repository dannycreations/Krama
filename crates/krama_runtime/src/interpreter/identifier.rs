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
    span: Span,
  ) -> Result<Object<'ast>, Error> {
    if let Some(distance) = self.locals.borrow().get(&expression.span) {
      if let Some(value) = self.get_at(*distance, name) {
        return Ok(value.clone());
      }
    }

    self.environment.borrow().get(name).map_or_else(
      || {
        Err(Error {
          span,
          kind: ErrorKind::ReferenceError(format!("'{}' is not defined", name)),
          file_path: None,
          source: None,
        })
      },
      |v| Ok(v.clone()),
    )
  }
}
