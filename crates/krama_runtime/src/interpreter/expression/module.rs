use std::{
  path::{Path, PathBuf},
  sync::Arc,
};

use ahash::AHashMap;
use indexmap::IndexMap;
use krama_core::{
  Binding, Error, ErrorKind, FunctionKind, ObjectKind, ObjectResult, Span,
};
use krama_std::MODULES;
use parking_lot::RwLock;
use path_clean::PathClean;
use tokio::fs;

use crate::interpreter::{types, Interpreter};

impl Interpreter {
  pub async fn eval_import(&self, path: &str, span: Span) -> ObjectResult {
    if path.starts_with("std:") {
      self.eval_std_module(path, span)
    } else {
      self.eval_file_module(path, span).await
    }
  }

  async fn resolve_import_path(
    &self,
    path: &str,
    span: &Span,
  ) -> crate::ErrorResult<(PathBuf, String)> {
    let base_path = self
      .path
      .as_ref()
      .and_then(|p| Path::new(p).parent())
      .unwrap_or_else(|| Path::new(""));

    let path_buf = base_path.join(path).clean();

    if let Ok(content) = fs::read_to_string(&path_buf).await {
      return Ok((path_buf, content));
    }

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

  fn eval_std_module(&self, path: &str, span: Span) -> ObjectResult {
    let module_name = path.strip_prefix("std:").unwrap().to_string();

    if let Some(module) = self.modules.read().get(module_name.as_str()) {
      return Ok(module.clone());
    }

    MODULES
      .get(module_name.as_str())
      .map(|bindings| {
        let mut scope_bindings = AHashMap::with_capacity(bindings.len());
        for (name, native_fn) in bindings {
          scope_bindings.insert(
            (*name).into(),
            Binding {
              value: ObjectKind::Function(FunctionKind::Native(*native_fn)),
              public: true,
              constant: true,
            },
          );
        }

        let module = krama_core::Scope {
          name: Some(module_name.clone().into()),
          bindings: scope_bindings,
          parent: None,
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

  async fn eval_file_module(&self, path: &str, span: Span) -> ObjectResult {
    let (resolved_path, source) = self.resolve_import_path(path, &span).await?;
    let resolved_path_key = resolved_path.to_string_lossy().to_string();

    if let Some(module) = self.modules.read().get(resolved_path_key.as_str()) {
      return Ok(module.clone());
    }

    let new_interpreter = Interpreter::new(Some(resolved_path_key.clone()));

    new_interpreter.eval(&source).await?;

    let public_values = new_interpreter.stack.read().get_public_bindings();

    let mut bindings = AHashMap::with_capacity(public_values.len());
    for (name, value) in public_values {
      bindings.insert(
        name,
        Binding {
          value,
          public: true,
          constant: true,
        },
      );
    }

    let module = ObjectKind::Scope(Arc::new(RwLock::new(krama_core::Scope {
      name: Some(resolved_path_key.clone().into()),
      bindings,
      parent: None,
    })));

    self
      .modules
      .write()
      .insert(resolved_path_key, module.clone());

    Ok(module)
  }

  pub async fn eval_struct_construction(
    &self,
    properties: &[(krama_core::Expression, krama_core::Expression)],
    span: Span,
  ) -> ObjectResult {
    let this_obj = self.get_this(span)?;
    let ObjectKind::Struct(definition) = this_obj else {
      return Err(Error::new(
        ErrorKind::TypeError(format!(
          "'this' is not a struct definition, found {}",
          this_obj.type_name()
        )),
        span,
      ));
    };

    let fields = self.eval_properties(properties).await?;
    let mut final_fields = IndexMap::with_capacity(definition.fields.len());
    for field in &definition.fields {
      let value = match fields.get(field.name.as_ref()) {
        Some(val) => val.clone(),
        None => match &field.default {
          Some(default) => self.eval_expression(default, None).await?,
          None => {
            return Err(Error::new(
              ErrorKind::TypeError(format!("Missing field '{}'", field.name)),
              span,
            ))
          }
        },
      };
      types::check_type(&field.kind, &value)?;
      final_fields.insert(field.name.clone(), value);
    }

    Ok(
      self
        .heap
        .write()
        .alloc_object(final_fields, Some(definition), false),
    )
  }
}
