use super::ast::expression::FunctionBody;
use super::ast::statement::Parameter;
use super::ast::types::Type;
use crate::error::Error;
pub use bumpalo::collections::Vec as BumpVec;
use bumpalo::Bump;
use futures::future::LocalBoxFuture;
use rustc_hash::FxHashMap;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

pub type NativeFnCallback<'ast> =
  fn(
    &'ast Bump,
    BumpVec<'ast, Object<'ast>>,
  ) -> LocalBoxFuture<'ast, Result<Object<'ast>, Error>>;

pub type ObjectFuture<'ast> =
  Rc<RefCell<Option<LocalBoxFuture<'ast, Result<Object<'ast>, Error>>>>>;

#[derive(Clone)]
pub struct NativeFn<'ast> {
  pub name: &'static str,
  pub callback: NativeFnCallback<'ast>,
}

impl<'ast> PartialEq for NativeFn<'ast> {
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name && self.callback as usize == other.callback as usize
  }
}

impl<'ast> fmt::Debug for NativeFn<'ast> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("NativeFunction")
      .field("name", &self.name)
      .finish()
  }
}

#[derive(Debug, Clone)]
pub struct UserFn<'ast> {
  pub parameters: BumpVec<'ast, Parameter<'ast>>,
  pub body: FunctionBody<'ast>,
  pub kind: Option<Type<'ast>>,
}

#[derive(Clone)]
pub enum Function<'ast> {
  Native(NativeFn<'ast>),
  User(Rc<UserFn<'ast>>),
}

impl<'ast> PartialEq for Function<'ast> {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Function::Native(a), Function::Native(b)) => a == b,
      (Function::User(a), Function::User(b)) => Rc::ptr_eq(a, b),
      _ => false,
    }
  }
}

impl<'ast> fmt::Debug for Function<'ast> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

pub enum Object<'ast> {
  Integer(i64),
  Float(f64),
  Boolean(bool),
  String(&'ast str),
  Array {
    elements: Rc<BumpVec<'ast, Object<'ast>>>,
    kind: Type<'ast>,
  },
  Tuple(Rc<BumpVec<'ast, Object<'ast>>>),
  Null,
  Void,
  Module(Rc<RefCell<ModuleObject<'ast>>>),
  Global(Rc<RefCell<GlobalObject<'ast>>>),
  Function(Function<'ast>),
  Return(Box<Object<'ast>>),
  Break,
  Continue,
  Future(ObjectFuture<'ast>),
}

impl<'ast> Object<'ast> {
  pub fn type_name(&self) -> &'static str {
    match self {
      Object::Integer(_) => "integer",
      Object::Float(_) => "float",
      Object::Boolean(_) => "boolean",
      Object::String(_) => "string",
      Object::Array { .. } => "array",
      Object::Tuple(_) => "tuple",
      Object::Null => "null",
      Object::Void => "void",
      Object::Module(_) => "module",
      Object::Global(_) => "global",
      Object::Function(_) => "function",
      Object::Return(_) => "return",
      Object::Break => "break",
      Object::Continue => "continue",
      Object::Future(_) => "future",
    }
  }

  pub fn is_truthy(&self) -> bool {
    match self {
      Object::Boolean(b) => *b,
      Object::Integer(i) => *i != 0,
      Object::Float(f) => *f != 0.0,
      Object::String(s) => !s.is_empty(),
      Object::Array { elements, .. } => !elements.is_empty(),
      Object::Tuple(t) => !t.is_empty(),
      Object::Null | Object::Void => false,
      _ => true,
    }
  }

  fn format_elements(
    f: &mut fmt::Formatter,
    elements: &[Object<'ast>],
    debug: bool,
  ) -> fmt::Result {
    write!(f, "[")?;
    for (i, element) in elements.iter().enumerate() {
      if i > 0 {
        write!(f, ", ")?;
      }
      if debug {
        write!(f, "{:?}", element)?;
      } else {
        write!(f, "{}", element)?;
      }
    }
    write!(f, "]")
  }
}

impl<'ast> fmt::Display for Object<'ast> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Object::Integer(i) => write!(f, "{}", i),
      Object::Float(fl) => write!(f, "{}", fl),
      Object::Boolean(b) => write!(f, "{}", b),
      Object::String(s) => write!(f, "{}", s),
      Object::Array { elements, .. } => {
        Object::format_elements(f, elements, false)
      }
      Object::Tuple(elements) => Object::format_elements(f, elements, false),
      Object::Null => write!(f, "null"),
      Object::Void => write!(f, "void"),
      Object::Module(module) => {
        if let Ok(guard) = module.try_borrow() {
          write!(f, "module {}", guard.name)
        } else {
          write!(f, "module <locked>")
        }
      }
      Object::Global(_) => write!(f, "global"),
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

impl<'ast> Clone for Object<'ast> {
  fn clone(&self) -> Self {
    match self {
      Object::Integer(i) => Object::Integer(*i),
      Object::Float(f) => Object::Float(*f),
      Object::Boolean(b) => Object::Boolean(*b),
      Object::String(s) => Object::String(s),
      Object::Array { elements, kind } => Object::Array {
        elements: elements.clone(),
        kind: kind.clone(),
      },
      Object::Tuple(t) => Object::Tuple(t.clone()),
      Object::Null => Object::Null,
      Object::Void => Object::Void,
      Object::Module(m) => Object::Module(m.clone()),
      Object::Global(g) => Object::Global(g.clone()),
      Object::Function(f) => Object::Function(f.clone()),
      Object::Return(v) => Object::Return(v.clone()),
      Object::Break => Object::Break,
      Object::Continue => Object::Continue,
      Object::Future(f) => Object::Future(f.clone()),
    }
  }
}

impl<'ast> std::fmt::Debug for Object<'ast> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Object::Integer(i) => write!(f, "Integer({})", i),
      Object::Float(fl) => write!(f, "Float({})", fl),
      Object::Boolean(b) => write!(f, "Boolean({})", b),
      Object::String(s) => write!(f, "String(\"{}\")", s),
      Object::Array { elements, .. } => {
        f.debug_tuple("Array").field(elements).finish()
      }
      Object::Tuple(elements) => {
        f.debug_tuple("Tuple").field(elements).finish()
      }
      Object::Null => write!(f, "Null"),
      Object::Void => write!(f, "Void"),
      Object::Module(module) => f.debug_tuple("Module").field(module).finish(),
      Object::Global(g) => f.debug_tuple("Global").field(g).finish(),
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
      (Object::Integer(a), Object::Integer(b)) => a == b,
      (Object::Float(a), Object::Float(b)) => a == b,
      (Object::Boolean(a), Object::Boolean(b)) => a == b,
      (Object::String(a), Object::String(b)) => a == b,
      (
        Object::Array { elements: a, .. },
        Object::Array { elements: b, .. },
      ) => a == b,
      (Object::Tuple(a), Object::Tuple(b)) => a == b,
      (Object::Null, Object::Null) => true,
      (Object::Void, Object::Void) => true,
      (Object::Function(a), Object::Function(b)) => a == b,
      (Object::Return(a), Object::Return(b)) => *a == *b,
      (Object::Break, Object::Break) => true,
      (Object::Continue, Object::Continue) => true,
      (Object::Module(a), Object::Module(b)) => Rc::ptr_eq(a, b),
      (Object::Global(a), Object::Global(b)) => Rc::ptr_eq(a, b),
      (Object::Future(_), Object::Future(_)) => false,
      _ => false,
    }
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ModuleObject<'ast> {
  pub name: &'ast str,
  pub exports: FxHashMap<&'ast str, Object<'ast>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GlobalObject<'ast> {
  pub bindings: FxHashMap<&'ast str, Object<'ast>>,
}
