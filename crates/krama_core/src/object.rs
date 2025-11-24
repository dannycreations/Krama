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

#[derive(Debug, Clone, PartialEq)]
pub struct UserFn<'ast> {
  pub parameters: BumpVec<'ast, Parameter<'ast>>,
  pub body: FunctionBody<'ast>,
  pub kind: Option<Type<'ast>>,
}

pub enum Object<'ast> {
  Integer(i64),
  Float(f64),
  Boolean(bool),
  String(&'ast str),
  Array {
    elements: BumpVec<'ast, Object<'ast>>,
    kind: Type<'ast>,
  },
  Tuple(BumpVec<'ast, Object<'ast>>),
  Null,
  Void,
  Module(Rc<RefCell<ModuleObject<'ast>>>),
  Global(Rc<RefCell<GlobalObject<'ast>>>),
  NativeFn(NativeFn<'ast>),
  UserFn(Rc<UserFn<'ast>>),
  Return(&'ast Object<'ast>),
  Break,
  Continue,
  Future(Rc<RefCell<LocalBoxFuture<'ast, Result<Object<'ast>, Error>>>>),
}

impl<'ast> Object<'ast> {
  pub fn is_truthy(&self) -> bool {
    match self {
      Object::Boolean(b) => *b,
      Object::Integer(i) => *i != 0,
      Object::Float(f) => *f != 0.0,
      Object::String(s) => !s.is_empty(),
      Object::Array { elements, .. } => !elements.is_empty(),
      Object::Tuple(t) => !t.is_empty(),
      Object::Null => false,
      Object::Void => false,
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
      Object::NativeFn(_) => write!(f, "[native function]"),
      Object::UserFn(_) => write!(f, "[function]"),
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
      Object::NativeFn(f) => Object::NativeFn(f.clone()),
      Object::UserFn(f) => Object::UserFn(f.clone()),
      Object::Return(v) => Object::Return(v),
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
        write!(f, "Array(")?;
        Object::format_elements(f, elements, true)?;
        write!(f, ")")
      }
      Object::Tuple(elements) => {
        write!(f, "Tuple(")?;
        Object::format_elements(f, elements, true)?;
        write!(f, ")")
      }
      Object::Null => write!(f, "Null"),
      Object::Void => write!(f, "Void"),
      Object::Module(module) => f.debug_tuple("Module").field(module).finish(),
      Object::Global(g) => f.debug_tuple("Global").field(g).finish(),
      Object::NativeFn(func) => {
        f.debug_tuple("NativeFunction").field(&func.name).finish()
      }
      Object::UserFn(_) => write!(f, "[function]"),
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
      (Object::NativeFn(a), Object::NativeFn(b)) => a == b,
      (Object::UserFn(a), Object::UserFn(b)) => Rc::ptr_eq(a, b),
      (Object::Return(a), Object::Return(b)) => std::ptr::eq(*a, *b),
      (Object::Break, Object::Break) => true,
      (Object::Continue, Object::Continue) => true,
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
