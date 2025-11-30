pub mod eval;
pub mod expression;
pub mod flow;
pub mod function;
pub mod identifier;
pub mod import;
pub mod literal;
pub mod statement;
pub mod types;

use std::cell::{RefCell, RefMut};

use bumpalo::Bump;
use krama_core::{
  ast::{expression::Expression, statement::Statement, Program},
  error::ErrorKind,
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
  pub environment: &'ast RefCell<Environment<'ast>>,
  pub(super) modules: &'ast RefCell<FxHashMap<&'ast str, Object<'ast>>>,
  pub(super) arena: &'ast Bump,
  pub path: Option<&'ast str>,
  pub(super) props:
    &'ast FxHashMap<&'static str, FxHashMap<&'static str, PropFn<'ast>>>,
  locals: RefCell<FxHashMap<Span<'ast>, usize>>,
}

impl<'ast> Interpreter<'ast> {
  pub fn new(arena: &'ast Bump, path: Option<&'ast str>) -> Self {
    let mut env = Environment::new();
    for (name, function) in globals::get_globals() {
      env.set(name, function, true);
    }

    Self {
      environment: arena.alloc(RefCell::new(env)),
      modules: arena.alloc(RefCell::new(FxHashMap::default())),
      arena,
      path,
      props: arena.alloc(props::get_props()),
      locals: RefCell::new(FxHashMap::default()),
    }
  }

  fn new_enclosed(&self) -> Self {
    Self {
      environment: self
        .arena
        .alloc(RefCell::new(Environment::new_enclosed(self.environment))),
      modules: self.modules,
      arena: self.arena,
      path: self.path,
      props: self.props,
      locals: self.locals.clone(),
    }
  }

  fn ancestor(&self, distance: usize) -> &'ast RefCell<Environment<'ast>> {
    let mut environment = self.environment;
    for _ in 0..distance {
      let outer = environment.borrow().outer.unwrap();
      environment = outer;
    }
    environment
  }

  pub(super) fn get_at(
    &self,
    distance: usize,
    name: &str,
  ) -> Option<Object<'ast>> {
    self.ancestor(distance).borrow().get(name)
  }

  pub(super) fn assign_at(
    &self,
    distance: usize,
    name: &'ast str,
    value: Object<'ast>,
  ) {
    self.ancestor(distance).borrow_mut().set(name, value, false);
  }

  pub fn alloc_str(&self, s: &str) -> &'ast str {
    self.arena.alloc_str(s)
  }

  pub fn check(
    &self,
    source: &'ast str,
  ) -> Result<(), (ErrorKind, Span<'ast>)> {
    self.parse_and_resolve(source)?;
    Ok(())
  }

  pub async fn eval(
    &self,
    source: &'ast str,
  ) -> Result<Object<'ast>, (ErrorKind, Span<'ast>)> {
    let program = self.parse_and_resolve(source)?;
    self.eval_program_statements(&program.statements).await
  }

  pub fn parse_and_resolve(
    &self,
    source: &'ast str,
  ) -> Result<Program<'ast>, (ErrorKind, Span<'ast>)> {
    let lexer = Lexer::new(source, self.path);
    let mut parser = Parser::new(lexer, self.arena);
    let program = parser.parse()?;
    let mut resolver = Resolver::new();
    let locals = resolver.resolve(&program)?;
    *self.locals.borrow_mut() = locals;
    Ok(program)
  }

  async fn eval_program_statements<'s>(
    &'s self,
    statements: &'s [Statement<'ast>],
  ) -> Result<Object<'ast>, (ErrorKind, Span<'ast>)> {
    let mut result = Object::Void;
    for statement in statements {
      result = self.eval_statement(statement).await?;
    }
    Ok(result)
  }

  pub(super) fn env_mut(
    &self,
    span: Span<'ast>,
  ) -> Result<RefMut<'_, Environment<'ast>>, (ErrorKind, Span<'ast>)> {
    self
      .environment
      .try_borrow_mut()
      .map_err(|e| (ErrorKind::RuntimeError(e.to_string()), span))
  }

  pub(crate) fn look_up_variable(
    &self,
    expr: &Expression<'ast>,
  ) -> Option<usize> {
    self.locals.borrow().get(&expr.span).copied()
  }
}
