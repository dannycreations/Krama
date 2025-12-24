use bumpalo::collections::Vec as BumpVec;
use krama_core::{Error, ErrorKind, ForBinding, ObjectKind, Span};

use crate::interpreter::Interpreter;

impl<'ast> Interpreter<'ast> {
  /// Collects elements from an iterable for a for-loop.
  pub fn collect_iterable_elements(
    &self,
    iterable: &ObjectKind<'ast>,
    binding: &ForBinding<'ast>,
    span: Span,
  ) -> Result<Vec<ObjectKind<'ast>>, Error<'ast>> {
    match iterable {
      ObjectKind::Array { elements, .. } => Ok(elements.read().to_vec()),
      ObjectKind::Tuple { elements } => Ok(elements.to_vec()),
      ObjectKind::String(s) => Ok(
        s.chars()
          .map(|c| ObjectKind::String(self.arena.alloc_str(&c.to_string())))
          .collect(),
      ),
      ObjectKind::Object { properties, .. } => {
        let props = properties.read();
        let mut yields = Vec::with_capacity(props.len());

        match binding {
          // If destructuring key-value pairs: [k, v] in obj
          ForBinding::Array(bindings) if bindings.len() == 2 => {
            for (k, v) in props.iter() {
              let mut elements = BumpVec::with_capacity_in(2, self.arena);
              elements.push(ObjectKind::String(k));
              elements.push(v.clone());
              yields.push(ObjectKind::Tuple {
                elements: elements.into_bump_slice(),
              });
            }
          }
          // Default to iterating over keys.
          _ => {
            for &k in props.keys() {
              yields.push(ObjectKind::String(k));
            }
          }
        }
        Ok(yields)
      }
      _ => Err(Error::new(
        ErrorKind::TypeError(format!(
          "Expected array, tuple, string or object for for..in loop, found {}",
          iterable.type_name()
        )),
        span,
      )),
    }
  }

  /// Assigns a loop element to the loop binding.
  pub fn assign_for_binding(
    &self,
    interpreter: &Interpreter<'ast>,
    binding: &ForBinding<'ast>,
    value: ObjectKind<'ast>,
    span: Span,
  ) -> Result<(), Error<'ast>> {
    match binding {
      ForBinding::Identifier(name) => {
        interpreter.environment.borrow_mut().set(
          name,
          value.clone(),
          false,
          false,
        );
        Ok(())
      }
      ForBinding::Array(bindings) => {
        let elements = match &value {
          ObjectKind::Array { elements, .. } => elements.read().to_vec(),
          ObjectKind::Tuple { elements } => elements.to_vec(),
          _ => {
            return Err(Error::new(
              ErrorKind::TypeError(format!(
                "Expected array or tuple for destructuring, found {}",
                value.type_name()
              )),
              span,
            ));
          }
        };

        for (i, binding) in bindings.iter().enumerate() {
          let val = elements.get(i).cloned().unwrap_or(ObjectKind::Void);
          self.assign_for_binding(interpreter, binding, val, span)?;
        }
        Ok(())
      }
    }
  }
}
