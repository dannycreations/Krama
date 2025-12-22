use bumpalo::collections::Vec as BumpVec;
use krama_core::{
  Error, ErrorKind, FunctionBody, FunctionKind, ObjectKind, Parameter, Span,
  Type, UserFunction,
};

use super::{types::check_type, Interpreter};

impl<'ast> Interpreter<'ast> {
  /// Allocates a new UserFunction in the interpreter's arena.
  /// Centralizes function creation to avoid duplication in eval.rs and statement.rs.
  pub fn alloc_user_function(
    &self,
    parameters: BumpVec<'ast, Parameter<'ast>>,
    body: FunctionBody<'ast>,
    kind: Option<Type<'ast>>,
  ) -> ObjectKind<'ast> {
    let user_fn = self.arena.alloc(UserFunction {
      parameters,
      body,
      kind,
    });
    ObjectKind::Function(FunctionKind::User(user_fn))
  }

  pub async fn eval_call_expression(
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

  pub async fn eval_call_expression_with_this(
    &self,
    function: ObjectKind<'ast>,
    arguments: &'ast [ObjectKind<'ast>],
    this: ObjectKind<'ast>,
    span: Span,
  ) -> Result<ObjectKind<'ast>, Error<'ast>> {
    match function {
      ObjectKind::Function(function) => match function {
        FunctionKind::Native(native_fn) => {
          (native_fn.callback)(self.arena, arguments)
            .await
            .map_err(|kind| Error::new(kind, span))
        }
        FunctionKind::User(user_fn) => {
          self
            .eval_user_function_call_with_this(user_fn, arguments, this, span)
            .await
        }
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
    let new_interpreter = self.new_enclosed();

    // Set 'this' and metadata in the new scope
    if !matches!(this, ObjectKind::Void) {
      let mut env = new_interpreter.environment.borrow_mut();
      env.set("this", this.clone(), false, true);

      let struct_name = match &this {
        ObjectKind::StructInstance { definition, .. } => Some(definition.name),
        ObjectKind::Struct(definition) => Some(definition.name),
        _ => None,
      };

      if let Some(name) = struct_name {
        env.set("__current_struct__", ObjectKind::String(name), false, true);
      }
    }

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

    let result = match &user_fn.body {
      FunctionBody::Block(block) => {
        new_interpreter.eval_statements(&block.statements).await
      }
      FunctionBody::Expression(expr) => {
        new_interpreter.eval_expression(expr, None).await
      }
    }?;

    if let ObjectKind::Return(value) = result {
      if let ObjectKind::Err(_) = value {
        return Ok(ObjectKind::Return(value));
      }
      Ok(value.clone())
    } else {
      Ok(result)
    }
  }
}
