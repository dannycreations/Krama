use krama_core::{
  ast::{
    expression::{Expression, ExpressionKind, FunctionBody, MatchPattern},
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
      StatementKind::Expression { expression } => {
        self.resolve_expression(expression)?
      }
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
      StatementKind::Fn {
        name,
        parameters,
        body,
        ..
      } => {
        self.declare(name);
        self.define(name);
        self.begin_scope();
        for param in parameters {
          self.declare(param.name);
          self.define(param.name);
        }
        match body {
          FunctionBody::Block(block) => {
            for statement in &block.statements {
              self.resolve_statement(statement)?;
            }
          }
          FunctionBody::Expression(expression) => {
            self.resolve_expression(expression)?;
          }
        }
        self.end_scope();
      }
      StatementKind::Return { value } => {
        if let Some(value) = value {
          self.resolve_expression(value)?;
        }
      }
      StatementKind::While { condition, body } => {
        self.resolve_expression(condition)?;
        for statement in &body.statements {
          self.resolve_statement(statement)?;
        }
      }
      StatementKind::Test { body, .. } => {
        self.begin_scope();
        for statement in &body.statements {
          self.resolve_statement(statement)?;
        }
        self.end_scope();
      }
      StatementKind::Break | StatementKind::Continue => {}
    }
    Ok(())
  }

  fn resolve_expression(
    &mut self,
    expression: &Expression<'ast>,
  ) -> Result<(), Error> {
    match &expression.kind {
      ExpressionKind::Identifier(name) => {
        if let Some(scope) = self.scopes.last() {
          if let Some(defined) = scope.get(name) {
            if !defined {
              return Err(Error {
                span: expression.span,
                kind: ErrorKind::SyntaxError(
                  "Cannot read local variable in its own initializer"
                    .to_string(),
                ),
              });
            }
          }
        }
        self.resolve_local(expression, name);
      }
      ExpressionKind::Assignment { left, right, .. } => {
        self.resolve_expression(right)?;
        self.resolve_expression(left)?;
      }
      ExpressionKind::Update { argument, .. } => {
        self.resolve_expression(argument)?;
      }
      ExpressionKind::Binary { left, right, .. } => {
        self.resolve_expression(left)?;
        self.resolve_expression(right)?;
      }
      ExpressionKind::Unary { right, .. } => {
        self.resolve_expression(right)?;
      }
      ExpressionKind::Call {
        function,
        arguments,
        ..
      } => {
        self.resolve_expression(function)?;
        for argument in arguments {
          self.resolve_expression(argument)?;
        }
      }
      ExpressionKind::Member { object, property } => {
        self.resolve_expression(object)?;
        self.resolve_expression(property)?;
      }
      ExpressionKind::Index { object, index } => {
        self.resolve_expression(object)?;
        self.resolve_expression(index)?;
      }
      ExpressionKind::If {
        condition,
        then_branch,
        else_branch,
      } => {
        self.resolve_expression(condition)?;
        self.resolve_expression(then_branch)?;
        if let Some(else_branch) = else_branch {
          self.resolve_expression(else_branch)?;
        }
      }
      ExpressionKind::Match { subject, arms } => {
        self.resolve_expression(subject)?;
        for arm in arms {
          for pattern in &arm.patterns {
            match pattern {
              MatchPattern::Expression(expression) => {
                self.resolve_expression(expression)?
              }
              MatchPattern::Range(start, end) => {
                self.resolve_expression(start)?;
                self.resolve_expression(end)?;
              }
              _ => {}
            }
          }
          match &arm.body {
            FunctionBody::Block(block) => {
              self.begin_scope();
              for statement in &block.statements {
                self.resolve_statement(statement)?;
              }
              self.end_scope();
            }
            FunctionBody::Expression(expression) => {
              self.begin_scope();
              self.resolve_expression(expression)?;
              self.end_scope();
            }
          }
        }
      }
      ExpressionKind::Block(block) => {
        self.begin_scope();
        for statement in &block.statements {
          self.resolve_statement(statement)?;
        }
        self.end_scope();
      }
      ExpressionKind::Fn {
        parameters, body, ..
      } => {
        self.begin_scope();
        for param in parameters {
          self.declare(param.name);
          self.define(param.name);
        }

        match body {
          FunctionBody::Block(block) => {
            for statement in &block.statements {
              self.resolve_statement(statement)?;
            }
          }
          FunctionBody::Expression(expression) => {
            self.resolve_expression(expression)?
          }
        }
        self.end_scope();
      }
      ExpressionKind::Collection { elements } => {
        for element in elements {
          self.resolve_expression(element)?;
        }
      }
      ExpressionKind::Import { .. } | ExpressionKind::Literal(_) => {}
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

  fn begin_scope(&mut self) {
    self.scopes.push(FxHashMap::default());
  }

  fn end_scope(&mut self) {
    self.scopes.pop();
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
