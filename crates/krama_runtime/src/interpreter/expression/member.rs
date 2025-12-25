use std::sync::Arc;

use krama_core::{
  Error, ErrorKind, Expression, ExpressionKind, FunctionKind, ObjectKind,
  ObjectResult, Span, StructMethod, UserFunction,
};
use krama_std::PROPS;

use crate::Interpreter;

impl Interpreter {
  /// Evaluates a member access expression (e.g., object.property).
  /// Supports objects, structs, modules, and built-in properties.
  pub async fn eval_member_expression(
    &self,
    object: ObjectKind,
    property: &Expression,
    span: Span,
  ) -> ObjectResult {
    // 1. Extract property name from the identifier.
    let property_name = if let ExpressionKind::Identifier(name) = &property.kind
    {
      name
    } else {
      return Err(Error::new(
        ErrorKind::TypeError("Invalid property for member access".to_string()),
        span,
      ));
    };

    match object {
      // 2. Handle Object literals and Struct instances.
      ObjectKind::Object {
        properties,
        definition,
        ..
      } => {
        // a. Check properties (fields) first.
        if let Some(value) = properties.read().get(property_name) {
          if let Some(definition) = definition {
            if let Some(field_def) =
              definition.fields.iter().find(|f| f.name == *property_name)
            {
              self.ensure_accessible(
                field_def.public,
                property_name,
                &definition.name,
                span,
              )?;
            }
          }
          return Ok(value.clone());
        }

        // b. Check methods if it's a struct instance.
        if let Some(ref definition) = definition {
          if let Some(method) =
            definition.methods.iter().find(|m| m.name == *property_name)
          {
            self.ensure_accessible(
              method.public,
              property_name,
              &definition.name,
              span,
            )?;
            return Ok(Self::from_method(method));
          }
        }

        if let Some(ref definition) = definition {
          Err(Error::new(
            ErrorKind::ReferenceError(format!(
              "Property or method '{}' not found in struct '{}'",
              property_name, definition.name
            )),
            span,
          ))
        } else {
          Ok(ObjectKind::Void)
        }
      }

      // 4. Handle Struct definitions (Static methods).
      ObjectKind::Struct(definition) => {
        if let Some(method) =
          definition.methods.iter().find(|m| m.name == *property_name)
        {
          self.ensure_accessible(
            method.public,
            property_name,
            &definition.name,
            span,
          )?;
          return Ok(Self::from_method(method));
        }

        Err(Error::new(
          ErrorKind::ReferenceError(format!(
            "Method '{}' not found in struct '{}'",
            property_name, definition.name
          )),
          span,
        ))
      }

      // 5. Handle Scopes (Modules).
      ObjectKind::Scope(scope) => {
        let scope = scope.read();
        if let Some(binding) = scope.get_local(property_name) {
          if binding.public {
            Ok(binding.value.clone())
          } else {
            Err(Error::new(
              ErrorKind::TypeError(format!(
                "Property '{}' is private",
                property_name
              )),
              span,
            ))
          }
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

      // 6. Handle Built-in properties (Standard library extensions).
      _ => {
        let type_name = object.type_name();
        if let Some(callback) = PROPS
          .get(type_name)
          .and_then(|type_props| type_props.get(property_name.as_str()))
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

  /// Allocates a new UserFunction from a StructMethod.
  fn from_method(method: &StructMethod) -> ObjectKind {
    ObjectKind::Function(FunctionKind::User {
      func: Arc::new(UserFunction {
        parameters: method.parameters.clone(),
        body: method.body.clone(),
        kind: method.kind.clone(),
      }),
      env: None, // Methods don't capture environment at definition time in the same way, or handle 'this' dynamically
    })
  }
}
