use krama_core::{error::ErrorKind, object::Object, span::Span};

use crate::interpreter::Interpreter;

impl<'ast> Interpreter<'ast> {
  pub(crate) async fn eval_index_expression(
    &self,
    mut object: Object<'ast>,
    index: Object<'ast>,
    span: Span<'ast>,
  ) -> Result<Object<'ast>, (ErrorKind, Span<'ast>)> {
    let index = self.resolve_object(index).await?;
    match &mut object {
      Object::Array { elements, .. } => {
        Self::eval_index_expression_for_sequence(elements, index, span)
      }
      Object::Tuple { elements } => {
        Self::eval_index_expression_for_sequence(elements, index, span)
      }
      Object::String(s) => {
        let idx = match index {
          Object::Integer(i) => i,
          _ => {
            return Err((
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
      _ => Err((
        ErrorKind::TypeError(format!(
          "{} does not support indexing",
          object.type_name()
        )),
        span,
      )),
    }
  }

  fn eval_index_expression_for_sequence(
    elements: &[Object<'ast>],
    index: Object<'ast>,
    span: Span<'ast>,
  ) -> Result<Object<'ast>, (ErrorKind, Span<'ast>)> {
    let idx = match index {
      Object::Integer(i) => i,
      _ => {
        return Err((
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
}
