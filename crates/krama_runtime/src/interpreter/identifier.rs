use krama_core::{
  error::{Error, ErrorKind},
  object::Object,
  span::Span,
};

use super::Interpreter;

impl<'ast> Interpreter<'ast> {
  pub(super) async fn eval_identifier(
    &self,
    name: &'ast str,
    span: Span,
  ) -> Result<Object<'ast>, Error> {
    self.environment.borrow().get(name).ok_or_else(|| Error {
      span,
      kind: ErrorKind::ReferenceError(name.to_string()),
    })
  }
}
