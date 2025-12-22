use ahash::AHashMap;

use super::ObjectKind;

/// Scope represents a single level of variable bindings.
/// Optimized with AHashMap for O(1) lookups and memory efficiency.
#[derive(Debug, Clone, PartialEq)]
pub struct Scope<'ast> {
  pub name: Option<&'ast str>,
  pub bindings: AHashMap<&'ast str, ObjectKind<'ast>>,
}

impl<'ast> Scope<'ast> {
  /// Creates a new scope.
  #[inline(always)]
  pub fn new(name: Option<&'ast str>) -> Self {
    Self {
      name,
      bindings: AHashMap::with_capacity(0), // Start with zero capacity to save memory for empty scopes
    }
  }

  /// Retrieves a binding from the scope.
  #[inline(always)]
  pub fn get_binding(&self, name: &str) -> Option<&ObjectKind<'ast>> {
    self.bindings.get(name)
  }
}
