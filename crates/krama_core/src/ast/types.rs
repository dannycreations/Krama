use std::fmt::{Display, Formatter, Result as FmtResult};

use bumpalo::collections::Vec as BumpVec;
use indexmap::IndexMap;

use crate::{ErrorKind, LiteralKind, Node, ObjectKind};

pub type Type<'ast> = Node<'ast, TypeKind<'ast>>;

#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind<'ast> {
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
  Identifier(&'ast str),
  Array {
    element: &'ast Type<'ast>,
    size: Option<LiteralKind<'ast>>,
  },
  Tuple(BumpVec<'ast, Type<'ast>>),
  Object(IndexMap<&'ast str, ObjectProperty<'ast>>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectProperty<'ast> {
  pub kind: Type<'ast>,
  pub optional: bool,
}

impl<'ast> Type<'ast> {
  /// Validates that an object matches this type.
  pub fn check(&self, object: &ObjectKind<'ast>) -> Result<(), ErrorKind> {
    match (&self.kind, object) {
      // Integer types
      (
        TypeKind::I8
        | TypeKind::I16
        | TypeKind::I32
        | TypeKind::I64
        | TypeKind::I128
        | TypeKind::Isize
        | TypeKind::U8
        | TypeKind::U16
        | TypeKind::U32
        | TypeKind::U64
        | TypeKind::U128
        | TypeKind::Usize,
        ObjectKind::Integer(_),
      ) => Ok(()),

      // Float types
      (TypeKind::F32 | TypeKind::F64, ObjectKind::Float(_)) => Ok(()),

      // Primitives
      (TypeKind::Bool, ObjectKind::Boolean(_)) => Ok(()),
      (TypeKind::Str, ObjectKind::String(_)) => Ok(()),
      (TypeKind::Void, ObjectKind::Void) => Ok(()),
      (TypeKind::Null, ObjectKind::Null) => Ok(()),

      // Custom types / Identifiers
      (TypeKind::Identifier(name), _) if object.type_name() == *name => Ok(()),

      // Arrays
      (
        TypeKind::Array {
          element: element_type,
          size,
        },
        ObjectKind::Array { elements, .. },
      ) => {
        let elements = elements.read();
        if let Some(LiteralKind::Integer(max_size)) = size {
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
      (TypeKind::Tuple(types), ObjectKind::Tuple { elements }) => {
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
        ObjectKind::Object {
          properties: obj_props,
          ..
        },
      ) => {
        let obj_props = obj_props.read();
        for (name, prop) in properties {
          if let Some(val) = obj_props.get(name) {
            prop.kind.check(val)?;
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
        "Expected type {}, but got {}",
        self.kind,
        object.type_name()
      ))),
    }
  }
}

impl<'ast> Display for TypeKind<'ast> {
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
      Self::Bool => write!(f, "boolean"),
      Self::Str => write!(f, "string"),
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
            prop.kind
          )?;
        }
        write!(f, "}}")
      }
    }
  }
}
