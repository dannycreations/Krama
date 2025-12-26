use std::sync::Arc;

use ahash::AHashMap;

use crate::{StructField, StructMethod};

/// Represents a structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Struct {
  pub name: Arc<str>,
  pub fields: Vec<StructField>,
  pub methods: Vec<StructMethod>,
  pub field_map: AHashMap<Arc<str>, usize>,
  pub method_map: AHashMap<Arc<str>, usize>,
}

/// Represents an enum definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Enum {
  pub name: Arc<str>,
  pub variant: Arc<str>,
  pub field_count: usize,
}

/// Represents an instance of an enum variant.
/// Boxed in ObjectKind to reduce the size of the ObjectKind enum.
#[derive(Debug, Clone, PartialEq)]
pub struct EnumInstance {
  pub name: Arc<str>,
  pub variant: Arc<str>,
  pub fields: Option<Arc<[crate::ObjectKind]>>,
}
