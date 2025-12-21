use std::sync::Arc;

use krama_core::{
  Error, ErrorKind, Function, FunctionBody, Object, Span, UserFunction,
};

use super::{types::check_type, Interpreter};

impl<'ast> Interpreter<'ast> {
  pub async fn eval_call_expression(
    &self,
    function: Object<'ast>,
    arguments: &'ast [Object<'ast>],
    span: Span,
  ) -> Result<Object<'ast>, Error<'ast>> {
    match function {
      Object::Function(function) => match function {
        Function::Native(native_fn) => {
          (native_fn.callback)(self.arena, arguments)
            .await
            .map_err(|kind| Error::new(kind, span))
        }
        Function::User(user_fn) => {
          self.eval_user_function_call(user_fn, arguments, span).await
        }
        Function::Enum(constructor) => {
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
          Ok(Object::Enum {
            name: constructor.name,
            variant: constructor.variant,
            fields: Some(arguments),
          })
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

  async fn eval_user_function_call(
    &self,
    user_fn: &'ast UserFunction<'ast>,
    arguments: &'ast [Object<'ast>],
    span: Span,
  ) -> Result<Object<'ast>, Error<'ast>> {
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
        new_interpreter
          .eval_program_statements(&block.statements)
          .await
      }
      FunctionBody::Expression(expr) => {
        new_interpreter.eval_expression(expr, None).await
      }
    }?;

    if let Object::Return(value) = result {
      let object = Arc::unwrap_or_clone(value);
      Ok(object)
    } else {
      Ok(result)
    }
  }
}
