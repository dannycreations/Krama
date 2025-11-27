use bumpalo::collections::Vec as BumpVec;

use super::{
  literal::Literal,
  node::Node,
  operator::{
    AssignmentOperator, BinaryOperator, UnaryOperator, UpdateOperator,
  },
  statement::{BlockStatement, Parameter},
  types::Type,
};

pub type Expression<'ast> = Node<'ast, ExpressionKind<'ast>>;

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionBody<'ast> {
  Block(&'ast BlockStatement<'ast>),
  Expression(&'ast Expression<'ast>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionKind<'ast> {
  Identifier(&'ast str),
  Literal(Literal<'ast>),
  Block(BlockStatement<'ast>),
  Collection {
    elements: BumpVec<'ast, Expression<'ast>>,
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
    arms: BumpVec<'ast, MatchArm<'ast>>,
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
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm<'ast> {
  pub patterns: BumpVec<'ast, MatchPattern<'ast>>,
  pub body: FunctionBody<'ast>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchPattern<'ast> {
  Expression(Expression<'ast>),
  Range(Expression<'ast>, Expression<'ast>),
  Else,
}
