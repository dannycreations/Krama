use bumpalo::collections::Vec as BumpVec;
use bumpalo::Bump;
use futures::future::LocalBoxFuture;
use krama_core::ast::types::{Type, TypeKind};
use krama_core::error::Error;
use krama_core::error::ErrorKind;
use krama_core::object::NativeFn;
use krama_core::object::Object;
use rustc_hash::FxHashMap;
use std::path::Path;
use tokio::fs;

pub fn get_exports<'ast>() -> FxHashMap<&'static str, Object<'ast>> {
  let mut exports = FxHashMap::default();
  exports.insert(
    "readFile",
    Object::NativeFn(NativeFn {
      name: "readFile",
      callback: read_file,
    }),
  );
  exports.insert(
    "writeFile",
    Object::NativeFn(NativeFn {
      name: "writeFile",
      callback: write_file,
    }),
  );
  exports.insert(
    "exists",
    Object::NativeFn(NativeFn {
      name: "exists",
      callback: exists,
    }),
  );
  exports.insert(
    "rm",
    Object::NativeFn(NativeFn {
      name: "rm",
      callback: rm,
    }),
  );
  exports.insert(
    "readDir",
    Object::NativeFn(NativeFn {
      name: "readDir",
      callback: read_dir,
    }),
  );
  exports.insert(
    "mkdir",
    Object::NativeFn(NativeFn {
      name: "mkdir",
      callback: mkdir,
    }),
  );
  exports.insert(
    "rmdir",
    Object::NativeFn(NativeFn {
      name: "rmdir",
      callback: rmdir,
    }),
  );
  exports.insert(
    "isFile",
    Object::NativeFn(NativeFn {
      name: "isFile",
      callback: is_file,
    }),
  );
  exports.insert(
    "isDirectory",
    Object::NativeFn(NativeFn {
      name: "isDirectory",
      callback: is_directory,
    }),
  );
  exports
}

fn read_file<'ast>(
  arena: &'ast Bump,
  objects: BumpVec<'ast, Object<'ast>>,
) -> LocalBoxFuture<'ast, Result<Object<'ast>, Error>> {
  Box::pin(async move {
    if objects.len() != 1 {
      return Err(Error {
        span: Default::default(),
        kind: ErrorKind::WrongNumberOfArguments {
          expected: 1,
          got: objects.len(),
        },
      });
    }
    let path = match objects.first() {
      Some(Object::String(path)) => path,
      _ => {
        return Err(Error {
          span: Default::default(),
          kind: ErrorKind::TypeMismatch("Expected a string".into()),
        })
      }
    };
    let path = Path::new(path);
    let contents = fs::read_to_string(path).await.map_err(|e| Error {
      span: Default::default(),
      kind: ErrorKind::RuntimeError(e.to_string()),
    })?;

    Ok(Object::String(arena.alloc_str(&contents)))
  })
}

fn write_file<'ast>(
  _: &'ast Bump,
  objects: BumpVec<'ast, Object<'ast>>,
) -> LocalBoxFuture<'ast, Result<Object<'ast>, Error>> {
  Box::pin(async move {
    if objects.len() != 2 {
      return Err(Error {
        span: Default::default(),
        kind: ErrorKind::WrongNumberOfArguments {
          expected: 2,
          got: objects.len(),
        },
      });
    }

    let path = match &objects[0] {
      Object::String(path) => path,
      _ => {
        return Err(Error {
          span: Default::default(),
          kind: ErrorKind::TypeMismatch(
            "Expected a string as the first argument".into(),
          ),
        })
      }
    };

    let contents = match &objects[1] {
      Object::String(contents) => contents,
      _ => {
        return Err(Error {
          span: Default::default(),
          kind: ErrorKind::TypeMismatch(
            "Expected a string as the second argument".into(),
          ),
        })
      }
    };

    fs::write(path, contents).await.map_err(|e| Error {
      span: Default::default(),
      kind: ErrorKind::RuntimeError(e.to_string()),
    })?;

    Ok(Object::Void)
  })
}

fn exists<'ast>(
  _: &'ast Bump,
  objects: BumpVec<'ast, Object<'ast>>,
) -> LocalBoxFuture<'ast, Result<Object<'ast>, Error>> {
  Box::pin(async move {
    if objects.len() != 1 {
      return Err(Error {
        span: Default::default(),
        kind: ErrorKind::WrongNumberOfArguments {
          expected: 1,
          got: objects.len(),
        },
      });
    }
    let path = match objects.first() {
      Some(Object::String(path)) => path,
      _ => {
        return Err(Error {
          span: Default::default(),
          kind: ErrorKind::TypeMismatch("Expected a string".into()),
        })
      }
    };
    let path = Path::new(path);

    let metadata = fs::metadata(path).await;
    Ok(Object::Boolean(metadata.is_ok()))
  })
}

fn rm<'ast>(
  _: &'ast Bump,
  objects: BumpVec<'ast, Object<'ast>>,
) -> LocalBoxFuture<'ast, Result<Object<'ast>, Error>> {
  Box::pin(async move {
    if objects.len() != 1 {
      return Err(Error {
        span: Default::default(),
        kind: ErrorKind::WrongNumberOfArguments {
          expected: 1,
          got: objects.len(),
        },
      });
    }

    let path = match objects.first() {
      Some(Object::String(path)) => path,
      _ => {
        return Err(Error {
          span: Default::default(),
          kind: ErrorKind::TypeMismatch("Expected a string".into()),
        })
      }
    };

    fs::remove_file(path).await.map_err(|e| Error {
      span: Default::default(),
      kind: ErrorKind::RuntimeError(e.to_string()),
    })?;

    Ok(Object::Void)
  })
}

fn read_dir<'ast>(
  arena: &'ast Bump,
  mut objects: BumpVec<'ast, Object<'ast>>,
) -> LocalBoxFuture<'ast, Result<Object<'ast>, Error>> {
  Box::pin(async move {
    if objects.len() != 1 {
      return Err(Error {
        span: Default::default(),
        kind: ErrorKind::WrongNumberOfArguments {
          expected: 1,
          got: objects.len(),
        },
      });
    }
    let path_obj = objects.pop().unwrap();
    let path = match path_obj {
      Object::String(path) => path,
      _ => {
        return Err(Error {
          span: Default::default(),
          kind: ErrorKind::TypeMismatch("Expected a string".into()),
        })
      }
    };

    let mut entries = BumpVec::new_in(arena);

    let mut paths = fs::read_dir(path).await.map_err(|e| Error {
      span: Default::default(),
      kind: ErrorKind::RuntimeError(e.to_string()),
    })?;

    while let Some(path) = paths.next_entry().await.map_err(|e| Error {
      span: Default::default(),
      kind: ErrorKind::RuntimeError(e.to_string()),
    })? {
      let entry =
        path.file_name().into_string().map_err(|os_string| Error {
          span: Default::default(),
          kind: ErrorKind::RuntimeError(format!(
            "Invalid UTF-8 sequence in file name: {:?}",
            os_string
          )),
        })?;
      entries.push(Object::String(arena.alloc_str(&entry)));
    }

    Ok(Object::Array {
      elements: entries,
      kind: Type {
        kind: TypeKind::Identifier("str"),
        span: Default::default(),
      },
    })
  })
}

fn mkdir<'ast>(
  _: &'ast Bump,
  objects: BumpVec<'ast, Object<'ast>>,
) -> LocalBoxFuture<'ast, Result<Object<'ast>, Error>> {
  Box::pin(async move {
    if objects.len() != 1 {
      return Err(Error {
        span: Default::default(),
        kind: ErrorKind::WrongNumberOfArguments {
          expected: 1,
          got: objects.len(),
        },
      });
    }

    let path = match objects.first() {
      Some(Object::String(path)) => path,
      _ => {
        return Err(Error {
          span: Default::default(),
          kind: ErrorKind::TypeMismatch("Expected a string".into()),
        })
      }
    };

    fs::create_dir_all(path).await.map_err(|e| Error {
      span: Default::default(),
      kind: ErrorKind::RuntimeError(e.to_string()),
    })?;

    Ok(Object::Void)
  })
}

fn rmdir<'ast>(
  _: &'ast Bump,
  objects: BumpVec<'ast, Object<'ast>>,
) -> LocalBoxFuture<'ast, Result<Object<'ast>, Error>> {
  Box::pin(async move {
    if objects.len() != 1 {
      return Err(Error {
        span: Default::default(),
        kind: ErrorKind::WrongNumberOfArguments {
          expected: 1,
          got: objects.len(),
        },
      });
    }

    let path = match objects.first() {
      Some(Object::String(path)) => path,
      _ => {
        return Err(Error {
          span: Default::default(),
          kind: ErrorKind::TypeMismatch("Expected a string".into()),
        })
      }
    };

    fs::remove_dir(path).await.map_err(|e| Error {
      span: Default::default(),
      kind: ErrorKind::RuntimeError(e.to_string()),
    })?;

    Ok(Object::Void)
  })
}

fn is_file<'ast>(
  _: &'ast Bump,
  objects: BumpVec<'ast, Object<'ast>>,
) -> LocalBoxFuture<'ast, Result<Object<'ast>, Error>> {
  Box::pin(async move {
    if objects.len() != 1 {
      return Err(Error {
        span: Default::default(),
        kind: ErrorKind::WrongNumberOfArguments {
          expected: 1,
          got: objects.len(),
        },
      });
    }
    let path = match objects.first() {
      Some(Object::String(path)) => path,
      _ => {
        return Err(Error {
          span: Default::default(),
          kind: ErrorKind::TypeMismatch("Expected a string".into()),
        })
      }
    };
    let path = Path::new(path);

    let metadata = fs::metadata(path).await;
    Ok(Object::Boolean(
      metadata.map(|m| m.is_file()).unwrap_or(false),
    ))
  })
}

fn is_directory<'ast>(
  _: &'ast Bump,
  objects: BumpVec<'ast, Object<'ast>>,
) -> LocalBoxFuture<'ast, Result<Object<'ast>, Error>> {
  Box::pin(async move {
    if objects.len() != 1 {
      return Err(Error {
        span: Default::default(),
        kind: ErrorKind::WrongNumberOfArguments {
          expected: 1,
          got: objects.len(),
        },
      });
    }
    let path = match objects.first() {
      Some(Object::String(path)) => path,
      _ => {
        return Err(Error {
          span: Default::default(),
          kind: ErrorKind::TypeMismatch("Expected a string".into()),
        })
      }
    };
    let path = Path::new(path);

    let metadata = fs::metadata(path).await;
    Ok(Object::Boolean(
      metadata.map(|m| m.is_dir()).unwrap_or(false),
    ))
  })
}
