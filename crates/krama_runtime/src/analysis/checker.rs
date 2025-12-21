use indexmap::IndexMap;
use krama_core::{
  Binding, Error, ErrorKind, Expression, ExpressionKind, ForBinding,
  FunctionBody, MatchPattern, Program, Span, Statement, StatementKind,
};

pub struct Checker<'a> {
  scopes: Vec<IndexMap<&'a str, bool>>,
  locals: IndexMap<Span, usize>,
}

impl<'a> Default for Checker<'a> {
  fn default() -> Self {
    Self::new()
  }
}

impl<'a> Checker<'a> {
  pub fn new() -> Self {
    Self {
      scopes: vec![IndexMap::default()],
      locals: IndexMap::default(),
    }
  }

  pub fn check(
    &mut self,
    program: &Program<'a>,
  ) -> Result<IndexMap<Span, usize>, Error<'a>> {
    for statement in &program.statements {
      self.check_statement(statement)?;
    }
    Ok(self.locals.clone())
  }

  fn check_statement(
    &mut self,
    statement: &Statement<'a>,
  ) -> Result<(), Error<'a>> {
    match &statement.kind {
      StatementKind::Expression { expression } => {
        self.check_expression(expression)?
      }
      StatementKind::Let { name, value, .. } => {
        self.check_expression(value)?;
        self.declare(name);
        self.define(name);
      }
      StatementKind::Const { binding, value, .. } => {
        self.check_expression(value)?;
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
          Binding::ModuleAndDestructure { alias, items } => {
            self.declare(alias);
            self.define(alias);
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
              self.check_statement(statement)?;
            }
          }
          FunctionBody::Expression(expression) => {
            self.check_expression(expression)?;
          }
        }
        self.end_scope();
      }
      StatementKind::Enum { name, .. } => {
        self.declare(name);
        self.define(name);
      }
      StatementKind::Type { name, .. } => {
        self.declare(name);
        self.define(name);
      }
      StatementKind::Return { value } => {
        if let Some(value) = value {
          self.check_expression(value)?;
        }
      }
      StatementKind::While { condition, body } => {
        self.check_expression(condition)?;
        for statement in &body.statements {
          self.check_statement(statement)?;
        }
      }
      StatementKind::For {
        binding,
        iterable,
        body,
      } => {
        self.check_expression(iterable)?;
        self.begin_scope();
        self.declare_for_binding(binding);
        for statement in &body.statements {
          self.check_statement(statement)?;
        }
        self.end_scope();
      }
      StatementKind::Test { body, .. } => {
        self.begin_scope();
        for statement in &body.statements {
          self.check_statement(statement)?;
        }
        self.end_scope();
      }
      StatementKind::Break | StatementKind::Continue => {}
    }
    Ok(())
  }

  fn declare_for_binding(&mut self, binding: &ForBinding<'a>) {
    match binding {
      ForBinding::Identifier(name) => {
        self.declare(name);
        self.define(name);
      }
      ForBinding::Array(bindings) => {
        for b in bindings {
          self.declare_for_binding(b);
        }
      }
    }
  }

  fn check_expression(
    &mut self,
    expression: &Expression<'a>,
  ) -> Result<(), Error<'a>> {
    match &expression.kind {
      ExpressionKind::Identifier(name) => {
        if let Some(scope) = self.scopes.last() {
          if let Some(defined) = scope.get(name) {
            if !defined {
              return Err(Error::new(
                ErrorKind::SyntaxError(
                  "Cannot read local variable in its own initializer"
                    .to_string(),
                ),
                expression.span,
              ));
            }
          }
        }
        self.check_local(expression, name);
      }
      ExpressionKind::Assignment { left, right, .. } => {
        self.check_expression(right)?;
        self.check_expression(left)?;
      }
      ExpressionKind::Update { argument, .. } => {
        self.check_expression(argument)?;
      }
      ExpressionKind::Binary { left, right, .. } => {
        self.check_expression(left)?;
        self.check_expression(right)?;
      }
      ExpressionKind::Unary { right, .. } => {
        self.check_expression(right)?;
      }
      ExpressionKind::Call {
        function,
        arguments,
        ..
      } => {
        self.check_expression(function)?;
        for argument in arguments {
          self.check_expression(argument)?;
        }
      }
      ExpressionKind::Member { object, property } => {
        self.check_expression(object)?;
        self.check_expression(property)?;
      }
      ExpressionKind::Index { object, index } => {
        self.check_expression(object)?;
        self.check_expression(index)?;
      }
      ExpressionKind::If {
        condition,
        then_branch,
        else_branch,
      } => {
        self.check_expression(condition)?;
        self.check_expression(then_branch)?;
        if let Some(else_branch) = else_branch {
          self.check_expression(else_branch)?;
        }
      }
      ExpressionKind::Match { subject, arms } => {
        self.check_expression(subject)?;
        for arm in arms {
          for pattern in &arm.patterns {
            match pattern {
              MatchPattern::Expression(expression) => {
                self.check_expression(expression)?
              }
              MatchPattern::Range(start, end) => {
                self.check_expression(start)?;
                self.check_expression(end)?;
              }
              _ => {}
            }
          }
          match &arm.body {
            FunctionBody::Block(block) => {
              self.begin_scope();
              for statement in &block.statements {
                self.check_statement(statement)?;
              }
              self.end_scope();
            }
            FunctionBody::Expression(expression) => {
              self.begin_scope();
              self.check_expression(expression)?;
              self.end_scope();
            }
          }
        }
      }
      ExpressionKind::Block(block) => {
        self.begin_scope();
        for statement in &block.statements {
          self.check_statement(statement)?;
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
              self.check_statement(statement)?;
            }
          }
          FunctionBody::Expression(expression) => {
            self.check_expression(expression)?
          }
        }
        self.end_scope();
      }
      ExpressionKind::Collection { elements } => {
        for element in elements {
          self.check_expression(element)?;
        }
      }
      ExpressionKind::Typed { expr, .. } => {
        self.check_expression(expr)?;
      }
      ExpressionKind::Import { .. } | ExpressionKind::Literal(_) => {}
      ExpressionKind::Object { properties } => {
        for (key, value) in properties {
          self.check_expression(key)?;
          self.check_expression(value)?;
        }
      }
      ExpressionKind::Try(expr) => {
        self.check_expression(expr)?;
      }
    }
    Ok(())
  }

  fn check_local(&mut self, expression: &Expression<'a>, name: &str) {
    for (i, scope) in self.scopes.iter().enumerate().rev() {
      if scope.contains_key(name) {
        self
          .locals
          .insert(expression.span, self.scopes.len() - 1 - i);
        return;
      }
    }
  }

  fn begin_scope(&mut self) {
    self.scopes.push(IndexMap::default());
  }

  fn end_scope(&mut self) {
    self.scopes.pop();
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
