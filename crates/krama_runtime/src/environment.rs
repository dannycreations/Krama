use std::rc::Rc;

use krama_core::object::Object;
use rustc_hash::FxHashMap;

#[derive(Debug, Default, Clone)]
pub struct Environment<'ast> {
  store: FxHashMap<&'ast str, (Rc<Object<'ast>>, bool)>,
  outer: Option<Rc<Environment<'ast>>>,
}

impl<'ast> Environment<'ast> {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn new_enclosed(outer: Rc<Environment<'ast>>) -> Self {
    let mut env = Environment::new();
    env.outer = Some(outer);
    env
  }

  pub fn get(&self, name: &str) -> Option<Rc<Object<'ast>>> {
    if let Some((obj, _)) = self.store.get(name) {
      return Some(obj.clone());
    }

    if let Some(outer) = &self.outer {
      return outer.get(name);
    }

    None
  }

  pub fn get_at(
    &self,
    distance: usize,
    name: &str,
  ) -> Option<Rc<Object<'ast>>> {
    if distance == 0 {
      return self.get(name);
    }

    if let Some(outer) = &self.outer {
      outer.get_at(distance - 1, name)
    } else {
      None
    }
  }

  pub fn set(
    &mut self,
    name: &'ast str,
    value: Rc<Object<'ast>>,
    public: bool,
  ) {
    self.store.insert(name, (value, public));
  }

  pub fn get_public_bindings(&self) -> FxHashMap<&'ast str, Rc<Object<'ast>>> {
    self
      .store
      .iter()
      .filter(|(_, (_, public))| *public)
      .map(|(name, (obj, _))| (*name, obj.clone()))
      .collect()
  }
}
