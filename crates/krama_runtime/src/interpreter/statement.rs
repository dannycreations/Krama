use std::rc::Rc;

use bumpalo::collections::Vec as BumpVec;
use futures::future::{FutureExt, LocalBoxFuture};
use krama_core::{
  ast::statement::{
    Binding, BlockStatement, DestructuredIdentifier, Statement, StatementKind,
  },
  error::{Error, ErrorKind},
  object::{Function, Object, UserFunction},
  span::Span,
};

use super::{types::check_type, Interpreter};

impl<'ast> Interpreter<'ast> {
  pub fn eval_statement<'s>(
    &'s self,
    statement: &'s Statement<'ast>,
  ) -> LocalBoxFuture<'s, Result<Object<'ast>, Error<'ast>>>
  where
    'ast: 's,
  {
    async move {
      let span = statement.span.clone();
      match &statement.kind {
        StatementKind::Expression { expression } => {
          self.eval_expression(expression, None).await
        }
        StatementKind::Let { name, value, kind } => {
          let value = self.eval_expression(value, kind.as_ref()).await?;

          if let Some(kind) = kind {
            check_type(kind, &value)?;
          }

          self.env_mut(span)?.set(name, value, false);
          Ok(Object::Void)
        }
        StatementKind::Test { name: _, body } => {
          self.eval_block_statement_with_new_scope(body).await
        }
        StatementKind::Const {
          binding,
          value,
          public,
          kind,
        } => {
          let value = self.eval_expression(value, kind.as_ref()).await?;

          if let Some(kind) = kind {
            check_type(kind, &value)?;
          }

          match binding {
            Binding::Identifier(name) => {
              self.env_mut(span)?.set(name, value, *public);
            }
            Binding::Destructure(items) => {
              self.destructure_scope(span, &value, items, *public)?;
            }
            Binding::ModuleAndDestructure {
              module_alias,
              items,
            } => {
              if let Object::Scope(_) = &value {
                self.env_mut(span.clone())?.set(
                  module_alias,
                  value.clone(),
                  *public,
                );
                self.destructure_scope(span, &value, items, *public)?;
              } else {
                return Err(Error::new(
                  ErrorKind::TypeError(
                    "Destructuring can only be done on modules".to_string(),
                  ),
                  span,
                ));
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
            Object::Function(Function::User(self.arena.alloc(UserFunction {
              parameters: parameters.clone(),
              body: body.clone(),
              kind: kind.clone(),
            })));
          self.env_mut(span)?.set(name, function, *public);
          Ok(Object::Void)
        }
        StatementKind::Return { value } => {
          let value = match value {
            Some(expression) => self.eval_expression(expression, None).await?,
            None => Object::Void,
          };
          Ok(Object::Return(Rc::new(value)))
        }
        StatementKind::Break => Ok(Object::Break),
        StatementKind::Continue => Ok(Object::Continue),
        StatementKind::While { condition, body } => {
          loop {
            let condition_result =
              self.eval_expression(condition, None).await?;
            if !bool::from(&condition_result) {
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

  async fn eval_statements<'s>(
    &'s self,
    statements: &'s [Statement<'ast>],
  ) -> Result<Object<'ast>, Error<'ast>> {
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
  ) -> Result<Object<'ast>, Error<'ast>> {
    self.eval_statements(&block.statements).await
  }

  pub(super) async fn eval_block_statement_with_new_scope(
    &self,
    block: &BlockStatement<'ast>,
  ) -> Result<Object<'ast>, Error<'ast>> {
    let new_interpreter = self.new_enclosed();
    new_interpreter.eval_statements(&block.statements).await
  }

  fn destructure_scope(
    &self,
    span: Span<'ast>,
    value: &Object<'ast>,
    items: &BumpVec<'ast, DestructuredIdentifier<'ast>>,
    public: bool,
  ) -> Result<(), Error<'ast>> {
    if let Object::Scope(scope) = value {
      for item in items.iter() {
        if let Some(export) = scope.bindings.get(item.name) {
          let name = item.alias.unwrap_or(item.name);
          self
            .env_mut(span.clone())?
            .set(name, export.clone(), public);
        } else {
          return Err(Error::new(
            ErrorKind::ReferenceError(format!(
              "'{}' is not exported from module '{}'",
              item.name,
              scope.name.unwrap_or("<anonymous>")
            )),
            span,
          ));
        }
      }
    } else {
      return Err(Error::new(
        ErrorKind::TypeError(
          "Destructuring can only be done on modules".to_string(),
        ),
        span,
      ));
    }
    Ok(())
  }
}
