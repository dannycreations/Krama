use std::{cell::RefCell, rc::Rc};

use bumpalo::collections::Vec as BumpVec;
use futures::FutureExt;
use krama_core::{
  ast::expression::FunctionBody,
  error::{Error, ErrorKind},
  object::{Function, Object, ObjectFuture},
  span::Span,
};

use super::{types::check_type, Interpreter};
use crate::environment::Environment;

impl<'ast> Interpreter<'ast> {
  pub(super) async fn eval_call_expression(
    &self,
    function: Object<'ast>,
    arguments: BumpVec<'ast, Object<'ast>>,
    span: Span,
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
                .map_err(|e| Error {
                  span,
                  kind: ErrorKind::RuntimeError(e.to_string()),
                })?
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
                check_type(kind, &value)?;
              }
              return Ok(value.as_ref().clone());
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
        kind: ErrorKind::TypeError(format!(
          "Object of type '{}' is not callable",
          function.type_name()
        )),
      }),
    }
  }
}
