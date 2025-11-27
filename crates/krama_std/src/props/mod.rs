pub mod length;

use futures::future::LocalBoxFuture;
use krama_core::object::Object;
use rustc_hash::FxHashMap;

use self::length::length;

pub type PropFn<'ast> =
  fn(
    Object<'ast>,
  ) -> LocalBoxFuture<'ast, Result<Object<'ast>, krama_core::error::Error>>;

pub fn get_props<'ast>() -> FxHashMap<(&'static str, &'static str), PropFn<'ast>>
{
  let mut props = FxHashMap::default();
  props.insert(("array", "length"), length as PropFn<'ast>);
  props.insert(("tuple", "length"), length as PropFn<'ast>);
  props.insert(("string", "length"), length as PropFn<'ast>);
  props
}
