use bumpalo::collections::Vec as BumpVec;
use indexmap::IndexMap;
use krama_core::{Error, ObjectKind, ObjectProperty, Type, TypeKind};

use super::Interpreter;

/// Validates that an object matches the expected type.
/// Now delegates to Type::check in krama_core.
pub fn check_type<'ast>(
  expected_type: &Type<'ast>,
  object: &ObjectKind<'ast>,
) -> Result<(), Error<'ast>> {
  expected_type
    .check(object)
    .map_err(|k| k.at(expected_type.span))
}

/// Recursively resolves type aliases and complex types.
pub fn resolve_type<'ast>(
  interpreter: &Interpreter<'ast>,
  kind: &Type<'ast>,
) -> Result<Type<'ast>, Error<'ast>> {
  match &kind.kind {
    TypeKind::Identifier(name) => {
      if let Some(ObjectKind::Type(resolved)) =
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
      let mut resolved_properties = IndexMap::with_capacity(properties.len());
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
