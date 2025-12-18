use ahash::AHashMap;
use krama_core::{
  Binding, Error, ErrorKind, Expression, ExpressionKind, FunctionBody,
  MatchPattern, Program, Span, Statement, StatementKind,
};

pub struct Resolver<'a> {
  scopes: Vec<AHashMap<&'a str, bool>>,
  locals: AHashMap<Span<'a>, usize>,
}

impl<'a> Default for Resolver<'a> {
  fn default() -> Self {
    Self::new()
  }
}

impl<'a> Resolver<'a> {
  pub fn new() -> Self {
    Self {
      scopes: vec![AHashMap::default()],
      locals: AHashMap::default(),
    }
  }

  pub fn resolve(
    &mut self,
    program: &Program<'a>,
  ) -> Result<AHashMap<Span<'a>, usize>, Error<'a>> {
    for statement in &program.statements {
      self.resolve_statement(statement)?;
    }
    Ok(self.locals.clone())
  }

  fn resolve_statement(
    &mut self,
    statement: &Statement<'a>,
  ) -> Result<(), Error<'a>> {
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
                expression.span.clone(),
              ));
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
      ExpressionKind::Typed { expr, .. } => {
        self.resolve_expression(expr)?;
      }
      ExpressionKind::Import { .. } | ExpressionKind::Literal(_) => {}
      ExpressionKind::Object { properties } => {
        for (key, value) in properties {
          self.resolve_expression(key)?;
          self.resolve_expression(value)?;
        }
      }
    }
    Ok(())
  }

  fn resolve_local(&mut self, expression: &Expression<'a>, name: &str) {
    for (i, scope) in self.scopes.iter().enumerate().rev() {
      if scope.contains_key(name) {
        self
          .locals
          .insert(expression.span.clone(), self.scopes.len() - 1 - i);
        return;
      }
    }
  }

  fn begin_scope(&mut self) {
    self.scopes.push(AHashMap::default());
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
