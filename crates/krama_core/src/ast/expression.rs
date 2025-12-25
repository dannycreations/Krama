use std::sync::Arc;

use crate::{
  AssignmentOperator, BinaryOperator, LiteralKind, Node, Parameter,
  StatementBlock, Type, UnaryOperator, UpdateOperator,
};

pub type Expression = Node<ExpressionKind>;

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionBody {
  Block(Box<StatementBlock>),
  Expression(Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionKind {
  Identifier(Arc<str>),
  Literal(LiteralKind),
  This,
  StructConstruction {
    properties: Vec<(Expression, Expression)>,
  },
  Block(Box<StatementBlock>),
  Collection {
    elements: Vec<Expression>,
  },
  Object {
    properties: Vec<(Expression, Expression)>,
  },
  Assignment {
    left: Box<Expression>,
    operator: AssignmentOperator,
    right: Box<Expression>,
  },
  Binary {
    left: Box<Expression>,
    operator: BinaryOperator,
    right: Box<Expression>,
  },
  Unary {
    operator: UnaryOperator,
    right: Box<Expression>,
  },
  Update {
    operator: UpdateOperator,
    argument: Box<Expression>,
    prefix: bool,
  },
  If {
    condition: Box<Expression>,
    then_branch: Box<Expression>,
    else_branch: Option<Box<Expression>>,
  },
  Match {
    subject: Box<Expression>,
    arms: Vec<Match>,
  },
  Import {
    path: Arc<str>,
    items: Option<Vec<Arc<str>>>,
  },
  Call {
    function: Box<Expression>,
    arguments: Vec<Expression>,
  },
  Fn {
    parameters: Vec<Parameter>,
    body: FunctionBody,
    kind: Option<Type>,
  },
  Member {
    object: Box<Expression>,
    property: Box<Expression>,
  },
  Index {
    object: Box<Expression>,
    index: Box<Expression>,
  },
  Typed {
    expr: Box<Expression>,
    kind: Type,
  },
  Try(Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Match {
  pub patterns: Vec<MatchPattern>,
  pub body: FunctionBody,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchPattern {
  Expression(Expression),
  Range(Expression, Expression),
  Else,
}
