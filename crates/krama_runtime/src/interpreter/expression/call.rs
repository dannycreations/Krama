use std::sync::Arc;

use futures::future::try_join_all;
use krama_core::{
  Error, ErrorKind, Expression, ExpressionKind, FunctionBody, FunctionKind,
  ObjectKind, ObjectResult, Parameter, Span, StructMethod, Type, UserFunction,
};

use crate::interpreter::{types::check_type, Interpreter};

impl Interpreter {
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

        let bound_this = if let ObjectKind::Function(FunctionKind::User {
          this: Some(ref t),
          ..
        }) = func
        {
          t.as_ref().clone()
        } else {
          obj_val
        };

        (func, bound_this)
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
        FunctionKind::User {
          func,
          env,
          this: bound_this,
        } => {
          let effective_this = bound_this
            .as_ref()
            .map(|t| t.as_ref().clone())
            .unwrap_or(this);
          self
            .eval_user_function_call_with_this(
              &func,
              env.clone(),
              arguments,
              effective_this,
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
          Ok(ObjectKind::Enum(Box::new(krama_core::EnumInstance {
            name: constructor.name.clone(),
            variant: constructor.variant.clone(),
            fields: Some(Arc::from(arguments)),
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

  pub async fn eval_user_function_call_with_this(
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

    let stack_ref = &self.stack;

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

    stack_ref.write().pop();

    Ok(result?.unwrap_return().clone())
  }

  pub fn alloc_user_function(
    &self,
    parameters: Vec<Parameter>,
    body: FunctionBody,
    kind: Option<Type>,
  ) -> ObjectKind {
    let user_fn = Arc::new(UserFunction {
      parameters,
      body,
      kind,
    });

    let env = Some(self.stack.read().current());

    ObjectKind::Function(FunctionKind::User {
      func: user_fn,
      env,
      this: None,
    })
  }

  pub fn from_method(method: &StructMethod) -> ObjectKind {
    ObjectKind::Function(FunctionKind::User {
      func: Arc::new(UserFunction {
        parameters: method.parameters.clone(),
        body: method.body.clone(),
        kind: method.kind.clone(),
      }),
      env: None,
      this: None,
    })
  }

  pub fn bind_method(
    method: &StructMethod,
    instance: ObjectKind,
  ) -> ObjectKind {
    ObjectKind::Function(FunctionKind::User {
      func: Arc::new(UserFunction {
        parameters: method.parameters.clone(),
        body: method.body.clone(),
        kind: method.kind.clone(),
      }),
      env: None,
      this: Some(Arc::new(instance)),
    })
  }
}
