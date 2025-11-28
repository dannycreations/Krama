mod eval;
mod expression;
mod flow;
mod function;
mod identifier;
mod import;
mod literal;
mod statement;
mod types;

use std::{
  cell::{RefCell, RefMut},
  rc::Rc,
};

use bumpalo::Bump;
use krama_core::{
  ast::{expression::Expression, statement::Statement, Program},
  error::{Error, ErrorKind},
  object::Object,
  span::Span,
};
use krama_lexer::lexer::Lexer;
use krama_parser::parser::Parser;
use krama_std::{globals, props, props::PropFn};
use rustc_hash::FxHashMap;

use crate::{environment::Environment, resolver::Resolver};

#[derive(Clone)]
pub struct Interpreter<'ast> {
  pub environment: Rc<RefCell<Environment<'ast>>>,
  pub(super) modules: Rc<RefCell<FxHashMap<String, Rc<Object<'ast>>>>>,
  pub(super) arena: &'ast Bump,
  pub path: Option<&'ast str>,
  pub(super) props: Rc<FxHashMap<(&'static str, &'static str), PropFn<'ast>>>,
  locals: RefCell<FxHashMap<Span, usize>>,
}

impl<'ast> Interpreter<'ast> {
  pub fn new(arena: &'ast Bump, path: Option<&'ast str>) -> Self {
    let mut env = Environment::new();
    for (name, function) in globals::get_globals() {
      env.set(name, Rc::new(function), true);
    }

    Self {
      environment: Rc::new(RefCell::new(env)),
      modules: Rc::new(RefCell::new(FxHashMap::default())),
      arena,
      path,
      props: Rc::new(props::get_props()),
      locals: RefCell::new(FxHashMap::default()),
    }
  }

  fn new_enclosed(&self) -> Self {
    Self {
      environment: Rc::new(RefCell::new(Environment::new_enclosed(
        self.environment.borrow().clone().into(),
      ))),
      modules: self.modules.clone(),
      arena: self.arena,
      path: self.path,
      props: self.props.clone(),
      locals: self.locals.clone(),
    }
  }

  pub fn alloc_str(&self, s: &str) -> &'ast str {
    self.arena.alloc_str(s)
  }

  pub async fn eval(&self, source: &'ast str) -> Result<Object<'ast>, Error> {
    let program = self.parse_and_resolve(source)?;
    self.eval_program_statements(&program.statements).await
  }

  pub fn parse_and_resolve(
    &self,
    source: &'ast str,
  ) -> Result<Program<'ast>, Error> {
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer, self.arena);
    let program = parser.parse()?;
    let mut resolver = Resolver::new(self);
    resolver.resolve(&program)?;
    Ok(program)
  }

  pub async fn eval_program_statements<'s>(
    &'s self,
    statements: &'s [Statement<'ast>],
  ) -> Result<Object<'ast>, Error> {
    let mut result = Object::Void;
    for statement in statements {
      result = self.eval_statement(statement).await?;
    }
    Ok(result)
  }

  async fn resolve_object(
    &self,
    object: Object<'ast>,
  ) -> Result<Object<'ast>, Error> {
    let mut current_object = object;
    while let Object::Future(future_rc) = current_object {
      let future = Rc::try_unwrap(future_rc).map_err(|_| Error {
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

  pub(crate) fn resolve(&self, expr: &Expression<'ast>, depth: usize) {
    self.locals.borrow_mut().insert(expr.span, depth);
  }
}
