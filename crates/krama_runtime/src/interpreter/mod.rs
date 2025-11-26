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

use std::cell::RefCell;
use std::rc::Rc;

use bumpalo::Bump;
use futures::future::{FutureExt, LocalBoxFuture};
use krama_core::ast::statement::Statement;
use krama_core::error::{Error, ErrorKind};
use krama_core::object::Object;
use krama_lexer::lexer::Lexer;
use krama_parser::parser::Parser;
use krama_std::props::PropFn;
use rustc_hash::FxHashMap;
pub use test::TestResult;

use crate::environment::Environment;
use crate::resolver::Resolver;

#[derive(Clone)]
pub struct Interpreter<'ast> {
  pub environment: Rc<RefCell<Environment<'ast>>>,
  resolver: Resolver,
  pub(super) modules: Rc<RefCell<FxHashMap<String, Object<'ast>>>>,
  pub(super) arena: &'ast Bump,
  pub path: Option<&'ast str>,
  pub(super) props: Rc<FxHashMap<(&'static str, &'static str), PropFn>>,
}

impl<'ast> Interpreter<'ast> {
  pub fn new(arena: &'ast Bump, path: Option<&'ast str>) -> Self {
    let mut env = Environment::new();
    for (name, function) in krama_std::globals::get_globals() {
      env.set(name, function, true);
    }

    Self {
      environment: Rc::new(RefCell::new(env)),
      resolver: Resolver::new(),
      modules: Rc::new(RefCell::new(FxHashMap::default())),
      arena,
      path,
      props: Rc::new(krama_std::props::get_props()),
    }
  }

  pub fn alloc_str(&self, s: &str) -> &'ast str {
    self.arena.alloc_str(s)
  }

  pub async fn eval(&self, source: &'ast str) -> Result<Object<'ast>, Error> {
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer, self.arena);
    let program = parser.parse()?;
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
        if matches!(result, Object::Return(_)) {
          return Ok(result);
        }
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
      let future = future_rc.take().ok_or_else(|| Error {
        span: Default::default(),
        kind: ErrorKind::RuntimeError(
          "Future is already being awaited".to_string(),
        ),
      })?;
      current_object = future.await?;
    }
    Ok(current_object)
  }
}
