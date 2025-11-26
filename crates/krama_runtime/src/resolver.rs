use std::{cell::RefCell, path::Path, rc::Rc};

use krama_core::{
  error::{Error, ErrorKind},
  object::{ModuleObject, Object},
};
use krama_std::modules;
use tokio::fs;

use crate::interpreter::Interpreter;

#[derive(Default, Clone)]
pub struct Resolver;

impl Resolver {
  pub fn new() -> Self {
    Self
  }

  pub async fn resolve<'ast>(
    &self,
    interpreter: &Interpreter<'ast>,
    path: &'ast str,
  ) -> Result<Object<'ast>, Error> {
    if let Ok(modules) = interpreter.modules.try_borrow() {
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

    let module = if path.starts_with("std:") {
      self.resolve_std_module(path)?
    } else {
      self.resolve_file_module(interpreter, path).await?
    };

    if let Ok(mut modules) = interpreter.modules.try_borrow_mut() {
      modules.insert(path.to_string(), module.clone());
    } else {
      return Err(Error {
        span: Default::default(),
        kind: ErrorKind::RuntimeError(
          "Failed to mutably borrow modules cache".to_string(),
        ),
      });
    }

    Ok(module)
  }

  fn resolve_std_module<'ast>(
    &self,
    path: &'ast str,
  ) -> Result<Object<'ast>, Error> {
    let module_name = path.strip_prefix("std:").ok_or_else(|| Error {
      span: Default::default(),
      kind: ErrorKind::ReferenceError(
        "Invalid standard module path".to_string(),
      ),
    })?;
    modules::get_modules(module_name)
      .map(|exports| {
        let module = ModuleObject {
          name: module_name,
          exports: exports.into_iter().collect(),
        };
        Object::Module(Rc::new(RefCell::new(module)))
      })
      .ok_or_else(|| Error {
        span: Default::default(),
        kind: ErrorKind::ReferenceError(format!(
          "Standard module not found: {}",
          module_name
        )),
      })
  }

  async fn resolve_file_module<'ast>(
    &self,
    interpreter: &Interpreter<'ast>,
    path: &'ast str,
  ) -> Result<Object<'ast>, Error> {
    let base_path = interpreter
      .path
      .as_ref()
      .and_then(|p| Path::new(p).parent())
      .unwrap_or_else(|| Path::new(""));

    let mut path_buf = base_path.join(path);
    if path_buf.extension().is_none() {
      path_buf.set_extension("km");
    }
    let canonical_path =
      fs::canonicalize(&path_buf).await.map_err(|e| Error {
        span: Default::default(),
        kind: ErrorKind::ReferenceError(format!(
          "Failed to canonicalize path: {}",
          e
        )),
      })?;
    let path_str =
      interpreter.alloc_str(canonical_path.to_str().ok_or_else(|| Error {
        span: Default::default(),
        kind: ErrorKind::ReferenceError("Invalid path encoding".to_string()),
      })?);

    interpreter.eval_and_cache(path_str).await
  }
}
