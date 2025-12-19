use krama_core::{Error, ErrorKind, Object, Span};

use crate::Interpreter;

impl<'ast> Interpreter<'ast> {
  pub async fn eval_index_expression(
    &self,
    mut object: Object<'ast>,
    index: Object<'ast>,
    span: Span<'ast>,
  ) -> Result<Object<'ast>, Error<'ast>> {
    match &mut object {
      Object::Array { elements, .. } | Object::Tuple { elements } => {
        let idx = match index {
          Object::Integer(i) => i,
          _ => {
            return Err(Error::new(
              ErrorKind::TypeError(format!(
                "indices must be integers, not {}",
                index.type_name()
              )),
              span,
            ))
          }
        };

        let element = if idx < 0 {
          elements.get((elements.len() as i64 + idx) as usize)
        } else {
          elements.get(idx as usize)
        };

        if let Some(element) = element {
          Ok(element.clone())
        } else {
          Ok(Object::Void)
        }
      }
      Object::String(s) => {
        let idx = match index {
          Object::Integer(i) => i,
          _ => {
            return Err(Error::new(
              ErrorKind::TypeError(format!(
                "string indices must be integers, not {}",
                index.type_name()
              )),
              span,
            ))
          }
        };

        let char = if idx < 0 {
          s.chars().nth_back((idx.abs() - 1) as usize)
        } else {
          s.chars().nth(idx as usize)
        };

        if let Some(c) = char {
          let new_str = self.arena.alloc_str(&c.to_string());
          Ok(Object::String(new_str))
        } else {
          Ok(Object::Void)
        }
      }
      Object::Object(map) => {
        let key = match index {
          Object::String(s) => s,
          _ => {
            return Err(Error::new(
              ErrorKind::TypeError(format!(
                "object keys must be strings, not {}",
                index.type_name()
              )),
              span,
            ))
          }
        };

        let map = map.read().await;
        if let Some(value) = map.get(key) {
          Ok(value.clone())
        } else {
          Ok(Object::Void)
        }
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
}
