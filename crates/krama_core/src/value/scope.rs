use indexmap::IndexMap;

use super::Object;

#[derive(Debug, Clone, PartialEq)]
pub struct Scope<'ast> {
  pub name: Option<&'ast str>,
  pub bindings: IndexMap<&'ast str, Object<'ast>>,
}

impl<'ast> Scope<'ast> {
  #[inline(always)]
  pub fn get_binding(&self, name: &str) -> Option<&Object<'ast>> {
    self.bindings.get(name)
  }
}
