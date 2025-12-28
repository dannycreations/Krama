use indexmap::IndexMap;
use krama_core::{ErrorResult, Object, Type, TypeKind, TypeProperty};

use super::Interpreter;

/// Validates that an object matches the expected type.
/// Delegates the core logic to Type::check in krama_core.
pub fn check_type(expected_type: &Type, object: &Object) -> ErrorResult {
  expected_type
    .check(object)
    .map_err(|k| k.at(expected_type.span))
}

/// Recursively resolves type aliases and complex types by looking up identifiers in the environment.
pub fn resolve_type(
  interpreter: &Interpreter,
  node: &Type,
) -> ErrorResult<Type> {
  // Use a helper to avoid re-wrapping identical types.
  let resolved_kind = match &node.kind {
    // 1. Resolve type aliases from the environment.
    TypeKind::Identifier(name) => {
      // We look up in the environment to see if the identifier refers to a type definition.
      if let Some(Object::Type(resolved)) = interpreter.stack.read().get(name) {
        return Ok(resolved.clone());
      }
      // If not found, we keep it as an identifier (might be resolved later or used as a nominal type).
      return Ok(node.clone());
    }
    // 2. Recursively resolve array element types.
    TypeKind::Array { element, size } => TypeKind::Array {
      element: Box::new(resolve_type(interpreter, element)?),
      size: size.clone(),
    },
    // 3. Recursively resolve tuple component types.
    TypeKind::Tuple(types) => TypeKind::Tuple(
      types
        .iter()
        .map(|t| resolve_type(interpreter, t))
        .collect::<ErrorResult<Vec<_>>>()?,
    ),
    // 4. Recursively resolve object property types.
    TypeKind::Object(properties) => TypeKind::Object(
      properties
        .iter()
        .map(|(name, prop)| {
          Ok((
            name.clone(),
            TypeProperty {
              ty: resolve_type(interpreter, &prop.ty)?,
              optional: prop.optional,
            },
          ))
        })
        .collect::<ErrorResult<IndexMap<_, _>>>()?,
    ),
    // 5. Primitives and other types remain unchanged.
    _ => return Ok(node.clone()),
  };

  Ok(Type::new(resolved_kind, node.span))
}
