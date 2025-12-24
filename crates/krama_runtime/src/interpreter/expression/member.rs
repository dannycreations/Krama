use bumpalo::Bump;
use krama_core::{
  Error, ErrorKind, Expression, ExpressionKind, FunctionKind, ObjectKind, Span,
  StructMethod, UserFunction,
};
use krama_std::PROPS;

use crate::Interpreter;

impl<'ast> Interpreter<'ast> {
  /// Evaluates a member access expression (e.g., object.property).
  /// Supports objects, structs, modules, and built-in properties.
  pub async fn eval_member_expression(
    &self,
    object: ObjectKind<'ast>,
    property: &Expression<'ast>,
    span: Span,
  ) -> Result<ObjectKind<'ast>, Error<'ast>> {
    // 1. Extract property name from the identifier.
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
      // 2. Handle Object literals.
      ObjectKind::Object { properties, .. } => Ok(
        properties
          .read()
          .get(property_name)
          .cloned()
          .unwrap_or(ObjectKind::Void),
      ),

      // 3. Handle Struct instances (Fields and Methods).
      ObjectKind::StructInstance { definition, fields } => {
        // a. Check fields first.
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

        // b. Check methods if field not found.
        if let Some(method) =
          definition.methods.iter().find(|m| m.name == property_name)
        {
          self.ensure_accessible(
            method.public,
            property_name,
            definition.name,
            span,
          )?;
          return Ok(Self::from_method(method, self.arena));
        }

        Err(Error::new(
          ErrorKind::ReferenceError(format!(
            "Property or method '{}' not found in struct '{}'",
            property_name, definition.name
          )),
          span,
        ))
      }

      // 4. Handle Struct definitions (Static methods).
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
          return Ok(Self::from_method(method, self.arena));
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

      // 6. Handle Built-in properties (Standard library extensions).
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

  /// Allocates a new UserFunction from a StructMethod.
  fn from_method(
    method: &StructMethod<'ast>,
    arena: &'ast Bump,
  ) -> ObjectKind<'ast> {
    ObjectKind::Function(FunctionKind::User(arena.alloc(UserFunction {
      parameters: method.parameters.clone(),
      body: method.body.clone(),
      kind: method.kind.clone(),
    })))
  }
}
