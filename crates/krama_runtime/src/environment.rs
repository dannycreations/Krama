use std::cell::RefCell;

use krama_core::object::Object;
use rustc_hash::FxHashMap;

#[derive(Debug, Default, Clone)]
pub struct Environment<'ast> {
  pub store: FxHashMap<&'ast str, (Object<'ast>, bool)>,
  pub outer: Option<&'ast RefCell<Environment<'ast>>>,
}

impl<'ast> Environment<'ast> {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn with_globals() -> Self {
    let mut env = Environment::new();
    for (name, obj) in krama_std::build_globals() {
      env.set(name, obj, true);
    }
    env
  }

  pub fn new_enclosed(outer: &'ast RefCell<Environment<'ast>>) -> Self {
    Environment {
      store: FxHashMap::default(),
      outer: Some(outer),
    }
  }

  pub fn get(&self, name: &str) -> Option<Object<'ast>> {
    if let Some((obj, _)) = self.store.get(name) {
      return Some(obj.clone());
    }

    let mut outer = self.outer;
    while let Some(outer_cell) = outer {
      let env = outer_cell.borrow();
      if let Some((obj, _)) = env.store.get(name) {
        return Some(obj.clone());
      }
      outer = env.outer;
    }

    None
  }

  pub fn set(&mut self, name: &'ast str, value: Object<'ast>, public: bool) {
    self.store.insert(name, (value, public));
  }

  pub fn get_public_bindings(&self) -> FxHashMap<&'ast str, Object<'ast>> {
    self
      .store
      .iter()
      .filter(|(_, (_, public))| *public)
      .map(|(name, (obj, _))| (*name, obj.clone()))
      .collect()
  }
}
