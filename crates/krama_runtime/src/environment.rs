use std::{cell::RefCell, rc::Rc};

use krama_core::object::Object;
use rustc_hash::FxHashMap;

#[derive(Debug, Default, Clone)]
pub struct Environment<'ast> {
  store: FxHashMap<&'ast str, (Object<'ast>, bool)>,
  outer: Option<Rc<RefCell<Environment<'ast>>>>,
}

impl<'ast> Environment<'ast> {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn new_enclosed(outer: Rc<RefCell<Environment<'ast>>>) -> Self {
    let mut env = Environment::new();
    env.outer = Some(outer);
    env
  }

  pub fn get(&self, name: &str) -> Option<Object<'ast>> {
    if let Some((obj, _)) = self.store.get(name) {
      return Some(obj.clone());
    }

    if let Some(outer) = &self.outer {
      return outer.borrow().get(name);
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
