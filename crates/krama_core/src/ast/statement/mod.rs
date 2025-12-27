mod kind;
use std::sync::Arc;

pub use kind::*;

use crate::{Expression, Node, Span, Type};

pub type Statement = Node<StatementKind>;

#[derive(Debug, Clone, PartialEq)]
pub struct StatementBlock {
  pub statements: Vec<Statement>,
  pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Destructure {
  pub name: Arc<str>,
  pub alias: Option<Arc<str>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConstBinding {
  Identifier(Arc<str>),
  Destructure(Vec<Destructure>),
  ModuleAndDestructure {
    alias: Arc<str>,
    items: Vec<Destructure>,
  },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
  pub name: Arc<str>,
  pub kind: Option<Type>,
  pub default: Option<Box<Expression>>,
  pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumVariant {
  pub name: Arc<str>,
  pub fields: Option<Vec<Type>>,
  pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructField {
  pub public: bool,
  pub name: Arc<str>,
  pub kind: Type,
  pub default: Option<Box<Expression>>,
  pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructMethod {
  pub public: bool,
  pub instance: bool,
  pub name: Arc<str>,
  pub parameters: Vec<Parameter>,
  pub body: crate::FunctionBody,
  pub kind: Option<Type>,
  pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ForBinding {
  Identifier(Arc<str>),
  Array(Vec<ForBinding>),
}
