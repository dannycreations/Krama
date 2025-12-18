use ahash::AHashMap;
use krama_core::{
  NativeFunction, PropertyFnCb, StandardGlobal, StandardModule,
  StandardProperty,
};
use once_cell::sync::Lazy;

mod globals;
mod modules;
mod props;

static GLOBALS: Lazy<AHashMap<&'static str, NativeFunction>> =
  Lazy::new(|| {
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

static MODULES: Lazy<AHashMap<String, AHashMap<&'static str, NativeFunction>>> =
  Lazy::new(|| {
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

static PROPS: Lazy<
  AHashMap<&'static str, AHashMap<&'static str, PropertyFnCb>>,
> = Lazy::new(|| {
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

pub fn get_globals() -> &'static AHashMap<&'static str, NativeFunction> {
  &GLOBALS
}

pub fn get_modules(
) -> &'static AHashMap<String, AHashMap<&'static str, NativeFunction>> {
  &MODULES
}

pub fn get_props(
) -> &'static AHashMap<&'static str, AHashMap<&'static str, PropertyFnCb>> {
  &PROPS
}
