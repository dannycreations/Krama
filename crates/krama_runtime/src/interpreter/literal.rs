use krama_core::{Error, Literal, Object};

use crate::Interpreter;

impl<'ast> Interpreter<'ast> {
  #[inline(always)]
  pub fn eval_literal(
    &self,
    literal: Literal<'ast>,
  ) -> Result<Object<'ast>, Error<'ast>> {
    Ok(match literal {
      Literal::Integer(i) => Object::Integer(i),
      Literal::Float(f) => Object::Float(f),
      Literal::String(s) => Object::String(s),
      Literal::Boolean(b) => Object::Boolean(b),
      Literal::Null => Object::Null,
    })
  }
}
