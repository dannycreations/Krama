use krama_core::{Error, ErrorKind, Expression, ObjectKind, Span};

use super::Interpreter;

impl<'ast> Interpreter<'ast> {
  /// Evaluates an identifier, checking resolved distances first for O(1) local access.
  /// Falls back to global environment lookup if no distance is resolved.
  #[inline]
  pub async fn eval_identifier(
    &self,
    expression: &Expression<'ast>,
    name: &'ast str,
    span: Span,
  ) -> Result<ObjectKind<'ast>, Error<'ast>> {
    // 1. Fast path: Use pre-resolved scope distance from semantic analysis.
    // This avoids traversing the environment chain manually.
    if let Some(distance) = self.get_resolved_distance(expression) {
      if let Some(value) = self.get_at(distance, name) {
        return Ok(value);
      }
    }

    // 2. Slow path: Global environment lookup for variables not captured by static analysis (e.g. dynamic globals).
    self.environment.borrow().get(name).ok_or_else(|| {
      Error::new(
        ErrorKind::ReferenceError(format!("'{}' is not defined", name)),
        span,
      )
    })
  }
}
