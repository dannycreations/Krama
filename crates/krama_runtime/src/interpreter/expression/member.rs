use krama_core::{
  ast::expression::{Expression, ExpressionKind},
  error::ErrorKind,
  object::Object,
  span::Span,
};

use crate::interpreter::Interpreter;

impl<'ast> Interpreter<'ast> {
  pub(crate) async fn eval_member_expression(
    &self,
    object: Object<'ast>,
    property: &Expression<'ast>,
    span: Span<'ast>,
  ) -> Result<Object<'ast>, (ErrorKind, Span<'ast>)> {
    let property_name = if let ExpressionKind::Identifier(name) = property.kind
    {
      name
    } else {
      return Err((
        ErrorKind::TypeError("Invalid member expression".to_string()),
        span,
      ));
    };

    let object_type = object.type_name();

    if let Some(props) = krama_std::get_props().get(object_type) {
      if let Some(prop) = props.get(property_name) {
        return prop(object).await.map_err(|kind| (kind, span));
      }
    }

    if let Object::Scope(scope) = object {
      if scope.name.is_some() {
        if let Some(export) = scope.bindings.get(property_name) {
          return Ok(export.clone());
        }
      }
    }

    Err((
      ErrorKind::ReferenceError(format!(
        "Property '{}' not found for type '{}'",
        property_name,
        object.type_name()
      )),
      span,
    ))
  }
}
