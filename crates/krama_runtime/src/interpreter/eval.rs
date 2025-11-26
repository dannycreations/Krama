use std::rc::Rc;

use bumpalo::collections::Vec as BumpVec;
use futures::future::{join_all, FutureExt, LocalBoxFuture};
use futures::join;
use krama_core::ast::expression::{Expression, ExpressionKind};
use krama_core::ast::types::{Type, TypeKind};
use krama_core::error::Error;
use krama_core::object::{Function, Object, UserFn};

use super::Interpreter;

impl<'ast> Interpreter<'ast> {
  pub(super) fn eval_expression<'s>(
    &'s self,
    expression: &'s Expression<'ast>,
    kind: Option<&'s Type<'ast>>,
  ) -> LocalBoxFuture<'s, Result<Object<'ast>, Error>> {
    async move {
      let span = expression.span;
      match &expression.kind {
        ExpressionKind::Literal(literal) => self.eval_literal(*literal).await,
        ExpressionKind::Identifier(name) => {
          self.eval_identifier(name, span).await
        }
        ExpressionKind::Unary { operator, right } => {
          let right = self.eval_expression(right, None).await?;
          self.eval_unary_expression(*operator, right, span).await
        }
        ExpressionKind::Binary {
          left,
          operator,
          right,
        } => {
          let (left, right) = join!(
            self.eval_expression(left, None),
            self.eval_expression(right, None)
          );
          self
            .eval_binary_expression(*operator, left?, right?, span)
            .await
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
            .eval_call_expression(function, evaluated_args, span)
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
        ExpressionKind::Block(block) => self.eval_block_statement(block).await,
        ExpressionKind::Fn {
          parameters,
          body,
          kind,
        } => Ok(Object::Function(Function::User(Rc::new(UserFn {
          parameters: parameters.clone(),
          body: body.clone(),
          kind: kind.clone(),
        })))),
        ExpressionKind::Member { object, property } => {
          let object = self.eval_expression(object, None).await?;
          self.eval_member_expression(object, property, span).await
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

          if let Some(hint) = kind {
            match hint.kind {
              TypeKind::Array { .. } => {
                return Ok(Object::Array {
                  elements: Rc::new(evaluated_elements),
                  kind: hint.clone(),
                })
              }
              TypeKind::Tuple(_) => {
                return Ok(Object::Tuple(Rc::new(evaluated_elements)))
              }
              _ => {}
            }
          }
          Ok(Object::Tuple(Rc::new(evaluated_elements)))
        }
      }
    }
    .boxed_local()
  }
}
