use super::Interpreter;
use krama_core::error::Error;
use krama_core::error::ErrorKind;
use krama_core::object::Object;
use krama_core::span::Span;

impl<'ast> Interpreter<'ast> {
  pub(super) async fn eval_identifier(
    &self,
    name: &'ast str,
    span: Span,
  ) -> Result<Object<'ast>, Error> {
    self.environment.borrow().get(name).ok_or_else(|| Error {
      span,
      kind: ErrorKind::IdentifierNotFound(name.to_string()),
    })
  }
}
