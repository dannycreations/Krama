use krama_core::{
  ast::{
    literal::Literal,
    types::{Type, TypeKind},
  },
  error::{Error, ErrorKind},
  object::Object,
};

pub(crate) fn check_type<'ast>(
  expected_type: &Type<'ast>,
  object: &Object<'ast>,
) -> Result<(), Error> {
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
    (TypeKind::Identifier(_), _) => {
      // TODO: check for functions and other identifiers
      false
    }
    (
      TypeKind::Array {
        element: element_type,
        size,
      },
      Object::Array { elements, .. },
    ) => {
      if let Some(Literal::Integer(size)) = size {
        if elements.len() > *size as usize {
          return Err(Error {
            span: expected_type.span,
            kind: ErrorKind::TypeError(format!(
              "Expected an array of size {}, but got {}",
              size,
              elements.len()
            )),
            file_path: None,
            source: None,
          });
        }
      }
      for element in elements.iter() {
        check_type(element_type, element)?;
      }
      return Ok(());
    }
    (TypeKind::Tuple(types), Object::Tuple { elements }) => {
      if types.len() != elements.len() {
        return Err(Error {
          span: expected_type.span,
          kind: ErrorKind::TypeError(format!(
            "Expected a tuple of {} elements, but got {}",
            types.len(),
            elements.len()
          )),
          file_path: None,
          source: None,
        });
      }

      for (kind, element) in types.iter().zip(elements.iter()) {
        check_type(kind, element)?;
      }
      return Ok(());
    }
    _ => true,
  };

  if mismatched {
    return Err(Error {
      span: expected_type.span,
      kind: ErrorKind::TypeError(format!(
        "Expected type {:?}, but got {:?}",
        expected_type.kind, object
      )),
      file_path: None,
      source: None,
    });
  }

  Ok(())
}
