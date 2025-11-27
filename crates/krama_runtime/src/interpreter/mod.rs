mod eval;
mod expression;
mod flow;
mod function;
mod identifier;
mod import;
mod literal;
mod statement;
mod test;
mod types;

use std::{
  cell::{RefCell, RefMut},
  rc::Rc,
};

use bumpalo::Bump;
use futures::future::{FutureExt, LocalBoxFuture};
use krama_core::{
  ast::statement::Statement,
  error::{Error, ErrorKind},
  object::Object,
  span::Span,
};
use krama_lexer::lexer::Lexer;
use krama_parser::parser::Parser;
use krama_std::{globals, props, props::PropFn};
use rustc_hash::FxHashMap;
pub use test::TestResult;

use crate::{environment::Environment, resolver::Resolver};

#[derive(Clone)]
pub struct Interpreter<'ast> {
  pub environment: Rc<RefCell<Environment<'ast>>>,
  pub(super) modules: Rc<RefCell<FxHashMap<String, Object<'ast>>>>,
  pub(super) arena: &'ast Bump,
  pub path: Option<&'ast str>,
  pub(super) props: Rc<FxHashMap<(&'static str, &'static str), PropFn>>,
}

impl<'ast> Interpreter<'ast> {
  pub fn new(arena: &'ast Bump, path: Option<&'ast str>) -> Self {
    let mut env = Environment::new();
    for (name, function) in globals::get_globals() {
      env.set(name, function, true);
    }

    Self {
      environment: Rc::new(RefCell::new(env)),
      modules: Rc::new(RefCell::new(FxHashMap::default())),
      arena,
      path,
      props: Rc::new(props::get_props()),
    }
  }

  pub fn alloc_str(&self, s: &str) -> &'ast str {
    self.arena.alloc_str(s)
  }

  pub async fn eval(&self, source: &'ast str) -> Result<Object<'ast>, Error> {
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer, self.arena);
    let program = parser.parse()?;
    let mut resolver = Resolver::new();
    resolver.resolve(&program)?;
    self.eval_program_statements(&program.statements).await
  }

  fn eval_program_statements<'s>(
    &'s self,
    statements: &'s [Statement<'ast>],
  ) -> LocalBoxFuture<'s, Result<Object<'ast>, Error>> {
    async move {
      let mut result = Object::Void;
      for statement in statements {
        result = self.eval_statement(statement).await?;
      }
      Ok(result)
    }
    .boxed_local()
  }

  async fn resolve_object(
    &self,
    object: Object<'ast>,
  ) -> Result<Object<'ast>, Error> {
    let mut current_object = object;
    while let Object::Future(future_rc) = current_object {
      let future = future_rc.borrow_mut().take().ok_or_else(|| Error {
        span: Default::default(),
        kind: ErrorKind::RuntimeError(
          "Future has already been consumed".to_string(),
        ),
      })?;
      current_object = future.await?;
    }
    Ok(current_object)
  }

  pub(super) fn env_mut(
    &self,
    span: Span,
  ) -> Result<RefMut<'_, Environment<'ast>>, Error> {
    self.environment.try_borrow_mut().map_err(|e| Error {
      span,
      kind: ErrorKind::RuntimeError(e.to_string()),
    })
  }
}
