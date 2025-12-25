mod binding;
mod iteration;

use std::sync::Arc;

use ahash::AHashMap;
use futures::future::{FutureExt, LocalBoxFuture};
use krama_core::{
  AssignmentOperator, Enum, EnumInstance, Expression, ExpressionKind,
  FunctionKind, ObjectKind, ObjectResult, Statement, StatementBlock,
  StatementKind, Struct, Type,
};

use crate::interpreter::{
  types::{check_type, resolve_type},
  Interpreter,
};

impl Interpreter {
  /// Evaluates a single statement.
  pub fn eval_statement<'s>(
    &'s self,
    statement: &'s Statement,
  ) -> LocalBoxFuture<'s, ObjectResult> {
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
          self.stack.write().define(name.clone(), value, false, false);
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
          let resolved_kind =
            kind.as_ref().map(|k| resolve_type(self, k)).transpose()?;
          let function = self.alloc_user_function(
            parameters.clone(),
            body.clone(),
            resolved_kind,
          );
          self
            .stack
            .write()
            .define(name.clone(), function, *public, true);
          Ok(ObjectKind::Void)
        }
        StatementKind::Enum {
          public,
          name,
          variants,
        } => {
          // Using Arc<str> for names to reduce allocations and improve equality checks.
          let name_arc = name.clone();
          let mut properties = AHashMap::with_capacity(variants.len());
          for variant in variants {
            let variant_name_arc = variant.name.clone();
            let obj = if let Some(fields) = &variant.fields {
              ObjectKind::Function(FunctionKind::Enum(Arc::new(Enum {
                name: name_arc.clone(),
                variant: variant_name_arc.clone(),
                field_count: fields.len(),
              })))
            } else {
              ObjectKind::Enum(Box::new(EnumInstance {
                name: name_arc.clone(),
                variant: variant_name_arc.clone(),
                fields: None,
              }))
            };
            properties.insert(variant_name_arc, obj);
          }
          let enum_obj = self.heap.write().alloc_object(
            properties.into_iter().collect(),
            None,
            true,
          );
          self
            .stack
            .write()
            .define(name.clone(), enum_obj, *public, true);
          Ok(ObjectKind::Void)
        }
        StatementKind::Struct {
          public,
          name,
          fields,
          methods,
        } => {
          // Struct definitions now use Arc<str> for name to optimize memory footprint.
          // O(1) field and method lookup maps are pre-computed during definition for efficient runtime access.
          let name_arc = name.clone();
          let field_map = fields
            .iter()
            .enumerate()
            .map(|(i, f)| (f.name.clone(), i))
            .collect();

          let method_map = methods
            .iter()
            .enumerate()
            .map(|(i, m)| (m.name.clone(), i))
            .collect();

          let struct_def = Arc::new(Struct {
            name: name_arc,
            fields: fields.clone(),
            methods: methods.clone(),
            field_map,
            method_map,
          });
          self.stack.write().define(
            name.clone(),
            ObjectKind::Struct(struct_def),
            *public,
            true,
          );
          Ok(ObjectKind::Void)
        }
        StatementKind::Type { public, name, kind } => {
          let resolved = resolve_type(self, kind)?;
          self.stack.write().define(
            name.clone(),
            ObjectKind::Type(resolved),
            *public,
            true,
          );
          Ok(ObjectKind::Void)
        }
        StatementKind::Return { value } => {
          let value = match value {
            Some(expr) => self.eval_expression(expr, None).await?,
            None => ObjectKind::Void,
          };
          Ok(ObjectKind::Return(Arc::new(value)))
        }
        StatementKind::Break => Ok(ObjectKind::Break),
        StatementKind::Continue => Ok(ObjectKind::Continue),
        StatementKind::While { condition, body } => {
          loop {
            if let Some(bindings) = self.try_match_assignment(condition).await?
            {
              for (name, val) in bindings {
                self.stack.write().define(name.clone(), val, false, false);
              }
            } else {
              // Special case for pattern matching in while
              if let ExpressionKind::Assignment {
                left,
                operator: AssignmentOperator::Assign,
                ..
              } = &condition.kind
              {
                if let ExpressionKind::Call { function, .. } = &left.kind {
                  if let ExpressionKind::Identifier(name) = &function.kind {
                    if name.as_ref() == "Ok" || name.as_ref() == "Err" {
                      break;
                    }
                  }
                }
              }
              if !self.eval_expression(condition, None).await?.is_truthy() {
                break;
              }
            }

            let result = self.eval_block_statement(body).await?;
            if result.is_control_signal() {
              match result {
                ObjectKind::Break => break,
                ObjectKind::Continue => continue,
                ObjectKind::Return(inner) if inner.is_result_err() => continue,
                _ => return Ok(result),
              }
            }
          }
          Ok(ObjectKind::Void)
        }
        StatementKind::For {
          binding,
          iterable,
          body,
        } => {
          let iterable_val = self.eval_expression(iterable, None).await?;
          let elements =
            self.collect_iterable_elements(&iterable_val, binding, span)?;
          for element in elements {
            // Push/Pop scope without holding the lock during await.
            {
              let mut stack = self.stack.write();
              stack.push("for_loop_iter".into(), None);
            }

            self.assign_for_binding(binding, element, span)?;
            let result = self.eval_block_statement(body).await;

            self.stack.write().pop();

            let result = result?;
            if result.is_control_signal() {
              match result {
                ObjectKind::Break => break,
                ObjectKind::Continue => continue,
                ObjectKind::Return(inner) if inner.is_result_err() => continue,
                _ => return Ok(result),
              }
            }
          }
          Ok(ObjectKind::Void)
        }
        StatementKind::Test { body, .. } => {
          self.eval_block_statement_with_new_scope(body).await
        }
      }
    }
    .boxed_local()
  }

  /// Helper to evaluate an expression and check its type.
  pub async fn eval_and_check_type(
    &self,
    expr: &Expression,
    kind_hint: Option<&Type>,
  ) -> ObjectResult {
    let resolved = kind_hint.map(|k| resolve_type(self, k)).transpose()?;
    let value = self.eval_expression(expr, resolved.as_ref()).await?;
    if let Some(kind) = &resolved {
      check_type(kind, &value)?;
    }
    Ok(value)
  }

  /// Evaluates a sequence of statements.
  pub async fn eval_statements<'s>(
    &'s self,
    statements: &'s [Statement],
  ) -> ObjectResult {
    let mut result = ObjectKind::Void;
    for statement in statements {
      result = self.eval_statement(statement).await?;
      if result.is_control_signal() {
        return Ok(result);
      }
    }
    Ok(result)
  }

  /// Evaluates a statement block.
  pub async fn eval_block_statement(
    &self,
    block: &StatementBlock,
  ) -> ObjectResult {
    self.eval_statements(&block.statements).await
  }

  /// Evaluates a statement block with a new enclosed scope.
  pub async fn eval_block_statement_with_new_scope(
    &self,
    block: &StatementBlock,
  ) -> ObjectResult {
    self.stack.write().push("block".into(), None);
    let result = self.eval_statements(&block.statements).await;
    self.stack.write().pop();
    result
  }
}
