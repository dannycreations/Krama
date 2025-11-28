use std::rc::Rc;

use rustc_hash::FxHashMap;

use crate::object::Object;

#[derive(Debug, Clone, PartialEq)]
pub struct Scope<'ast> {
  pub name: Option<&'ast str>,
  pub bindings: FxHashMap<&'ast str, Rc<Object<'ast>>>,
}
