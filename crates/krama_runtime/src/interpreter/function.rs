use std::rc::Rc;

use bumpalo::collections::Vec as BumpVec;
use futures::future::LocalBoxFuture;
use krama_core::{
  ast::expression::FunctionBody,
  error::{Error, ErrorKind},
  object::{Function, NativeFunction, Object, UserFunction},
  span::Span,
};

use crate::interpreter::Interpreter;

impl<'ast> Interpreter<'ast> {
  pub(super) fn eval_call_expression<'s>(
    &'s self,
    function: Object<'ast>,
    arguments: BumpVec<'ast, Object<'ast>>,
    span: Span,
  ) -> LocalBoxFuture<'s, Result<Object<'ast>, Error>> {
    Box::pin(async move {
      match function {
        Object::Function(function) => match function {
          Function::Native(native_fn) => {
            self.eval_native_function_call(native_fn, arguments).await
          }
          Function::User(user_fn) => {
            self.eval_user_function_call(user_fn, arguments, span).await
          }
        },
        _ => Err(Error {
          span,
          kind: ErrorKind::TypeError(format!(
            "Expected a function, but got {}",
            function.type_name()
          )),
        }),
      }
    })
  }

  async fn eval_native_function_call(
    &self,
    native_fn: NativeFunction<'ast>,
    arguments: BumpVec<'ast, Object<'ast>>,
  ) -> Result<Object<'ast>, Error> {
    (native_fn.callback)(self.arena, arguments).await
  }

  async fn eval_user_function_call(
    &self,
    user_fn: Rc<UserFunction<'ast>>,
    arguments: BumpVec<'ast, Object<'ast>>,
    span: Span,
  ) -> Result<Object<'ast>, Error> {
    if arguments.len() > user_fn.parameters.len() {
      return Err(Error {
        span,
        kind: ErrorKind::TypeError(format!(
          "Expected {} arguments, but got {}",
          user_fn.parameters.len(),
          arguments.len()
        )),
      });
    }
    let new_interpreter = self.new_enclosed();

    for (i, param) in user_fn.parameters.iter().enumerate() {
      let value = if let Some(arg) = arguments.get(i) {
        arg.clone()
      } else if let Some(default) = param.default {
        new_interpreter.eval_expression(default, None).await?
      } else {
        return Err(Error {
          span,
          kind: ErrorKind::TypeError(format!(
            "Missing argument for parameter '{}'",
            param.name
          )),
        });
      };
      new_interpreter.environment.borrow_mut().set(
        param.name,
        Rc::new(value.clone()),
        false,
      );
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
      Ok((*value).clone())
    } else {
      Ok(result)
    }
  }
}
