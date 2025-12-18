use std::{io::Error, path::Path, str};

use bumpalo::{collections::Vec as BumpVec, Bump};
use krama_core::{ErrorKind, Object, Span, Type, TypeKind};
use krama_macro::register_module;
use tokio::fs;

fn io_err_to_krama_err(e: Error) -> ErrorKind {
  ErrorKind::ReferenceError(e.to_string())
}

#[register_module(name = "readFile", module = "fs")]
async fn read_file<'ast>(
  arena: &'ast Bump,
  objects: &'ast [Object<'ast>],
) -> Result<Object<'ast>, ErrorKind> {
  parse_args!(objects, "readFile"; path_str: Object::String(path_str));
  let path = Path::new(*path_str);
  let contents = fs::read(path).await.map_err(io_err_to_krama_err)?;
  let contents_str = str::from_utf8(&contents)
    .map_err(|e| ErrorKind::TypeError(e.to_string()))?;
  Ok(Object::String(arena.alloc_str(contents_str)))
}

#[register_module(name = "writeFile", module = "fs")]
async fn write_file<'ast>(
  _arena: &'ast Bump,
  objects: &'ast [Object<'ast>],
) -> Result<Object<'ast>, ErrorKind> {
  parse_args!(objects, "writeFile"; path_str: Object::String(path_str), contents: Object::String(contents));

  fs::write(*path_str, *contents)
    .await
    .map_err(io_err_to_krama_err)?;

  Ok(Object::Void)
}

#[register_module(name = "exists", module = "fs")]
async fn exists<'ast>(
  _arena: &'ast Bump,
  objects: &'ast [Object<'ast>],
) -> Result<Object<'ast>, ErrorKind> {
  parse_args!(objects, "exists"; path_str: Object::String(path_str));
  let path = Path::new(*path_str);

  let metadata = fs::metadata(path).await;
  Ok(Object::Boolean(metadata.is_ok()))
}

#[register_module(name = "rm", module = "fs")]
async fn rm<'ast>(
  _arena: &'ast Bump,
  objects: &'ast [Object<'ast>],
) -> Result<Object<'ast>, ErrorKind> {
  parse_args!(objects, "rm"; path_str: Object::String(path_str));
  fs::remove_file(*path_str)
    .await
    .map_err(io_err_to_krama_err)?;
  Ok(Object::Void)
}

#[register_module(name = "readDir", module = "fs")]
async fn read_dir<'ast>(
  arena: &'ast Bump,
  objects: &'ast [Object<'ast>],
) -> Result<Object<'ast>, ErrorKind> {
  parse_args!(objects, "readDir"; path_str: Object::String(path_str));

  let mut paths = fs::read_dir(*path_str).await.map_err(io_err_to_krama_err)?;

  let mut entries = BumpVec::new_in(arena);
  while let Some(path) =
    paths.next_entry().await.map_err(io_err_to_krama_err)?
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

#[register_module(name = "mkdir", module = "fs")]
async fn mkdir<'ast>(
  _arena: &'ast Bump,
  objects: &'ast [Object<'ast>],
) -> Result<Object<'ast>, ErrorKind> {
  parse_args!(objects, "mkdir"; path_str: Object::String(path_str));
  fs::create_dir_all(*path_str)
    .await
    .map_err(io_err_to_krama_err)?;
  Ok(Object::Void)
}

#[register_module(name = "rmdir", module = "fs")]
async fn rmdir<'ast>(
  _arena: &'ast Bump,
  objects: &'ast [Object<'ast>],
) -> Result<Object<'ast>, ErrorKind> {
  parse_args!(objects, "rmdir"; path_str: Object::String(path_str));
  fs::remove_dir(*path_str)
    .await
    .map_err(io_err_to_krama_err)?;
  Ok(Object::Void)
}

#[register_module(name = "isFile", module = "fs")]
async fn is_file<'ast>(
  _arena: &'ast Bump,
  objects: &'ast [Object<'ast>],
) -> Result<Object<'ast>, ErrorKind> {
  parse_args!(objects, "isFile"; path_str: Object::String(path_str));
  let path = Path::new(*path_str);
  let metadata = fs::metadata(path).await;
  Ok(Object::Boolean(
    metadata.map(|m| m.is_file()).unwrap_or(false),
  ))
}

#[register_module(name = "isDirectory", module = "fs")]
async fn is_directory<'ast>(
  _arena: &'ast Bump,
  objects: &'ast [Object<'ast>],
) -> Result<Object<'ast>, ErrorKind> {
  parse_args!(objects, "isDirectory"; path_str: Object::String(path_str));
  let path = Path::new(*path_str);
  let metadata = fs::metadata(path).await;
  Ok(Object::Boolean(
    metadata.map(|m| m.is_dir()).unwrap_or(false),
  ))
}
