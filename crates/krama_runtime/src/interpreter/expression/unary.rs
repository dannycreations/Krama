use krama_core::{Error, ErrorKind, Object, Span, UnaryOperator};

use crate::Interpreter;

impl<'ast> Interpreter<'ast> {
  pub fn eval_unary_expression(
    &self,
    operator: UnaryOperator,
    right: Object<'ast>,
    span: Span,
  ) -> Result<Object<'ast>, Error<'ast>> {
    match operator {
      UnaryOperator::Not => Ok(Object::Boolean(!bool::from(&right))),
      UnaryOperator::Negate => match right {
        Object::Integer(i) => Ok(Object::Integer(-i)),
        Object::Float(f) => Ok(Object::Float(-f)),
        _ => Err(Error::new(
          ErrorKind::TypeError(
            "Unary '-' operator can only be applied to numbers".to_string(),
          ),
          span,
        )),
      },
      UnaryOperator::BitwiseNot => match right {
        Object::Integer(i) => Ok(Object::Integer(!i)),
        _ => Err(Error::new(
          ErrorKind::TypeError(
            "Bitwise not operator can only be applied to integers".to_string(),
          ),
          span,
        )),
      },
    }
  }
}
