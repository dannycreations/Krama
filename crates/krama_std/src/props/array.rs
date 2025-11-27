use rustc_hash::FxHashMap;

use super::{length::length, PropFn};

pub fn get_props<'ast>() -> FxHashMap<(&'static str, &'static str), PropFn<'ast>>
{
  let mut props = FxHashMap::default();
  props.insert(("array", "length"), length as PropFn<'ast>);
  props.insert(("tuple", "length"), length as PropFn<'ast>);
  props
}
