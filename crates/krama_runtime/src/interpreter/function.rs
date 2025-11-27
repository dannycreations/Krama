use std::rc::Rc;

use bumpalo::collections::Vec as BumpVec;
use futures::future::LocalBoxFuture;
use krama_core::{
  ast::expression::FunctionBody,
  error::{Error, ErrorKind},
  object::{Function, NativeFn, Object, UserFn},
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
            self.eval_user_function_call(user_fn, arguments).await
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
    native_fn: NativeFn<'ast>,
    arguments: BumpVec<'ast, Object<'ast>>,
  ) -> Result<Object<'ast>, Error> {
    (native_fn.callback)(self.arena, arguments).await
  }

  async fn eval_user_function_call(
    &self,
    user_fn: Rc<UserFn<'ast>>,
    arguments: BumpVec<'ast, Object<'ast>>,
  ) -> Result<Object<'ast>, Error> {
    let new_interpreter = self.new_enclosed();

    for (i, param) in user_fn.parameters.iter().enumerate() {
      let value = arguments.get(i).unwrap_or(&Object::Null);
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
      Ok(value.clone())
    } else {
      Ok(result)
    }
  }
}
