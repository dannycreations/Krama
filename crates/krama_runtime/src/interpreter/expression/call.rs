use std::sync::Arc;

use futures::future::try_join_all;
use krama_core::{
  EnumInstance, Error, ErrorKind, Expression, ExpressionKind, FunctionBody,
  FunctionKind, ObjectKind, ObjectResult, Span, UserFunction,
};

use crate::{check_type, Interpreter};

impl Interpreter {
  /// Evaluates a function call, handling both direct calls and method calls.
  pub async fn eval_call(
    &self,
    function: &Expression,
    arguments: &[Expression],
    span: Span,
  ) -> ObjectResult {
    let (func_obj, this_binding) = match &function.kind {
      ExpressionKind::Member { object, property } => {
        let obj_val = self.eval_expression(object, None).await?;
        let func = self
          .eval_member_expression(obj_val.clone(), property, span)
          .await?;
        (func, obj_val)
      }
      _ => (
        self.eval_expression(function, None).await?,
        ObjectKind::Void,
      ),
    };

    let evaluated_args =
      try_join_all(arguments.iter().map(|arg| self.eval_expression(arg, None)))
        .await?;

    self
      .eval_call_expression_with_this(
        func_obj,
        &evaluated_args,
        this_binding,
        span,
      )
      .await
  }

  /// Evaluates a function call with a specific 'this' binding.
  pub async fn eval_call_expression_with_this(
    &self,
    function: ObjectKind,
    arguments: &[ObjectKind],
    this: ObjectKind,
    span: Span,
  ) -> ObjectResult {
    match function {
      ObjectKind::Function(function) => match function {
        FunctionKind::Native(native_fn) => (native_fn.callback)(arguments)
          .await
          .map_err(|kind| Error::new(kind, span)),
        FunctionKind::User { func, env } => {
          self
            .eval_user_function_call_with_this(
              &func,
              env.clone(),
              arguments,
              this,
              span,
            )
            .await
        }
        FunctionKind::Enum(constructor) => {
          if arguments.len() != constructor.field_count {
            return Err(Error::new(
              ErrorKind::TypeError(format!(
                "Expected {} arguments for variant {}.{}, found {}",
                constructor.field_count,
                constructor.name,
                constructor.variant,
                arguments.len()
              )),
              span,
            ));
          }
          // Enum values store fields as Arc<[ObjectKind]> for efficient sharing and size predictability.
          Ok(ObjectKind::Enum(Box::new(EnumInstance {
            name: constructor.name.clone(),
            variant: constructor.variant.clone(),
            fields: Some(arguments.into()),
          })))
        }
      },
      _ => Err(Error::new(
        ErrorKind::TypeError(format!(
          "Expected a function, but got {}",
          function.type_name()
        )),
        span,
      )),
    }
  }

  /// Internal helper to execute a user-defined function.
  async fn eval_user_function_call_with_this(
    &self,
    user_fn: &UserFunction,
    closure_env: Option<Arc<parking_lot::RwLock<krama_core::Scope>>>,
    arguments: &[ObjectKind],
    this: ObjectKind,
    span: Span,
  ) -> ObjectResult {
    if arguments.len() > user_fn.parameters.len() {
      return Err(Error::new(
        ErrorKind::TypeError(format!(
          "Expected {} arguments, but got {}",
          user_fn.parameters.len(),
          arguments.len()
        )),
        span,
      ));
    }

    // Reuse the stack reference to avoid multiple Arc clones.
    let stack_ref = &self.stack;

    // Push a new scope onto the stack.
    stack_ref.write().push("function_call".into(), closure_env);

    if !matches!(this, ObjectKind::Void) {
      let mut stack = stack_ref.write();
      stack.define("this".into(), this.clone(), false, true);

      let struct_name = match &this {
        ObjectKind::Object {
          definition: Some(definition),
          ..
        } => Some(definition.name.clone()),
        ObjectKind::Struct(definition) => Some(definition.name.clone()),
        _ => None,
      };

      if let Some(name) = struct_name {
        stack.define(
          "__current_struct__".into(),
          ObjectKind::String(name),
          false,
          true,
        );
      }
    }

    for (i, param) in user_fn.parameters.iter().enumerate() {
      let value = if let Some(arg) = arguments.get(i) {
        arg.clone()
      } else if let Some(default) = &param.default {
        self.eval_expression(default, None).await?
      } else {
        return Err(Error::new(
          ErrorKind::TypeError(format!(
            "Missing argument for parameter '{}'",
            param.name
          )),
          span,
        ));
      };

      if let Some(param_type) = &param.kind {
        check_type(param_type, &value)?;
      }

      stack_ref
        .write()
        .define(param.name.clone(), value, false, false);
    }

    let result = match &user_fn.body {
      FunctionBody::Block(block) => {
        self.eval_statements(&block.statements).await
      }
      FunctionBody::Expression(expr) => self.eval_expression(expr, None).await,
    };

    // Pop the stack frame (which is the current scope)
    stack_ref.write().pop();

    // Functions always unwrap Return signals to return the underlying value.
    Ok(result?.unwrap_return().clone())
  }
}
