use std::{
  path::{Path, PathBuf},
  str,
};

use krama_core::{error::ErrorKind, object::Object, scope::Scope, span::Span};
use path_clean::PathClean;
use rustc_hash::FxHashMap;
use tokio::fs;

use super::Interpreter;

impl<'ast> Interpreter<'ast> {
  async fn resolve_import_path(
    &self,
    path: &str,
    span: &Span<'ast>,
  ) -> Result<PathBuf, (ErrorKind, Span<'ast>)> {
    let base_path = self
      .path
      .and_then(|p| Path::new(p).parent())
      .unwrap_or_else(|| Path::new(""));
    let path_buf = base_path.join(path);

    if fs::metadata(&path_buf).await.is_ok() {
      return Ok(path_buf.clean());
    }

    let path_with_ext = path_buf.with_extension("km");
    if fs::metadata(&path_with_ext).await.is_ok() {
      return Ok(path_with_ext.clean());
    }

    Err((
      ErrorKind::ReferenceError(format!(
        "Failed to find module file: {}",
        path
      )),
      span.clone(),
    ))
  }

  fn eval_std_module(
    &self,
    path: &'ast str,
    span: Span<'ast>,
  ) -> Result<Object<'ast>, (ErrorKind, Span<'ast>)> {
    let module_name = self.arena.alloc_str(path.strip_prefix("std:").unwrap());

    if let Some(module) = self.loaded_modules.borrow().get(module_name) {
      return Ok(module.clone());
    }

    self
      .native_modules
      .get(module_name)
      .map(|bindings| {
        let module = Scope {
          name: Some(module_name),
          bindings: bindings.clone(),
        };
        let object = Object::Scope(self.arena.alloc(module));
        self
          .loaded_modules
          .borrow_mut()
          .insert(module_name, object.clone());
        object
      })
      .ok_or_else(|| {
        (
          ErrorKind::ReferenceError(format!(
            "Standard module not found: {}",
            module_name
          )),
          span,
        )
      })
  }

  async fn eval_file_module(
    &self,
    path: &'ast str,
    span: Span<'ast>,
  ) -> Result<Object<'ast>, (ErrorKind, Span<'ast>)> {
    let resolved_path = self.resolve_import_path(path, &span).await?;
    let resolved_path_str = resolved_path.to_str().ok_or_else(|| {
      (
        ErrorKind::RuntimeError("Invalid path encoding".to_string()),
        span.clone(),
      )
    })?;
    let resolved_path_key = self.alloc_str(resolved_path_str);

    if let Some(module) = self.loaded_modules.borrow().get(resolved_path_key) {
      return Ok(module.clone());
    }

    let source = fs::read_to_string(&resolved_path)
      .await
      .map_err(|e| (ErrorKind::ReferenceError(e.to_string()), span))?;

    let source_str = self.arena.alloc_str(&source);

    let new_interpreter = Interpreter::new(self.arena, Some(resolved_path_key));
    if let Err(mut err) = new_interpreter.eval(source_str).await {
      err.1.file = Some(resolved_path_key);
      err.1.source = Some(source_str);
      return Err(err);
    }

    let bindings: FxHashMap<_, _> = new_interpreter
      .environment
      .borrow()
      .get_public_bindings()
      .into_iter()
      .collect();

    let module = Object::Scope(self.arena.alloc(Scope {
      name: Some(resolved_path_key),
      bindings,
    }));

    self
      .loaded_modules
      .borrow_mut()
      .insert(resolved_path_key, module.clone());

    Ok(module)
  }

  pub(super) async fn eval_import(
    &self,
    path: &'ast str,
    span: Span<'ast>,
  ) -> Result<Object<'ast>, (ErrorKind, Span<'ast>)> {
    if path.starts_with("std:") {
      self.eval_std_module(path, span)
    } else {
      self.eval_file_module(path, span).await
    }
  }
}
