use bumpalo::collections::Vec as BumpVec;
use futures::{
  future::{join_all, FutureExt, LocalBoxFuture},
  join,
};
use krama_core::{
  ast::{
    expression::{Expression, ExpressionKind},
    operator::BinaryOperator,
    types::{Type, TypeKind},
  },
  error::{Error, ErrorKind},
  object::{Function, Object, UserFunction},
};
use rustc_hash::FxHashMap;

use super::{types::check_type, Interpreter};

impl<'ast> Interpreter<'ast> {
  pub(crate) fn eval_expression<'s>(
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
          self.eval_identifier(expression, name, span.clone()).await
        }
        ExpressionKind::Unary { operator, right } => {
          let right = self.eval_expression(right, None).await?;
          self.eval_unary_expression(*operator, right, span.clone())
        }
        ExpressionKind::Binary {
          left,
          operator,
          right,
        } => match operator {
          // Handle logical operators with short-circuiting
          BinaryOperator::LogicalOr | BinaryOperator::LogicalAnd => {
            let left_value = self.eval_expression(left, None).await?;
            let left_bool = bool::from(&left_value);

            if *operator == BinaryOperator::LogicalOr {
              if left_bool {
                return Ok(left_value);
              }
            } else if !left_bool {
              return Ok(left_value);
            }
            // if we reach here, we evaluate the right side
            self.eval_expression(right, None).await
          }
          // For all other operators, evaluate concurrently
          _ => {
            let (left_res, right_res) = join!(
              self.eval_expression(left, None),
              self.eval_expression(right, None)
            );
            let (left_obj, right_obj) = (left_res?, right_res?);
            self.eval_binary_expression(
              *operator,
              left_obj,
              right_obj,
              span.clone(),
            )
          }
        },
        ExpressionKind::Assignment {
          left,
          operator,
          right,
        } => {
          self
            .eval_assignment_expression(left, *operator, right, span.clone())
            .await
        }
        ExpressionKind::Update {
          operator,
          argument,
          prefix,
        } => {
          self
            .eval_update_expression(*operator, argument, *prefix, span.clone())
            .await
        }
        ExpressionKind::Import { path, .. } => {
          self.eval_import(path, span.clone()).await
        }
        ExpressionKind::Call {
          function,
          arguments,
        } => {
          let func_future = self.eval_expression(function, None);
          let args_futures =
            arguments.iter().map(|arg| self.eval_expression(arg, None));

          let (function_obj, evaluated_args_res) =
            join!(func_future, join_all(args_futures));

          let function = function_obj?;
          let mut evaluated_args = BumpVec::new_in(self.arena);
          for arg in evaluated_args_res {
            evaluated_args.push(arg?);
          }

          self
            .eval_call_expression(
              function,
              evaluated_args.into_bump_slice(),
              span.clone(),
            )
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
          self
            .eval_match_expression(subject, arms, span.clone())
            .await
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
          self
            .eval_member_expression(object, property, span.clone())
            .await
        }
        ExpressionKind::Index { object, index } => {
          let (object, index) = join!(
            self.eval_expression(object, None),
            self.eval_expression(index, None)
          );
          let (object, index) = (object?, index?);
          self
            .eval_index_expression(object, index, span.clone())
            .await
        }
        ExpressionKind::Collection { elements } => {
          let mut element_kind = None;

          if let Some(hint) = kind {
            if let TypeKind::Array { element, .. } = &hint.kind {
              element_kind = Some(*element);
            }
          }

          let element_futures = elements
            .iter()
            .map(|e| self.eval_expression(e, element_kind));
          let results = join_all(element_futures).await;

          let mut evaluated_elements = BumpVec::new_in(self.arena);
          for result in results {
            evaluated_elements.push(result?);
          }
          let elements_slice = evaluated_elements.into_bump_slice();

          if let Some(hint) = kind {
            match hint.kind {
              TypeKind::Array { .. } => {
                return Ok(Object::Array {
                  elements: elements_slice,
                  kind: hint.clone(),
                })
              }
              TypeKind::Tuple(_) => {
                return Ok(Object::Tuple {
                  elements: elements_slice,
                })
              }
              _ => {}
            }
          }
          Ok(Object::Tuple {
            elements: elements_slice,
          })
        }
        ExpressionKind::Object { properties } => {
          let mut object = FxHashMap::default();
          for (key, value) in properties {
            let key = match self.eval_expression(key, None).await? {
              Object::String(s) => s,
              // for now, we only support string keys
              _ => {
                return Err(Error::new(
                  ErrorKind::TypeError("Expected string key".to_string()),
                  key.span.clone(),
                ))
              }
            };
            let value = self.eval_expression(value, None).await?;
            object.insert(key, value);
          }
          Ok(Object::Object(object))
        }
        ExpressionKind::Typed { expr, kind } => {
          let value = self.eval_expression(expr, Some(kind)).await?;
          check_type(kind, &value)?;
          Ok(value)
        }
      }
    }
    .boxed_local()
  }
}
