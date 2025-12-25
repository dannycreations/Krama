use std::{
  path::{Path, PathBuf},
  sync::Arc,
};

use ahash::AHashMap;
use krama_core::{Error, ErrorKind, FunctionKind, ObjectKind, Scope, Span};
use krama_std::MODULES;
use parking_lot::RwLock;
use path_clean::PathClean;
use tokio::fs;

use super::Interpreter;

impl Interpreter {
  /// Evaluates an import expression, supporting both standard library and file-based modules.
  pub async fn eval_import(
    &self,
    path: &str,
    span: Span,
  ) -> Result<ObjectKind, Error> {
    if path.starts_with("std:") {
      self.eval_std_module(path, span)
    } else {
      self.eval_file_module(path, span).await
    }
  }

  /// Resolves a relative import path to an absolute path and its content.
  async fn resolve_import_path(
    &self,
    path: &str,
    span: &Span,
  ) -> Result<(PathBuf, String), Error> {
    // 1. Determine base path for resolution.
    let base_path = self
      .path
      .as_ref()
      .and_then(|p| Path::new(p).parent())
      .unwrap_or_else(|| Path::new(""));

    let path_buf = base_path.join(path).clean();

    // 2. Try reading the exact path.
    if let Ok(content) = fs::read_to_string(&path_buf).await {
      return Ok((path_buf, content));
    }

    // 3. Try appending .km extension.
    let path_with_ext = path_buf.with_extension("km");
    if let Ok(content) = fs::read_to_string(&path_with_ext).await {
      return Ok((path_with_ext, content));
    }

    Err(Error::new(
      ErrorKind::ReferenceError(format!(
        "Failed to find module file: {}",
        path
      )),
      *span,
    ))
  }

  /// Loads and caches a standard library module.
  fn eval_std_module(
    &self,
    path: &str,
    span: Span,
  ) -> Result<ObjectKind, Error> {
    let module_name = path.strip_prefix("std:").unwrap().to_string();

    // Check module cache.
    if let Some(module) = self.modules.read().get(&module_name) {
      return Ok(module.clone());
    }

    // Initialize module from krama_std definitions.
    MODULES
      .get(module_name.as_str())
      .map(|bindings| {
        let mut scope_bindings = AHashMap::with_capacity(bindings.len());
        for (name, native_fn) in bindings {
          scope_bindings.insert(
            name.to_string(),
            ObjectKind::Function(FunctionKind::Native(*native_fn)),
          );
        }

        let module = Scope {
          name: Some(module_name.clone()),
          bindings: scope_bindings,
        };
        let object = ObjectKind::Scope(Arc::new(RwLock::new(module)));
        self
          .modules
          .write()
          .insert(module_name.clone(), object.clone());
        object
      })
      .ok_or_else(|| {
        Error::new(
          ErrorKind::ReferenceError(format!(
            "Standard module not found: {}",
            module_name
          )),
          span,
        )
      })
  }

  /// Loads, executes, and caches a file-based module.
  async fn eval_file_module(
    &self,
    path: &str,
    span: Span,
  ) -> Result<ObjectKind, Error> {
    let (resolved_path, source) = self.resolve_import_path(path, &span).await?;
    let resolved_path_str = resolved_path.to_str().ok_or_else(|| {
      Error::new(
        ErrorKind::RuntimeError("Invalid path encoding".to_string()),
        span,
      )
    })?;
    let resolved_path_key = resolved_path_str.to_string();

    // Check module cache.
    if let Some(module) = self.modules.read().get(&resolved_path_key) {
      return Ok(module.clone());
    }

    // 1. Create a fresh interpreter for the module.
    let new_interpreter = Interpreter::new(Some(resolved_path_key.clone()));

    // Evaluate the module source
    new_interpreter.eval(&source).await?;

    // 2. Extract public bindings from the module's environment.
    let bindings = new_interpreter
      .environment
      .read()
      .get_public_bindings()
      .into_iter()
      .collect();

    let module = ObjectKind::Scope(Arc::new(RwLock::new(Scope {
      name: Some(resolved_path_key.clone()),
      bindings,
    })));

    // 3. Cache the module for future imports.
    self
      .modules
      .write()
      .insert(resolved_path_key, module.clone());

    Ok(module)
  }
}
