use ahash::AHashMap;

use crate::object::Object;

#[derive(Debug, Clone, PartialEq)]
pub struct Scope<'ast> {
  pub name: Option<&'ast str>,
  pub bindings: AHashMap<&'ast str, Object<'ast>>,
}
