use ahash::AHashMap;
use bumpalo::collections::Vec as BumpVec;
use futures::future::{FutureExt, LocalBoxFuture};
use krama_core::{
  ConstBinding, Destructure, Enum, Error, ErrorKind, Expression, ForBinding,
  FunctionKind, ObjectKind, Span, Statement, StatementBlock, StatementKind,
  Struct, Type,
};
use parking_lot::RwLock;

use super::{
  types::{check_type, resolve_type},
  Interpreter,
};

impl<'ast> Interpreter<'ast> {
  /// Evaluates a single statement.
  /// Uses LocalBoxFuture to handle async recursion.
  pub fn eval_statement<'s>(
    &'s self,
    statement: &'s Statement<'ast>,
  ) -> LocalBoxFuture<'s, Result<ObjectKind<'ast>, Error<'ast>>>
  where
    'ast: 's,
  {
    async move {
      let span = statement.span;
      match &statement.kind {
        StatementKind::Expression { expression } => {
          self.eval_expression(expression, None).await
        }
        StatementKind::Let { name, value, kind } => {
          let mut value =
            self.eval_and_check_type(value, kind.as_ref()).await?;
          value.set_constant(false);
          self.env_mut(span)?.set(name, value, false, false);
          Ok(ObjectKind::Void)
        }
        StatementKind::Test { name: _, body } => {
          self.eval_block_statement_with_new_scope(body).await
        }
        StatementKind::Const {
          binding,
          value,
          public,
          kind,
        } => {
          let mut value =
            self.eval_and_check_type(value, kind.as_ref()).await?;
          value.set_constant(true);
          self.apply_binding(binding, value, *public, span)?;
          Ok(ObjectKind::Void)
        }
        StatementKind::Fn {
          name,
          parameters,
          body,
          public,
          kind,
        } => {
          let resolved_kind = if let Some(k) = kind {
            Some(resolve_type(self, k)?)
          } else {
            None
          };

          let function = self.alloc_user_function(
            parameters.clone(),
            body.clone(),
            resolved_kind,
          );
          self.env_mut(span)?.set(name, function, *public, true);
          Ok(ObjectKind::Void)
        }
        StatementKind::Enum {
          public,
          name,
          variants,
        } => {
          let mut properties = AHashMap::with_capacity(variants.len());
          for variant in variants {
            let variant_name = variant.name;

            if let Some(fields) = &variant.fields {
              let field_count = fields.len();
              let constructor = self.arena.alloc(Enum {
                name,
                variant: variant_name,
                field_count,
              });
              properties.insert(
                variant_name,
                ObjectKind::Function(FunctionKind::Enum(constructor)),
              );
            } else {
              properties.insert(
                variant_name,
                ObjectKind::Enum {
                  name,
                  variant: variant_name,
                  fields: None,
                },
              );
            }
          }

          let enum_obj = ObjectKind::Object {
            properties: self
              .arena
              .alloc(RwLock::new(properties.into_iter().collect())),
            constant: true,
          };

          self.env_mut(span)?.set(name, enum_obj, *public, true);
          Ok(ObjectKind::Void)
        }
        StatementKind::Struct {
          public,
          name,
          fields,
          methods,
        } => {
          let struct_def = self.arena.alloc(Struct {
            name,
            fields: fields.clone(),
            methods: methods.clone(),
          });
          self.env_mut(span)?.set(
            name,
            ObjectKind::Struct(struct_def),
            *public,
            true,
          );
          Ok(ObjectKind::Void)
        }
        StatementKind::Type { public, name, kind } => {
          let resolved = resolve_type(self, kind)?;
          self.env_mut(span)?.set(
            name,
            ObjectKind::Type(resolved),
            *public,
            true,
          );
          Ok(ObjectKind::Void)
        }
        StatementKind::Return { value } => {
          let value = match value {
            Some(expression) => self.eval_expression(expression, None).await?,
            None => ObjectKind::Void,
          };
          Ok(ObjectKind::Return(self.arena.alloc(value)))
        }
        StatementKind::Break => Ok(ObjectKind::Break),
        StatementKind::Continue => Ok(ObjectKind::Continue),
        StatementKind::While { condition, body } => {
          loop {
            let condition_result =
              self.eval_expression(condition, None).await?;
            if !condition_result.is_truthy() {
              break;
            }
            let result = self.eval_block_statement(body).await?;
            if matches!(result, ObjectKind::Return(_)) {
              return Ok(result);
            }
            if matches!(result, ObjectKind::Break) {
              break;
            }
            if matches!(result, ObjectKind::Continue) {
              continue;
            }
          }
          Ok(ObjectKind::Void)
        }
        StatementKind::For {
          binding,
          iterable,
          body,
        } => {
          let iterable_value = self.eval_expression(iterable, None).await?;
          let elements =
            self.collect_iterable_elements(&iterable_value, binding, span)?;

          for element in elements {
            let new_interpreter = self.new_enclosed();
            self.assign_for_binding(
              &new_interpreter,
              binding,
              element,
              span,
            )?;

            let result = new_interpreter.eval_block_statement(body).await?;

            if matches!(result, ObjectKind::Return(_)) {
              return Ok(result);
            }
            if matches!(result, ObjectKind::Break) {
              break;
            }
            if matches!(result, ObjectKind::Continue) {
              continue;
            }
          }
          Ok(ObjectKind::Void)
        }
      }
    }
    .boxed_local()
  }

  /// Helper to evaluate an expression and check its type if a hint is provided.
  async fn eval_and_check_type(
    &self,
    value_expr: &Expression<'ast>,
    kind_hint: Option<&Type<'ast>>,
  ) -> Result<ObjectKind<'ast>, Error<'ast>> {
    let resolved_kind = if let Some(k) = kind_hint {
      Some(resolve_type(self, k)?)
    } else {
      None
    };

    let value = self
      .eval_expression(value_expr, resolved_kind.as_ref())
      .await?;

    if let Some(kind) = &resolved_kind {
      check_type(kind, &value)?;
    }
    Ok(value)
  }

  /// Applies a binding to the environment.
  fn apply_binding(
    &self,
    binding: &ConstBinding<'ast>,
    value: ObjectKind<'ast>,
    public: bool,
    span: Span,
  ) -> Result<(), Error<'ast>> {
    match binding {
      ConstBinding::Identifier(name) => {
        self.env_mut(span)?.set(name, value, public, true);
      }
      ConstBinding::Destructure(items) => {
        self.apply_destructuring(None, items, value, public, span)?;
      }
      ConstBinding::ModuleAndDestructure { alias, items } => {
        self.apply_destructuring(Some(alias), items, value, public, span)?;
      }
    }
    Ok(())
  }

  /// Handles destructuring logic for modules.
  fn apply_destructuring(
    &self,
    alias: Option<&'ast str>,
    items: &[Destructure<'ast>],
    value: ObjectKind<'ast>,
    public: bool,
    span: Span,
  ) -> Result<(), Error<'ast>> {
    if let ObjectKind::Scope(scope) = &value {
      if let Some(alias_name) = alias {
        self
          .env_mut(span)?
          .set(alias_name, value.clone(), public, true);
      }
      for item in items {
        if let Some(export) = scope.get_binding(item.name) {
          let name = item.alias.unwrap_or(item.name);
          self.env_mut(span)?.set(name, export.clone(), public, true);
        } else {
          return Err(Error::new(
            ErrorKind::ReferenceError(format!(
              "'{}' is not exported from module '{}'",
              item.name,
              scope.name.unwrap_or("<anonymous>")
            )),
            span,
          ));
        }
      }
      Ok(())
    } else {
      Err(Error::new(
        ErrorKind::TypeError(
          "Destructuring can only be done on modules".to_string(),
        ),
        span,
      ))
    }
  }

  /// Collects elements from an iterable for a for-loop.
  fn collect_iterable_elements(
    &self,
    iterable: &ObjectKind<'ast>,
    binding: &ForBinding<'ast>,
    span: Span,
  ) -> Result<Vec<ObjectKind<'ast>>, Error<'ast>> {
    match iterable {
      ObjectKind::Array { elements, .. } => Ok(elements.read().to_vec()),
      ObjectKind::Tuple { elements } => Ok(elements.to_vec()),
      ObjectKind::String(s) => Ok(
        s.chars()
          .map(|c| ObjectKind::String(self.arena.alloc_str(&c.to_string())))
          .collect(),
      ),
      ObjectKind::Object { properties, .. } => {
        let props = properties.read();
        let mut yields = Vec::with_capacity(props.len());

        match binding {
          ForBinding::Array(bindings) if bindings.len() == 2 => {
            for (k, v) in props.iter() {
              let mut elements = BumpVec::with_capacity_in(2, self.arena);
              elements.push(ObjectKind::String(k));
              elements.push(v.clone());
              yields.push(ObjectKind::Tuple {
                elements: elements.into_bump_slice(),
              });
            }
          }
          _ => {
            for &k in props.keys() {
              yields.push(ObjectKind::String(k));
            }
          }
        }
        Ok(yields)
      }
      _ => Err(Error::new(
        ErrorKind::TypeError(format!(
          "Expected array, tuple, string or object for for..in loop, found {}",
          iterable.type_name()
        )),
        span,
      )),
    }
  }

  fn assign_for_binding(
    &self,
    interpreter: &Interpreter<'ast>,
    binding: &ForBinding<'ast>,
    value: ObjectKind<'ast>,
    span: Span,
  ) -> Result<(), Error<'ast>> {
    match binding {
      ForBinding::Identifier(name) => {
        interpreter.environment.borrow_mut().set(
          name,
          value.clone(),
          false,
          false,
        );
        Ok(())
      }
      ForBinding::Array(bindings) => {
        let elements = match &value {
          ObjectKind::Array { elements, .. } => elements.read().to_vec(),
          ObjectKind::Tuple { elements } => elements.to_vec(),
          _ => {
            return Err(Error::new(
              ErrorKind::TypeError(format!(
                "Expected array or tuple for destructuring, found {}",
                value.type_name()
              )),
              span,
            ));
          }
        };

        for (i, binding) in bindings.iter().enumerate() {
          let val = elements.get(i).cloned().unwrap_or(ObjectKind::Void);
          self.assign_for_binding(interpreter, binding, val, span)?;
        }
        Ok(())
      }
    }
  }

  /// Evaluates a sequence of statements.
  pub async fn eval_statements<'s>(
    &'s self,
    statements: &'s [Statement<'ast>],
  ) -> Result<ObjectKind<'ast>, Error<'ast>> {
    let mut result = ObjectKind::Void;

    for statement in statements {
      result = self.eval_statement(statement).await?;

      if matches!(
        &result,
        ObjectKind::Return(_) | ObjectKind::Break | ObjectKind::Continue
      ) {
        return Ok(result);
      }
    }

    Ok(result)
  }

  /// Evaluates a statement block.
  pub async fn eval_block_statement(
    &self,
    block: &StatementBlock<'ast>,
  ) -> Result<ObjectKind<'ast>, Error<'ast>> {
    self.eval_statements(&block.statements).await
  }

  /// Evaluates a statement block with a new enclosed scope.
  pub async fn eval_block_statement_with_new_scope(
    &self,
    block: &StatementBlock<'ast>,
  ) -> Result<ObjectKind<'ast>, Error<'ast>> {
    let new_interpreter = self.new_enclosed();
    new_interpreter.eval_statements(&block.statements).await
  }
}
