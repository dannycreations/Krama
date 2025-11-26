use std::{cell::RefCell, path::Path, rc::Rc};

use krama_core::object::{ModuleObject, Object};
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
  ) -> Result<Object<'ast>, String> {
    if let Some(module) = interpreter.modules.try_borrow().unwrap().get(path) {
      return Ok(module.clone());
    }

    let module = if path.starts_with("std:") {
      self.resolve_std_module(path)?
    } else {
      self.resolve_file_module(interpreter, path).await?
    };

    interpreter
      .modules
      .try_borrow_mut()
      .unwrap()
      .insert(path.to_string(), module.clone());

    Ok(module)
  }

  fn resolve_std_module<'ast>(
    &self,
    path: &'ast str,
  ) -> Result<Object<'ast>, String> {
    let module_name = path.strip_prefix("std:").unwrap();
    modules::get_modules(module_name)
      .map(|exports| {
        let module = ModuleObject {
          name: module_name,
          exports: exports.into_iter().collect(),
        };
        Object::Module(Rc::new(RefCell::new(module)))
      })
      .ok_or_else(|| format!("Standard module not found: {}", module_name))
  }

  async fn resolve_file_module<'ast>(
    &self,
    interpreter: &Interpreter<'ast>,
    path: &'ast str,
  ) -> Result<Object<'ast>, String> {
    let base_path = interpreter
      .path
      .as_ref()
      .and_then(|p| Path::new(p).parent())
      .unwrap_or_else(|| Path::new(""));

    let mut path_buf = base_path.join(path);
    if path_buf.extension().is_none() {
      path_buf.set_extension("km");
    }
    let canonical_path = fs::canonicalize(&path_buf)
      .await
      .map_err(|e| format!("Failed to canonicalize path: {}", e))?;
    let path_str = interpreter.alloc_str(canonical_path.to_str().unwrap());

    interpreter.eval_and_cache(path_str).await
  }
}
