use std::{
  cell::RefCell,
  fmt::{Debug, Display, Formatter, Result as FmtResult},
  ptr,
};

pub use bumpalo::collections::Vec as BumpVec;
use bumpalo::Bump;
use futures::future::LocalBoxFuture;
use strum::EnumProperty;
use strum_macros::EnumProperty as EnumPropertyMacro;

use crate::{
  ast::{expression::FunctionBody, statement::Parameter, types::Type},
  error::ErrorKind,
  scope::Scope,
  span::Span,
};

pub type NativeFunctionCb<'ast> =
  fn(
    &'ast Bump,
    &'ast [Object<'ast>],
  ) -> LocalBoxFuture<'ast, Result<Object<'ast>, ErrorKind>>;

pub type ObjectFuture<'ast> =
  LocalBoxFuture<'ast, Result<Object<'ast>, (ErrorKind, Span<'ast>)>>;

#[derive(Clone, Copy)]
pub struct NativeFunction<'ast> {
  pub name: &'static str,
  pub callback: NativeFunctionCb<'ast>,
}

impl<'ast> PartialEq for NativeFunction<'ast> {
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name && self.callback as usize == other.callback as usize
  }
}

impl<'ast> Debug for NativeFunction<'ast> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("NativeFunction")
      .field("name", &self.name)
      .finish()
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserFunction<'ast> {
  pub parameters: BumpVec<'ast, Parameter<'ast>>,
  pub body: FunctionBody<'ast>,
  pub kind: Option<Type<'ast>>,
}

#[derive(Copy, Clone)]
pub enum Function<'ast> {
  Native(NativeFunction<'ast>),
  User(&'ast UserFunction<'ast>),
}

impl<'ast> PartialEq for Function<'ast> {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Function::Native(a), Function::Native(b)) => a == b,
      (Function::User(a), Function::User(b)) => a == b,
      _ => false,
    }
  }
}

impl<'ast> Debug for Function<'ast> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Function::Native(n) => n.fmt(f),
      Function::User(u) => f
        .debug_struct("UserFunction")
        .field("parameters", &u.parameters)
        .field("body", &u.body)
        .field("kind", &u.kind)
        .finish(),
    }
  }
}

#[derive(Clone, EnumPropertyMacro)]
pub enum Object<'ast> {
  #[strum(props(name = "integer"))]
  Integer(i64),
  #[strum(props(name = "float"))]
  Float(f64),
  #[strum(props(name = "boolean"))]
  Boolean(bool),
  #[strum(props(name = "string"))]
  String(&'ast str),
  #[strum(props(name = "array"))]
  Array {
    elements: &'ast [Object<'ast>],
    kind: Type<'ast>,
  },
  #[strum(props(name = "tuple"))]
  Tuple {
    elements: &'ast [Object<'ast>],
  },
  #[strum(props(name = "null"))]
  Null,
  #[strum(props(name = "void"))]
  Void,
  Scope(&'ast Scope<'ast>),
  #[strum(props(name = "function"))]
  Function(Function<'ast>),
  #[strum(props(name = "return"))]
  Return(&'ast Object<'ast>),
  #[strum(props(name = "break"))]
  Break,
  #[strum(props(name = "continue"))]
  Continue,
  #[strum(props(name = "future"))]
  Future(&'ast RefCell<Option<ObjectFuture<'ast>>>),
}

impl<'ast> Object<'ast> {
  pub fn type_name(&self) -> &'static str {
    match self {
      Object::Scope(scope) => {
        if scope.name.is_some() {
          "module"
        } else {
          "global"
        }
      }
      _ => self.get_str("name").unwrap(),
    }
  }

  fn format_elements(
    f: &mut Formatter,
    elements: &[Object<'ast>],
  ) -> FmtResult {
    write!(f, "[")?;
    for (i, element) in elements.iter().enumerate() {
      if i > 0 {
        write!(f, ", ")?;
      }
      write!(f, "{}", element)?;
    }
    write!(f, "]")
  }
}

impl<'ast> From<&Object<'ast>> for bool {
  fn from(obj: &Object<'ast>) -> bool {
    match obj {
      Object::Boolean(b) => *b,
      Object::Integer(i) => *i != 0,
      Object::Float(f) => *f != 0.0,
      Object::String(s) => !s.is_empty(),
      Object::Array { elements, .. } => !elements.is_empty(),
      Object::Tuple { elements } => !elements.is_empty(),
      Object::Null | Object::Void => false,
      _ => true,
    }
  }
}

impl<'ast> Display for Object<'ast> {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    match self {
      Object::Integer(i) => write!(f, "{}", i),
      Object::Float(fl) => write!(f, "{}", fl),
      Object::Boolean(b) => write!(f, "{}", b),
      Object::String(s) => write!(f, "{}", s),
      Object::Array { elements, .. } | Object::Tuple { elements } => {
        Object::format_elements(f, elements)
      }
      Object::Null => write!(f, "null"),
      Object::Void => write!(f, "void"),
      Object::Scope(scope) => {
        if let Some(name) = scope.name {
          write!(f, "module {}", name)
        } else {
          write!(f, "global")
        }
      }
      Object::Function(func) => match func {
        Function::Native(_) => write!(f, "[native function]"),
        Function::User(_) => write!(f, "[function]"),
      },
      Object::Return(value) => write!(f, "{}", value),
      Object::Break => write!(f, "break"),
      Object::Continue => write!(f, "continue"),
      Object::Future(_) => write!(f, "[future]"),
    }
  }
}

impl<'ast> Debug for Object<'ast> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Object::Integer(i) => write!(f, "Integer({})", i),
      Object::Float(fl) => write!(f, "Float({})", fl),
      Object::Boolean(b) => write!(f, "Boolean({})", b),
      Object::String(s) => write!(f, "String(\"{}\")", s),
      Object::Array { elements, .. } => {
        f.debug_tuple("Array").field(elements).finish()
      }
      Object::Tuple { elements } => {
        f.debug_tuple("Tuple").field(elements).finish()
      }
      Object::Null => write!(f, "Null"),
      Object::Void => write!(f, "Void"),
      Object::Scope(scope) => f.debug_tuple("Scope").field(scope).finish(),
      Object::Function(func) => func.fmt(f),
      Object::Return(value) => f.debug_tuple("Return").field(value).finish(),
      Object::Break => write!(f, "Break"),
      Object::Continue => write!(f, "Continue"),
      Object::Future(_) => write!(f, "[future]"),
    }
  }
}

impl<'ast> PartialEq for Object<'ast> {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Object::Null, Object::Null) => true,
      (Object::Void, Object::Void) => true,
      (Object::Boolean(a), Object::Boolean(b)) => a == b,
      (Object::Integer(a), Object::Integer(b)) => a == b,
      (Object::Float(a), Object::Float(b)) => a == b,
      (Object::String(a), Object::String(b)) => a == b,
      (
        Object::Array { elements: a, .. },
        Object::Array { elements: b, .. },
      )
      | (Object::Tuple { elements: a }, Object::Tuple { elements: b }) => {
        a == b
      }
      (Object::Function(a), Object::Function(b)) => a == b,
      (Object::Return(a), Object::Return(b)) => a == b,
      (Object::Break, Object::Break) => true,
      (Object::Continue, Object::Continue) => true,
      (Object::Scope(a), Object::Scope(b)) => ptr::eq(*a, *b),
      (Object::Future(_), Object::Future(_)) => false,
      _ => false,
    }
  }
}
