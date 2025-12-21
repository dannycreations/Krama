use krama_core::{
  Error, ErrorKind, Expression, ExpressionKind, Function, Object, Span,
  UserFunction,
};

use crate::Interpreter;

impl<'ast> Interpreter<'ast> {
  pub async fn eval_member_expression(
    &self,
    object: Object<'ast>,
    property: &Expression<'ast>,
    span: Span,
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
      Object::StructInstance { definition, fields } => {
        // 1. Check fields
        {
          let fields = fields.read();
          if let Some(value) = fields.get(property_name) {
            // Find the field definition
            if let Some(field_def) =
              definition.fields.iter().find(|f| f.name == property_name)
            {
              if !field_def.public {
                let env = self.environment.borrow();
                let current_struct = env.get("__current_struct__");
                let allowed = if let Some(Object::String(name)) = current_struct
                {
                  name == definition.name
                } else {
                  false
                };

                if !allowed {
                  return Err(Error::new(
                    ErrorKind::TypeError(format!(
                      "Property '{}' is private",
                      property_name
                    )),
                    span,
                  ));
                }
              }
            }

            return Ok(value.clone());
          }
        }

        // 2. Check methods
        if let Some(method) =
          definition.methods.iter().find(|m| m.name == property_name)
        {
          if !method.public {
            let env = self.environment.borrow();
            let current_struct = env.get("__current_struct__");
            let allowed = if let Some(Object::String(name)) = current_struct {
              name == definition.name
            } else {
              false
            };

            if !allowed {
              return Err(Error::new(
                ErrorKind::TypeError(format!(
                  "Method '{}' is private",
                  property_name
                )),
                span,
              ));
            }
          }

          let user_fn = self.arena.alloc(UserFunction {
            parameters: method.parameters.clone(),
            body: method.body.clone(),
            kind: method.kind.clone(),
          });
          return Ok(Object::Function(Function::User(user_fn)));
        }

        // 3. Not found
        Err(Error::new(
          ErrorKind::ReferenceError(format!(
            "Property or method '{}' not found in struct '{}'",
            property_name, definition.name
          )),
          span,
        ))
      }
      Object::Struct(definition) => {
        if let Some(method) =
          definition.methods.iter().find(|m| m.name == property_name)
        {
          if !method.public {
            let env = self.environment.borrow();
            let current_struct = env.get("__current_struct__");
            let allowed = if let Some(Object::String(name)) = current_struct {
              name == definition.name
            } else {
              false
            };

            if !allowed {
              return Err(Error::new(
                ErrorKind::TypeError(format!(
                  "Method '{}' is private",
                  property_name
                )),
                span,
              ));
            }
          }

          let user_fn = self.arena.alloc(UserFunction {
            parameters: method.parameters.clone(),
            body: method.body.clone(),
            kind: method.kind.clone(),
          });
          return Ok(Object::Function(Function::User(user_fn)));
        }

        Err(Error::new(
          ErrorKind::ReferenceError(format!(
            "Method '{}' not found in struct '{}'",
            property_name, definition.name
          )),
          span,
        ))
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
