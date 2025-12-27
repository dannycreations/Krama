use std::sync::Arc;

use futures::future::{FutureExt, LocalBoxFuture};
use krama_core::{Expression, ExpressionKind, ObjectKind, ObjectResult, Type};

use crate::interpreter::Interpreter;

mod call;
mod control;
mod lvalue;
mod member;
mod module;
mod primary;

impl Interpreter {
  /// Evaluates an expression and returns its resulting object.
  pub fn eval_expression<'s>(
    &'s self,
    expression: &'s Expression,
    kind: Option<&'s Type>,
  ) -> LocalBoxFuture<'s, ObjectResult> {
    async move {
      let span = expression.span;
      let result = match &expression.kind {
        ExpressionKind::Literal(literal) => Ok(literal.clone().into()),
        ExpressionKind::Identifier(name) => {
          self.eval_identifier(expression, name, span).await
        }
        ExpressionKind::This => self.get_this(span),
        ExpressionKind::StructConstruction { properties } => {
          self.eval_struct_construction(properties, span).await
        }
        ExpressionKind::Object { properties } => {
          self.eval_object_literal(properties).await
        }
        ExpressionKind::Collection { elements } => {
          self.eval_collection(elements, kind, span).await
        }
        ExpressionKind::Unary { operator, right } => {
          let right = self.eval_expression(right, None).await?;
          self.eval_unary_expression(*operator, right, span)
        }
        ExpressionKind::Binary {
          left,
          operator,
          right,
        } => {
          self
            .eval_binary_expression(left, *operator, right, span)
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
        ExpressionKind::Member { object, property } => {
          let object = self.eval_expression(object, None).await?;
          self.eval_member_expression(object, property, span).await
        }
        ExpressionKind::Index { object, index } => {
          let (object, index) = futures::try_join!(
            self.eval_expression(object, None),
            self.eval_expression(index, None)
          )?;
          self.eval_index_expression(object, index, span).await
        }
        ExpressionKind::Call {
          function,
          arguments,
        } => self.eval_call(function, arguments, span).await,
        ExpressionKind::If {
          condition,
          then_branch,
          else_branch,
        } => {
          self
            .eval_if_expression(
              condition,
              then_branch,
              else_branch.as_deref(),
              kind,
            )
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
        } => Ok(self.alloc_user_function(
          parameters.clone(),
          body.clone(),
          kind.clone(),
        )),
        ExpressionKind::Import { path, .. } => {
          self.eval_import(path, span).await
        }
        ExpressionKind::Typed { expr, kind } => {
          self.eval_and_check_type(expr, Some(kind)).await
        }
        ExpressionKind::Try(expr) => self.eval_result(expr, span).await,
      }?;

      if !matches!(expression.kind, ExpressionKind::Try(_))
        && result.is_result_err()
      {
        Ok(ObjectKind::Return(Arc::new(result)))
      } else {
        Ok(result)
      }
    }
    .boxed_local()
  }
}
