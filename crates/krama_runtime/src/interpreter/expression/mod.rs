mod assignment;
mod binary;
mod call;
mod collection;
mod control;
mod identifier;
mod import;
mod index;
mod member;
mod properties;
mod result;
mod structs;
mod unary;

use futures::{
  future::{FutureExt, LocalBoxFuture},
  try_join,
};
use krama_core::{
  Error, ErrorKind, Expression, ExpressionKind, LiteralKind, ObjectKind, Span,
};

use crate::interpreter::{types::check_type, Interpreter};

impl Interpreter {
  /// Evaluates an expression and returns its resulting object.
  pub fn eval_expression<'s>(
    &'s self,
    expression: &'s Expression,
    kind: Option<&'s krama_core::Type>,
  ) -> LocalBoxFuture<'s, Result<ObjectKind, Error>> {
    async move {
      let span = expression.span;
      let result = match &expression.kind {
        ExpressionKind::Literal(literal) => self.eval_literal(literal.clone()),
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
          let value = self.eval_expression(expr, Some(kind)).await?;
          check_type(kind, &value)?;
          Ok(value)
        }
        ExpressionKind::Try(expr) => self.eval_result(expr, span).await,
      }?;

      // Implicit error propagation: wrap Err in Return if not handled by Try.
      // Control signals (Return/Break/Continue) are propagated as-is.
      // Try expressions handle errors explicitly, so we skip propagation for them.
      if !matches!(expression.kind, ExpressionKind::Try(_))
        && result.is_result_err()
        && !result.is_control_signal()
      {
        return Ok(ObjectKind::Return(Box::new(result)));
      }

      Ok(result)
    }
    .boxed_local()
  }

  /// Retrieves the 'this' object from the current environment.
  pub fn get_this(&self, span: Span) -> Result<ObjectKind, Error> {
    self.stack.read().get("this").ok_or_else(|| {
      Error::new(
        ErrorKind::ReferenceError(
          "'this' is not defined in the current scope".into(),
        ),
        span,
      )
    })
  }

  /// Evaluates a literal into an ObjectKind.
  #[inline]
  pub fn eval_literal(
    &self,
    literal: LiteralKind,
  ) -> Result<ObjectKind, Error> {
    Ok(match literal {
      LiteralKind::Integer(i) => ObjectKind::Integer(i),
      LiteralKind::Float(f) => ObjectKind::Float(f),
      LiteralKind::String(s) => ObjectKind::String(s),
      LiteralKind::Boolean(b) => ObjectKind::Boolean(b),
      LiteralKind::Null => ObjectKind::Null,
    })
  }
}
