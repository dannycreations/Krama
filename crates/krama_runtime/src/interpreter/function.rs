use krama_core::{
  ast::expression::FunctionBody,
  error::ErrorKind,
  object::{Function, Object, UserFunction},
  span::Span,
};

use crate::interpreter::Interpreter;

impl<'ast> Interpreter<'ast> {
  pub(super) async fn eval_call_expression(
    &self,
    function: Object<'ast>,
    arguments: &'ast [Object<'ast>],
    span: Span<'ast>,
  ) -> Result<Object<'ast>, (ErrorKind, Span<'ast>)> {
    match function {
      Object::Function(function) => match function {
        Function::Native(native_fn) => {
          (native_fn.callback)(self.arena, arguments)
            .await
            .map_err(|kind| (kind, span))
        }
        Function::User(user_fn) => {
          self.eval_user_function_call(user_fn, arguments, span).await
        }
      },
      _ => Err((
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
    span: Span<'ast>,
  ) -> Result<Object<'ast>, (ErrorKind, Span<'ast>)> {
    if arguments.len() > user_fn.parameters.len() {
      return Err((
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
        return Err((
          ErrorKind::TypeError(format!(
            "Missing argument for parameter '{}'",
            param.name
          )),
          span,
        ));
      };
      new_interpreter
        .environment
        .borrow_mut()
        .set(param.name, value, false);
    }

    let result = match &user_fn.body {
      FunctionBody::Block(block) => {
        new_interpreter.eval_block_statement(block).await?
      }
      FunctionBody::Expression(expr) => {
        new_interpreter.eval_expression(expr, None).await?
      }
    };

    if let Object::Return(value) = result {
      Ok(value.clone())
    } else {
      Ok(result)
    }
  }
}
