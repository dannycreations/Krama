use std::sync::Arc;

use futures::{
  future::{FutureExt, LocalBoxFuture},
  try_join,
};
use krama_core::{Expression, ExpressionKind, Object, ObjectResult, Type};

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
    ty: Option<&'s Type>,
  ) -> LocalBoxFuture<'s, ObjectResult> {
    async move {
      let span = expression.span;
      let result = match &expression.kind {
        ExpressionKind::Literal(literal) => Ok(literal.clone().into()),
        ExpressionKind::Identifier(name) => {
          self.eval_identifier(expression, name, span).await
        }
        ExpressionKind::This => self.get_this(span),
        ExpressionKind::Struct { properties } => {
          self.eval_struct_expression(properties, span).await
        }
        ExpressionKind::Object { properties } => {
          self.eval_object_expression(properties).await
        }
        ExpressionKind::Array { elements } => {
          self.eval_array(elements, ty, span).await
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
          self.eval_access_expression(object, property, span).await
        }
        ExpressionKind::Index { object, index } => {
          let (object, index) = try_join!(
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
              ty,
            )
            .await
        }
        ExpressionKind::Match { subject, arms } => {
          self.eval_match_expression(subject, arms, span).await
        }
        ExpressionKind::Block(block) => {
          self.eval_block_statement_with_new_scope(block).await
        }
        ExpressionKind::Function {
          parameters,
          body,
          ty,
        } => Ok(self.alloc_user_function(
          parameters.clone(),
          body.clone(),
          ty.clone(),
        )),
        ExpressionKind::Import { path, .. } => {
          self.eval_import_expression(path, span).await
        }
        ExpressionKind::Cast { expr, ty } => {
          self.eval_and_check_type(expr, Some(ty)).await
        }
        ExpressionKind::Try(expr) => self.eval_result(expr, span).await,
      }?;

      if !matches!(expression.kind, ExpressionKind::Try(_))
        && result.is_result_err()
      {
        Ok(Object::Return(Arc::new(result)))
      } else {
        Ok(result)
      }
    }
    .boxed_local()
  }
}
