use krama_core::{Error, ErrorKind, Expression, ExpressionKind, Object, Span};

use crate::Interpreter;

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
        ErrorKind::TypeError("Invalid property for member access".to_string()),
        span,
      ));
    };

    match object {
      Object::Object { properties, .. } => {
        let properties = properties.read();
        if let Some(value) = properties.get(property_name) {
          Ok(value.clone())
        } else {
          Ok(Object::Void)
        }
      }
      Object::Scope(scope) => {
        if let Some(value) = scope.get_binding(property_name) {
          Ok(value.clone())
        } else {
          Err(Error::new(
            ErrorKind::ReferenceError(format!(
              "Property '{}' not found in module",
              property_name
            )),
            span,
          ))
        }
      }
      _ => {
        if let Some(prop) = self.get_standard_property(&object, property_name) {
          return (prop.callback)(object)
            .await
            .map_err(|kind| Error::new(kind, span));
        }

        Err(Error::new(
          ErrorKind::TypeError(format!(
            "{} does not support member access",
            object.type_name()
          )),
          span,
        ))
      }
    }
  }

  fn get_standard_property(
    &self,
    object: &Object<'ast>,
    name: &str,
  ) -> Option<&'static krama_core::StandardProperty> {
    let type_name = object.type_name();
    krama_core::STANDARD_PROPERTIES
      .iter()
      .find(|p| p.name == name && p.types.contains(&type_name))
  }
}
