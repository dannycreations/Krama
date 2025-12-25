use crate::{Expression, FunctionBody, Node, Span, Type};

pub type Statement = Node<StatementKind>;

#[derive(Debug, Clone, PartialEq)]
pub struct StatementBlock {
  pub statements: Vec<Statement>,
  pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Destructure {
  pub name: String,
  pub alias: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConstBinding {
  Identifier(String),
  Destructure(Vec<Destructure>),
  ModuleAndDestructure {
    alias: String,
    items: Vec<Destructure>,
  },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
  pub name: String,
  pub kind: Option<Type>,
  pub default: Option<Box<Expression>>,
  pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumVariant {
  pub name: String,
  pub fields: Option<Vec<Type>>,
  pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructField {
  pub public: bool,
  pub name: String,
  pub kind: Type,
  pub default: Option<Box<Expression>>,
  pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructMethod {
  pub public: bool,
  pub name: String,
  pub parameters: Vec<Parameter>,
  pub body: FunctionBody,
  pub kind: Option<Type>,
  pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ForBinding {
  Identifier(String),
  Array(Vec<ForBinding>),
}

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
    name: String,
    kind: Option<Type>,
    value: Box<Expression>,
  },
  Fn {
    public: bool,
    name: String,
    parameters: Vec<Parameter>,
    body: FunctionBody,
    kind: Option<Type>,
  },
  Enum {
    public: bool,
    name: String,
    variants: Vec<EnumVariant>,
  },
  Struct {
    public: bool,
    name: String,
    fields: Vec<StructField>,
    methods: Vec<StructMethod>,
  },
  Type {
    public: bool,
    name: String,
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
