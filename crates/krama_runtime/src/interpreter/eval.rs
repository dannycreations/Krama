use futures::{
  future::{try_join_all, FutureExt, LocalBoxFuture},
  try_join,
};
use indexmap::IndexMap;
use krama_core::{
  Error, ErrorKind, Expression, ExpressionKind, ObjectKind, Span,
};

use super::{types::check_type, Interpreter};

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
        ExpressionKind::Try(expr) => self.eval_try(expr, span).await,
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
  pub(crate) fn get_this(
    &self,
    span: Span,
  ) -> Result<ObjectKind<'ast>, Error<'ast>> {
    self.environment.borrow().get("this").ok_or_else(|| {
      Error::new(
        ErrorKind::ReferenceError(
          "'this' is not defined in the current scope".into(),
        ),
        span,
      )
    })
  }

  /// Evaluates a function call, handling both direct calls and method calls (member access).
  async fn eval_call(
    &self,
    function: &Expression<'ast>,
    arguments: &[Expression<'ast>],
    span: Span,
  ) -> Result<ObjectKind<'ast>, Error<'ast>> {
    // 1. Resolve function and potential 'this' binding.
    let (func_obj, this_binding) =
      if let ExpressionKind::Member { object, property } = &function.kind {
        let obj_val = self.eval_expression(object, None).await?;
        (
          self
            .eval_member_expression(obj_val.clone(), property, span)
            .await?,
          Some(obj_val),
        )
      } else {
        (self.eval_expression(function, None).await?, None)
      };

    // 2. Evaluate all arguments concurrently.
    let evaluated_args = if arguments.is_empty() {
      &[] as &[ObjectKind]
    } else {
      let results = try_join_all(
        arguments.iter().map(|arg| self.eval_expression(arg, None)),
      )
      .await?;
      self.arena.alloc_slice_fill_iter(results)
    };

    // 3. Dispatch call with or without 'this' context.
    if let Some(this) = this_binding {
      self
        .eval_call_expression_with_this(func_obj, evaluated_args, this, span)
        .await
    } else {
      self
        .eval_call_expression(func_obj, evaluated_args, span)
        .await
    }
  }

  /// Shared logic for evaluating property-based structures (Object, StructConstruction).
  pub(crate) async fn eval_properties(
    &self,
    properties: &[(Expression<'ast>, Expression<'ast>)],
  ) -> Result<IndexMap<&'ast str, ObjectKind<'ast>>, Error<'ast>> {
    let mut fields = IndexMap::with_capacity(properties.len());
    for (key, value) in properties {
      let key_obj = self.eval_expression(key, None).await?;
      let ObjectKind::String(key_str) = key_obj else {
        return Err(Error::new(
          ErrorKind::TypeError("Expected string key".into()),
          key.span,
        ));
      };
      fields.insert(key_str, self.eval_expression(value, None).await?);
    }
    Ok(fields)
  }

  /// Evaluates the postfix '?' operator (Try expression).
  async fn eval_try(
    &self,
    expr: &Expression<'ast>,
    _span: Span,
  ) -> Result<ObjectKind<'ast>, Error<'ast>> {
    let val = self.eval_expression(expr, None).await?;

    // If result is Return(Err), unwrap it to Err to allow the '?' operator to catch it.
    if let ObjectKind::Return(inner) = val {
      if let ObjectKind::Err(_) = inner {
        return Ok(inner.clone());
      }
      return Ok(ObjectKind::Return(inner));
    }

    Ok(val)
  }
}
