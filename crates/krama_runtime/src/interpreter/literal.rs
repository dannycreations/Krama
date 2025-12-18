use krama_core::{Error, Literal, Object};

use super::Interpreter;

impl<'ast> Interpreter<'ast> {
  pub fn eval_literal<'s>(
    &'s self,
    literal: Literal<'ast>,
  ) -> Result<Object<'ast>, Error<'ast>>
  where
    'ast: 's,
  {
    match literal {
      Literal::Integer(i) => Ok(Object::Integer(i)),
      Literal::Float(f) => Ok(Object::Float(f)),
      Literal::Boolean(b) => Ok(Object::Boolean(b)),
      Literal::String(s) => Ok(Object::String(s)),
      Literal::Null => Ok(Object::Null),
    }
  }
}
