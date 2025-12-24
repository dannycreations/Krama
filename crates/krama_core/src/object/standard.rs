use crate::{NativeFnCb, PropertyFnCb};

/// Registration for globally available functions.
pub struct StandardGlobal {
  pub name: &'static str,
  pub callback: NativeFnCb,
}

#[linkme::distributed_slice]
pub static STANDARD_GLOBALS: [StandardGlobal];

/// Registration for built-in modules.
pub struct StandardModule {
  pub name: &'static str,
  pub callback: NativeFnCb,
  pub module: &'static str,
}

#[linkme::distributed_slice]
pub static STANDARD_MODULES: [StandardModule];

/// Registration for built-in properties on specific types.
pub struct StandardProperty {
  pub name: &'static str,
  pub callback: PropertyFnCb,
  pub types: &'static [&'static str],
}

#[linkme::distributed_slice]
pub static STANDARD_PROPERTIES: [StandardProperty];
