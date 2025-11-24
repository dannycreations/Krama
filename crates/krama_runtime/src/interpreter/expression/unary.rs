use crate::interpreter::Interpreter;
use krama_core::{
  ast::operator::UnaryOperator,
  error::{Error, ErrorKind},
  object::Object,
  span::Span,
};

impl<'ast> Interpreter<'ast> {
  pub(crate) async fn eval_unary_expression(
    &self,
    operator: UnaryOperator,
    right: Object<'ast>,
    span: Span,
  ) -> Result<Object<'ast>, Error> {
    let right = self.resolve_object(right).await?;
    match operator {
      UnaryOperator::Not => Ok(Object::Boolean(!self.is_truthy(&right))),
      UnaryOperator::Negate => match right {
        Object::Integer(i) => Ok(Object::Integer(-i)),
        Object::Float(f) => Ok(Object::Float(-f)),
        _ => Err(Error {
          span,
          kind: ErrorKind::TypeMismatch(
            "Unary '-' operator can only be applied to numbers".to_string(),
          ),
        }),
      },
      UnaryOperator::BitwiseNot => match right {
        Object::Integer(i) => Ok(Object::Integer(!i)),
        _ => Err(Error {
          span,
          kind: ErrorKind::TypeMismatch(
            "Bitwise not operator can only be applied to integers".to_string(),
          ),
        }),
      },
    }
  }
}
