use std::sync::Arc;

use ahash::AHashMap;
use indexmap::IndexMap;
use krama_core::{FunctionKind, ObjectKind};
use krama_std::GLOBALS;
use parking_lot::RwLock;

/// Represents a variable binding with its metadata.
#[derive(Debug, Clone, PartialEq)]
pub struct Binding {
  pub value: ObjectKind,
  pub public: bool,
  pub constant: bool,
}

/// Manages variable bindings and scope chains.
/// Optimized for fast lookups and minimal memory overhead.
#[derive(Debug, Default, Clone)]
pub struct Environment {
  /// Local bindings in the current scope.
  pub store: AHashMap<String, Binding>,
  /// Parent scope for delegation.
  pub outer: Option<Arc<RwLock<Environment>>>,
}

impl Environment {
  /// Creates a new empty environment.
  pub fn new() -> Self {
    Self {
      store: AHashMap::with_capacity(0),
      outer: None,
    }
  }

  /// Creates a new environment populated with standard library globals.
  /// This is typically used for the root scope of the interpreter.
  pub fn with_globals() -> Self {
    let mut env = Environment::new();
    // Pre-allocate space for globals to avoid re-hashes.
    env.store.reserve(GLOBALS.len());
    for (name, native_fn) in GLOBALS.iter() {
      let function = ObjectKind::Function(FunctionKind::Native(*native_fn));
      env.set(name, function, true, true);
    }
    env
  }

  /// Creates a new environment that encloses an existing one.
  /// Small initial capacity is used to balance memory vs allocation frequency.
  pub fn new_enclosed(outer: Arc<RwLock<Environment>>) -> Self {
    Environment {
      store: AHashMap::with_capacity(4),
      outer: Some(outer),
    }
  }

  /// Retrieves a binding from the local scope only.
  /// Inlined for performance as it's a hot path in the interpreter.
  #[inline(always)]
  pub fn get_local(&self, name: &str) -> Option<ObjectKind> {
    self.store.get(name).map(|b| b.value.clone())
  }

  /// Retrieves a variable value by traversing the scope chain.
  /// Iterative approach avoids stack overflow in deeply nested scopes.
  pub fn get(&self, name: &str) -> Option<ObjectKind> {
    if let Some(obj) = self.get_local(name) {
      return Some(obj);
    }

    let mut current = self.outer.clone();
    while let Some(outer_cell) = current {
      let outer = outer_cell.read();
      if let Some(obj) = outer.get_local(name) {
        return Some(obj);
      }
      current = outer.outer.clone();
    }

    None
  }

  /// Sets or updates a binding in the current scope.
  /// If the binding already exists, it is overwritten.
  pub fn set(
    &mut self,
    name: &str,
    value: ObjectKind,
    public: bool,
    constant: bool,
  ) {
    self.store.insert(
      name.to_string(),
      Binding {
        value,
        public,
        constant,
      },
    );
  }

  /// Checks if a variable is marked as constant in the local scope.
  /// Used to prevent re-assignment to 'const' variables.
  #[inline(always)]
  pub fn is_constant(&self, name: &str) -> bool {
    self.store.get(name).map(|b| b.constant).unwrap_or(false)
  }

  /// Returns all public bindings from the current scope.
  /// Useful for module exports and reflection.
  pub fn get_public_bindings(&self) -> IndexMap<String, ObjectKind> {
    self
      .store
      .iter()
      .filter(|(_, b)| b.public)
      .map(|(name, b)| (name.clone(), b.value.clone()))
      .collect()
  }
}
