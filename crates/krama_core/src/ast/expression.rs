use bumpalo::collections::Vec as BumpVec;

use crate::{
  AssignmentOperator, BinaryOperator, LiteralKind, Node, Parameter,
  StatementBlock, Type, UnaryOperator, UpdateOperator,
};

pub type Expression<'ast> = Node<'ast, ExpressionKind<'ast>>;

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionBody<'ast> {
  Block(&'ast StatementBlock<'ast>),
  Expression(&'ast Expression<'ast>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionKind<'ast> {
  Identifier(&'ast str),
  Literal(LiteralKind<'ast>),
  This,
  StructConstruction {
    properties: BumpVec<'ast, (Expression<'ast>, Expression<'ast>)>,
  },
  Block(&'ast StatementBlock<'ast>),
  Collection {
    elements: BumpVec<'ast, Expression<'ast>>,
  },
  Object {
    properties: BumpVec<'ast, (Expression<'ast>, Expression<'ast>)>,
  },
  Assignment {
    left: &'ast Expression<'ast>,
    operator: AssignmentOperator,
    right: &'ast Expression<'ast>,
  },
  Binary {
    left: &'ast Expression<'ast>,
    operator: BinaryOperator,
    right: &'ast Expression<'ast>,
  },
  Unary {
    operator: UnaryOperator,
    right: &'ast Expression<'ast>,
  },
  Update {
    operator: UpdateOperator,
    argument: &'ast Expression<'ast>,
    prefix: bool,
  },
  If {
    condition: &'ast Expression<'ast>,
    then_branch: &'ast Expression<'ast>,
    else_branch: Option<&'ast Expression<'ast>>,
  },
  Match {
    subject: &'ast Expression<'ast>,
    arms: BumpVec<'ast, Match<'ast>>,
  },
  Import {
    path: &'ast str,
    items: Option<BumpVec<'ast, &'ast str>>,
  },
  Call {
    function: &'ast Expression<'ast>,
    arguments: BumpVec<'ast, Expression<'ast>>,
  },
  Fn {
    parameters: BumpVec<'ast, Parameter<'ast>>,
    body: FunctionBody<'ast>,
    kind: Option<Type<'ast>>,
  },
  Member {
    object: &'ast Expression<'ast>,
    property: &'ast Expression<'ast>,
  },
  Index {
    object: &'ast Expression<'ast>,
    index: &'ast Expression<'ast>,
  },
  Typed {
    expr: &'ast Expression<'ast>,
    kind: Type<'ast>,
  },
  Try(&'ast Expression<'ast>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Match<'ast> {
  pub patterns: BumpVec<'ast, MatchPattern<'ast>>,
  pub body: FunctionBody<'ast>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchPattern<'ast> {
  Expression(Expression<'ast>),
  Range(Expression<'ast>, Expression<'ast>),
  Else,
}
