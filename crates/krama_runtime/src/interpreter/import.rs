use std::{cell::RefCell, rc::Rc};

use krama_core::{
  error::{Error, ErrorKind},
  object::{ModuleObject, Object},
  span::Span,
};
use rustc_hash::FxHashMap;
use tokio::fs;

use super::Interpreter;

impl<'ast> Interpreter<'ast> {
  pub(super) async fn eval_import(
    &self,
    path: &'ast str,
    _span: Span,
  ) -> Result<Object<'ast>, Error> {
    let resolver = self.resolver.clone();
    resolver.resolve(self, path).await
  }

  pub async fn eval_and_cache(
    &self,
    path: &'ast str,
  ) -> Result<Object<'ast>, Error> {
    if let Ok(modules) = self.modules.try_borrow() {
      if let Some(module) = modules.get(path) {
        return Ok(module.clone());
      }
    } else {
      return Err(Error {
        span: Default::default(),
        kind: ErrorKind::RuntimeError(
          "Failed to borrow modules cache".to_string(),
        ),
      });
    }

    let source = fs::read_to_string(path).await.map_err(|e| Error {
      span: Default::default(),
      kind: ErrorKind::ReferenceError(format!(
        "Failed to read module file: {}",
        e
      )),
    })?;
    let source_str = self.arena.alloc_str(&source);

    let new_interpreter = Interpreter::new(self.arena, Some(path));
    let _ = new_interpreter.eval(source_str).await?;

    let exports: FxHashMap<_, _> = new_interpreter
      .environment
      .try_borrow()
      .map_err(|e| Error {
        span: Default::default(),
        kind: ErrorKind::RuntimeError(e.to_string()),
      })?
      .get_public_bindings()
      .into_iter()
      .collect();

    let module = Object::Module(Rc::new(RefCell::new(ModuleObject {
      name: self.arena.alloc_str(path),
      exports: exports.into_iter().map(|(k, v)| (k, v.clone())).collect(),
    })));

    self
      .modules
      .try_borrow_mut()
      .map_err(|e| Error {
        span: Default::default(),
        kind: ErrorKind::RuntimeError(e.to_string()),
      })?
      .insert(path.to_string(), module.clone());

    Ok(module)
  }
}
