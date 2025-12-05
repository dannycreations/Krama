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

use ahash::AHashMap;
use bumpalo::Bump;
use krama_core::{
  ast::{expression::Expression, statement::Statement, Program},
  error::{Error, ErrorKind},
  object::Object,
  span::Span,
};
use krama_lexer::lexer::Lexer;
use krama_parser::parser::Parser;

use crate::{environment::Environment, resolver::Resolver};

#[derive(Clone)]
pub struct Interpreter<'ast> {
  pub environment: &'ast RefCell<Environment<'ast>>,
  pub(super) loaded_modules: &'ast RefCell<AHashMap<&'ast str, Object<'ast>>>,
  pub(super) arena: &'ast Bump,
  pub path: Option<&'ast str>,
  locals: RefCell<AHashMap<Span<'ast>, usize>>,
}

impl<'ast> Interpreter<'ast> {
  pub fn new(arena: &'ast Bump, path: Option<&'ast str>) -> Self {
    let env = Environment::with_globals();

    Self {
      environment: arena.alloc(RefCell::new(env)),
      loaded_modules: arena.alloc(RefCell::new(AHashMap::default())),
      arena,
      path,
      locals: RefCell::new(AHashMap::default()),
    }
  }

  pub(super) fn new_enclosed(&self) -> Self {
    Self {
      environment: self
        .arena
        .alloc(RefCell::new(Environment::new_enclosed(self.environment))),
      loaded_modules: self.loaded_modules,
      arena: self.arena,
      path: self.path,
      locals: self.locals.clone(),
    }
  }

  pub(super) fn get_at(
    &self,
    distance: usize,
    name: &str,
  ) -> Option<Object<'ast>> {
    let mut env = self.environment.borrow();
    for _ in 0..distance {
      let outer = env.outer.unwrap();
      env = outer.borrow();
    }
    env.get_local(name)
  }

  pub(super) fn assign_at(
    &self,
    distance: usize,
    name: &'ast str,
    value: Object<'ast>,
  ) {
    let mut env = self.environment.borrow_mut();
    for _ in 0..distance {
      let outer = env.outer.unwrap();
      env = outer.borrow_mut();
    }
    env.set(name, value, false);
  }

  pub fn alloc_str(&self, s: &str) -> &'ast str {
    self.arena.alloc_str(s)
  }

  pub fn check(&self, source: &'ast str) -> Result<(), Error<'ast>> {
    self.parse_and_resolve(source)?;
    Ok(())
  }

  pub async fn eval(
    &self,
    source: &'ast str,
  ) -> Result<Object<'ast>, Error<'ast>> {
    let program = self.parse_and_resolve(source)?;
    self.eval_program_statements(&program.statements).await
  }

  pub fn parse_and_resolve(
    &self,
    source: &'ast str,
  ) -> Result<Program<'ast>, Error<'ast>> {
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
  ) -> Result<Object<'ast>, Error<'ast>> {
    let mut result = Object::Void;
    for statement in statements {
      result = self.eval_statement(statement).await?;
    }
    Ok(result)
  }

  pub(super) fn env_mut(
    &self,
    span: Span<'ast>,
  ) -> Result<RefMut<'_, Environment<'ast>>, Error<'ast>> {
    self
      .environment
      .try_borrow_mut()
      .map_err(|e| Error::new(ErrorKind::RuntimeError(e.to_string()), span))
  }

  pub(crate) fn get_resolved_distance(
    &self,
    expr: &Expression<'ast>,
  ) -> Option<usize> {
    self.locals.borrow().get(&expr.span).copied()
  }
}
