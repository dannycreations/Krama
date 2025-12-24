mod assignment;
mod binary;
mod call;
mod collection;
mod control;
mod identifier;
mod import;
mod index;
mod literal;
mod member;
mod properties;
mod result;
mod structs;
mod unary;

use futures::{
  future::{FutureExt, LocalBoxFuture},
  try_join,
};
use krama_core::{Error, Expression, ExpressionKind, ObjectKind};

use crate::interpreter::{types::check_type, Interpreter};

impl<'ast> Interpreter<'ast> {
  /// Evaluates an expression and returns its resulting object.
  /// This is the central dispatch for all expression types.
  pub fn eval_expression<'s>(
    &'s self,
    expression: &'s Expression<'ast>,
    kind: Option<&'s krama_core::Type<'ast>>,
  ) -> LocalBoxFuture<'s, Result<ObjectKind<'ast>, Error<'ast>>>
  where
    'ast: 's,
  {
    async move {
      let span = expression.span;
      let result = match &expression.kind {
        // 1. Core Literals and Identifiers
        ExpressionKind::Literal(literal) => self.eval_literal(*literal),
        ExpressionKind::Identifier(name) => {
          self.eval_identifier(expression, name, span).await
        }
        ExpressionKind::This => self.get_this(span),

        // 2. Structural Construction
        ExpressionKind::StructConstruction { properties } => {
          self.eval_struct_construction(properties, span).await
        }
        ExpressionKind::Object { properties } => {
          self.eval_object_literal(properties).await
        }
        ExpressionKind::Collection { elements } => {
          self.eval_collection(elements, kind, span).await
        }

        // 3. Operators
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

        // 4. Access and Calls
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

        // 5. Control Flow Expressions
        ExpressionKind::If {
          condition,
          then_branch,
          else_branch,
          ..
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

        // 6. Functions and Imports
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

        // 7. Advanced and Type-related
        ExpressionKind::Typed { expr, kind } => {
          let value = self.eval_expression(expr, Some(kind)).await?;
          check_type(kind, &value)?;
          Ok(value)
        }
        ExpressionKind::Try(expr) => self.eval_result(expr, span).await,
      }?;

      // 8. Automatic Error Propagation
      // Unless it's a Try expression (which handles errors explicitly),
      // result errors are wrapped in Return to trigger early exit in statement blocks.
      if !matches!(expression.kind, ExpressionKind::Try(_)) {
        if let ObjectKind::Err(_) = &result {
          return Ok(ObjectKind::Return(self.arena.alloc(result)));
        }
      }

      Ok(result)
    }
    .boxed_local()
  }

  /// Retrieves the 'this' object from the current environment.
  pub fn get_this(
    &self,
    span: krama_core::Span,
  ) -> Result<ObjectKind<'ast>, Error<'ast>> {
    self.environment.borrow().get("this").ok_or_else(|| {
      Error::new(
        krama_core::ErrorKind::ReferenceError(
          "'this' is not defined in the current scope".into(),
        ),
        span,
      )
    })
  }
}
