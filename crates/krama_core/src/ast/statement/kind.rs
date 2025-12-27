use std::sync::Arc;

use super::{
  ConstBinding, EnumVariant, ForBinding, Parameter, StatementBlock,
  StructField, StructMethod,
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
