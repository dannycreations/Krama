pub mod array;
pub mod length;
pub mod string;

use futures::future::LocalBoxFuture;
use krama_core::object::Object;
use rustc_hash::FxHashMap;

pub type PropFn<'ast> =
  fn(
    Object<'ast>,
  ) -> LocalBoxFuture<'ast, Result<Object<'ast>, krama_core::error::Error>>;

pub fn get_props<'ast>() -> FxHashMap<(&'static str, &'static str), PropFn<'ast>>
{
  let mut props = FxHashMap::default();
  props.extend(array::get_props());
  props.extend(string::get_props());
  props
}
