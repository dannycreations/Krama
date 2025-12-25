use ahash::AHashMap;

use super::ObjectKind;

/// Scope represents a single level of variable bindings.
/// Optimized with AHashMap for O(1) lookups and memory efficiency.
#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
  pub name: Option<String>,
  pub bindings: AHashMap<String, ObjectKind>,
}

impl Scope {
  /// Creates a new scope.
  #[inline(always)]
  pub fn new(name: Option<String>) -> Self {
    Self {
      name,
      bindings: AHashMap::with_capacity(0), // Start with zero capacity to save memory for empty scopes
    }
  }

  /// Retrieves a binding from the scope.
  #[inline(always)]
  pub fn get_binding(&self, name: &str) -> Option<&ObjectKind> {
    self.bindings.get(name)
  }
}
