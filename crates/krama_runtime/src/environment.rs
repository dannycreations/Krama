use krama_core::object::Object;
use rustc_hash::FxHashMap;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub struct Environment<'ast> {
  store: FxHashMap<&'ast str, (Rc<Object<'ast>>, bool)>,
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

  pub fn get(&self, name: &str) -> Option<Rc<Object<'ast>>> {
    if let Some((obj, _)) = self.store.get(name) {
      return Some(obj.clone());
    }

    let mut current_outer = self.outer.clone();
    while let Some(env_rc) = current_outer {
      let env_borrow = env_rc.borrow();
      if let Some((obj, _)) = env_borrow.store.get(name) {
        return Some(obj.clone());
      }
      current_outer = env_borrow.outer.clone();
    }

    None
  }

  pub fn set(&mut self, name: &'ast str, value: Object<'ast>, public: bool) {
    self.store.insert(name, (Rc::new(value), public));
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
