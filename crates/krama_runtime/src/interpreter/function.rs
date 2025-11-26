use std::cell::RefCell;
use std::rc::Rc;

use bumpalo::collections::Vec as BumpVec;
use futures::FutureExt;
use krama_core::ast::expression::FunctionBody;
use krama_core::error::{Error, ErrorKind};
use krama_core::object::{Function, Object, ObjectFuture};

use super::types::check_type;
use super::Interpreter;
use crate::environment::Environment;

impl<'ast> Interpreter<'ast> {
  pub(super) async fn eval_call_expression(
    &self,
    function: Object<'ast>,
    arguments: BumpVec<'ast, Object<'ast>>,
    span: krama_core::span::Span,
  ) -> Result<Object<'ast>, Error> {
    match function {
      Object::Function(function) => match function {
        Function::Native(function) => {
          let future = (function.callback)(self.arena, arguments);
          let object_future: ObjectFuture =
            Rc::new(RefCell::new(Some(future.boxed_local())));
          Ok(Object::Future(object_future))
        }
        Function::User(function) => {
          let mut new_interpreter = self.clone();
          let future = async move {
            if function.parameters.len() != arguments.len() {
              return Err(Error {
                span,
                kind: ErrorKind::TypeError(format!(
                  "Expected {} arguments, but got {}",
                  function.parameters.len(),
                  arguments.len()
                )),
              });
            }
            let new_env = Rc::new(RefCell::new(Environment::new_enclosed(
              new_interpreter.environment.clone(),
            )));
            for (param, arg) in function.parameters.iter().zip(arguments) {
              if let Some(kind) = &param.kind {
                check_type(kind, &arg)?;
              }
              new_env
                .try_borrow_mut()
                .unwrap()
                .set(param.name, arg, false);
            }
            new_interpreter.environment = new_env;

            let result = match &function.body {
              FunctionBody::Block(block) => {
                new_interpreter.eval_block_statement(block).await
              }
              FunctionBody::Expression(expr) => {
                new_interpreter
                  .eval_expression(expr, function.kind.as_ref())
                  .await
              }
            };

            if let Ok(Object::Return(value)) = result {
              if let Some(kind) = &function.kind {
                check_type(kind, value)?;
              }
              return Ok((*value).clone());
            }
            result
          };
          let object_future: ObjectFuture =
            Rc::new(RefCell::new(Some(future.boxed_local())));
          Ok(Object::Future(object_future))
        }
      },
      _ => Err(Error {
        span,
        kind: ErrorKind::TypeError("".to_string()),
      }),
    }
  }
}
