use bumpalo::collections::Vec as BumpVec;

use crate::{StructField, StructMethod};

/// Represents an Enum definition in Krama.
#[derive(Debug, Clone, PartialEq)]
pub struct Enum<'ast> {
  pub name: &'ast str,
  pub variant: &'ast str,
  pub field_count: usize,
}

/// Represents a Struct definition in Krama.
#[derive(Debug, Clone, PartialEq)]
pub struct Struct<'ast> {
  pub name: &'ast str,
  pub fields: BumpVec<'ast, StructField<'ast>>,
  pub methods: BumpVec<'ast, StructMethod<'ast>>,
}
