pub mod globals;
pub mod modules;
pub mod props;

use krama_core::object::{
  Function, NativeFunction, Object, PropertyFnCb, StandardNative,
  StandardProperty,
};
use rustc_hash::FxHashMap;

fn build_object_map<'ast>(
  natives: impl Iterator<Item = &'static StandardNative>,
) -> FxHashMap<&'static str, Object<'ast>> {
  natives
    .map(|native| {
      (
        native.name,
        Object::Function(Function::Native(NativeFunction {
          name: native.name,
          callback: native.callback,
        })),
      )
    })
    .collect()
}

pub fn build_globals<'ast>() -> FxHashMap<&'static str, Object<'ast>> {
  build_object_map(
    inventory::iter::<StandardNative>
      .into_iter()
      .filter(|n| n.module == "globals"),
  )
}

pub fn build_modules<'ast>(
) -> FxHashMap<String, FxHashMap<&'static str, Object<'ast>>> {
  let mut modules = FxHashMap::default();

  for native in inventory::iter::<StandardNative> {
    if native.module != "globals" {
      let module = modules
        .entry(native.module.to_string())
        .or_insert_with(FxHashMap::default);

      module.insert(
        native.name,
        Object::Function(Function::Native(NativeFunction {
          name: native.name,
          callback: native.callback,
        })),
      );
    }
  }

  modules
}

pub fn build_props(
) -> FxHashMap<&'static str, FxHashMap<&'static str, PropertyFnCb>> {
  let mut props = FxHashMap::default();

  for prop in inventory::iter::<StandardProperty>() {
    for type_name in prop.types {
      let type_props =
        props.entry(*type_name).or_insert_with(FxHashMap::default);
      type_props.insert(prop.name, prop.callback);
    }
  }

  props
}
