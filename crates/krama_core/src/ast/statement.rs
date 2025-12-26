use std::sync::Arc;

use crate::{Expression, FunctionBody, Node, Span, Type};

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
  pub is_public: bool,
  pub is_static: bool,
  pub name: Arc<str>,
  pub parameters: Vec<Parameter>,
  pub body: FunctionBody,
  pub kind: Option<Type>,
  pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ForBinding {
  Identifier(Arc<str>),
  Array(Vec<ForBinding>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatementKind {
  Test {
    name: Box<Expression>,
    body: Box<StatementBlock>,
  },
  Const {
    public: bool,
    binding: ConstBinding,
    kind: Option<Type>,
    value: Box<Expression>,
  },
  Let {
    name: Arc<str>,
    kind: Option<Type>,
    value: Box<Expression>,
  },
  Fn {
    public: bool,
    name: Arc<str>,
    parameters: Vec<Parameter>,
    body: FunctionBody,
    kind: Option<Type>,
  },
  Enum {
    public: bool,
    name: Arc<str>,
    variants: Vec<EnumVariant>,
  },
  Struct {
    public: bool,
    name: Arc<str>,
    fields: Vec<StructField>,
    methods: Vec<StructMethod>,
  },
  Type {
    public: bool,
    name: Arc<str>,
    kind: Type,
  },
  Expression {
    expression: Box<Expression>,
  },
  Return {
    value: Option<Box<Expression>>,
  },
  While {
    condition: Box<Expression>,
    body: Box<StatementBlock>,
  },
  For {
    binding: ForBinding,
    iterable: Box<Expression>,
    body: Box<StatementBlock>,
  },
  Break,
  Continue,
}
