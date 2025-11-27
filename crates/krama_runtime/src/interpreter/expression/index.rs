use futures::future::LocalBoxFuture;
use krama_core::{
  error::{Error, ErrorKind},
  object::Object,
  span::Span,
};

use crate::interpreter::Interpreter;

impl<'ast> Interpreter<'ast> {
  pub(crate) fn eval_index_expression<'s>(
    &'s self,
    object: Object<'ast>,
    index: Object<'ast>,
    span: Span,
  ) -> LocalBoxFuture<'s, Result<Object<'ast>, Error>> {
    Box::pin(async move {
      match object {
        Object::Array { elements, .. } => {
          Self::eval_index_expression_for_sequence(elements, index, span)
        }
        Object::Tuple(elements) => {
          Self::eval_index_expression_for_sequence(elements, index, span)
        }
        Object::String(s) => {
          let idx = match index {
            Object::Integer(i) => i,
            _ => {
              return Err(Error {
                kind: ErrorKind::TypeError(format!(
                  "string indices must be integers, not {}",
                  index.type_name()
                )),
                span,
              })
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
        _ => Err(Error {
          kind: ErrorKind::TypeError(format!(
            "{} does not support indexing",
            object.type_name()
          )),
          span,
        }),
      }
    })
  }

  fn eval_index_expression_for_sequence(
    elements: &'ast [Object<'ast>],
    index: Object<'ast>,
    span: Span,
  ) -> Result<Object<'ast>, Error> {
    let idx = match index {
      Object::Integer(i) => i,
      _ => {
        return Err(Error {
          kind: ErrorKind::TypeError(format!(
            "indices must be integers, not {}",
            index.type_name()
          )),
          span,
        })
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
