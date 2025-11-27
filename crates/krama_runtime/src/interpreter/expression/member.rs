use krama_core::{
  ast::expression::{Expression, ExpressionKind},
  error::{Error, ErrorKind},
  object::Object,
  span::Span,
};

use crate::interpreter::Interpreter;

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
        kind: ErrorKind::TypeError("Invalid member expression".to_string()),
      });
    };

    let object_type = match &resolved_object {
      Object::Array { .. } => "array",
      Object::String(_) => "string",
      Object::Tuple(_) => "tuple",
      _ => "",
    };

    if !object_type.is_empty() {
      if let Some(prop) = self.props.get(&(object_type, property_name)) {
        return prop(resolved_object).await;
      }
    }

    if let Object::Scope(ref scope) = resolved_object {
      let scope = scope.try_borrow().map_err(|e| Error {
        span,
        kind: ErrorKind::ReferenceError(e.to_string()),
      })?;

      if scope.name.is_some() {
        if let Some(export) = scope.bindings.get(property_name) {
          return Ok(export.clone());
        }
      }
    }

    Err(Error {
      span,
      kind: ErrorKind::ReferenceError(format!(
        "Property '{}' not found for type '{}'",
        property_name,
        resolved_object.type_name()
      )),
    })
  }
}
