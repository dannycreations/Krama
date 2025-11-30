use std::{fs::Metadata, future::Future, path::Path, str};

use bumpalo::{collections::Vec as BumpVec, Bump};
use futures::future::{FutureExt, LocalBoxFuture};
use krama_core::{
  ast::types::{Type, TypeKind},
  error::ErrorKind,
  object::{NativeFunctionCb, Object},
  span::Span,
};
use rustc_hash::FxHashMap;
use tokio::fs;

use crate::{build_native_functions, parse_args};

pub fn get_exports<'ast>() -> FxHashMap<&'static str, Object<'ast>> {
  let functions: &[(&'static str, NativeFunctionCb<'ast>)] = &[
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

fn get_path_metadata_prop<'ast>(
  objects: &'ast [Object<'ast>],
  fn_name: &'static str,
  prop_fn: impl Fn(&Metadata) -> bool + 'ast,
) -> LocalBoxFuture<'ast, Result<Object<'ast>, ErrorKind>> {
  async move {
    parse_args!(objects, fn_name; path_str: Object::String(path_str));
    let path = Path::new(*path_str);
    let metadata = fs::metadata(path).await;
    Ok(Object::Boolean(
      metadata.map(|m| prop_fn(&m)).unwrap_or(false),
    ))
  }
  .boxed_local()
}

fn fs_path_void_op<'ast, F, Fut>(
  objects: &'ast [Object<'ast>],
  fn_name: &'static str,
  op: F,
) -> LocalBoxFuture<'ast, Result<Object<'ast>, ErrorKind>>
where
  F: Fn(&'ast str) -> Fut + 'ast,
  Fut: Future<Output = Result<(), std::io::Error>> + 'ast,
{
  async move {
    parse_args!(objects, fn_name; path_str: Object::String(path_str));
    op(path_str)
      .await
      .map_err(|e| ErrorKind::ReferenceError(e.to_string()))?;
    Ok(Object::Void)
  }
  .boxed_local()
}

fn read_file<'ast>(
  arena: &'ast Bump,
  objects: &'ast [Object<'ast>],
) -> LocalBoxFuture<'ast, Result<Object<'ast>, ErrorKind>> {
  async move {
    parse_args!(objects, "readFile"; path_str: Object::String(path_str));
    let path = Path::new(*path_str);
    let contents = fs::read(path)
      .await
      .map_err(|e| ErrorKind::ReferenceError(e.to_string()))?;
    let contents_str = str::from_utf8(&contents)
      .map_err(|e| ErrorKind::TypeError(e.to_string()))?;
    Ok(Object::String(arena.alloc_str(contents_str)))
  }
  .boxed_local()
}

fn write_file<'ast>(
  _arena: &'ast Bump,
  objects: &'ast [Object<'ast>],
) -> LocalBoxFuture<'ast, Result<Object<'ast>, ErrorKind>> {
  async move {
    parse_args!(objects, "writeFile"; path_str: Object::String(path_str), contents: Object::String(contents));

    fs::write(*path_str, *contents)
      .await
      .map_err(|e| ErrorKind::ReferenceError(e.to_string()))?;

    Ok(Object::Void)
  }
  .boxed_local()
}

fn exists<'ast>(
  _arena: &'ast Bump,
  objects: &'ast [Object<'ast>],
) -> LocalBoxFuture<'ast, Result<Object<'ast>, ErrorKind>> {
  async move {
    parse_args!(objects, "exists"; path_str: Object::String(path_str));
    let path = Path::new(*path_str);

    let metadata = fs::metadata(path).await;
    Ok(Object::Boolean(metadata.is_ok()))
  }
  .boxed_local()
}

fn rm<'ast>(
  _arena: &'ast Bump,
  objects: &'ast [Object<'ast>],
) -> LocalBoxFuture<'ast, Result<Object<'ast>, ErrorKind>> {
  fs_path_void_op(objects, "rm", fs::remove_file)
}

fn read_dir<'ast>(
  arena: &'ast Bump,
  objects: &'ast [Object<'ast>],
) -> LocalBoxFuture<'ast, Result<Object<'ast>, ErrorKind>> {
  async move {
    parse_args!(objects, "readDir"; path_str: Object::String(path_str));

    let mut paths = fs::read_dir(*path_str)
      .await
      .map_err(|e| ErrorKind::ReferenceError(e.to_string()))?;

    let mut entries = BumpVec::new_in(arena);
    while let Some(path) = paths
      .next_entry()
      .await
      .map_err(|e| ErrorKind::ReferenceError(e.to_string()))?
    {
      let entry = path.file_name().into_string().map_err(|os_string| {
        ErrorKind::TypeError(format!(
          "Invalid UTF-8 sequence in file name: {:?}",
          os_string
        ))
      })?;
      entries.push(Object::String(arena.alloc_str(&entry)));
    }

    Ok(Object::Array {
      elements: entries.into_bump_slice(),
      kind: Type::new(TypeKind::Str, Span::empty()),
    })
  }
  .boxed_local()
}

fn mkdir<'ast>(
  _arena: &'ast Bump,
  objects: &'ast [Object<'ast>],
) -> LocalBoxFuture<'ast, Result<Object<'ast>, ErrorKind>> {
  fs_path_void_op(objects, "mkdir", fs::create_dir_all)
}

fn rmdir<'ast>(
  _arena: &'ast Bump,
  objects: &'ast [Object<'ast>],
) -> LocalBoxFuture<'ast, Result<Object<'ast>, ErrorKind>> {
  fs_path_void_op(objects, "rmdir", fs::remove_dir)
}

fn is_file<'ast>(
  _arena: &'ast Bump,
  objects: &'ast [Object<'ast>],
) -> LocalBoxFuture<'ast, Result<Object<'ast>, ErrorKind>> {
  get_path_metadata_prop(objects, "isFile", |m| m.is_file())
}

fn is_directory<'ast>(
  _arena: &'ast Bump,
  objects: &'ast [Object<'ast>],
) -> LocalBoxFuture<'ast, Result<Object<'ast>, ErrorKind>> {
  get_path_metadata_prop(objects, "isDirectory", |m| m.is_dir())
}
