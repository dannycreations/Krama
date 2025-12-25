use crate::{StructField, StructMethod};

/// Represents an Enum definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Enum {
  pub name: String,
  pub variant: String,
  pub field_count: usize,
}

/// Represents a Struct definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Struct {
  pub name: String,
  pub fields: Vec<StructField>,
  pub methods: Vec<StructMethod>,
}
