pub mod print;

use krama_core::object::{NativeFunctionCb, Object};
use rustc_hash::FxHashMap;

use crate::build_native_functions;

pub fn get_globals<'ast>() -> FxHashMap<&'static str, Object<'ast>> {
  let globals: &[(&'static str, NativeFunctionCb<'ast>)] =
    &[("print", print::print)];
  build_native_functions(globals)
}
