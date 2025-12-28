use std::sync::Arc;

use super::Expression;
use crate::{
  AssignmentOperator, BinaryOperator, Literal, Parameter, StatementBlock, Type,
  UnaryOperator, UpdateOperator,
};

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionBody {
  Block(Box<StatementBlock>),
  Expression(Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionKind {
  Identifier(Arc<str>),
  Literal(Literal),
  This,
  Struct {
    properties: Vec<(Expression, Expression)>,
  },
  Block(Box<StatementBlock>),
  Array {
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
  Function {
    parameters: Vec<Parameter>,
    body: FunctionBody,
    ty: Option<Type>,
  },
  Member {
    object: Box<Expression>,
    property: Box<Expression>,
  },
  Index {
    object: Box<Expression>,
    index: Box<Expression>,
  },
  Cast {
    expr: Box<Expression>,
    ty: Type,
  },
  Try(Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Match {
  pub patterns: Vec<Pattern>,
  pub body: FunctionBody,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
  Expression(Expression),
  Range(Expression, Expression),
  Else,
}
