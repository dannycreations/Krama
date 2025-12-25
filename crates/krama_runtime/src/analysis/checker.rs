use indexmap::IndexMap;
use krama_core::{
  ConstBinding, Error, ErrorKind, Expression, ExpressionKind, ForBinding,
  FunctionBody, MatchPattern, Span, Statement, StatementKind,
};

pub struct Checker {
  scopes: Vec<IndexMap<String, bool>>,
  locals: IndexMap<Span, usize>,
}

impl Default for Checker {
  fn default() -> Self {
    Self::new()
  }
}

impl Checker {
  pub fn new() -> Self {
    Self {
      scopes: vec![IndexMap::default()],
      locals: IndexMap::default(),
    }
  }

  pub fn check(
    &mut self,
    statements: &[Statement],
  ) -> Result<IndexMap<Span, usize>, Error> {
    for statement in statements {
      self.check_statement(statement)?;
    }
    Ok(self.locals.clone())
  }

  fn check_statement(&mut self, statement: &Statement) -> Result<(), Error> {
    match &statement.kind {
      StatementKind::Expression { expression } => {
        self.check_expression(expression)?
      }
      StatementKind::Let { name, value, .. } => {
        self.check_expression(value)?;
        self.define_var(name.to_string());
      }
      StatementKind::Const { binding, value, .. } => {
        self.check_expression(value)?;
        match binding {
          ConstBinding::Identifier(name) => self.define_var(name.to_string()),
          ConstBinding::Destructure(items) => {
            for item in items {
              self.define_var(item.name.to_string());
            }
          }
          ConstBinding::ModuleAndDestructure { alias, items } => {
            self.define_var(alias.to_string());
            for item in items {
              self.define_var(item.name.to_string());
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
        self.define_var(name.to_string());
        self.check_function(parameters, body)?;
      }
      StatementKind::Enum { name, .. } | StatementKind::Type { name, .. } => {
        self.define_var(name.to_string());
      }
      StatementKind::Struct {
        name,
        fields,
        methods,
        ..
      } => {
        self.define_var(name.to_string());
        for field in fields {
          if let Some(default) = &field.default {
            self.check_expression(default)?;
          }
        }
        for method in methods {
          self.check_function(&method.parameters, &method.body)?;
        }
      }
      StatementKind::Return { value } => {
        if let Some(value) = value {
          self.check_expression(value)?;
        }
      }
      StatementKind::While { condition, body } => {
        self.check_expression(condition)?;
        // While loop body does NOT create a new scope in the interpreter.
        self.check_block_content(body)?;
      }
      StatementKind::For {
        binding,
        iterable,
        body,
      } => {
        self.check_expression(iterable)?;
        self.with_scope(|this| {
          this.declare_for_binding(binding);
          this.check_block_content(body)
        })?;
      }
      StatementKind::Test { body, .. } => {
        self.with_scope(|this| this.check_block_content(body))?;
      }
      StatementKind::Break | StatementKind::Continue => {}
    }
    Ok(())
  }

  fn check_function(
    &mut self,
    params: &[krama_core::Parameter],
    body: &FunctionBody,
  ) -> Result<(), Error> {
    self.with_scope(|this| {
      for param in params {
        this.define_var(param.name.to_string());
      }
      this.check_body(body)
    })
  }

  fn check_body(&mut self, body: &FunctionBody) -> Result<(), Error> {
    match body {
      FunctionBody::Block(block) => self.check_block_content(block),
      FunctionBody::Expression(expr) => self.check_expression(expr),
    }
  }

  fn check_block(
    &mut self,
    block: &krama_core::StatementBlock,
  ) -> Result<(), Error> {
    self.with_scope(|this| this.check_block_content(block))
  }

  fn check_block_content(
    &mut self,
    block: &krama_core::StatementBlock,
  ) -> Result<(), Error> {
    for statement in &block.statements {
      self.check_statement(statement)?;
    }
    Ok(())
  }

  fn declare_for_binding(&mut self, binding: &ForBinding) {
    match binding {
      ForBinding::Identifier(name) => self.define_var(name.to_string()),
      ForBinding::Array(bindings) => {
        for b in bindings {
          self.declare_for_binding(b);
        }
      }
    }
  }

  fn check_expression(&mut self, expression: &Expression) -> Result<(), Error> {
    match &expression.kind {
      ExpressionKind::Identifier(name) => {
        if let Some(scope) = self.scopes.last() {
          if let Some(false) = scope.get(name) {
            return Err(Error::new(
              ErrorKind::SyntaxError(
                "Cannot read local variable in its own initializer".into(),
              ),
              expression.span,
            ));
          }
        }
        self.check_local(expression, name);
      }
      ExpressionKind::Assignment { left, right, .. } => {
        self.check_expression(right)?;
        self.check_expression(left)?;
      }
      ExpressionKind::Update { argument, .. } => {
        self.check_expression(argument)?
      }
      ExpressionKind::Binary { left, right, .. } => {
        self.check_expression(left)?;
        self.check_expression(right)?;
      }
      ExpressionKind::Unary { right, .. } => self.check_expression(right)?,
      ExpressionKind::Call {
        function,
        arguments,
      } => {
        self.check_expression(function)?;
        for arg in arguments {
          self.check_expression(arg)?;
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
            if let MatchPattern::Expression(expr) = pattern {
              self.check_expression(expr)?
            } else if let MatchPattern::Range(start, end) = pattern {
              self.check_expression(start)?;
              self.check_expression(end)?;
            }
          }
          self.with_scope(|this| this.check_body(&arm.body))?;
        }
      }
      ExpressionKind::Block(block) => self.check_block(block)?,
      ExpressionKind::Fn {
        parameters, body, ..
      } => self.check_function(parameters, body)?,
      ExpressionKind::Collection { elements } => {
        for el in elements {
          self.check_expression(el)?;
        }
      }
      ExpressionKind::Typed { expr, .. } => self.check_expression(expr)?,
      ExpressionKind::Object { properties }
      | ExpressionKind::StructConstruction { properties } => {
        for (key, value) in properties {
          self.check_expression(key)?;
          self.check_expression(value)?;
        }
      }
      ExpressionKind::Try(expr) => self.check_expression(expr)?,
      _ => {}
    }
    Ok(())
  }

  fn check_local(&mut self, expression: &Expression, name: &str) {
    for (i, scope) in self.scopes.iter().enumerate().rev() {
      if scope.contains_key(name) {
        self
          .locals
          .insert(expression.span, self.scopes.len() - 1 - i);
        return;
      }
    }
  }

  fn with_scope<F, R>(&mut self, f: F) -> R
  where
    F: FnOnce(&mut Self) -> R,
  {
    self.scopes.push(IndexMap::default());
    let result = f(self);
    self.scopes.pop();
    result
  }

  fn define_var(&mut self, name: String) {
    if let Some(scope) = self.scopes.last_mut() {
      scope.insert(name, true);
    }
  }
}
