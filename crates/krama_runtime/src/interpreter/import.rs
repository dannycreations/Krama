use std::{
  path::{Component, Path, PathBuf},
  str,
};

use krama_core::{error::ErrorKind, object::Object, scope::Scope, span::Span};
use krama_std::modules;
use rustc_hash::FxHashMap;
use tokio::fs;

use super::Interpreter;

fn clean_path(path: &Path) -> PathBuf {
  let mut components = path.components().peekable();
  let mut ret =
    if let Some(c @ Component::Prefix(..)) = components.peek().cloned() {
      components.next();
      PathBuf::from(c.as_os_str())
    } else {
      PathBuf::new()
    };

  for component in components {
    match component {
      Component::RootDir => {
        ret.push(component.as_os_str());
      }
      Component::CurDir => {}
      Component::ParentDir => {
        ret.pop();
      }
      Component::Normal(c) => {
        ret.push(c);
      }
      Component::Prefix(_) => unreachable!(),
    }
  }
  ret
}

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
      return Ok(clean_path(&path_buf));
    }

    let path_with_ext = path_buf.with_extension("km");
    if fs::metadata(&path_with_ext).await.is_ok() {
      return Ok(clean_path(&path_with_ext));
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

    if let Some(module) = self.modules.borrow().get(module_name) {
      return Ok(module.clone());
    }

    let module = modules::get_modules(module_name)
      .map(|bindings| {
        let module = Scope {
          name: Some(module_name),
          bindings,
        };
        Object::Scope(self.arena.alloc(module))
      })
      .ok_or_else(|| {
        (
          ErrorKind::ReferenceError(format!(
            "Standard module not found: {}",
            module_name
          )),
          span,
        )
      })?;

    self
      .modules
      .borrow_mut()
      .insert(module_name, module.clone());
    Ok(module)
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

    if let Some(module) = self.modules.borrow().get(resolved_path_key) {
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
      .modules
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
