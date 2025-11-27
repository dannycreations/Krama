use std::{cell::RefCell, rc::Rc};

use krama_core::{
  error::{Error, ErrorKind},
  object::{Object, Scope},
  span::Span,
};
use krama_std::modules;
use rustc_hash::FxHashMap;
use tokio::fs;

use super::Interpreter;

impl<'ast> Interpreter<'ast> {
  pub(super) async fn eval_import(
    &self,
    path: &'ast str,
    _span: Span,
  ) -> Result<Object<'ast>, Error> {
    if path.starts_with("std:") {
      let module_name = path.strip_prefix("std:").unwrap();
      if let Ok(modules) = self.modules.try_borrow() {
        if let Some(module) = modules.get(module_name) {
          return Ok(module.clone());
        }
      }
      let module = modules::get_modules(module_name)
        .map(|bindings| {
          let module = Scope {
            name: Some(module_name),
            bindings: bindings.into_iter().collect(),
          };
          Object::Scope(Rc::new(RefCell::new(module)))
        })
        .ok_or_else(|| Error {
          span: Default::default(),
          kind: ErrorKind::ReferenceError(format!(
            "Standard module not found: {}",
            module_name
          )),
        })?;
      self
        .modules
        .try_borrow_mut()
        .map_err(|e| Error {
          span: Default::default(),
          kind: ErrorKind::RuntimeError(e.to_string()),
        })?
        .insert(module_name.to_string(), module.clone());
      return Ok(module);
    }
    self.eval_and_cache(path).await
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

    let bindings: FxHashMap<_, _> = new_interpreter
      .environment
      .try_borrow()
      .map_err(|e| Error {
        span: Default::default(),
        kind: ErrorKind::RuntimeError(e.to_string()),
      })?
      .get_public_bindings()
      .into_iter()
      .collect();

    let module = Object::Scope(Rc::new(RefCell::new(Scope {
      name: Some(self.arena.alloc_str(path)),
      bindings: bindings.into_iter().map(|(k, v)| (k, v.clone())).collect(),
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
