use bumpalo::collections::Vec as BumpVec;
use indexmap::IndexMap;

use crate::{Literal, Node};

pub type Type<'ast> = Node<'ast, TypeKind<'ast>>;

#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind<'ast> {
  I8,
  I16,
  I32,
  I64,
  I128,
  Isize,
  U8,
  U16,
  U32,
  U64,
  U128,
  Usize,
  F32,
  F64,
  Bool,
  Str,
  Null,
  Void,
  Identifier(&'ast str),
  Array {
    element: &'ast Type<'ast>,
    size: Option<Literal<'ast>>,
  },
  Tuple(BumpVec<'ast, Type<'ast>>),
  Object(IndexMap<&'ast str, ObjectProperty<'ast>>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectProperty<'ast> {
  pub kind: Type<'ast>,
  pub optional: bool,
}
