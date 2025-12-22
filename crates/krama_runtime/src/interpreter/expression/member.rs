use krama_core::{
  Error, ErrorKind, Expression, ExpressionKind, ObjectKind, Span,
};
use krama_std::PROPS;

use crate::Interpreter;

impl<'ast> Interpreter<'ast> {
  pub async fn eval_member_expression(
    &self,
    object: ObjectKind<'ast>,
    property: &Expression<'ast>,
    span: Span,
  ) -> Result<ObjectKind<'ast>, Error<'ast>> {
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
      ObjectKind::Object { properties, .. } => Ok(
        properties
          .read()
          .get(property_name)
          .cloned()
          .unwrap_or(ObjectKind::Void),
      ),
      ObjectKind::StructInstance { definition, fields } => {
        // 1. Check fields
        if let Some(value) = fields.read().get(property_name) {
          if let Some(field_def) =
            definition.fields.iter().find(|f| f.name == property_name)
          {
            self.ensure_accessible(
              field_def.public,
              property_name,
              definition.name,
              span,
            )?;
          }
          return Ok(value.clone());
        }

        // 2. Check methods
        if let Some(method) =
          definition.methods.iter().find(|m| m.name == property_name)
        {
          self.ensure_accessible(
            method.public,
            property_name,
            definition.name,
            span,
          )?;
          return Ok(ObjectKind::from_method(method, self.arena));
        }

        Err(Error::new(
          ErrorKind::ReferenceError(format!(
            "Property or method '{}' not found in struct '{}'",
            property_name, definition.name
          )),
          span,
        ))
      }
      ObjectKind::Struct(definition) => {
        if let Some(method) =
          definition.methods.iter().find(|m| m.name == property_name)
        {
          self.ensure_accessible(
            method.public,
            property_name,
            definition.name,
            span,
          )?;
          return Ok(ObjectKind::from_method(method, self.arena));
        }

        Err(Error::new(
          ErrorKind::ReferenceError(format!(
            "Method '{}' not found in struct '{}'",
            property_name, definition.name
          )),
          span,
        ))
      }
      ObjectKind::Scope(scope) => {
        scope.get_binding(property_name).cloned().ok_or_else(|| {
          Error::new(
            ErrorKind::ReferenceError(format!(
              "Property '{}' not found in module",
              property_name
            )),
            span,
          )
        })
      }
      _ => {
        let type_name = object.type_name();
        if let Some(callback) = PROPS
          .get(type_name)
          .and_then(|type_props| type_props.get(property_name))
        {
          return (callback)(object)
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

  fn ensure_accessible(
    &self,
    public: bool,
    member_name: &str,
    struct_name: &str,
    span: Span,
  ) -> Result<(), Error<'ast>> {
    if public {
      return Ok(());
    }

    let env = self.environment.borrow();
    let current_struct = env.get("__current_struct__");
    let allowed = if let Some(ObjectKind::String(name)) = current_struct {
      name == struct_name
    } else {
      false
    };

    if !allowed {
      return Err(Error::new(
        ErrorKind::TypeError(format!("Member '{}' is private", member_name)),
        span,
      ));
    }
    Ok(())
  }
}
