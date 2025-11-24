use crate::interpreter::Interpreter;
use krama_core::{
  ast::{expression::Expression, types::Type},
  error::Error,
  object::Object,
};

impl<'ast> Interpreter<'ast> {
  pub(crate) async fn eval_if_expression(
    &self,
    condition: &Expression<'ast>,
    then_branch: &Expression<'ast>,
    else_branch: Option<&'ast Expression<'ast>>,
    kind: Option<&Type<'ast>>,
  ) -> Result<Object<'ast>, Error> {
    let condition = self.eval_expression(condition, None).await?;
    let condition = self.resolve_object(condition).await?;

    if self.is_truthy(&condition) {
      self.eval_expression(then_branch, kind).await
    } else if let Some(else_branch) = else_branch {
      self.eval_expression(else_branch, kind).await
    } else {
      Ok(Object::Void)
    }
  }
}
