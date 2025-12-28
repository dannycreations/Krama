use std::sync::Arc;

use krama_core::{Error, ErrorKind, ErrorResult, Expression, Object, Span};

use super::Interpreter;

impl Interpreter {
  /// Retrieves a variable value from a specific scope distance.
  pub fn get_at(&self, distance: usize, name: &str) -> Option<Object> {
    let stack = self.stack.read();
    let mut current_scope = stack.current();

    for _ in 0..distance {
      let next = current_scope.read().parent.as_ref().map(Arc::clone)?;
      current_scope = next;
    }

    let scope = current_scope.read();
    scope.get_local(name).map(|b| b.value.clone())
  }

  /// Assigns a value to a variable at a specific scope distance.
  pub fn assign_at(
    &self,
    distance: usize,
    name: &str,
    value: Object,
    span: Span,
  ) -> ErrorResult {
    let stack = self.stack.read();
    let mut current_scope = stack.current();

    for _ in 0..distance {
      let next = current_scope
        .read()
        .parent
        .as_ref()
        .map(Arc::clone)
        .ok_or_else(|| {
          Error::new(
            ErrorKind::RuntimeError(format!(
              "Invalid scope distance {} for '{}'",
              distance, name
            )),
            span,
          )
        })?;
      current_scope = next;
    }

    let mut scope = current_scope.write();
    if let Some(binding) = scope.bindings.get_mut(name) {
      if binding.constant {
        return Err(Error::new(
          ErrorKind::TypeError(format!("Cannot assign to constant '{}'", name)),
          span,
        ));
      }
      binding.value = value;
      Ok(())
    } else {
      Err(Error::new(
        ErrorKind::ReferenceError(format!(
          "Variable '{}' not found at distance {}",
          name, distance
        )),
        span,
      ))
    }
  }

  /// Returns the resolved scope distance for a given expression.
  pub fn get_resolved_distance(&self, expr: &Expression) -> Option<usize> {
    self.locals.read().get(&expr.span).copied()
  }
}
