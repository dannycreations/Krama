use std::sync::Arc;

use ahash::AHashMap;
use parking_lot::RwLock;

use super::ObjectKind;

/// Represents a variable binding with its metadata.
#[derive(Debug, Clone, PartialEq)]
pub struct Binding {
  pub value: ObjectKind,
  pub public: bool,
  pub constant: bool,
}

/// Scope represents a single level of variable bindings.
/// Optimized with AHashMap for O(1) lookups and memory efficiency.
#[derive(Debug, Clone)]
pub struct Scope {
  pub name: Option<String>,
  pub bindings: AHashMap<String, Binding>,
  pub parent: Option<Arc<RwLock<Scope>>>,
}

impl Scope {
  /// Creates a new scope, optionally with a parent.
  /// Pre-allocates space for bindings if a parent is provided (likely a block or function).
  pub fn new(name: Option<String>, parent: Option<Arc<RwLock<Scope>>>) -> Self {
    Self {
      name,
      bindings: AHashMap::with_capacity(if parent.is_some() { 4 } else { 0 }),
      parent,
    }
  }

  /// Retrieves a binding from the scope (local only).
  #[inline(always)]
  pub fn get_local(&self, name: &str) -> Option<&Binding> {
    self.bindings.get(name)
  }

  /// Retrieves a binding by traversing the scope chain.
  pub fn get(&self, name: &str) -> Option<ObjectKind> {
    if let Some(binding) = self.get_local(name) {
      return Some(binding.value.clone());
    }

    let mut current = self.parent.clone();
    while let Some(parent_cell) = current {
      let parent = parent_cell.read();
      if let Some(binding) = parent.get_local(name) {
        return Some(binding.value.clone());
      }
      current = parent.parent.clone();
    }
    None
  }

  /// Sets or updates a binding in the current scope.
  pub fn set(
    &mut self,
    name: &str,
    value: ObjectKind,
    public: bool,
    constant: bool,
  ) {
    self.bindings.insert(
      name.to_string(),
      Binding {
        value,
        public,
        constant,
      },
    );
  }
}
