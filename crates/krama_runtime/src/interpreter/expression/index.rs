use krama_core::{Error, ErrorKind, ObjectKind, Span};

use crate::Interpreter;

impl<'ast> Interpreter<'ast> {
  pub async fn eval_index_expression(
    &self,
    mut object: ObjectKind<'ast>,
    index: ObjectKind<'ast>,
    span: Span,
  ) -> Result<ObjectKind<'ast>, Error<'ast>> {
    match &mut object {
      ObjectKind::Array { elements, .. } => {
        let idx = self.ensure_int_index(&index, span)?;
        Ok(self.get_by_index(&elements.read(), idx))
      }
      ObjectKind::Tuple { elements } => {
        let idx = self.ensure_int_index(&index, span)?;
        Ok(self.get_by_index(elements, idx))
      }
      ObjectKind::String(s) => {
        let idx = self.ensure_int_index(&index, span)?;
        let real_idx = self.resolve_index(idx, s.len());

        Ok(if let Some(i) = real_idx {
          ObjectKind::String(
            self.arena.alloc_str(&s.chars().nth(i).unwrap().to_string()),
          )
        } else {
          ObjectKind::Void
        })
      }
      ObjectKind::Object { properties, .. } => {
        let key = if let ObjectKind::String(s) = index {
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

        Ok(
          properties
            .read()
            .get(key)
            .cloned()
            .unwrap_or(ObjectKind::Void),
        )
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
    index: &ObjectKind<'ast>,
    span: Span,
  ) -> Result<i64, Error<'ast>> {
    if let ObjectKind::Integer(i) = index {
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

  /// Resolves a potentially negative index into a valid absolute index.
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
  pub fn get_by_index(
    &self,
    elements: &[ObjectKind<'ast>],
    idx: i64,
  ) -> ObjectKind<'ast> {
    if let Some(i) = self.resolve_index(idx, elements.len()) {
      elements[i].clone()
    } else {
      ObjectKind::Void
    }
  }
}
