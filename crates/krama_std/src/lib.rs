use std::sync::LazyLock;

use ahash::AHashMap;
use krama_core::{
  NativeFunction, PropertyFnCb, STANDARD_GLOBALS, STANDARD_MODULES,
  STANDARD_PROPERTIES,
};

mod globals;
mod modules;
mod props;

// Optimized standard library maps using ahash for O(1) lookups.
// LazyLock ensures zero initialization cost until the first access.

/// Global functions available in the default scope.
pub static GLOBALS: LazyLock<AHashMap<&'static str, NativeFunction>> =
  LazyLock::new(|| {
    let mut map = AHashMap::with_capacity(STANDARD_GLOBALS.len());
    for native in STANDARD_GLOBALS {
      map.insert(
        native.name,
        NativeFunction {
          name: native.name,
          callback: native.callback,
        },
      );
    }
    map
  });

/// Built-in modules organized by name.
pub static MODULES: LazyLock<
  AHashMap<&'static str, AHashMap<&'static str, NativeFunction>>,
> = LazyLock::new(|| {
  let mut modules = AHashMap::with_capacity(STANDARD_MODULES.len());

  for native in STANDARD_MODULES {
    let module = modules
      .entry(native.module)
      .or_insert_with(|| AHashMap::with_capacity(4));

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

/// Built-in properties available on specific types.
pub static PROPS: LazyLock<
  AHashMap<&'static str, AHashMap<&'static str, PropertyFnCb>>,
> = LazyLock::new(|| {
  let mut props = AHashMap::with_capacity(STANDARD_PROPERTIES.len());

  for prop in STANDARD_PROPERTIES {
    for type_name in prop.types {
      let type_props = props
        .entry(*type_name)
        .or_insert_with(|| AHashMap::with_capacity(4));
      type_props.insert(prop.name, prop.callback);
    }
  }

  props
});
