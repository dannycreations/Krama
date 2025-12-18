use krama_core::{Error, ErrorKind, Expression, ExpressionKind, Object, Span};

use crate::interpreter::Interpreter;

impl<'ast> Interpreter<'ast> {
  pub async fn eval_member_expression(
    &self,
    object: Object<'ast>,
    property: &Expression<'ast>,
    span: Span<'ast>,
  ) -> Result<Object<'ast>, Error<'ast>> {
    let property_name = if let ExpressionKind::Identifier(name) = property.kind
    {
      name
    } else {
      return Err(Error::new(
        ErrorKind::TypeError("Invalid member expression".to_string()),
        span,
      ));
    };

    let object_type = object.type_name();

    if let Some(props) = krama_std::get_props().get(object_type) {
      if let Some(prop) = props.get(property_name) {
        return prop(object).await.map_err(|kind| Error::new(kind, span));
      }
    }

    if let Object::Scope(scope) = object {
      if scope.name.is_some() {
        if let Some(export) = scope.bindings.get(property_name) {
          return Ok(export.clone());
        }
      }
    }

    Err(Error::new(
      ErrorKind::ReferenceError(format!(
        "Property '{}' not found for type '{}'",
        property_name,
        object.type_name()
      )),
      span,
    ))
  }
}
