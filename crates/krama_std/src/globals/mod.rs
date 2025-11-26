pub mod print;

use krama_core::object::{NativeFnCallback, Object};
use rustc_hash::FxHashMap;

use crate::build_native_functions;

pub fn get_globals<'ast>() -> FxHashMap<&'static str, Object<'ast>> {
  let globals: &[(&'static str, NativeFnCallback<'ast>)] =
    &[("print", print::print)];
  build_native_functions(globals)
}
