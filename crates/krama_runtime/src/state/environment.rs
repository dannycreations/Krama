use std::cell::RefCell;

use ahash::AHashMap;
use indexmap::IndexMap;
use krama_core::{Function, Object};

/// Represents a variable binding with its metadata.
/// Packed into a struct to potentially allow for better cache locality and future bitfield optimizations.
#[derive(Debug, Clone, PartialEq)]
pub struct Binding<'ast> {
  pub value: Object<'ast>,
  pub public: bool,
  pub constant: bool,
}

#[derive(Debug, Default, Clone)]
pub struct Environment<'ast> {
  /// Store bindings directly using ahash for O(1) lookups.
  pub store: AHashMap<&'ast str, Binding<'ast>>,
  pub outer: Option<&'ast RefCell<Environment<'ast>>>,
}

impl<'ast> Environment<'ast> {
  pub fn new() -> Self {
    Self {
      store: AHashMap::with_capacity(0),
      outer: None,
    }
  }

  pub fn with_globals() -> Self {
    let mut env = Environment::new();
    // Static iteration avoids LazyLock overhead in tight loops.
    for (name, native_fn) in krama_std::GLOBALS.iter() {
      let function = Object::Function(Function::Native(*native_fn));
      env.set(name, function, true, true);
    }
    env
  }

  pub fn new_enclosed(outer: &'ast RefCell<Environment<'ast>>) -> Self {
    Environment {
      store: AHashMap::with_capacity(4), // Small initial capacity for enclosed scopes
      outer: Some(outer),
    }
  }

  pub fn get_local(&self, name: &str) -> Option<Object<'ast>> {
    // Object is designed to be cheap to clone (references or small primitives).
    self.store.get(name).map(|b| b.value.clone())
  }

  pub fn get(&self, name: &str) -> Option<Object<'ast>> {
    if let Some(obj) = self.get_local(name) {
      return Some(obj);
    }

    if let Some(outer_env) = self.outer {
      return outer_env.borrow().get(name);
    }

    None
  }

  pub fn set(
    &mut self,
    name: &'ast str,
    value: Object<'ast>,
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

  pub fn is_constant(&self, name: &str) -> bool {
    self.store.get(name).map(|b| b.constant).unwrap_or(false)
  }

  pub fn get_public_bindings(&self) -> IndexMap<&'ast str, Object<'ast>> {
    self
      .store
      .iter()
      .filter(|(_, b)| b.public)
      .map(|(name, b)| (*name, b.value.clone()))
      .collect()
  }
}
