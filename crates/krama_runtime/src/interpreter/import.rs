use super::Interpreter;
use krama_core::error::Error;
use krama_core::error::ErrorKind;
use krama_core::object::Object;
use krama_core::span::Span;
use rustc_hash::FxHashMap;
use std::cell::RefCell;
use std::rc::Rc;

impl<'ast> Interpreter<'ast> {
  pub(super) async fn eval_import(
    &self,
    path: &'ast str,
    span: Span,
  ) -> Result<Object<'ast>, Error> {
    let resolver = self.resolver.clone();
    let module = resolver.resolve(self, path).await.map_err(|e| Error {
      span,
      kind: ErrorKind::SyntaxError(e.to_string()),
    })?;
    Ok(module)
  }

  pub async fn eval_and_cache(
    &self,
    path: &'ast str,
  ) -> Result<Object<'ast>, String> {
    if let Some(module) = self.modules.try_borrow().unwrap().get(path) {
      return Ok(module.clone());
    }

    let source = tokio::fs::read_to_string(path)
      .await
      .map_err(|e| format!("Failed to read module file: {}", e))?;
    let source_str = self.arena.alloc_str(&source);

    let new_interpreter = Interpreter::new(self.arena, Some(path));
    let _ = new_interpreter
      .eval(source_str)
      .await
      .map_err(|e| format!("Failed to evaluate module: {}", e))?;

    let exports: FxHashMap<_, _> = new_interpreter
      .environment
      .try_borrow()
      .unwrap()
      .get_public_bindings()
      .into_iter()
      .collect();

    let module =
      Object::Module(Rc::new(RefCell::new(krama_core::object::ModuleObject {
        name: self.arena.alloc_str(path),
        exports: exports
          .into_iter()
          .map(|(k, v)| (k, (*v).clone()))
          .collect(),
      })));

    self
      .modules
      .try_borrow_mut()
      .unwrap()
      .insert(path.to_string(), module.clone());

    Ok(module)
  }
}
