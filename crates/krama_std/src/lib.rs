use std::sync::LazyLock;

use ahash::AHashMap;
use krama_core::{
  NativeFunction, PropertyFnCb, StandardGlobal, StandardModule,
  StandardProperty,
};

mod globals;
mod modules;
mod props;

pub static GLOBALS: LazyLock<AHashMap<&'static str, NativeFunction>> =
  LazyLock::new(|| {
    inventory::iter::<StandardGlobal>
      .into_iter()
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

pub static MODULES: LazyLock<
  AHashMap<String, AHashMap<&'static str, NativeFunction>>,
> = LazyLock::new(|| {
  let mut modules = AHashMap::default();

  for native in inventory::iter::<StandardModule> {
    let module = modules
      .entry(native.module.to_string())
      .or_insert_with(AHashMap::default);

    module.insert(
      native.name,
      NativeFunction {
        name: native.name,
        callback: native.callback,
      },
    );
  }

  modules
});

pub static PROPS: LazyLock<
  AHashMap<&'static str, AHashMap<&'static str, PropertyFnCb>>,
> = LazyLock::new(|| {
  let mut props = AHashMap::default();

  for prop in inventory::iter::<StandardProperty>() {
    for type_name in prop.types {
      let type_props =
        props.entry(*type_name).or_insert_with(AHashMap::default);
      type_props.insert(prop.name, prop.callback);
    }
  }

  props
});
