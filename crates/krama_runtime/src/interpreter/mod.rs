mod eval;
mod expression;
mod flow;
mod function;
mod identifier;
mod import;
mod literal;
mod statement;
mod types;

use std::cell::{RefCell, RefMut};

use ahash::AHashMap;
use bumpalo::Bump;
use krama_core::{
  Error, ErrorKind, Expression, Object, Program, Span, Statement,
};

use crate::{Checker, Environment, Lexer, Parser};

#[derive(Clone)]
pub struct Interpreter<'ast> {
  pub environment: &'ast RefCell<Environment<'ast>>,
  pub modules: &'ast RefCell<AHashMap<&'ast str, Object<'ast>>>,
  pub arena: &'ast Bump,
  pub path: Option<&'ast str>,
  locals: RefCell<AHashMap<Span<'ast>, usize>>,
}

impl<'ast> Interpreter<'ast> {
  pub fn new(arena: &'ast Bump, path: Option<&'ast str>) -> Self {
    let env = Environment::with_globals();

    Self {
      environment: arena.alloc(RefCell::new(env)),
      modules: arena.alloc(RefCell::new(AHashMap::default())),
      arena,
      path,
      locals: RefCell::new(AHashMap::default()),
    }
  }

  pub fn new_enclosed(&self) -> Self {
    Self {
      environment: self
        .arena
        .alloc(RefCell::new(Environment::new_enclosed(self.environment))),
      modules: self.modules,
      arena: self.arena,
      path: self.path,
      locals: self.locals.clone(),
    }
  }

  pub fn get_at(&self, distance: usize, name: &str) -> Option<Object<'ast>> {
    let mut env = self.environment.borrow();
    for _ in 0..distance {
      let outer = env.outer.unwrap();
      env = outer.borrow();
    }
    env.get_local(name)
  }

  pub fn assign_at(
    &self,
    distance: usize,
    name: &'ast str,
    value: Object<'ast>,
  ) -> Result<(), Error<'ast>> {
    let mut env_cell = self.environment;
    for _ in 0..distance {
      env_cell = env_cell.borrow().outer.unwrap();
    }
    let mut env = env_cell.borrow_mut();
    if env.is_constant(name) {
      return Err(Error::new(
        ErrorKind::TypeError(format!("Cannot assign to constant '{}'", name)),
        Span::empty(), // TODO: pass span
      ));
    }
    env.set(name, value, false, false);
    Ok(())
  }

  pub fn alloc_str(&self, s: &str) -> &'ast str {
    self.arena.alloc_str(s)
  }

  pub fn check(&self, source: &'ast str) -> Result<(), Error<'ast>> {
    self.parse_and_check(source)?;
    Ok(())
  }

  pub async fn eval(
    &self,
    source: &'ast str,
  ) -> Result<Object<'ast>, Error<'ast>> {
    let program = self.parse_and_check(source)?;
    self.eval_program_statements(&program.statements).await
  }

  pub fn parse_and_check(
    &self,
    source: &'ast str,
  ) -> Result<Program<'ast>, Error<'ast>> {
    let lexer = Lexer::new(source, self.path);
    let mut parser = Parser::new(lexer, self.arena);
    let program = parser.parse()?;
    let mut checker = Checker::new();
    let locals = checker.check(&program)?;
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

  pub fn env_mut(
    &self,
    span: Span<'ast>,
  ) -> Result<RefMut<'_, Environment<'ast>>, Error<'ast>> {
    self
      .environment
      .try_borrow_mut()
      .map_err(|e| Error::new(ErrorKind::RuntimeError(e.to_string()), span))
  }

  pub fn get_resolved_distance(
    &self,
    expr: &Expression<'ast>,
  ) -> Option<usize> {
    self.locals.borrow().get(&expr.span).copied()
  }
}
