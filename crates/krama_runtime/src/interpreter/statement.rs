use std::rc::Rc;

use bumpalo::collections::Vec as BumpVec;
use futures::future::{FutureExt, LocalBoxFuture};
use krama_core::{
  ast::{
    expression::FunctionBody,
    statement::{Binding, BlockStatement, Statement, StatementKind},
  },
  error::{Error, ErrorKind},
  object::{Function, Object, UserFn},
};
use tokio::task;

use super::{types::check_type, Interpreter};

impl<'ast> Interpreter<'ast> {
  pub(super) fn eval_statement<'s>(
    &'s self,
    statement: &'s Statement<'ast>,
  ) -> LocalBoxFuture<'s, Result<Object<'ast>, Error>> {
    async move {
      let span = statement.span;
      match &statement.kind {
        StatementKind::Expression { expression } => {
          let value = self.eval_expression(expression, None).await?;
          self.resolve_object(value).await
        }
        StatementKind::Let { name, value, kind } => {
          let value = self.eval_expression(value, kind.as_ref()).await?;
          let value = self.resolve_object(value).await?;

          if let Some(kind) = kind {
            check_type(kind, &value)?;
          }

          self
            .environment
            .try_borrow_mut()
            .map_err(|e| Error {
              span,
              kind: ErrorKind::RuntimeError(e.to_string()),
            })?
            .set(name, value, false);
          Ok(Object::Void)
        }
        StatementKind::Test { name: _, body } => {
          let function = Object::Function(Function::User(Rc::new(UserFn {
            parameters: BumpVec::new_in(self.arena),
            body: FunctionBody::Block(body),
            kind: None,
          })));
          self
            .eval_call_expression(function, BumpVec::new_in(self.arena), span)
            .await
        }
        StatementKind::Const {
          binding,
          value,
          public,
          kind,
        } => {
          let value = self.eval_expression(value, kind.as_ref()).await?;
          let value = self.resolve_object(value).await?;

          if let Some(kind) = kind {
            check_type(kind, &value)?;
          }

          match binding {
            Binding::Identifier(name) => {
              self
                .environment
                .try_borrow_mut()
                .map_err(|e| Error {
                  span,
                  kind: ErrorKind::RuntimeError(e.to_string()),
                })?
                .set(name, value, *public);
            }
            Binding::Destructure(items) => {
              if let Object::Module(module) = value {
                let module = module.try_borrow().map_err(|e| Error {
                  span,
                  kind: ErrorKind::RuntimeError(e.to_string()),
                })?;
                for item in items.iter() {
                  if let Some(export) = module.exports.get(item.name) {
                    let name = item.alias.unwrap_or(item.name);
                    self
                      .environment
                      .try_borrow_mut()
                      .map_err(|e| Error {
                        span,
                        kind: ErrorKind::RuntimeError(e.to_string()),
                      })?
                      .set(name, export.clone(), *public);
                  } else {
                    return Err(Error {
                      span,
                      kind: ErrorKind::ReferenceError(item.name.to_string()),
                    });
                  }
                }
              } else {
                return Err(Error {
                  span,
                  kind: ErrorKind::TypeError(
                    "Destructuring can only be done on modules".to_string(),
                  ),
                });
              }
            }
            Binding::ModuleAndDestructure {
              module_alias,
              items,
            } => {
              if let Object::Module(module_obj) = &value {
                self
                  .environment
                  .try_borrow_mut()
                  .map_err(|e| Error {
                    span,
                    kind: ErrorKind::RuntimeError(e.to_string()),
                  })?
                  .set(module_alias, value.clone(), *public);
                let module = module_obj.try_borrow().map_err(|e| Error {
                  span,
                  kind: ErrorKind::RuntimeError(e.to_string()),
                })?;
                for item in items.iter() {
                  if let Some(export) = module.exports.get(item.name) {
                    let name = item.alias.unwrap_or(item.name);
                    self
                      .environment
                      .try_borrow_mut()
                      .map_err(|e| Error {
                        span,
                        kind: ErrorKind::RuntimeError(e.to_string()),
                      })?
                      .set(name, export.clone(), *public);
                  } else {
                    return Err(Error {
                      span,
                      kind: ErrorKind::ReferenceError(item.name.to_string()),
                    });
                  }
                }
              } else {
                return Err(Error {
                  span,
                  kind: ErrorKind::TypeError(
                    "Destructuring can only be done on modules".to_string(),
                  ),
                });
              }
            }
          }
          Ok(Object::Void)
        }
        StatementKind::Fn {
          name,
          parameters,
          body,
          public,
          kind,
        } => {
          let function = Object::Function(Function::User(Rc::new(UserFn {
            parameters: parameters.clone(),
            body: FunctionBody::Block(body),
            kind: kind.clone(),
          })));
          self
            .environment
            .try_borrow_mut()
            .map_err(|e| Error {
              span,
              kind: ErrorKind::RuntimeError(e.to_string()),
            })?
            .set(name, function, *public);
          Ok(Object::Void)
        }
        StatementKind::Return { value } => {
          let value = match value {
            Some(expression) => self.eval_expression(expression, None).await?,
            None => Object::Void,
          };
          let value = self.resolve_object(value).await?;
          Ok(Object::Return(Rc::new(value)))
        }
        StatementKind::Break => Ok(Object::Break),
        StatementKind::Continue => Ok(Object::Continue),
        StatementKind::While { condition, body } => {
          loop {
            task::yield_now().await;
            let condition_result =
              self.eval_expression(condition, None).await?;
            let condition_result =
              self.resolve_object(condition_result).await?;
            if !condition_result.is_truthy() {
              break;
            }
            let result = self.eval_block_statement(body).await?;
            if matches!(result, Object::Return(_)) {
              return Ok(result);
            }
            if matches!(result, Object::Break) {
              break;
            }
            if matches!(result, Object::Continue) {
              continue;
            }
          }
          Ok(Object::Void)
        }
      }
    }
    .boxed_local()
  }

  fn eval_statements<'s>(
    &'s self,
    statements: &'s [Statement<'ast>],
  ) -> LocalBoxFuture<'s, Result<Object<'ast>, Error>> {
    async move {
      let mut result = Object::Void;

      for statement in statements {
        result = self.eval_statement(statement).await?;

        if matches!(
          &result,
          Object::Return(_) | Object::Break | Object::Continue
        ) {
          return Ok(result);
        }
      }

      Ok(result)
    }
    .boxed_local()
  }

  pub(super) async fn eval_block_statement(
    &self,
    block: &BlockStatement<'ast>,
  ) -> Result<Object<'ast>, Error> {
    self.eval_statements(&block.statements).await
  }
}
