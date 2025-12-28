use std::{io::Error as IoError, path::Path, str, sync::Arc};

use krama_core::{ErrorKind, Object, Span, Type, TypeKind};
use krama_macro::register_module;
use parking_lot::RwLock;
use tokio::fs::{
  create_dir_all, metadata as tokio_metadata, read as tokio_read,
  read_dir as tokio_read_dir, remove_dir, remove_file, write as tokio_write,
};

/// Maps standard IO errors to ReferenceErrors.
fn error(e: IoError) -> ErrorKind {
  ErrorKind::ReferenceError(e.to_string())
}

#[register_module(name = "readFile", module = "fs")]
async fn read_file(objects: &[Object]) -> ObjectResult {
  parse_path_arg!(objects, "readFile"; path_str);
  let contents = tokio_read(Path::new(path_str.as_ref()))
    .await
    .map_err(error)?;
  let contents_str = str::from_utf8(&contents)
    .map_err(|e| ErrorKind::TypeError(e.to_string()))?;
  Ok(Object::String(contents_str.into()))
}

#[register_module(name = "writeFile", module = "fs")]
async fn write_file(objects: &[Object]) -> ObjectResult {
  parse_args!(objects, "writeFile"; path_str: Object::String(path_str), contents: Object::String(contents));
  tokio_write(path_str.as_ref(), contents.as_ref())
    .await
    .map_err(error)?;
  Ok(Object::Void)
}

#[register_module(name = "exists", module = "fs")]
async fn exists(objects: &[Object]) -> ObjectResult {
  parse_path_arg!(objects, "exists"; path_str);
  let metadata = tokio_metadata(Path::new(path_str.as_ref())).await;
  Ok(Object::Bool(metadata.is_ok()))
}

#[register_module(name = "rm", module = "fs")]
async fn rm(objects: &[Object]) -> ObjectResult {
  parse_path_arg!(objects, "rm"; path_str);
  remove_file(path_str.as_ref()).await.map_err(error)?;
  Ok(Object::Void)
}

#[register_module(name = "readDir", module = "fs")]
async fn read_dir(objects: &[Object]) -> ObjectResult {
  parse_path_arg!(objects, "readDir"; path_str);
  let mut paths = tokio_read_dir(path_str.as_ref()).await.map_err(error)?;
  let mut entries = Vec::new();
  while let Some(path) = paths.next_entry().await.map_err(error)? {
    let entry = path.file_name().into_string().map_err(|os_string| {
      ErrorKind::TypeError(format!(
        "Invalid UTF-8 sequence in file name: {:?}",
        os_string
      ))
    })?;
    entries.push(Object::String(entry.into()));
  }
  Ok(Object::Array {
    elements: Arc::new(RwLock::new(entries)),
    ty: Type::new(TypeKind::Str, Span::empty()),
    constant: true,
  })
}

#[register_module(name = "mkdir", module = "fs")]
async fn mkdir(objects: &[Object]) -> ObjectResult {
  parse_path_arg!(objects, "mkdir"; path_str);
  create_dir_all(path_str.as_ref()).await.map_err(error)?;
  Ok(Object::Void)
}

#[register_module(name = "rmdir", module = "fs")]
async fn rmdir(objects: &[Object]) -> ObjectResult {
  parse_path_arg!(objects, "rmdir"; path_str);
  remove_dir(path_str.as_ref()).await.map_err(error)?;
  Ok(Object::Void)
}

#[register_module(name = "isFile", module = "fs")]
async fn is_file(objects: &[Object]) -> ObjectResult {
  parse_path_arg!(objects, "isFile"; path_str);
  let metadata = tokio_metadata(Path::new(path_str.as_ref())).await;
  Ok(Object::Bool(metadata.map(|m| m.is_file()).unwrap_or(false)))
}

#[register_module(name = "isDirectory", module = "fs")]
async fn is_directory(objects: &[Object]) -> ObjectResult {
  parse_path_arg!(objects, "isDirectory"; path_str);
  let metadata = tokio_metadata(Path::new(path_str.as_ref())).await;
  Ok(Object::Bool(metadata.map(|m| m.is_dir()).unwrap_or(false)))
}
