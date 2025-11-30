pub mod globals;
pub mod modules;
pub mod props;

use krama_core::object::{
  NativeFunction, PropertyFnCb, StandardNative, StandardProperty,
};
use once_cell::sync::Lazy;
use rustc_hash::FxHashMap;

static GLOBALS: Lazy<FxHashMap<&'static str, NativeFunction>> =
  Lazy::new(|| {
    inventory::iter::<StandardNative>
      .into_iter()
      .filter(|n| n.module == "globals")
      .map(|native| {
        (
          native.name,
          NativeFunction {
            name: native.name,
            callback: native.callback,
          },
        )
      })
      .collect()
  });

static MODULES: Lazy<
  FxHashMap<String, FxHashMap<&'static str, NativeFunction>>,
> = Lazy::new(|| {
  let mut modules = FxHashMap::default();

  for native in inventory::iter::<StandardNative> {
    if native.module != "globals" {
      let module = modules
        .entry(native.module.to_string())
        .or_insert_with(FxHashMap::default);

      module.insert(
        native.name,
        NativeFunction {
          name: native.name,
          callback: native.callback,
        },
      );
    }
  }

  modules
});

static PROPS: Lazy<
  FxHashMap<&'static str, FxHashMap<&'static str, PropertyFnCb>>,
> = Lazy::new(|| {
  let mut props = FxHashMap::default();

  for prop in inventory::iter::<StandardProperty>() {
    for type_name in prop.types {
      let type_props =
        props.entry(*type_name).or_insert_with(FxHashMap::default);
      type_props.insert(prop.name, prop.callback);
    }
  }

  props
});

pub fn get_globals() -> &'static FxHashMap<&'static str, NativeFunction> {
  &GLOBALS
}

pub fn get_modules(
) -> &'static FxHashMap<String, FxHashMap<&'static str, NativeFunction>> {
  &MODULES
}

pub fn get_props(
) -> &'static FxHashMap<&'static str, FxHashMap<&'static str, PropertyFnCb>> {
  &PROPS
}
