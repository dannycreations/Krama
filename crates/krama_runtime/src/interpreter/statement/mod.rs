mod binding;
mod iteration;

use ahash::AHashMap;
use futures::future::{FutureExt, LocalBoxFuture};
use krama_core::{
  AssignmentOperator, Enum, Error, Expression, ExpressionKind, FunctionKind,
  ObjectKind, Statement, StatementBlock, StatementKind, Struct, Type,
};
use parking_lot::RwLock;

use crate::interpreter::{
  types::{check_type, resolve_type},
  Interpreter,
};

impl<'ast> Interpreter<'ast> {
  /// Evaluates a single statement.
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
              let constructor = self.arena.alloc(Enum {
                name,
                variant: variant_name,
                field_count: fields.len(),
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
            // Check for pattern matching in while condition (e.g. while (Ok(v) = expr)).
            if let Some(bindings) = self.try_match_assignment(condition).await?
            {
              for (name, val) in bindings {
                self.env_mut(span)?.set(name, val, false, false);
              }
            } else {
              // Handle special case where pattern match failed (break loop).
              if let ExpressionKind::Assignment {
                left,
                operator: AssignmentOperator::Assign,
                ..
              } = &condition.kind
              {
                if let ExpressionKind::Call { function, .. } = &left.kind {
                  if let ExpressionKind::Identifier(name) = &function.kind {
                    if *name == "Ok" || *name == "Err" {
                      break;
                    }
                  }
                }
              }

              // Fallback to normal truthy evaluation.
              let condition_val = self.eval_expression(condition, None).await?;
              if !condition_val.is_truthy() {
                break;
              }
            }

            let result = self.eval_block_statement(body).await?;

            // Propagate Return/Break/Continue signals.
            if result.is_control_signal() {
              if let ObjectKind::Return(inner) = &result {
                if inner.is_result_err() {
                  continue;
                }
              }
              if matches!(result, ObjectKind::Break) {
                break;
              }
              if matches!(result, ObjectKind::Continue) {
                continue;
              }
              return Ok(result);
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

            if result.is_control_signal() {
              if let ObjectKind::Return(inner) = &result {
                if inner.is_result_err() {
                  continue;
                }
              }
              if matches!(result, ObjectKind::Break) {
                break;
              }
              if matches!(result, ObjectKind::Continue) {
                continue;
              }
              return Ok(result);
            }
          }
          Ok(ObjectKind::Void)
        }
        StatementKind::Test { name: _, body } => {
          self.eval_block_statement_with_new_scope(body).await
        }
      }
    }
    .boxed_local()
  }

  /// Helper to evaluate an expression and check its type if a hint is provided.
  pub async fn eval_and_check_type(
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

  /// Evaluates a sequence of statements.
  pub async fn eval_statements<'s>(
    &'s self,
    statements: &'s [Statement<'ast>],
  ) -> Result<ObjectKind<'ast>, Error<'ast>> {
    let mut result = ObjectKind::Void;

    for statement in statements {
      result = self.eval_statement(statement).await?;

      // Return/Break/Continue trigger early exits from statement sequences.
      if result.is_control_signal() {
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
