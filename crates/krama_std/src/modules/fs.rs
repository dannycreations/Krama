use std::{path::Path, str};

use bumpalo::{collections::Vec as BumpVec, Bump};
use krama_core::{
  ast::types::{Type, TypeKind},
  error::ErrorKind,
  object::Object,
  span::Span,
};
use krama_macro::register_native;
use tokio::fs;

#[register_native(name = "readFile", module = "fs")]
async fn read_file<'ast>(
  arena: &'ast Bump,
  objects: &'ast [Object<'ast>],
) -> Result<Object<'ast>, ErrorKind> {
  parse_args!(objects, "readFile"; path_str: Object::String(path_str));
  let path = Path::new(*path_str);
  let contents = fs::read(path)
    .await
    .map_err(|e| ErrorKind::ReferenceError(e.to_string()))?;
  let contents_str = str::from_utf8(&contents)
    .map_err(|e| ErrorKind::TypeError(e.to_string()))?;
  Ok(Object::String(arena.alloc_str(contents_str)))
}

#[register_native(name = "writeFile", module = "fs")]
async fn write_file<'ast>(
  _arena: &'ast Bump,
  objects: &'ast [Object<'ast>],
) -> Result<Object<'ast>, ErrorKind> {
  parse_args!(objects, "writeFile"; path_str: Object::String(path_str), contents: Object::String(contents));

  fs::write(*path_str, *contents)
    .await
    .map_err(|e| ErrorKind::ReferenceError(e.to_string()))?;

  Ok(Object::Void)
}

#[register_native(name = "exists", module = "fs")]
async fn exists<'ast>(
  _arena: &'ast Bump,
  objects: &'ast [Object<'ast>],
) -> Result<Object<'ast>, ErrorKind> {
  parse_args!(objects, "exists"; path_str: Object::String(path_str));
  let path = Path::new(*path_str);

  let metadata = fs::metadata(path).await;
  Ok(Object::Boolean(metadata.is_ok()))
}

#[register_native(name = "rm", module = "fs")]
async fn rm<'ast>(
  _arena: &'ast Bump,
  objects: &'ast [Object<'ast>],
) -> Result<Object<'ast>, ErrorKind> {
  parse_args!(objects, "rm"; path_str: Object::String(path_str));
  fs::remove_file(*path_str)
    .await
    .map_err(|e| ErrorKind::ReferenceError(e.to_string()))?;
  Ok(Object::Void)
}

#[register_native(name = "readDir", module = "fs")]
async fn read_dir<'ast>(
  arena: &'ast Bump,
  objects: &'ast [Object<'ast>],
) -> Result<Object<'ast>, ErrorKind> {
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

#[register_native(name = "mkdir", module = "fs")]
async fn mkdir<'ast>(
  _arena: &'ast Bump,
  objects: &'ast [Object<'ast>],
) -> Result<Object<'ast>, ErrorKind> {
  parse_args!(objects, "mkdir"; path_str: Object::String(path_str));
  fs::create_dir_all(*path_str)
    .await
    .map_err(|e| ErrorKind::ReferenceError(e.to_string()))?;
  Ok(Object::Void)
}

#[register_native(name = "rmdir", module = "fs")]
async fn rmdir<'ast>(
  _arena: &'ast Bump,
  objects: &'ast [Object<'ast>],
) -> Result<Object<'ast>, ErrorKind> {
  parse_args!(objects, "rmdir"; path_str: Object::String(path_str));
  fs::remove_dir(*path_str)
    .await
    .map_err(|e| ErrorKind::ReferenceError(e.to_string()))?;
  Ok(Object::Void)
}

#[register_native(name = "isFile", module = "fs")]
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

#[register_native(name = "isDirectory", module = "fs")]
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
