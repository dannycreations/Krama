use std::path::Path;
use std::rc::Rc;

use bumpalo::collections::Vec as BumpVec;
use bumpalo::Bump;
use futures::future::LocalBoxFuture;
use krama_core::ast::types::{Type, TypeKind};
use krama_core::error::{Error, ErrorKind};
use krama_core::object::{NativeFnCallback, Object};
use rustc_hash::FxHashMap;
use tokio::fs;

use crate::{build_native_functions, count_args, parse_args};

pub fn get_exports<'ast>() -> FxHashMap<&'static str, Object<'ast>> {
  let functions: &[(&'static str, NativeFnCallback<'ast>)] = &[
    ("readFile", read_file),
    ("writeFile", write_file),
    ("exists", exists),
    ("rm", rm),
    ("readDir", read_dir),
    ("mkdir", mkdir),
    ("rmdir", rmdir),
    ("isFile", is_file),
    ("isDirectory", is_directory),
  ];
  build_native_functions(functions)
}

fn read_file<'ast>(
  arena: &'ast Bump,
  objects: BumpVec<'ast, Object<'ast>>,
) -> LocalBoxFuture<'ast, Result<Object<'ast>, Error>> {
  Box::pin(async move {
    parse_args!(objects, path_str: Object::String(path_str));
    let path = Path::new(*path_str);
    let contents = fs::read(path).await.map_err(|e| Error {
      span: Default::default(),
      kind: ErrorKind::RuntimeError(e.to_string()),
    })?;
    let contents_str = std::str::from_utf8(&contents).map_err(|e| Error {
      span: Default::default(),
      kind: ErrorKind::RuntimeError(e.to_string()),
    })?;
    Ok(Object::String(arena.alloc_str(contents_str)))
  })
}

fn write_file<'ast>(
  _arena: &'ast Bump,
  objects: BumpVec<'ast, Object<'ast>>,
) -> LocalBoxFuture<'ast, Result<Object<'ast>, Error>> {
  Box::pin(async move {
    parse_args!(objects, path_str: Object::String(path_str), contents: Object::String(contents));

    fs::write(*path_str, *contents).await.map_err(|e| Error {
      span: Default::default(),
      kind: ErrorKind::RuntimeError(e.to_string()),
    })?;

    Ok(Object::Void)
  })
}

fn exists<'ast>(
  _arena: &'ast Bump,
  objects: BumpVec<'ast, Object<'ast>>,
) -> LocalBoxFuture<'ast, Result<Object<'ast>, Error>> {
  Box::pin(async move {
    parse_args!(objects, path_str: Object::String(path_str));
    let path = Path::new(*path_str);

    let metadata = fs::metadata(path).await;
    Ok(Object::Boolean(metadata.is_ok()))
  })
}

fn rm<'ast>(
  _arena: &'ast Bump,
  objects: BumpVec<'ast, Object<'ast>>,
) -> LocalBoxFuture<'ast, Result<Object<'ast>, Error>> {
  Box::pin(async move {
    parse_args!(objects, path_str: Object::String(path_str));

    fs::remove_file(*path_str).await.map_err(|e| Error {
      span: Default::default(),
      kind: ErrorKind::RuntimeError(e.to_string()),
    })?;

    Ok(Object::Void)
  })
}

fn read_dir<'ast>(
  arena: &'ast Bump,
  objects: BumpVec<'ast, Object<'ast>>,
) -> LocalBoxFuture<'ast, Result<Object<'ast>, Error>> {
  Box::pin(async move {
    parse_args!(objects, path_str: Object::String(path_str));

    let mut entries = BumpVec::new_in(arena);

    let mut paths = fs::read_dir(*path_str).await.map_err(|e| Error {
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
      elements: Rc::new(entries),
      kind: Type {
        kind: TypeKind::Identifier("str"),
        span: Default::default(),
      },
    })
  })
}

fn mkdir<'ast>(
  _arena: &'ast Bump,
  objects: BumpVec<'ast, Object<'ast>>,
) -> LocalBoxFuture<'ast, Result<Object<'ast>, Error>> {
  Box::pin(async move {
    parse_args!(objects, path_str: Object::String(path_str));

    fs::create_dir_all(*path_str).await.map_err(|e| Error {
      span: Default::default(),
      kind: ErrorKind::RuntimeError(e.to_string()),
    })?;

    Ok(Object::Void)
  })
}

fn rmdir<'ast>(
  _arena: &'ast Bump,
  objects: BumpVec<'ast, Object<'ast>>,
) -> LocalBoxFuture<'ast, Result<Object<'ast>, Error>> {
  Box::pin(async move {
    parse_args!(objects, path_str: Object::String(path_str));

    fs::remove_dir(*path_str).await.map_err(|e| Error {
      span: Default::default(),
      kind: ErrorKind::RuntimeError(e.to_string()),
    })?;

    Ok(Object::Void)
  })
}

fn is_file<'ast>(
  _arena: &'ast Bump,
  objects: BumpVec<'ast, Object<'ast>>,
) -> LocalBoxFuture<'ast, Result<Object<'ast>, Error>> {
  Box::pin(async move {
    parse_args!(objects, path_str: Object::String(path_str));
    let path = Path::new(*path_str);

    let metadata = fs::metadata(path).await;
    Ok(Object::Boolean(
      metadata.map(|m| m.is_file()).unwrap_or(false),
    ))
  })
}

fn is_directory<'ast>(
  _arena: &'ast Bump,
  objects: BumpVec<'ast, Object<'ast>>,
) -> LocalBoxFuture<'ast, Result<Object<'ast>, Error>> {
  Box::pin(async move {
    parse_args!(objects, path_str: Object::String(path_str));
    let path = Path::new(*path_str);

    let metadata = fs::metadata(path).await;
    Ok(Object::Boolean(
      metadata.map(|m| m.is_dir()).unwrap_or(false),
    ))
  })
}
