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
    let mut env = Some(self);
    while let Some(current_env) = env {
      if let Some((obj, _)) = current_env.store.get(name) {
        return Some(obj.clone());
      }
      env = current_env.outer.as_deref();
    }
    None
  }

  pub fn get_at(
    &self,
    distance: usize,
    name: &str,
  ) -> Option<Rc<Object<'ast>>> {
    let mut env = Some(self);
    for _ in 0..distance {
      if let Some(current_env) = env {
        env = current_env.outer.as_deref();
      } else {
        return None;
      }
    }
    env.and_then(|e| e.get(name))
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
