use std::sync::Arc;

use ahash::AHashMap;
use parking_lot::RwLock;

use super::kind::ObjectKind;

/// Represents a variable binding with its metadata.
#[derive(Debug, Clone, PartialEq)]
pub struct Binding {
  pub value: ObjectKind,
  pub public: bool,
  pub constant: bool,
}

/// Scope represents a single level of variable bindings.
#[derive(Debug, Clone)]
pub struct Scope {
  pub name: Option<Arc<str>>,
  pub bindings: AHashMap<Arc<str>, Binding>,
  pub parent: Option<Arc<RwLock<Scope>>>,
}

impl Scope {
  pub fn new(
    name: Option<Arc<str>>,
    parent: Option<Arc<RwLock<Scope>>>,
  ) -> Self {
    Self {
      name,
      bindings: AHashMap::with_capacity(if parent.is_some() { 4 } else { 0 }),
      parent,
    }
  }

  #[inline(always)]
  pub fn get_local(&self, name: &str) -> Option<&Binding> {
    self.bindings.get(name)
  }

  pub fn get(&self, name: &str) -> Option<ObjectKind> {
    if let Some(binding) = self.get_local(name) {
      return Some(binding.value.clone());
    }

    let mut current = self.parent.as_ref().map(Arc::clone);
    while let Some(parent_cell) = current {
      let parent = parent_cell.read();
      if let Some(binding) = parent.get_local(name) {
        return Some(binding.value.clone());
      }
      current = parent.parent.as_ref().map(Arc::clone);
    }
    None
  }

  pub fn set(
    &mut self,
    name: Arc<str>,
    value: ObjectKind,
    public: bool,
    constant: bool,
  ) {
    self.bindings.insert(
      name,
      Binding {
        value,
        public,
        constant,
      },
    );
  }
}
