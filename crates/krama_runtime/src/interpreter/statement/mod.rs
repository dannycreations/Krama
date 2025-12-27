use futures::future::{FutureExt, LocalBoxFuture};
use krama_core::{ObjectKind, ObjectResult, Statement, StatementKind};

use crate::interpreter::Interpreter;

mod binding;
mod control;
mod declaration;
mod iteration;
mod utils;

impl Interpreter {
  /// Evaluates a single statement.
  pub fn eval_statement<'s>(
    &'s self,
    statement: &'s Statement,
  ) -> LocalBoxFuture<'s, ObjectResult> {
    async move {
      let span = statement.span;
      match &statement.kind {
        StatementKind::Expression { expression } => {
          self.eval_expression(expression, None).await
        }
        StatementKind::Let { name, value, kind } => {
          self.eval_let_statement(name, value, kind.as_ref()).await
        }
        StatementKind::Const {
          binding,
          value,
          public,
          kind,
        } => {
          self
            .eval_const_statement(binding, value, *public, kind.as_ref(), span)
            .await
        }
        StatementKind::Fn {
          name,
          parameters,
          body,
          public,
          kind,
        } => {
          self
            .eval_fn_statement(name, parameters, body, *public, kind.as_ref())
            .await
        }
        StatementKind::Enum {
          public,
          name,
          variants,
        } => self.eval_enum_statement(name, variants, *public).await,
        StatementKind::Struct {
          public,
          name,
          fields,
          methods,
        } => {
          self
            .eval_struct_statement(name, fields, methods, *public)
            .await
        }
        StatementKind::Type { public, name, kind } => {
          self.eval_type_statement(name, kind, *public).await
        }
        StatementKind::Return { value } => {
          self.eval_return_statement(value.as_deref()).await
        }
        StatementKind::Break => Ok(ObjectKind::Break),
        StatementKind::Continue => Ok(ObjectKind::Continue),
        StatementKind::While { condition, body } => {
          self.eval_while_statement(condition, body).await
        }
        StatementKind::For {
          binding,
          iterable,
          body,
        } => self.eval_for_statement(binding, iterable, body, span).await,
        StatementKind::Test { body, .. } => {
          self.eval_block_statement_with_new_scope(body).await
        }
      }
    }
    .boxed_local()
  }

  pub async fn eval_statements<'s>(
    &'s self,
    statements: &'s [Statement],
  ) -> ObjectResult {
    let mut result = ObjectKind::Void;
    for statement in statements {
      result = self.eval_statement(statement).await?;
      if result.is_control_signal() {
        return Ok(result);
      }
    }
    Ok(result)
  }

  pub async fn eval_block_statement(
    &self,
    block: &krama_core::StatementBlock,
  ) -> ObjectResult {
    self.eval_statements(&block.statements).await
  }

  pub async fn eval_block_statement_with_new_scope(
    &self,
    block: &krama_core::StatementBlock,
  ) -> ObjectResult {
    self.stack.write().push("block".into(), None);
    let result = self.eval_statements(&block.statements).await;
    self.stack.write().pop();
    result
  }
}
