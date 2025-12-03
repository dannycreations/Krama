use std::cell::RefCell;

use ahash::AHashMap;
use krama_core::object::{Function, Object};

#[derive(Debug, Default, Clone)]
pub struct Environment<'ast> {
  pub store: AHashMap<&'ast str, (Object<'ast>, bool)>,
  pub outer: Option<&'ast RefCell<Environment<'ast>>>,
}

impl<'ast> Environment<'ast> {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn with_globals() -> Self {
    let mut env = Environment::new();
    for (name, native_fn) in krama_std::get_globals().iter() {
      let function = Object::Function(Function::Native(*native_fn));
      env.set(name, function, true);
    }
    env
  }

  pub fn new_enclosed(outer: &'ast RefCell<Environment<'ast>>) -> Self {
    Environment {
      store: AHashMap::default(),
      outer: Some(outer),
    }
  }

  pub fn get_local(&self, name: &str) -> Option<Object<'ast>> {
    self.store.get(name).map(|(obj, _)| obj.clone())
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

  pub fn set(&mut self, name: &'ast str, value: Object<'ast>, public: bool) {
    self.store.insert(name, (value, public));
  }

  pub fn get_public_bindings(&self) -> AHashMap<&'ast str, Object<'ast>> {
    self
      .store
      .iter()
      .filter(|(_, (_, public))| *public)
      .map(|(name, (obj, _))| (*name, obj.clone()))
      .collect()
  }
}
