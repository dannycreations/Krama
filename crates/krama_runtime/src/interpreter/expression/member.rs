use krama_core::{
  Error, ErrorKind, Expression, ExpressionKind, Object, ObjectResult, Span,
};
use krama_std::PROPS;

use crate::interpreter::Interpreter;

impl Interpreter {
  pub async fn eval_access_expression(
    &self,
    object: Object,
    property: &Expression,
    span: Span,
  ) -> ObjectResult {
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
      Object::Object {
        properties,
        definition,
        ..
      } => {
        if let Some(value) = properties.read().get(property_name) {
          if let Some(definition) = definition {
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

        if let Some(ref definition) = definition {
          if let Some(&index) = definition.method_map.get(property_name) {
            let method = &definition.methods[index];
            self.ensure_accessible(
              method.public,
              property_name,
              &definition.name,
              span,
            )?;
            return if method.instance {
              Ok(Self::bind_method(method, object.clone()))
            } else {
              Ok(Self::from_method(method))
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
          Ok(Object::Void)
        }
      }

      Object::Struct(definition) => {
        if let Some(&index) = definition.method_map.get(property_name) {
          let method = &definition.methods[index];
          self.ensure_accessible(
            method.public,
            property_name,
            &definition.name,
            span,
          )?;
          return if method.instance {
            Err(Error::new(
              ErrorKind::TypeError(format!(
                "Method '{}' in struct '{}' is an instance method and requires an instance",
                property_name, definition.name
              )),
              span,
            ))
          } else {
            Ok(Self::from_method(method))
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

      Object::Scope(scope) => {
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

      Object::Enum(instance) => {
        if property_name.as_ref() == "variant" {
          return Ok(Object::String(instance.variant.clone()));
        }
        if property_name.as_ref() == "name" {
          return Ok(Object::String(instance.name.clone()));
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

  pub async fn eval_index_expression(
    &self,
    mut object: Object,
    index: Object,
    span: Span,
  ) -> ObjectResult {
    match &mut object {
      Object::Array { elements, .. } => {
        let idx = self.ensure_int_index(&index, span)?;
        Ok(self.get_by_index(&elements.read(), idx))
      }
      Object::Tuple(elements) => {
        let idx = self.ensure_int_index(&index, span)?;
        Ok(self.get_by_index(elements.as_ref(), idx))
      }
      Object::String(s) => {
        let idx = self.ensure_int_index(&index, span)?;
        let real_idx = self.resolve_index(idx, s.len());

        Ok(if let Some(i) = real_idx {
          Object::String(s.chars().nth(i).unwrap().to_string().into())
        } else {
          Object::Void
        })
      }
      Object::Object { properties, .. } => {
        let key = if let Object::String(s) = index {
          s
        } else {
          return Err(Error::new(
            ErrorKind::TypeError(format!(
              "object keys must be strings, not {}",
              index.type_name()
            )),
            span,
          ));
        };

        Ok(properties.read().get(&key).cloned().unwrap_or(Object::Void))
      }
      _ => Err(Error::new(
        ErrorKind::TypeError(format!(
          "{} does not support indexing",
          object.type_name()
        )),
        span,
      )),
    }
  }

  #[inline]
  pub fn ensure_int_index(
    &self,
    index: &Object,
    span: Span,
  ) -> crate::ErrorResult<i64> {
    if let Object::Integer(i) = index {
      Ok(*i)
    } else {
      Err(Error::new(
        ErrorKind::TypeError(format!(
          "indices must be integers, not {}",
          index.type_name()
        )),
        span,
      ))
    }
  }

  #[inline]
  pub fn resolve_index(&self, idx: i64, len: usize) -> Option<usize> {
    let real_idx = if idx < 0 { len as i64 + idx } else { idx };

    if real_idx >= 0 && (real_idx as usize) < len {
      Some(real_idx as usize)
    } else {
      None
    }
  }

  #[inline]
  pub fn get_by_index(&self, elements: &[Object], idx: i64) -> Object {
    if let Some(i) = self.resolve_index(idx, elements.len()) {
      elements[i].clone()
    } else {
      Object::Void
    }
  }

  fn ensure_accessible(
    &self,
    public: bool,
    member_name: &str,
    struct_name: &str,
    span: Span,
  ) -> crate::ErrorResult {
    if public {
      return Ok(());
    }

    let stack = self.stack.read();
    let current_struct = stack.get("__current_struct__");
    let allowed = if let Some(Object::String(name)) = current_struct {
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
}
