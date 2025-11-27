pub mod assert;
pub mod fs;

use krama_core::object::Object;
use once_cell::sync::Lazy;
use phf::phf_map;
use rustc_hash::FxHashMap;

type ModuleGetter = fn() -> FxHashMap<&'static str, Object<'static>>;

static MODULES: Lazy<phf::Map<&'static str, ModuleGetter>> = Lazy::new(|| {
  phf_map! {
    "assert" => assert::get_exports,
    "fs" => fs::get_exports,
  }
});

pub fn get_modules<'ast>(
  name: &str,
) -> Option<FxHashMap<&'static str, Object<'ast>>> {
  MODULES.get(name).map(|get_exports| {
    let map: FxHashMap<&'static str, Object> = get_exports();
    map
      .into_iter()
      .map(|(k, v)| {
        (k, unsafe {
          std::mem::transmute::<Object<'static>, Object<'ast>>(v)
        })
      })
      .collect()
  })
}
