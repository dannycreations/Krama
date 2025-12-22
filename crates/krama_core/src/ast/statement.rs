use bumpalo::collections::Vec as BumpVec;

use crate::{Expression, FunctionBody, Node, Span, Type};

pub type Statement<'ast> = Node<'ast, StatementKind<'ast>>;

#[derive(Debug, Clone, PartialEq)]
pub struct StatementBlock<'ast> {
  pub statements: BumpVec<'ast, Statement<'ast>>,
  pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Destructure<'ast> {
  pub name: &'ast str,
  pub alias: Option<&'ast str>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConstBinding<'ast> {
  Identifier(&'ast str),
  Destructure(BumpVec<'ast, Destructure<'ast>>),
  ModuleAndDestructure {
    alias: &'ast str,
    items: BumpVec<'ast, Destructure<'ast>>,
  },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter<'ast> {
  pub name: &'ast str,
  pub kind: Option<Type<'ast>>,
  pub default: Option<&'ast Expression<'ast>>,
  pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumVariant<'ast> {
  pub name: &'ast str,
  pub fields: Option<BumpVec<'ast, Type<'ast>>>,
  pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructField<'ast> {
  pub public: bool,
  pub name: &'ast str,
  pub kind: Type<'ast>,
  pub default: Option<&'ast Expression<'ast>>,
  pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructMethod<'ast> {
  pub public: bool,
  pub name: &'ast str,
  pub parameters: BumpVec<'ast, Parameter<'ast>>,
  pub body: FunctionBody<'ast>,
  pub kind: Option<Type<'ast>>,
  pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ForBinding<'ast> {
  Identifier(&'ast str),
  Array(BumpVec<'ast, ForBinding<'ast>>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatementKind<'ast> {
  Test {
    name: &'ast Expression<'ast>,
    body: &'ast StatementBlock<'ast>,
  },
  Const {
    public: bool,
    binding: ConstBinding<'ast>,
    kind: Option<Type<'ast>>,
    value: &'ast Expression<'ast>,
  },
  Let {
    name: &'ast str,
    kind: Option<Type<'ast>>,
    value: &'ast Expression<'ast>,
  },
  Fn {
    public: bool,
    name: &'ast str,
    parameters: BumpVec<'ast, Parameter<'ast>>,
    body: FunctionBody<'ast>,
    kind: Option<Type<'ast>>,
  },
  Enum {
    public: bool,
    name: &'ast str,
    variants: BumpVec<'ast, EnumVariant<'ast>>,
  },
  Struct {
    public: bool,
    name: &'ast str,
    fields: BumpVec<'ast, StructField<'ast>>,
    methods: BumpVec<'ast, StructMethod<'ast>>,
  },
  Type {
    public: bool,
    name: &'ast str,
    kind: Type<'ast>,
  },
  Expression {
    expression: &'ast Expression<'ast>,
  },
  Return {
    value: Option<&'ast Expression<'ast>>,
  },
  While {
    condition: &'ast Expression<'ast>,
    body: &'ast StatementBlock<'ast>,
  },
  For {
    binding: ForBinding<'ast>,
    iterable: &'ast Expression<'ast>,
    body: &'ast StatementBlock<'ast>,
  },
  Break,
  Continue,
}
