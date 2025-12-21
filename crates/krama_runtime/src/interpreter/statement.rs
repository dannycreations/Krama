use std::sync::Arc;

use futures::future::{FutureExt, LocalBoxFuture};
use krama_core::{
  Binding, BlockStatement, Error, ErrorKind, ForBinding, Function, Object,
  Statement, StatementKind, UserFunction,
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
      let span = statement.span;
      match &statement.kind {
        StatementKind::Expression { expression } => {
          self.eval_expression(expression, None).await
        }
        StatementKind::Let { name, value, kind } => {
          let value = self.eval_expression(value, kind.as_ref()).await?;

          if let Some(kind) = kind {
            check_type(kind, &value)?;
          }

          let mut value = value;
          match &mut value {
            Object::Array { constant, .. } => {
              *constant = false;
            }
            Object::Object { constant, .. } => {
              *constant = false;
            }
            _ => {}
          }

          self.env_mut(span)?.set(name, value, false, false);
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

          let mut value = value;
          match &mut value {
            Object::Array { constant, .. } => {
              *constant = true;
            }
            Object::Object { constant, .. } => {
              *constant = true;
            }
            _ => {}
          }

          match binding {
            Binding::Identifier(name) => {
              self.env_mut(span)?.set(name, value, *public, true);
            }
            Binding::Destructure(items) => {
              if let Object::Scope(scope) = &value {
                for item in items.iter() {
                  if let Some(export) = scope.get_binding(item.name) {
                    let name = item.alias.unwrap_or(item.name);
                    self.env_mut(span)?.set(
                      name,
                      export.clone(),
                      *public,
                      true,
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
                self.env_mut(span)?.set(alias, value.clone(), *public, true);
                for item in items.iter() {
                  if let Some(export) = scope.get_binding(item.name) {
                    let name = item.alias.unwrap_or(item.name);
                    self.env_mut(span)?.set(
                      name,
                      export.clone(),
                      *public,
                      true,
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
          self.env_mut(span)?.set(name, function, *public, true);
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
          binding,
          iterable,
          body,
        } => {
          let iterable_value = self.eval_expression(iterable, None).await?;
          let elements = match &iterable_value {
            Object::Array { elements, .. } => elements.read().to_vec(),
            Object::Tuple { elements } => elements.to_vec(),
            Object::String(s) => s
              .chars()
              .map(|c| Object::String(self.arena.alloc_str(&c.to_string())))
              .collect(),
            Object::Object { properties, .. } => {
              let props = properties.read();
              let mut yields = Vec::new();

              match binding {
                ForBinding::Identifier(_) => {
                  // If single identifier, yield keys
                  for &k in props.keys() {
                    yields.push(Object::String(k));
                  }
                }
                ForBinding::Array(bindings) if bindings.len() == 2 => {
                  // If [k, v], yield [key, value] tuples
                  for (k, v) in props.iter() {
                    let key = Object::String(k);
                    let value = v.clone();
                    let mut elements =
                      bumpalo::collections::Vec::new_in(self.arena);
                    elements.push(key);
                    elements.push(value);
                    yields.push(Object::Tuple {
                      elements: elements.into_bump_slice(),
                    });
                  }
                }
                _ => {
                  // Default to keys for other patterns, or maybe error?
                  // For now, let's yield keys.
                  for &k in props.keys() {
                    yields.push(Object::String(k));
                  }
                }
              }
              yields
            }
            _ => {
              return Err(Error::new(
                ErrorKind::TypeError(format!(
                  "Expected array, tuple, string or object for for..in loop, found {}",
                  iterable_value.type_name()
                )),
                span,
              ));
            }
          };

          for element in elements {
            let new_interpreter = self.new_enclosed();
            self.assign_for_binding(&new_interpreter, binding, element, span)?;

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

  fn assign_for_binding(
    &self,
    interpreter: &Interpreter<'ast>,
    binding: &ForBinding<'ast>,
    value: Object<'ast>,
    span: krama_core::Span<'ast>,
  ) -> Result<(), Error<'ast>> {
    match binding {
      ForBinding::Identifier(name) => {
        interpreter.environment.borrow_mut().set(
          name,
          value.clone(),
          false,
          false,
        );
        Ok(())
      }
      ForBinding::Array(bindings) => {
        let elements = match &value {
          Object::Array { elements, .. } => elements.read().to_vec(),
          Object::Tuple { elements } => elements.to_vec(),
          _ => {
            return Err(Error::new(
              ErrorKind::TypeError(format!(
                "Expected array or tuple for destructuring, found {}",
                value.type_name()
              )),
              span,
            ));
          }
        };

        for (i, binding) in bindings.iter().enumerate() {
          let val = elements.get(i).cloned().unwrap_or(Object::Void);
          self.assign_for_binding(interpreter, binding, val, span)?;
        }
        Ok(())
      }
    }
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
