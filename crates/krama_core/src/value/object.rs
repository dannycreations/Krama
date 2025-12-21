use std::{
  fmt::{Debug, Display, Formatter, Result as FmtResult},
  sync::Arc,
};

use ahash::AHashMap;
use bumpalo::{collections::Vec as BumpVec, Bump};
use futures::future::LocalBoxFuture;
use parking_lot::RwLock;
use strum::EnumProperty;
use strum_macros::EnumProperty as EnumPropertyMacro;

use crate::{ErrorKind, FunctionBody, Parameter, Scope, Type};

pub type NativeFnCb =
  for<'ast> fn(
    &'ast Bump,
    &'ast [Object<'ast>],
  ) -> LocalBoxFuture<'ast, Result<Object<'ast>, ErrorKind>>;

pub type PropertyFnCb =
  for<'ast> fn(
    Object<'ast>,
  ) -> LocalBoxFuture<'ast, Result<Object<'ast>, ErrorKind>>;

#[derive(Clone, Copy)]
pub struct NativeFunction {
  pub name: &'static str,
  pub callback: NativeFnCb,
}

impl PartialEq for NativeFunction {
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name && self.callback as usize == other.callback as usize
  }
}

impl Debug for NativeFunction {
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
  Native(NativeFunction),
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
      Function::User(u) => u.fmt(f),
    }
  }
}

impl<'ast> Display for Function<'ast> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Function::Native(_) => write!(f, "[native function]"),
      Function::User(_) => write!(f, "[function]"),
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
    elements: Arc<RwLock<BumpVec<'ast, Object<'ast>>>>,
    kind: Type<'ast>,
    constant: bool,
  },
  #[strum(props(name = "tuple"))]
  Tuple {
    elements: &'ast [Object<'ast>],
  },
  #[strum(props(name = "object"))]
  Object {
    properties: Arc<RwLock<AHashMap<&'ast str, Object<'ast>>>>,
    constant: bool,
  },
  #[strum(props(name = "null"))]
  Null,
  #[strum(props(name = "void"))]
  Void,
  Scope(&'ast Scope<'ast>),
  #[strum(props(name = "function"))]
  Function(Function<'ast>),
  #[strum(props(name = "return"))]
  Return(Arc<Object<'ast>>),
  #[strum(props(name = "break"))]
  Break,
  #[strum(props(name = "continue"))]
  Continue,
  #[strum(props(name = "ok"))]
  Ok(Arc<Object<'ast>>),
  #[strum(props(name = "err"))]
  Err(Arc<Object<'ast>>),
}

// We implement Send and Sync for Object because it's required by tokio/futures
// when used in async blocks that are spawned or boxed.
unsafe impl<'ast> Send for Object<'ast> {}
unsafe impl<'ast> Sync for Object<'ast> {}

#[allow(clippy::arc_with_non_send_sync)]
impl<'ast> PartialEq for Object<'ast> {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Integer(l), Self::Integer(r)) => l == r,
      (Self::Float(l), Self::Float(r)) => l == r,
      (Self::Boolean(l), Self::Boolean(r)) => l == r,
      (Self::String(l), Self::String(r)) => l == r,
      (
        Self::Array {
          elements: l,
          kind: lk,
          constant: lc,
        },
        Self::Array {
          elements: r,
          kind: rk,
          constant: rc,
        },
      ) => Arc::ptr_eq(l, r) && lk == rk && lc == rc,
      (Self::Tuple { elements: l }, Self::Tuple { elements: r }) => l == r,
      (
        Self::Object {
          properties: l,
          constant: lc,
        },
        Self::Object {
          properties: r,
          constant: rc,
        },
      ) => Arc::ptr_eq(l, r) && lc == rc,
      (Self::Null, Self::Null) => true,
      (Self::Void, Self::Void) => true,
      (Self::Scope(l), Self::Scope(r)) => std::ptr::eq(*l, *r),
      (Self::Function(l), Self::Function(r)) => l == r,
      (Self::Return(l), Self::Return(r)) => Arc::ptr_eq(l, r),
      (Self::Break, Self::Break) => true,
      (Self::Continue, Self::Continue) => true,
      (Self::Ok(l), Self::Ok(r)) => Arc::ptr_eq(l, r),
      (Self::Err(l), Self::Err(r)) => Arc::ptr_eq(l, r),
      _ => false,
    }
  }
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
      _ => self.get_str("name").unwrap_or("unknown"),
    }
  }
}

impl<'ast> From<&Object<'ast>> for bool {
  fn from(obj: &Object<'ast>) -> bool {
    match obj {
      Object::Boolean(b) => *b,
      Object::Integer(i) => *i != 0,
      Object::Float(f) => *f != 0.0,
      Object::String(s) => !s.is_empty(),
      Object::Array { elements, .. } => !elements.read().is_empty(),
      Object::Tuple { elements } => !elements.is_empty(),
      Object::Object { .. } => true,
      Object::Null | Object::Void => false,
      Object::Ok(_) => true,
      Object::Err(_) => false,
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
      Object::Array { elements, .. } => {
        let elements = elements.read();
        write!(f, "[")?;
        for (i, element) in elements.iter().enumerate() {
          if i > 0 {
            write!(f, ", ")?;
          }
          write!(f, "{}", element)?;
        }
        write!(f, "]")
      }
      Object::Tuple { elements } => {
        write!(f, "[")?;
        for (i, element) in elements.iter().enumerate() {
          if i > 0 {
            write!(f, ", ")?;
          }
          write!(f, "{}", element)?;
        }
        write!(f, "]")
      }
      Object::Object { .. } => {
        write!(f, "[object]")
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
      Object::Function(func) => write!(f, "{}", func),
      Object::Return(value) => write!(f, "{}", value),
      Object::Break => write!(f, "break"),
      Object::Continue => write!(f, "continue"),
      Object::Ok(value) => write!(f, "Ok({})", value),
      Object::Err(error) => write!(f, "Err({})", error),
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
      Object::Array {
        elements, constant, ..
      } => {
        let elements = elements.read();
        f.debug_struct("Array")
          .field("elements", &*elements)
          .field("constant", constant)
          .finish()
      }
      Object::Tuple { elements } => {
        f.debug_tuple("Tuple").field(elements).finish()
      }
      Object::Object { constant, .. } => f
        .debug_struct("Object")
        .field("constant", constant)
        .finish_non_exhaustive(),
      Object::Null => write!(f, "Null"),
      Object::Void => write!(f, "Void"),
      Object::Scope(scope) => f.debug_tuple("Scope").field(scope).finish(),
      Object::Function(func) => Debug::fmt(func, f),
      Object::Return(value) => f.debug_tuple("Return").field(value).finish(),
      Object::Break => write!(f, "Break"),
      Object::Continue => write!(f, "Continue"),
      Object::Ok(value) => f.debug_tuple("Ok").field(value).finish(),
      Object::Err(error) => f.debug_tuple("Err").field(error).finish(),
    }
  }
}

pub struct StandardGlobal {
  pub name: &'static str,
  pub callback: NativeFnCb,
}

#[linkme::distributed_slice]
pub static STANDARD_GLOBALS: [StandardGlobal];

pub struct StandardModule {
  pub name: &'static str,
  pub callback: NativeFnCb,
  pub module: &'static str,
}

#[linkme::distributed_slice]
pub static STANDARD_MODULES: [StandardModule];

pub struct StandardProperty {
  pub name: &'static str,
  pub callback: PropertyFnCb,
  pub types: &'static [&'static str],
}

#[linkme::distributed_slice]
pub static STANDARD_PROPERTIES: [StandardProperty];
