use super::Interpreter;
use krama_core::ast::literal::Literal;
use krama_core::error::Error;
use krama_core::object::Object;

impl<'ast> Interpreter<'ast> {
  pub(super) async fn eval_literal(
    &self,
    literal: Literal<'ast>,
  ) -> Result<Object<'ast>, Error> {
    match literal {
      Literal::Integer(i) => Ok(Object::Integer(i)),
      Literal::Float(f) => Ok(Object::Float(f)),
      Literal::Boolean(b) => Ok(Object::Boolean(b)),
      Literal::String(s) => Ok(Object::String(s)),
      Literal::Null => Ok(Object::Null),
    }
  }
}
