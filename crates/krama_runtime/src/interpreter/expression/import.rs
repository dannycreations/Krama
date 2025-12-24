use std::path::{Path, PathBuf};

use ahash::AHashMap;
use krama_core::{Error, ErrorKind, FunctionKind, ObjectKind, Scope, Span};
use krama_std::MODULES;
use path_clean::PathClean;
use tokio::fs;

use super::Interpreter;

impl<'ast> Interpreter<'ast> {
  /// Evaluates an import expression, supporting both standard library and file-based modules.
  pub async fn eval_import(
    &self,
    path: &'ast str,
    span: Span,
  ) -> Result<ObjectKind<'ast>, Error<'ast>> {
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
  ) -> Result<(PathBuf, String), Error<'ast>> {
    // 1. Determine base path for resolution.
    let base_path = self
      .path
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
    path: &'ast str,
    span: Span,
  ) -> Result<ObjectKind<'ast>, Error<'ast>> {
    let module_name = self.arena.alloc_str(path.strip_prefix("std:").unwrap());

    // Check module cache.
    if let Some(module) = self.modules.borrow().get(module_name) {
      return Ok(module.clone());
    }

    // Initialize module from krama_std definitions.
    MODULES
      .get(module_name)
      .map(|bindings| {
        let mut scope_bindings = AHashMap::with_capacity(bindings.len());
        for (name, native_fn) in bindings {
          scope_bindings.insert(
            *name,
            ObjectKind::Function(FunctionKind::Native(*native_fn)),
          );
        }

        let module = Scope {
          name: Some(module_name),
          bindings: scope_bindings,
        };
        let object = ObjectKind::Scope(self.arena.alloc(module));
        self
          .modules
          .borrow_mut()
          .insert(module_name, object.clone());
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
    path: &'ast str,
    span: Span,
  ) -> Result<ObjectKind<'ast>, Error<'ast>> {
    let (resolved_path, source) = self.resolve_import_path(path, &span).await?;
    let resolved_path_str = resolved_path.to_str().ok_or_else(|| {
      Error::new(
        ErrorKind::RuntimeError("Invalid path encoding".to_string()),
        span,
      )
    })?;
    let resolved_path_key = self.alloc_str(resolved_path_str);

    // Check module cache.
    if let Some(module) = self.modules.borrow().get(resolved_path_key) {
      return Ok(module.clone());
    }

    let source_str = self.arena.alloc_str(&source);

    // 1. Create a fresh interpreter for the module.
    let new_interpreter = Interpreter::new(self.arena, Some(resolved_path_key));
    new_interpreter.eval(source_str).await?;

    // 2. Extract public bindings from the module's environment.
    let bindings = new_interpreter
      .environment
      .borrow()
      .get_public_bindings()
      .into_iter()
      .collect();

    let module = ObjectKind::Scope(self.arena.alloc(Scope {
      name: Some(resolved_path_key),
      bindings,
    }));

    // 3. Cache the module for future imports.
    self
      .modules
      .borrow_mut()
      .insert(resolved_path_key, module.clone());

    Ok(module)
  }
}
