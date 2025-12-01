use krama_core::{
  ast::operator::UnaryOperator,
  error::{Error, ErrorKind},
  object::Object,
  span::Span,
};

use crate::interpreter::Interpreter;

impl<'ast> Interpreter<'ast> {
  pub(crate) fn eval_unary_expression(
    &self,
    operator: UnaryOperator,
    right: Object<'ast>,
    span: Span<'ast>,
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
