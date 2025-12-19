use std::sync::Arc;

use futures::future::{FutureExt, LocalBoxFuture};
use krama_core::{
  Binding, BlockStatement, Error, ErrorKind, Function, Object, Statement,
  StatementKind, UserFunction,
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
              if let Object::Scope(scope) = &value {
                for item in items.iter() {
                  if let Some(export) = scope.bindings.get(item.name) {
                    let name = item.alias.unwrap_or(item.name);
                    self.env_mut(span.clone())?.set(
                      name,
                      export.clone(),
                      *public,
                    );
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
            }
            Binding::ModuleAndDestructure { alias, items } => {
              if let Object::Scope(scope) = &value {
                self
                  .env_mut(span.clone())?
                  .set(alias, value.clone(), *public);
                for item in items.iter() {
                  if let Some(export) = scope.bindings.get(item.name) {
                    let name = item.alias.unwrap_or(item.name);
                    self.env_mut(span.clone())?.set(
                      name,
                      export.clone(),
                      *public,
                    );
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
          #[allow(clippy::arc_with_non_send_sync)]
          Ok(Object::Return(Arc::new(value)))
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
        StatementKind::For {
          name,
          iterable,
          body,
        } => {
          let iterable_value = self.eval_expression(iterable, None).await?;
          let elements: &[Object<'ast>] = match &iterable_value {
            Object::Array { elements, .. } => elements,
            Object::Tuple { elements } => elements,
            _ => {
              return Err(Error::new(
                ErrorKind::TypeError(format!(
                  "Expected array or tuple for for..in loop, found {}",
                  iterable_value.type_name()
                )),
                span,
              ));
            }
          };

          for element in elements {
            let new_interpreter = self.new_enclosed();
            new_interpreter.environment.borrow_mut().set(
              name,
              element.clone(),
              false,
            );

            let result = new_interpreter.eval_block_statement(body).await?;

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

  pub async fn eval_block_statement(
    &self,
    block: &BlockStatement<'ast>,
  ) -> Result<Object<'ast>, Error<'ast>> {
    self.eval_statements(&block.statements).await
  }

  pub async fn eval_block_statement_with_new_scope(
    &self,
    block: &BlockStatement<'ast>,
  ) -> Result<Object<'ast>, Error<'ast>> {
    let new_interpreter = self.new_enclosed();
    new_interpreter.eval_statements(&block.statements).await
  }
}
