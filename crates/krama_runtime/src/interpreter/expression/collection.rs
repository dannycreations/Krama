use std::vec::Vec;

use bumpalo::collections::Vec as BumpVec;
use futures::future::try_join_all;
use krama_core::{Error, Expression, ObjectKind, Span, Type, TypeKind};
use parking_lot::RwLock;

use crate::interpreter::Interpreter;

impl<'ast> Interpreter<'ast> {
  /// Evaluates a collection literal (array or tuple).
  pub async fn eval_collection(
    &self,
    elements: &[Expression<'ast>],
    kind_hint: Option<&Type<'ast>>,
    span: Span,
  ) -> Result<ObjectKind<'ast>, Error<'ast>> {
    // 1. Determine element type hint from the parent collection hint.
    let el_hint = kind_hint.and_then(|hint| {
      if let TypeKind::Array { element, .. } = &hint.kind {
        Some(*element)
      } else {
        None
      }
    });

    // 2. Evaluate all elements concurrently.
    let results = if elements.is_empty() {
      Vec::new()
    } else {
      try_join_all(elements.iter().map(|e| self.eval_expression(e, el_hint)))
        .await?
    };

    // 3. Construct the specific collection type if a hint is present.
    if let Some(hint) = kind_hint {
      match &hint.kind {
        TypeKind::Array { .. } => {
          let mut evals = BumpVec::with_capacity_in(results.len(), self.arena);
          evals.extend(results);
          return Ok(ObjectKind::Array {
            elements: self.arena.alloc(RwLock::new(evals)),
            kind: hint.clone(),
            constant: false,
          });
        }
        TypeKind::Tuple(_) => {
          return Ok(ObjectKind::Tuple {
            elements: self.arena.alloc_slice_fill_iter(results),
          })
        }
        _ => {}
      }
    }

    // 4. Default to Array if empty, or Tuple if non-empty and no hint is available.
    if results.is_empty() {
      Ok(ObjectKind::Array {
        elements: self.arena.alloc(RwLock::new(BumpVec::new_in(self.arena))),
        kind: Type::new(
          TypeKind::Array {
            element: self.arena.alloc(Type::new(TypeKind::Void, span)),
            size: None,
          },
          span,
        ),
        constant: false,
      })
    } else {
      Ok(ObjectKind::Tuple {
        elements: self.arena.alloc_slice_fill_iter(results),
      })
    }
  }

  /// Evaluates an object literal expression by resolving its properties.
  pub async fn eval_object_literal(
    &self,
    properties: &[(Expression<'ast>, Expression<'ast>)],
  ) -> Result<ObjectKind<'ast>, Error<'ast>> {
    let object = self.eval_properties(properties).await?;
    Ok(ObjectKind::Object {
      properties: self.arena.alloc(RwLock::new(object)),
      constant: false,
    })
  }
}
