use std::{
  io,
  path::{Path, PathBuf},
  str,
};

use ahash::AHashMap;
use krama_core::{Error, ErrorKind, Function, Object, Scope, Span};
use path_clean::PathClean;
use tokio::fs;

use super::Interpreter;

impl<'ast> Interpreter<'ast> {
  pub async fn eval_import(
    &self,
    path: &'ast str,
    span: Span<'ast>,
  ) -> Result<Object<'ast>, Error<'ast>> {
    if path.starts_with("std:") {
      self.eval_std_module(path, span)
    } else {
      self.eval_file_module(path, span).await
    }
  }

  async fn resolve_import_path(
    &self,
    path: &str,
    span: &Span<'ast>,
  ) -> Result<(PathBuf, String), Error<'ast>> {
    let base_path = self
      .path
      .and_then(|p| Path::new(p).parent())
      .unwrap_or_else(|| Path::new(""));

    let path_buf = base_path.join(path).clean();

    // Try reading path_buf directly
    match fs::read_to_string(&path_buf).await {
      Ok(content) => return Ok((path_buf, content)),
      Err(e) if e.kind() != io::ErrorKind::NotFound => {
        return Err(Error::new(
          ErrorKind::ReferenceError(e.to_string()),
          span.clone(),
        ));
      }
      _ => {}
    }

    // Try reading path_buf with .km extension
    let path_with_ext = path_buf.with_extension("km");
    match fs::read_to_string(&path_with_ext).await {
      Ok(content) => return Ok((path_with_ext, content)),
      Err(e) if e.kind() != io::ErrorKind::NotFound => {
        return Err(Error::new(
          ErrorKind::ReferenceError(e.to_string()),
          span.clone(),
        ));
      }
      _ => {}
    }

    Err(Error::new(
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
  ) -> Result<Object<'ast>, Error<'ast>> {
    let module_name = self.arena.alloc_str(path.strip_prefix("std:").unwrap());

    if let Some(module) = self.loaded_modules.borrow().get(module_name) {
      return Ok(module.clone());
    }

    krama_std::get_modules()
      .get(module_name)
      .map(|bindings| {
        let scope_bindings = bindings
          .iter()
          .map(|(name, native_fn)| {
            (*name, Object::Function(Function::Native(*native_fn)))
          })
          .collect();

        let module = Scope {
          name: Some(module_name),
          bindings: scope_bindings,
        };
        let object = Object::Scope(self.arena.alloc(module));
        self
          .loaded_modules
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

  async fn eval_file_module(
    &self,
    path: &'ast str,
    span: Span<'ast>,
  ) -> Result<Object<'ast>, Error<'ast>> {
    let (resolved_path, source) = self.resolve_import_path(path, &span).await?;
    let resolved_path_str = resolved_path.to_str().ok_or_else(|| {
      Error::new(
        ErrorKind::RuntimeError("Invalid path encoding".to_string()),
        span.clone(),
      )
    })?;
    let resolved_path_key = self.alloc_str(resolved_path_str);

    if let Some(module) = self.loaded_modules.borrow().get(resolved_path_key) {
      return Ok(module.clone());
    }

    let source_str = self.arena.alloc_str(&source);

    let new_interpreter = Interpreter::new(self.arena, Some(resolved_path_key));
    new_interpreter.eval(source_str).await?;

    let bindings: AHashMap<_, _> = new_interpreter
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
}
