use std::rc::Rc;

use bumpalo::collections::Vec as BumpVec;
use futures::future::LocalBoxFuture;
use krama_core::{
  ast::{
    expression::FunctionBody,
    statement::{
      Binding, BlockStatement, DestructuredIdentifier, Statement, StatementKind,
    },
  },
  error::{Error, ErrorKind},
  object::{Function, Object, UserFunction},
  span::Span,
};
use tokio::task;

use super::{types::check_type, Interpreter};

impl<'ast> Interpreter<'ast> {
  pub fn eval_statement<'s>(
    &'s self,
    statement: &'s Statement<'ast>,
  ) -> LocalBoxFuture<'s, Result<Object<'ast>, Error>> {
    Box::pin(async move {
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

          self.env_mut(span)?.set(name, Rc::new(value), false);
          Ok(Object::Void)
        }
        StatementKind::Test { name: _, body } => {
          let function =
            Object::Function(Function::User(Rc::new(UserFunction {
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
              self.env_mut(span)?.set(name, Rc::new(value), *public);
            }
            Binding::Destructure(items) => {
              self.destructure_scope(span, &value, items, *public)?;
            }
            Binding::ModuleAndDestructure {
              module_alias,
              items,
            } => {
              if let Object::Scope(_) = &value {
                self.env_mut(span)?.set(
                  module_alias,
                  Rc::new(value.clone()),
                  *public,
                );
                self.destructure_scope(span, &value, items, *public)?;
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
          let function =
            Object::Function(Function::User(Rc::new(UserFunction {
              parameters: parameters.clone(),
              body: body.clone(),
              kind: kind.clone(),
            })));
          self.env_mut(span)?.set(name, Rc::new(function), *public);
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
    })
  }

  async fn eval_statements<'s>(
    &'s self,
    statements: &'s [Statement<'ast>],
  ) -> Result<Object<'ast>, Error> {
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

  pub(super) async fn eval_block_statement(
    &self,
    block: &BlockStatement<'ast>,
  ) -> Result<Object<'ast>, Error> {
    self.eval_statements(&block.statements).await
  }

  fn destructure_scope(
    &self,
    span: Span,
    value: &Object<'ast>,
    items: &BumpVec<'ast, DestructuredIdentifier<'ast>>,
    public: bool,
  ) -> Result<(), Error> {
    if let Object::Scope(scope) = value {
      for item in items.iter() {
        if let Some(export) = scope.bindings.get(item.name) {
          let name = item.alias.unwrap_or(item.name);
          self.env_mut(span)?.set(name, export.clone(), public);
        } else {
          return Err(Error {
            span,
            kind: ErrorKind::ReferenceError(format!(
              "'{}' is not exported from module '{}'",
              item.name,
              scope.name.unwrap_or("<anonymous>")
            )),
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
    Ok(())
  }
}
