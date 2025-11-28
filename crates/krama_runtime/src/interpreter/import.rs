use std::path::Path;

use krama_core::{
  error::{Error, ErrorKind},
  object::Object,
  scope::Scope,
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
      let module_name =
        self.arena.alloc_str(path.strip_prefix("std:").unwrap());
      if let Ok(modules) = self.modules.try_borrow() {
        if let Some(module) = modules.get(module_name) {
          return Ok(module.clone());
        }
      }
      let module = modules::get_modules(module_name)
        .map(|bindings| {
          let module = Scope {
            name: Some(module_name),
            bindings,
          };
          Object::Scope(self.arena.alloc(module))
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
        .insert(module_name, module.clone());
      return Ok(module);
    }

    self.eval_and_cache(path).await
  }

  pub async fn eval_and_cache(
    &self,
    path: &'ast str,
  ) -> Result<Object<'ast>, Error> {
    // Try to resolve path and get source content.
    // First try the path as is, then with a `.km` extension.
    let path = self
      .path
      .and_then(|current_path| {
        Path::new(current_path).parent().map(|p| p.join(path))
      })
      .map(|p| self.arena.alloc_str(p.to_str().unwrap()))
      .map_or(path, |v| v);
    let (source, resolved_path) = match fs::read_to_string(path).await {
      Ok(source) => (source, path),
      Err(e1) => {
        let path_with_ext_str = format!("{}.km", path);
        let path_with_ext: &str = self.arena.alloc_str(&path_with_ext_str);
        match fs::read_to_string(path_with_ext).await {
          Ok(source) => (source, path_with_ext),
          Err(_) => {
            // On second failure, return the first, more relevant error.
            return Err(Error {
              span: Default::default(),
              kind: ErrorKind::ReferenceError(format!(
                "Failed to read module file: {}",
                e1
              )),
            });
          }
        }
      }
    };

    // Now that we have the correctly resolved path, check the cache.
    if let Ok(modules) = self.modules.try_borrow() {
      if let Some(module) = modules.get(resolved_path) {
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

    // We have the source from the resolution step, so just allocate it.
    let source_str = self.arena.alloc_str(&source);

    let new_interpreter = Interpreter::new(self.arena, Some(resolved_path));
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

    let module = Object::Scope(self.arena.alloc(Scope {
      name: Some(resolved_path),
      bindings,
    }));

    self
      .modules
      .try_borrow_mut()
      .map_err(|e| Error {
        span: Default::default(),
        kind: ErrorKind::RuntimeError(e.to_string()),
      })?
      .insert(resolved_path, module.clone());

    Ok(module)
  }
}
