use std::cell::RefCell;

use ahash::AHashMap;
use indexmap::IndexMap;
use krama_core::{FunctionKind, ObjectKind};
use krama_std::GLOBALS;

/// Represents a variable binding with its metadata.
#[derive(Debug, Clone, PartialEq)]
pub struct Binding<'ast> {
  pub value: ObjectKind<'ast>,
  pub public: bool,
  pub constant: bool,
}

/// Manages variable bindings and scope chains.
#[derive(Debug, Default, Clone)]
pub struct Environment<'ast> {
  /// Local bindings in the current scope.
  pub store: AHashMap<&'ast str, Binding<'ast>>,
  /// Parent scope for delegation.
  pub outer: Option<&'ast RefCell<Environment<'ast>>>,
}

impl<'ast> Environment<'ast> {
  /// Creates a new empty environment.
  pub fn new() -> Self {
    Self {
      store: AHashMap::with_capacity(0),
      outer: None,
    }
  }

  /// Creates a new environment populated with standard library globals.
  pub fn with_globals() -> Self {
    let mut env = Environment::new();
    // Populate environment with built-in functions.
    for (name, native_fn) in GLOBALS.iter() {
      let function = ObjectKind::Function(FunctionKind::Native(*native_fn));
      env.set(name, function, true, true);
    }
    env
  }

  /// Creates a new environment that encloses an existing one.
  pub fn new_enclosed(outer: &'ast RefCell<Environment<'ast>>) -> Self {
    Environment {
      store: AHashMap::with_capacity(4), // Small initial capacity for efficiency.
      outer: Some(outer),
    }
  }

  /// Retrieves a binding from the local scope only.
  #[inline(always)]
  pub fn get_local(&self, name: &str) -> Option<ObjectKind<'ast>> {
    self.store.get(name).map(|b| b.value.clone())
  }

  /// Retrieves a variable value by traversing the scope chain.
  /// Iterative approach avoids stack overflow in deep nesting.
  pub fn get(&self, name: &str) -> Option<ObjectKind<'ast>> {
    if let Some(obj) = self.get_local(name) {
      return Some(obj);
    }

    let mut current = self.outer;
    while let Some(outer_cell) = current {
      let outer = outer_cell.borrow();
      if let Some(obj) = outer.get_local(name) {
        return Some(obj);
      }
      current = outer.outer;
    }

    None
  }

  /// Sets or updates a binding in the current scope.
  pub fn set(
    &mut self,
    name: &'ast str,
    value: ObjectKind<'ast>,
    public: bool,
    constant: bool,
  ) {
    self.store.insert(
      name,
      Binding {
        value,
        public,
        constant,
      },
    );
  }

  /// Checks if a variable is marked as constant in the local scope.
  pub fn is_constant(&self, name: &str) -> bool {
    self.store.get(name).map(|b| b.constant).unwrap_or(false)
  }

  /// Returns all public bindings from the current scope.
  pub fn get_public_bindings(&self) -> IndexMap<&'ast str, ObjectKind<'ast>> {
    self
      .store
      .iter()
      .filter(|(_, b)| b.public)
      .map(|(name, b)| (*name, b.value.clone()))
      .collect()
  }
}
