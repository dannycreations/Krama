use krama_core::{
  ast::{
    expression::{Expression, ExpressionKind},
    statement::{Binding, Statement, StatementKind},
    Program,
  },
  error::{Error, ErrorKind},
};
use rustc_hash::FxHashMap;

/// The Resolver is responsible for resolving variable bindings.
///
/// It performs a static analysis pass over the AST, determining the scope
/// of each variable declaration and usage. This allows the interpreter to
/// look up variables with a constant-time operation, rather than a
/// potentially slow walk up the environment chain.
pub struct Resolver<'a> {
  scopes: Vec<FxHashMap<&'a str, bool>>,
}

impl<'a> Default for Resolver<'a> {
  fn default() -> Self {
    Self::new()
  }
}

impl<'a> Resolver<'a> {
  pub fn new() -> Self {
    Self {
      scopes: vec![FxHashMap::default()],
    }
  }

  pub fn resolve(&mut self, program: &Program<'a>) -> Result<(), Error> {
    for statement in &program.statements {
      self.resolve_statement(statement)?;
    }
    Ok(())
  }

  fn resolve_statement(
    &mut self,
    statement: &Statement<'a>,
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
    expression: &Expression<'a>,
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
    }
    Ok(())
  }

  fn declare(&mut self, name: &'a str) {
    if let Some(scope) = self.scopes.last_mut() {
      scope.insert(name, false);
    }
  }

  fn define(&mut self, name: &'a str) {
    if let Some(scope) = self.scopes.last_mut() {
      scope.insert(name, true);
    }
  }
}
