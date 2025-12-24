use bumpalo::collections::Vec as BumpVec;
use indexmap::IndexMap;
use krama_core::{Error, ObjectKind, ObjectProperty, Type, TypeKind};

use super::Interpreter;

/// Validates that an object matches the expected type.
/// Delegates the core logic to Type::check in krama_core.
pub fn check_type<'ast>(
  expected_type: &Type<'ast>,
  object: &ObjectKind<'ast>,
) -> Result<(), Error<'ast>> {
  expected_type
    .check(object)
    .map_err(|k| k.at(expected_type.span))
}

/// Recursively resolves type aliases and complex types by looking up identifiers in the environment.
pub fn resolve_type<'ast>(
  interpreter: &Interpreter<'ast>,
  kind: &Type<'ast>,
) -> Result<Type<'ast>, Error<'ast>> {
  // Use a helper to avoid re-wrapping identical types.
  let resolved_kind = match &kind.kind {
    // 1. Resolve type aliases from the environment.
    TypeKind::Identifier(name) => {
      if let Some(ObjectKind::Type(resolved)) =
        interpreter.environment.borrow().get(name)
      {
        return Ok(resolved.clone());
      }
      return Ok(kind.clone());
    }
    // 2. Recursively resolve array element types.
    TypeKind::Array { element, size } => TypeKind::Array {
      element: interpreter.arena.alloc(resolve_type(interpreter, element)?),
      size: *size,
    },
    // 3. Recursively resolve tuple component types.
    TypeKind::Tuple(types) => {
      let mut resolved_types = BumpVec::new_in(interpreter.arena);
      for t in types {
        resolved_types.push(resolve_type(interpreter, t)?);
      }
      TypeKind::Tuple(resolved_types)
    }
    // 4. Recursively resolve object property types.
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
      TypeKind::Object(resolved_properties)
    }
    // 5. Primitives and other types remain unchanged.
    _ => return Ok(kind.clone()),
  };

  Ok(Type::new(resolved_kind, kind.span))
}
