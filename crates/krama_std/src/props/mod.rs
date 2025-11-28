pub mod length;

use std::iter::FromIterator;

use futures::future::LocalBoxFuture;
use krama_core::object::Object;
use rustc_hash::FxHashMap;

use self::length::length;

pub type PropFn<'ast> =
  fn(
    Object<'ast>,
  ) -> LocalBoxFuture<'ast, Result<Object<'ast>, krama_core::error::Error>>;

pub fn get_props<'ast>(
) -> FxHashMap<&'static str, FxHashMap<&'static str, PropFn<'ast>>> {
  let mut props = FxHashMap::default();
  let length_prop = FxHashMap::from_iter([("length", length as PropFn)]);

  props.insert("array", length_prop.clone());
  props.insert("tuple", length_prop.clone());
  props.insert("string", length_prop);

  props
}
