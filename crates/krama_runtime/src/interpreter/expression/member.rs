use std::sync::Arc;

use krama_core::{
  Error, ErrorKind, ErrorResult, Expression, ExpressionKind, FunctionKind,
  ObjectKind, ObjectResult, Span, StructMethod, UserFunction,
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

    match &object {
      // 2. Handle Object literals and Struct instances.
      ObjectKind::Object {
        properties,
        definition,
        ..
      } => {
        // a. O(1) property lookup.
        if let Some(value) = properties.read().get(property_name) {
          if let Some(definition) = definition {
            // O(1) field visibility check using pre-computed field_map.
            if let Some(&index) = definition.field_map.get(property_name) {
              let field_def = &definition.fields[index];
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
          // Optimized O(1) method lookup using pre-computed method_map.
          if let Some(&index) = definition.method_map.get(property_name) {
            let method = &definition.methods[index];
            self.ensure_accessible(
              method.is_public,
              property_name,
              &definition.name,
              span,
            )?;
            return if method.is_static {
              Ok(Self::from_method(method))
            } else {
              Ok(Self::bind_method(method, object.clone()))
            };
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
        // Optimized O(1) method lookup using pre-computed method_map.
        if let Some(&index) = definition.method_map.get(property_name) {
          let method = &definition.methods[index];
          self.ensure_accessible(
            method.is_public,
            property_name,
            &definition.name,
            span,
          )?;
          return if method.is_static {
            Ok(Self::from_method(method))
          } else {
            Err(Error::new(
              ErrorKind::TypeError(format!(
                "Method '{}' in struct '{}' is an instance method and requires an instance",
                property_name, definition.name
              )),
              span,
            ))
          };
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

      // 6. Handle Enum variants.
      ObjectKind::Enum(instance) => {
        if property_name.as_ref() == "variant" {
          return Ok(ObjectKind::String(instance.variant.clone()));
        }
        if property_name.as_ref() == "name" {
          return Ok(ObjectKind::String(instance.name.clone()));
        }

        let type_name = object.type_name();
        if let Some(callback) = PROPS
          .get(type_name)
          .and_then(|type_props| type_props.get(property_name.as_ref()))
        {
          return (callback)(object.clone())
            .await
            .map_err(|kind| Error::new(kind, span));
        }

        Err(Error::new(
          ErrorKind::TypeError(format!(
            "Enum variant {}.{} does not support member access for '{}'",
            instance.name, instance.variant, property_name
          )),
          span,
        ))
      }

      // 7. Handle Built-in properties (Standard library extensions).
      _ => {
        let type_name = object.type_name();
        if let Some(callback) = PROPS
          .get(type_name)
          .and_then(|type_props| type_props.get(property_name.as_ref()))
        {
          return (callback)(object.clone())
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

  /// Verifies if a member is accessible based on its visibility and the current execution context.
  fn ensure_accessible(
    &self,
    public: bool,
    member_name: &str,
    struct_name: &str,
    span: Span,
  ) -> ErrorResult {
    if public {
      return Ok(());
    }

    // Check if the current scope is within the same struct definition.
    let stack = self.stack.read();
    let current_struct = stack.get("__current_struct__");
    let allowed = if let Some(ObjectKind::String(name)) = current_struct {
      name.as_ref() == struct_name
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

  /// Allocates a new UserFunction from a StructMethod.
  fn from_method(method: &StructMethod) -> ObjectKind {
    ObjectKind::Function(FunctionKind::User {
      func: Arc::new(UserFunction {
        parameters: method.parameters.clone(),
        body: method.body.clone(),
        kind: method.kind.clone(),
      }),
      env: None,
      this: None,
    })
  }

  /// Binds a method to an instance.
  fn bind_method(method: &StructMethod, instance: ObjectKind) -> ObjectKind {
    ObjectKind::Function(FunctionKind::User {
      func: Arc::new(UserFunction {
        parameters: method.parameters.clone(),
        body: method.body.clone(),
        kind: method.kind.clone(),
      }),
      env: None,
      this: Some(Arc::new(instance)),
    })
  }
}
