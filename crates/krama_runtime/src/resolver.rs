use krama_core::{
  ast::{
    expression::{Expression, ExpressionKind},
    statement::{Binding, Statement, StatementKind},
    Program,
  },
  error::{Error, ErrorKind},
};
use rustc_hash::FxHashMap;

use crate::interpreter::Interpreter;

pub struct Resolver<'a, 'ast> {
  interpreter: &'a Interpreter<'ast>,
  scopes: Vec<FxHashMap<&'a str, bool>>,
}

impl<'a, 'ast> Resolver<'a, 'ast> {
  pub fn new(interpreter: &'a Interpreter<'ast>) -> Self {
    Self {
      interpreter,
      scopes: vec![FxHashMap::default()],
    }
  }

  pub fn resolve(&mut self, program: &Program<'ast>) -> Result<(), Error> {
    for statement in &program.statements {
      self.resolve_statement(statement)?;
    }
    Ok(())
  }

  fn resolve_statement(
    &mut self,
    statement: &Statement<'ast>,
  ) -> Result<(), Error> {
    match &statement.kind {
      StatementKind::Let { name, value, .. } => {
        self.resolve_expression(value)?;
        self.declare(name);
        self.define(name);
      }
      StatementKind::Const { binding, value, .. } => {
        self.resolve_expression(value)?;
        match binding {
          Binding::Identifier(name) => {
            self.declare(name);
            self.define(name);
          }
          Binding::Destructure(items) => {
            for item in items {
              self.declare(item.name);
              self.define(item.name);
            }
          }
          Binding::ModuleAndDestructure {
            module_alias,
            items,
          } => {
            self.declare(module_alias);
            self.define(module_alias);
            for item in items {
              self.declare(item.name);
              self.define(item.name);
            }
          }
        }
      }
      StatementKind::Fn { name, .. } => {
        self.declare(name);
        self.define(name);
      }
      _ => {}
    }
    Ok(())
  }

  fn resolve_expression(
    &mut self,
    expression: &Expression<'ast>,
  ) -> Result<(), Error> {
    if let ExpressionKind::Identifier(name) = &expression.kind {
      if let Some(scope) = self.scopes.last() {
        if let Some(defined) = scope.get(name) {
          if !defined {
            return Err(Error {
              span: expression.span,
              kind: ErrorKind::SyntaxError(
                "Cannot read local variable in its own initializer".to_string(),
              ),
            });
          }
        }
      }
      self.resolve_local(expression, name);
    }
    Ok(())
  }

  fn resolve_local(&self, expression: &Expression<'ast>, name: &str) {
    for (i, scope) in self.scopes.iter().enumerate().rev() {
      if scope.contains_key(name) {
        self
          .interpreter
          .resolve(expression, self.scopes.len() - 1 - i);
        return;
      }
    }
  }

  fn declare(&mut self, name: &'ast str) {
    if let Some(scope) = self.scopes.last_mut() {
      scope.insert(name, false);
    }
  }

  fn define(&mut self, name: &'ast str) {
    if let Some(scope) = self.scopes.last_mut() {
      scope.insert(name, true);
    }
  }
}
