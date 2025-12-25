use krama_core::{Error, ErrorKind, Expression, ObjectResult, Span};

use super::Interpreter;

impl Interpreter {
  /// Evaluates an identifier, checking resolved distances first for O(1) local access.
  /// Falls back to global environment lookup if no distance is resolved.
  #[inline]
  pub async fn eval_identifier(
    &self,
    expression: &Expression,
    name: &str,
    span: Span,
  ) -> ObjectResult {
    // 1. O(1) Fast path: Use pre-resolved scope distance from semantic analysis.
    // This avoids O(N) traversal of the environment chain where N is the scope depth.
    if let Some(distance) = self.get_resolved_distance(expression) {
      // get_at is O(distance) but avoids locking intermediate scopes for too long.
      if let Some(value) = self.get_at(distance, name) {
        return Ok(value);
      }
    }

    // 2. Slow path: Global environment lookup for variables not captured by static analysis (e.g. dynamic globals).
    // This is O(N) traversal up the scope chain.
    self.stack.read().get(name).ok_or_else(|| {
      Error::new(
        ErrorKind::ReferenceError(format!("'{}' is not defined", name)),
        span,
      )
    })
  }
}
