use krama_core::{Error, ErrorKind, ErrorResult, ForBinding, ObjectKind, Span};

use crate::interpreter::Interpreter;

impl Interpreter {
  /// Collects elements from an iterable for a for-loop.
  pub fn collect_iterable_elements(
    &self,
    iterable: &ObjectKind,
    binding: &ForBinding,
    span: Span,
  ) -> ErrorResult<Vec<ObjectKind>> {
    match iterable {
      ObjectKind::Array { elements, .. } => Ok(elements.read().to_vec()),
      ObjectKind::Tuple(elements) => Ok(elements.as_ref().to_vec()),
      ObjectKind::String(s) => Ok(
        s.chars()
          .map(|c| ObjectKind::String(c.to_string().into()))
          .collect(),
      ),
      ObjectKind::Object { properties, .. } => {
        let props = properties.read();
        let mut yields = Vec::with_capacity(props.len());

        match binding {
          // If destructuring key-value pairs: [k, v] in obj
          ForBinding::Array(bindings) if bindings.len() == 2 => {
            for (k, v) in props.iter() {
              let elements = vec![ObjectKind::String(k.clone()), v.clone()];
              yields.push(self.heap.write().alloc_tuple(elements));
            }
          }
          // Default to iterating over keys.
          _ => {
            for k in props.keys() {
              yields.push(ObjectKind::String(k.clone()));
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
    binding: &ForBinding,
    value: ObjectKind,
    span: Span,
  ) -> ErrorResult {
    match binding {
      ForBinding::Identifier(name) => {
        self
          .stack
          .write()
          .define(name.clone(), value.clone(), false, false);
        Ok(())
      }
      ForBinding::Array(bindings) => {
        let elements = match &value {
          ObjectKind::Array { elements, .. } => elements.read().to_vec(),
          ObjectKind::Tuple(elements) => elements.as_ref().to_vec(),
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
          self.assign_for_binding(binding, val, span)?;
        }
        Ok(())
      }
    }
  }
}
