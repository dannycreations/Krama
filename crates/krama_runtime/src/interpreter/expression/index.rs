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
        let real_idx = if idx < 0 { s.len() as i64 + idx } else { idx };

        Ok(if real_idx >= 0 && (real_idx as usize) < s.len() {
          ObjectKind::String(
            self.arena.alloc_str(
              &s.chars().nth(real_idx as usize).unwrap().to_string(),
            ),
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
  fn ensure_int_index(
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

  #[inline]
  fn get_by_index(
    &self,
    elements: &[ObjectKind<'ast>],
    idx: i64,
  ) -> ObjectKind<'ast> {
    let real_idx = if idx < 0 {
      elements.len() as i64 + idx
    } else {
      idx
    };

    if real_idx >= 0 && (real_idx as usize) < elements.len() {
      elements[real_idx as usize].clone()
    } else {
      ObjectKind::Void
    }
  }
}
