use bumpalo::collections::Vec as BumpVec;

use super::expression::Expression;
use super::types::Type;
use crate::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Statement<'ast> {
  pub kind: StatementKind<'ast>,
  pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockStatement<'ast> {
  pub statements: BumpVec<'ast, Statement<'ast>>,
  pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DestructuredIdentifier<'ast> {
  pub name: &'ast str,
  pub alias: Option<&'ast str>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Binding<'ast> {
  Identifier(&'ast str),
  Destructure(BumpVec<'ast, DestructuredIdentifier<'ast>>),
  ModuleAndDestructure {
    module_alias: &'ast str,
    items: BumpVec<'ast, DestructuredIdentifier<'ast>>,
  },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter<'ast> {
  pub name: &'ast str,
  pub kind: Option<Type<'ast>>,
  pub span: Span,
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
    body: &'ast BlockStatement<'ast>,
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
