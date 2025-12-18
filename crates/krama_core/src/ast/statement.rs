use bumpalo::collections::Vec as BumpVec;

use crate::{Expression, FunctionBody, Node, Span, Type};

pub type Statement<'ast> = Node<'ast, StatementKind<'ast>>;

#[derive(Debug, Clone, PartialEq)]
pub struct BlockStatement<'ast> {
  pub statements: BumpVec<'ast, Statement<'ast>>,
  pub span: Span<'ast>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Destructure<'ast> {
  pub name: &'ast str,
  pub alias: Option<&'ast str>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Binding<'ast> {
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
  pub span: Span<'ast>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatementKind<'ast> {
  Test {
    name: &'ast Expression<'ast>,
    body: &'ast BlockStatement<'ast>,
  },
  Const {
    public: bool,
    binding: Binding<'ast>,
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
  Expression {
    expression: &'ast Expression<'ast>,
  },
  Return {
    value: Option<&'ast Expression<'ast>>,
  },
  While {
    condition: &'ast Expression<'ast>,
    body: &'ast BlockStatement<'ast>,
  },
  Break,
  Continue,
}
