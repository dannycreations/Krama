pub mod array;
pub mod string;

use futures::future::LocalBoxFuture;
use krama_core::{error::Error, object::Object};
use rustc_hash::FxHashMap;

pub type PropFn =
  for<'ast> fn(
    Object<'ast>,
  ) -> LocalBoxFuture<'ast, Result<Object<'ast>, Error>>;

pub fn get_props() -> FxHashMap<(&'static str, &'static str), PropFn> {
  let mut props = FxHashMap::default();
  props.extend(array::get_props());
  props.extend(string::get_props());
  props
}
