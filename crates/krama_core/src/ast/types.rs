use std::{
  fmt::{Display, Formatter, Result as FmtResult},
  sync::Arc,
};

use indexmap::IndexMap;

use crate::{ErrorKind, ErrorKindResult, Literal, Node, Object};

pub type Type = Node<TypeKind>;

#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind {
  I8,
  I16,
  I32,
  I64,
  I128,
  Isize,
  U8,
  U16,
  U32,
  U64,
  U128,
  Usize,
  F32,
  F64,
  Bool,
  Str,
  Null,
  Void,
  Identifier(Arc<str>),
  Array {
    element: Box<Type>,
    size: Option<Literal>,
  },
  Tuple(Vec<Type>),
  Object(IndexMap<Arc<str>, TypeProperty>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeProperty {
  pub ty: Type,
  pub optional: bool,
}

impl TypeKind {
  /// Checks if this type represents any integer variant.
  #[inline(always)]
  pub fn is_integer(&self) -> bool {
    matches!(
      self,
      Self::I8
        | Self::I16
        | Self::I32
        | Self::I64
        | Self::I128
        | Self::Isize
        | Self::U8
        | Self::U16
        | Self::U32
        | Self::U64
        | Self::U128
        | Self::Usize
    )
  }

  /// Checks if this type represents any float variant.
  #[inline(always)]
  pub fn is_float(&self) -> bool {
    matches!(self, Self::F32 | Self::F64)
  }
}

impl Type {
  /// Validates that an object matches this type.
  pub fn check(&self, object: &Object) -> ErrorKindResult<()> {
    match (&self.kind, object) {
      // Integer types - Simplified using helper method.
      (k, Object::Integer(_)) if k.is_integer() => Ok(()),

      // Float types - Simplified using helper method.
      (k, Object::Float(_)) if k.is_float() => Ok(()),

      // Primitives
      (TypeKind::Bool, Object::Bool(_)) => Ok(()),
      (TypeKind::Str, Object::String(_)) => Ok(()),
      (TypeKind::Void, Object::Void) => Ok(()),
      (TypeKind::Null, Object::Null) => Ok(()),

      // Custom types / Identifiers
      (TypeKind::Identifier(name), _)
        if object.type_name() == name.as_ref() =>
      {
        Ok(())
      }

      // Arrays
      (
        TypeKind::Array {
          element: element_type,
          size,
        },
        Object::Array { elements, .. },
      ) => {
        let elements = elements.read();
        if let Some(Literal::Integer(max_size)) = size {
          if elements.len() > *max_size as usize {
            return Err(ErrorKind::TypeError(format!(
              "Expected an array of size {}, but got {}",
              max_size,
              elements.len()
            )));
          }
        }
        for element in elements.iter() {
          element_type.check(element)?;
        }
        Ok(())
      }

      // Tuples
      (TypeKind::Tuple(types), Object::Tuple(elements)) => {
        if types.len() != elements.len() {
          return Err(ErrorKind::TypeError(format!(
            "Expected a tuple of {} elements, but got {}",
            types.len(),
            elements.len()
          )));
        }

        for (kind, element) in types.iter().zip(elements.iter()) {
          kind.check(element)?;
        }
        Ok(())
      }

      // Objects
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
            prop.ty.check(val)?;
          } else if !prop.optional {
            return Err(ErrorKind::TypeError(format!(
              "Missing property '{}'",
              name
            )));
          }
        }
        Ok(())
      }

      _ => Err(ErrorKind::TypeError(format!(
        "Expected type '{}', but got '{}'",
        self.kind,
        object.type_name()
      ))),
    }
  }
}

impl Display for TypeKind {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::I8 => write!(f, "i8"),
      Self::I16 => write!(f, "i16"),
      Self::I32 => write!(f, "i32"),
      Self::I64 => write!(f, "i64"),
      Self::I128 => write!(f, "i128"),
      Self::Isize => write!(f, "isize"),
      Self::U8 => write!(f, "u8"),
      Self::U16 => write!(f, "u16"),
      Self::U32 => write!(f, "u32"),
      Self::U64 => write!(f, "u64"),
      Self::U128 => write!(f, "u128"),
      Self::Usize => write!(f, "usize"),
      Self::F32 => write!(f, "f32"),
      Self::F64 => write!(f, "f64"),
      Self::Bool => write!(f, "bool"),
      Self::Str => write!(f, "str"),
      Self::Null => write!(f, "null"),
      Self::Void => write!(f, "void"),
      Self::Identifier(name) => write!(f, "{}", name),
      Self::Array { element, size } => {
        write!(f, "[]{}", element)?;
        if let Some(size) = size {
          write!(f, "[{}]", size)?;
        }
        Ok(())
      }
      Self::Tuple(types) => {
        write!(f, "(")?;
        for (i, t) in types.iter().enumerate() {
          if i > 0 {
            write!(f, ", ")?;
          }
          write!(f, "{}", t)?;
        }
        write!(f, ")")
      }
      Self::Object(props) => {
        write!(f, "{{")?;
        for (i, (name, prop)) in props.iter().enumerate() {
          if i > 0 {
            write!(f, ", ")?;
          }
          write!(
            f,
            "{}{}: {}",
            name,
            if prop.optional { "?" } else { "" },
            prop.ty
          )?;
        }
        write!(f, "}}")
      }
    }
  }
}
