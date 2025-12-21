use bumpalo::collections::Vec as BumpVec;
use futures::{
  future::{try_join_all, FutureExt, LocalBoxFuture},
  try_join,
};
use indexmap::IndexMap;
use krama_core::{
  BinaryOperator, Error, ErrorKind, Expression, ExpressionKind, Function,
  Object, Type, TypeKind, UserFunction,
};
use parking_lot::RwLock;

use super::{types::check_type, Interpreter};

impl<'ast> Interpreter<'ast> {
  /// Evaluates an expression with an optional type hint.
  /// Uses LocalBoxFuture to handle recursion in an async context efficiently.
  pub fn eval_expression<'s>(
    &'s self,
    expression: &'s Expression<'ast>,
    kind: Option<&'s Type<'ast>>,
  ) -> LocalBoxFuture<'s, Result<Object<'ast>, Error<'ast>>>
  where
    'ast: 's,
  {
    async move {
      let span = &expression.span;
      match &expression.kind {
        ExpressionKind::Literal(literal) => self.eval_literal(*literal),
        ExpressionKind::Identifier(name) => {
          self.eval_identifier(expression, name, *span).await
        }
        ExpressionKind::Unary { operator, right } => {
          let right = self.eval_expression(right, None).await?;
          self.eval_unary_expression(*operator, right, *span)
        }
        ExpressionKind::Binary {
          left,
          operator,
          right,
        } => {
          // Short-circuiting logical operators to avoid unnecessary evaluations.
          if *operator == BinaryOperator::LogicalOr
            || *operator == BinaryOperator::LogicalAnd
          {
            let left_value = self.eval_expression(left, None).await?;
            let left_bool = bool::from(&left_value);

            if *operator == BinaryOperator::LogicalOr {
              if left_bool {
                return Ok(left_value);
              }
            } else if !left_bool {
              return Ok(left_value);
            }
            return self.eval_expression(right, None).await;
          }

          // Parallel evaluation of binary operands when possible.
          let (left, right) = try_join!(
            self.eval_expression(left, None),
            self.eval_expression(right, None)
          )?;

          self.eval_binary_expression(*operator, left, right, *span)
        }
        ExpressionKind::Assignment {
          left,
          operator,
          right,
        } => {
          self
            .eval_assignment_expression(left, *operator, right, *span)
            .await
        }
        ExpressionKind::Update {
          operator,
          argument,
          prefix,
        } => {
          self
            .eval_update_expression(*operator, argument, *prefix, *span)
            .await
        }
        ExpressionKind::Import { path, .. } => {
          self.eval_import(path, *span).await
        }
        ExpressionKind::Call {
          function,
          arguments,
        } => {
          let func_future = self.eval_expression(function, None);

          if arguments.is_empty() {
            let function_obj = func_future.await?;
            return self.eval_call_expression(function_obj, &[], *span).await;
          }

          // Parallelize argument evaluation to improve performance.
          let args_futures =
            arguments.iter().map(|arg| self.eval_expression(arg, None));

          let (function_obj, evaluated_args_vec) =
            try_join!(func_future, try_join_all(args_futures))?;

          // Direct allocation into the arena to minimize intermediate heap usage.
          let args_slice = self.arena.alloc_slice_fill_iter(evaluated_args_vec);

          self
            .eval_call_expression(function_obj, args_slice, *span)
            .await
        }
        ExpressionKind::If {
          condition,
          then_branch,
          else_branch,
        } => {
          self
            .eval_if_expression(condition, then_branch, *else_branch, kind)
            .await
        }
        ExpressionKind::Match { subject, arms } => {
          self.eval_match_expression(subject, arms, *span).await
        }
        ExpressionKind::Block(block) => {
          self.eval_block_statement_with_new_scope(block).await
        }
        ExpressionKind::Fn {
          parameters,
          body,
          kind,
        } => {
          let user_fn = self.arena.alloc(UserFunction {
            parameters: parameters.clone(),
            body: body.clone(),
            kind: kind.clone(),
          });
          Ok(Object::Function(Function::User(user_fn)))
        }
        ExpressionKind::Member { object, property } => {
          let object = self.eval_expression(object, None).await?;
          self.eval_member_expression(object, property, *span).await
        }
        ExpressionKind::Index { object, index } => {
          let (object, index) = try_join!(
            self.eval_expression(object, None),
            self.eval_expression(index, None)
          )?;

          self.eval_index_expression(object, index, *span).await
        }
        ExpressionKind::Collection { elements } => {
          let mut element_kind = None;

          if let Some(hint) = kind {
            if let TypeKind::Array { element, .. } = &hint.kind {
              element_kind = Some(*element);
            }
          }

          if elements.is_empty() {
            if let Some(hint) = kind {
              match &hint.kind {
                TypeKind::Array { .. } => {
                  return Ok(Object::Array {
                    elements: self
                      .arena
                      .alloc(RwLock::new(BumpVec::new_in(self.arena))),
                    kind: hint.clone(),
                    constant: false,
                  })
                }
                TypeKind::Tuple(_) => {
                  return Ok(Object::Tuple { elements: &[] })
                }
                _ => {}
              }
            }
            return Ok(Object::Array {
              elements: self
                .arena
                .alloc(RwLock::new(BumpVec::new_in(self.arena))),
              kind: Type::new(
                TypeKind::Array {
                  element: self.arena.alloc(Type::new(TypeKind::Void, *span)),
                  size: None,
                },
                *span,
              ),
              constant: false,
            });
          }

          let element_futures = elements
            .iter()
            .map(|e| self.eval_expression(e, element_kind));
          let results = try_join_all(element_futures).await?;

          if let Some(hint) = kind {
            match hint.kind {
              TypeKind::Array { .. } => {
                let mut evaluated_elements =
                  BumpVec::with_capacity_in(results.len(), self.arena);
                evaluated_elements.extend(results);
                return Ok(Object::Array {
                  elements: self.arena.alloc(RwLock::new(evaluated_elements)),
                  kind: hint.clone(),
                  constant: false,
                });
              }
              TypeKind::Tuple(_) => {
                return Ok(Object::Tuple {
                  elements: self.arena.alloc_slice_fill_iter(results),
                })
              }
              _ => {}
            }
          }

          Ok(Object::Tuple {
            elements: self.arena.alloc_slice_fill_iter(results),
          })
        }
        ExpressionKind::Object { properties } => {
          let mut object = IndexMap::with_capacity(properties.len());
          for (key, value) in properties {
            let key = match self.eval_expression(key, None).await? {
              Object::String(s) => s,
              _ => {
                return Err(Error::new(
                  ErrorKind::TypeError("Expected string key".to_string()),
                  key.span,
                ))
              }
            };
            let value = self.eval_expression(value, None).await?;
            object.insert(key, value);
          }
          Ok(Object::Object {
            properties: self.arena.alloc(RwLock::new(object)),
            constant: false,
          })
        }
        ExpressionKind::Typed { expr, kind } => {
          let value = self.eval_expression(expr, Some(kind)).await?;
          check_type(kind, &value)?;
          Ok(value)
        }
        ExpressionKind::Try(expr) => {
          let value = self.eval_expression(expr, None).await?;
          match value {
            Object::Ok(v) => Ok(v.clone()),
            Object::Err(e) => {
              Err(Error::new(ErrorKind::RuntimeError(format!("{}", e)), *span))
            }
            _ => Err(Error::new(
              ErrorKind::TypeError(format!(
                "Expected Result type for ? operator, found {}",
                value.type_name()
              )),
              *span,
            )),
          }
        }
      }
    }
    .boxed_local()
  }
}
