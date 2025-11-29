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
  error::ErrorKind,
  object::{Function, Object, UserFunction},
  span::Span,
};

use super::Interpreter;

impl<'ast> Interpreter<'ast> {
  pub(super) fn eval_expression<'s>(
    &'s self,
    expression: &'s Expression<'ast>,
    kind: Option<&'s Type<'ast>>,
  ) -> LocalBoxFuture<'s, Result<Object<'ast>, (ErrorKind, Span<'ast>)>>
  where
    'ast: 's,
  {
    async move {
      let span = expression.span.clone();
      match &expression.kind {
        ExpressionKind::Literal(literal) => self.eval_literal(*literal),
        ExpressionKind::Identifier(name) => {
          self.eval_identifier(expression, name, span).await
        }
        ExpressionKind::Unary { operator, right } => {
          let right = self.eval_expression(right, None).await?;
          let right = self.resolve_object(right).await?;
          self.eval_unary_expression(*operator, right, span)
        }
        ExpressionKind::Binary {
          left,
          operator,
          right,
        } => {
          if *operator == BinaryOperator::LogicalAnd {
            let left = self.eval_expression(left, None).await?;
            if !bool::from(&left) {
              return Ok(Object::Boolean(false));
            }
            let right = self.eval_expression(right, None).await?;
            return Ok(Object::Boolean(bool::from(&right)));
          }

          if *operator == BinaryOperator::LogicalOr {
            let left = self.eval_expression(left, None).await?;
            if bool::from(&left) {
              return Ok(Object::Boolean(true));
            }
            let right = self.eval_expression(right, None).await?;
            return Ok(Object::Boolean(bool::from(&right)));
          }

          let (left, right) = join!(
            self.eval_expression(left, None),
            self.eval_expression(right, None)
          );
          let (left, right) = (
            self.resolve_object(left?).await?,
            self.resolve_object(right?).await?,
          );
          self.eval_binary_expression(*operator, left, right, span)
        }
        ExpressionKind::Assignment {
          left,
          operator,
          right,
        } => {
          self
            .eval_assignment_expression(left, *operator, right, span)
            .await
        }
        ExpressionKind::Update {
          operator,
          argument,
          prefix,
        } => {
          self
            .eval_update_expression(*operator, argument, *prefix, span)
            .await
        }
        ExpressionKind::Import { path, .. } => {
          self.eval_import(path, span).await
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
              span,
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
          self.eval_match_expression(subject, arms, span).await
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
          let object = self.resolve_object(object).await?;
          self.eval_member_expression(object, property, span).await
        }
        ExpressionKind::Index { object, index } => {
          let (object, index) = join!(
            self.eval_expression(object, None),
            self.eval_expression(index, None)
          );
          let (object, index) = (
            self.resolve_object(object?).await?,
            self.resolve_object(index?).await?,
          );
          self.eval_index_expression(object, index, span).await
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
      }
    }
    .boxed_local()
  }
}
