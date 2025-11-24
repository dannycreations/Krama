use crate::interpreter::Interpreter;
use krama_core::{
  ast::expression::{Expression, ExpressionKind},
  error::{Error, ErrorKind},
  object::Object,
  span::Span,
};

impl<'ast> Interpreter<'ast> {
  pub(crate) async fn eval_member_expression(
    &self,
    object: Object<'ast>,
    property: &Expression<'ast>,
    span: Span,
  ) -> Result<Object<'ast>, Error> {
    let resolved_object = self.resolve_object(object).await?;
    let property_name = if let ExpressionKind::Identifier(name) = property.kind
    {
      name
    } else {
      return Err(Error {
        span,
        kind: ErrorKind::InvalidExpression(
          "Invalid member expression".to_string(),
        ),
      });
    };

    let object_type = match resolved_object {
      Object::Array { .. } => "array",
      Object::String(_) => "string",
      _ => "",
    };

    if let Some(prop) = self.props.get(&(object_type, property_name)) {
      return prop(resolved_object).await;
    }

    match resolved_object {
      Object::Module(module) => {
        let module = module.try_borrow().unwrap();
        if let Some(export) = module.exports.get(property_name) {
          Ok(export.clone())
        } else {
          Err(Error {
            span,
            kind: ErrorKind::IdentifierNotFound(property_name.to_string()),
          })
        }
      }
      _ => Err(Error {
        span,
        kind: ErrorKind::InvalidExpression(
          "Member expression can only be used on modules".to_string(),
        ),
      }),
    }
  }
}
