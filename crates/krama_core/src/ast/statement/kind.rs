use std::sync::Arc;

use super::{
  Binding, EnumVariant, Iteration, Parameter, StatementBlock, StructField,
  StructMethod,
};
use crate::{Expression, FunctionBody, Type};

#[derive(Debug, Clone, PartialEq)]
pub enum StatementKind {
  Test {
    name: Box<Expression>,
    body: Box<StatementBlock>,
  },
  Const {
    public: bool,
    binding: Binding,
    ty: Option<Type>,
    value: Box<Expression>,
  },
  Let {
    binding: Binding,
    ty: Option<Type>,
    value: Box<Expression>,
  },
  Function {
    public: bool,
    name: Arc<str>,
    parameters: Vec<Parameter>,
    body: FunctionBody,
    ty: Option<Type>,
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
    ty: Type,
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
    binding: Iteration,
    iterable: Box<Expression>,
    body: Box<StatementBlock>,
  },
  Break,
  Continue,
}
