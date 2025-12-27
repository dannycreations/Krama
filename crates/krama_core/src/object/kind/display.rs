use std::fmt::{self, Display, Formatter};

use super::ObjectKind;
use crate::object::function::FunctionKind;

impl Display for ObjectKind {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::Null => write!(f, "null"),
      Self::Void => write!(f, "void"),
      Self::Boolean(b) => write!(f, "{}", b),
      Self::Integer(i) => write!(f, "{}", i),
      Self::Float(fl) => write!(f, "{}", fl),
      Self::String(s) => write!(f, "{}", s),
      Self::Array { elements, .. } => {
        let elements = elements.read();
        write!(f, "[")?;
        for (i, el) in elements.iter().enumerate() {
          if i > 0 {
            write!(f, ", ")?;
          }
          write!(f, "{el}")?;
        }
        write!(f, "]")
      }
      Self::Tuple(elements) => {
        write!(f, "(")?;
        for (i, el) in elements.iter().enumerate() {
          if i > 0 {
            write!(f, ", ")?;
          }
          write!(f, "{el}")?;
        }
        write!(f, ")")
      }
      Self::Object {
        properties,
        definition,
        ..
      } => {
        if let Some(def) = definition {
          write!(f, "{} {{", def.name)?;
        } else {
          write!(f, "{{")?;
        }
        let properties = properties.read();
        for (i, (key, value)) in properties.iter().enumerate() {
          if i > 0 {
            write!(f, ", ")?;
          }
          write!(f, "{key}: {value}")?;
        }
        write!(f, "}}")
      }
      Self::Scope(s) => write!(
        f,
        "Scope({})",
        s.read().name.as_deref().unwrap_or("anonymous")
      ),
      Self::Function(kind) => match kind {
        FunctionKind::Native(n) => write!(f, "fn {}() [native]", n.name),
        FunctionKind::User { .. } => write!(f, "fn() [user]"),
        FunctionKind::Enum(e) => {
          write!(f, "fn {}.{}() [enum]", e.name, e.variant)
        }
      },
      Self::Return(v) => write!(f, "return {}", v),
      Self::Break => write!(f, "break"),
      Self::Continue => write!(f, "continue"),
      Self::Ok(v) => write!(f, "Ok({})", v),
      Self::Err(v) => write!(f, "Err({})", v),
      Self::Enum(instance) => {
        write!(f, "{}.{}", instance.name, instance.variant)?;
        if let Some(fields) = &instance.fields {
          write!(f, "(")?;
          for (i, field) in fields.iter().enumerate() {
            if i > 0 {
              write!(f, ", ")?;
            }
            write!(f, "{field}")?;
          }
          write!(f, ")")?;
        }
        Ok(())
      }
      Self::Struct(s) => write!(f, "struct {}", s.name),
      Self::Type(t) => write!(f, "type {}", t),
    }
  }
}
