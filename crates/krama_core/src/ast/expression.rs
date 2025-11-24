use super::literal::Literal;
use super::operator::BinaryOperator;
use super::operator::UnaryOperator;
use super::operator::UpdateOperator;
use super::statement::{BlockStatement, Parameter};
use super::types::Type;
use crate::span::Span;
use bumpalo::collections::Vec as BumpVec;

#[derive(Debug, Clone, PartialEq)]
pub struct Expression<'ast> {
  pub kind: ExpressionKind<'ast>,
  pub span: Span,
}

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
    operator: BinaryOperator,
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
