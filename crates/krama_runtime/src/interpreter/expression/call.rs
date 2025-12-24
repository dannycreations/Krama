use futures::future::try_join_all;
use krama_core::{
  Error, ErrorKind, Expression, ExpressionKind, FunctionBody, FunctionKind,
  ObjectKind, Span, UserFunction,
};

use crate::{check_type, Interpreter};

impl<'ast> Interpreter<'ast> {
  /// Evaluates a function call, handling both direct calls and method calls (member access).
  pub async fn eval_call(
    &self,
    function: &Expression<'ast>,
    arguments: &[Expression<'ast>],
    span: Span,
  ) -> Result<ObjectKind<'ast>, krama_core::Error<'ast>> {
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

  /// Evaluates a function call with a default 'void' this binding.
  async fn eval_call_expression(
    &self,
    function: ObjectKind<'ast>,
    arguments: &'ast [ObjectKind<'ast>],
    span: Span,
  ) -> Result<ObjectKind<'ast>, Error<'ast>> {
    self
      .eval_call_expression_with_this(
        function,
        arguments,
        ObjectKind::Void,
        span,
      )
      .await
  }

  /// Evaluates a function call with a specific 'this' binding.
  /// Handles Native functions, User functions, and Enum variants.
  async fn eval_call_expression_with_this(
    &self,
    function: ObjectKind<'ast>,
    arguments: &'ast [ObjectKind<'ast>],
    this: ObjectKind<'ast>,
    span: Span,
  ) -> Result<ObjectKind<'ast>, Error<'ast>> {
    match function {
      ObjectKind::Function(function) => match function {
        // 1. Dispatch to Rust native callback.
        FunctionKind::Native(native_fn) => {
          (native_fn.callback)(self.arena, arguments)
            .await
            .map_err(|kind| Error::new(kind, span))
        }
        // 2. Dispatch to Krama user-defined function.
        FunctionKind::User(user_fn) => {
          self
            .eval_user_function_call_with_this(user_fn, arguments, this, span)
            .await
        }
        // 3. Construct an Enum variant.
        FunctionKind::Enum(constructor) => {
          if arguments.len() != constructor.field_count {
            return Err(Error::new(
              ErrorKind::TypeError(format!(
                "Expected {} arguments for variant {}::{}, found {}",
                constructor.field_count,
                constructor.name,
                constructor.variant,
                arguments.len()
              )),
              span,
            ));
          }
          Ok(ObjectKind::Enum {
            name: constructor.name,
            variant: constructor.variant,
            fields: Some(arguments),
          })
        }
      },
      // Structs are not callable directly; they require .new() or similar methods.
      ObjectKind::Struct(_) => Err(Error::new(
        ErrorKind::TypeError(format!(
          "{} is not callable directly. Use .new() or other static methods.",
          function.type_name()
        )),
        span,
      )),
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
  /// Handles scope creation, parameter binding, and 'this' context.
  async fn eval_user_function_call_with_this(
    &self,
    user_fn: &'ast UserFunction<'ast>,
    arguments: &'ast [ObjectKind<'ast>],
    this: ObjectKind<'ast>,
    span: Span,
  ) -> Result<ObjectKind<'ast>, Error<'ast>> {
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

    // Create a new enclosed environment for the function execution.
    let new_interpreter = self.new_enclosed();

    // 1. Set 'this' and metadata in the new scope if it's a method call.
    if !matches!(this, ObjectKind::Void) {
      let mut env = new_interpreter.environment.borrow_mut();
      env.set("this", this.clone(), false, true);

      // Store the current struct name for private member access checks.
      let struct_name = match &this {
        ObjectKind::StructInstance { definition, .. } => Some(definition.name),
        ObjectKind::Struct(definition) => Some(definition.name),
        _ => None,
      };

      if let Some(name) = struct_name {
        env.set("__current_struct__", ObjectKind::String(name), false, true);
      }
    }

    // 2. Bind arguments to parameters, handling defaults and type checks.
    for (i, param) in user_fn.parameters.iter().enumerate() {
      let value = if let Some(arg) = arguments.get(i) {
        arg.clone()
      } else if let Some(default) = param.default {
        new_interpreter.eval_expression(default, None).await?
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

      new_interpreter
        .environment
        .borrow_mut()
        .set(param.name, value, false, false);
    }

    // 3. Execute the function body.
    let result = match &user_fn.body {
      FunctionBody::Block(block) => {
        new_interpreter.eval_statements(&block.statements).await
      }
      FunctionBody::Expression(expr) => {
        new_interpreter.eval_expression(expr, None).await
      }
    }?;

    // 4. Unwrap Return signal if present.
    if let ObjectKind::Return(value) = result {
      Ok(value.clone())
    } else {
      Ok(result)
    }
  }
}
