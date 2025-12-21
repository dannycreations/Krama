use bumpalo::collections::Vec as BumpVec;
use indexmap::IndexMap;
use krama_core::{
  Error, ErrorKind, Literal, Object, ObjectProperty, Type, TypeKind,
};

pub fn check_type<'ast>(
  expected_type: &Type<'ast>,
  object: &Object<'ast>,
) -> Result<(), Error<'ast>> {
  let mismatched = match (&expected_type.kind, object) {
    (TypeKind::I8, Object::Integer(_)) => false,
    (TypeKind::I16, Object::Integer(_)) => false,
    (TypeKind::I32, Object::Integer(_)) => false,
    (TypeKind::I64, Object::Integer(_)) => false,
    (TypeKind::I128, Object::Integer(_)) => false,
    (TypeKind::Isize, Object::Integer(_)) => false,
    (TypeKind::U8, Object::Integer(_)) => false,
    (TypeKind::U16, Object::Integer(_)) => false,
    (TypeKind::U32, Object::Integer(_)) => false,
    (TypeKind::U64, Object::Integer(_)) => false,
    (TypeKind::U128, Object::Integer(_)) => false,
    (TypeKind::Usize, Object::Integer(_)) => false,
    (TypeKind::F32, Object::Float(_)) => false,
    (TypeKind::F64, Object::Float(_)) => false,
    (TypeKind::Bool, Object::Boolean(_)) => false,
    (TypeKind::Str, Object::String(_)) => false,
    (TypeKind::Identifier(name), _) => {
      // If the expected type is an identifier, we check if the object's type name matches.
      // This handles custom types like Enums.
      object.type_name() != *name
    }
    (
      TypeKind::Array {
        element: element_type,
        size,
      },
      Object::Array { elements, .. },
    ) => {
      let elements = elements.read();
      if let Some(Literal::Integer(size)) = size {
        if elements.len() > *size as usize {
          return Err(Error::new(
            ErrorKind::TypeError(format!(
              "Expected an array of size {}, but got {}",
              size,
              elements.len()
            )),
            expected_type.span,
          ));
        }
      }
      for element in elements.iter() {
        check_type(element_type, element)?;
      }
      return Ok(());
    }
    (TypeKind::Tuple(types), Object::Tuple { elements }) => {
      if types.len() != elements.len() {
        return Err(Error::new(
          ErrorKind::TypeError(format!(
            "Expected a tuple of {} elements, but got {}",
            types.len(),
            elements.len()
          )),
          expected_type.span,
        ));
      }

      for (kind, element) in types.iter().zip(elements.iter()) {
        check_type(kind, element)?;
      }
      return Ok(());
    }
    (
      TypeKind::Object(properties),
      Object::Object {
        properties: obj_props,
        ..
      },
    ) => {
      let obj_props = obj_props.read();
      for (name, prop) in properties {
        if let Some(val) = obj_props.get(name) {
          check_type(&prop.kind, val)?;
        } else if !prop.optional {
          return Err(Error::new(
            ErrorKind::TypeError(format!("Missing property '{}'", name)),
            expected_type.span,
          ));
        }
      }
      return Ok(());
    }
    (TypeKind::Void, Object::Void) => false,
    (TypeKind::Null, Object::Null) => false,
    _ => true,
  };

  if mismatched {
    return Err(Error::new(
      ErrorKind::TypeError(format!(
        "Expected type {:?}, but got {:?}",
        expected_type.kind, object
      )),
      expected_type.span,
    ));
  }

  Ok(())
}

pub fn resolve_type<'ast>(
  interpreter: &super::Interpreter<'ast>,
  kind: &Type<'ast>,
) -> Result<Type<'ast>, Error<'ast>> {
  match &kind.kind {
    TypeKind::Identifier(name) => {
      if let Some(Object::Type(resolved)) =
        interpreter.environment.borrow().get(name)
      {
        Ok(resolved.clone())
      } else {
        Ok(kind.clone())
      }
    }
    TypeKind::Array { element, size } => {
      let resolved_element = resolve_type(interpreter, element)?;
      Ok(Type::new(
        TypeKind::Array {
          element: interpreter.arena.alloc(resolved_element),
          size: *size,
        },
        kind.span,
      ))
    }
    TypeKind::Tuple(types) => {
      let mut resolved_types = BumpVec::new_in(interpreter.arena);
      for t in types {
        resolved_types.push(resolve_type(interpreter, t)?);
      }
      Ok(Type::new(TypeKind::Tuple(resolved_types), kind.span))
    }
    TypeKind::Object(properties) => {
      let mut resolved_properties = IndexMap::new();
      for (name, prop) in properties {
        resolved_properties.insert(
          *name,
          ObjectProperty {
            kind: resolve_type(interpreter, &prop.kind)?,
            optional: prop.optional,
          },
        );
      }
      Ok(Type::new(TypeKind::Object(resolved_properties), kind.span))
    }
    _ => Ok(kind.clone()),
  }
}
