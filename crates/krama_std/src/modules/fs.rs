use std::{io::Error as IoError, path::Path, str, sync::Arc};

use krama_core::{ErrorKind, ObjectKind, Span, Type, TypeKind};
use krama_macro::register_module;
use parking_lot::RwLock;
use tokio::fs;

/// Maps standard IO errors to ReferenceErrors.
fn error(e: IoError) -> ErrorKind {
  ErrorKind::ReferenceError(e.to_string())
}

#[register_module(name = "readFile", module = "fs")]
async fn read_file(objects: &[ObjectKind]) -> Result<ObjectKind, ErrorKind> {
  parse_path_arg!(objects, "readFile"; path_str);
  let contents = fs::read(Path::new(path_str)).await.map_err(error)?;
  let contents_str = str::from_utf8(&contents)
    .map_err(|e| ErrorKind::TypeError(e.to_string()))?;
  Ok(ObjectKind::String(contents_str.to_string()))
}

#[register_module(name = "writeFile", module = "fs")]
async fn write_file(objects: &[ObjectKind]) -> Result<ObjectKind, ErrorKind> {
  parse_args!(objects, "writeFile"; path_str: ObjectKind::String(path_str), contents: ObjectKind::String(contents));
  fs::write(path_str, contents).await.map_err(error)?;
  Ok(ObjectKind::Void)
}

#[register_module(name = "exists", module = "fs")]
async fn exists(objects: &[ObjectKind]) -> Result<ObjectKind, ErrorKind> {
  parse_path_arg!(objects, "exists"; path_str);
  let metadata = fs::metadata(Path::new(path_str)).await;
  Ok(ObjectKind::Boolean(metadata.is_ok()))
}

#[register_module(name = "rm", module = "fs")]
async fn rm(objects: &[ObjectKind]) -> Result<ObjectKind, ErrorKind> {
  parse_path_arg!(objects, "rm"; path_str);
  fs::remove_file(path_str).await.map_err(error)?;
  Ok(ObjectKind::Void)
}

#[register_module(name = "readDir", module = "fs")]
async fn read_dir(objects: &[ObjectKind]) -> Result<ObjectKind, ErrorKind> {
  parse_path_arg!(objects, "readDir"; path_str);
  let mut paths = fs::read_dir(path_str).await.map_err(error)?;
  let mut entries = Vec::new();
  while let Some(path) = paths.next_entry().await.map_err(error)? {
    let entry = path.file_name().into_string().map_err(|os_string| {
      ErrorKind::TypeError(format!(
        "Invalid UTF-8 sequence in file name: {:?}",
        os_string
      ))
    })?;
    entries.push(ObjectKind::String(entry));
  }

  Ok(ObjectKind::Array {
    elements: Arc::new(RwLock::new(entries)),
    kind: Type::new(TypeKind::Str, Span::empty()),
    constant: true,
  })
}

#[register_module(name = "mkdir", module = "fs")]
async fn mkdir(objects: &[ObjectKind]) -> Result<ObjectKind, ErrorKind> {
  parse_path_arg!(objects, "mkdir"; path_str);
  fs::create_dir_all(path_str).await.map_err(error)?;
  Ok(ObjectKind::Void)
}

#[register_module(name = "rmdir", module = "fs")]
async fn rmdir(objects: &[ObjectKind]) -> Result<ObjectKind, ErrorKind> {
  parse_path_arg!(objects, "rmdir"; path_str);
  fs::remove_dir(path_str).await.map_err(error)?;
  Ok(ObjectKind::Void)
}

#[register_module(name = "isFile", module = "fs")]
async fn is_file(objects: &[ObjectKind]) -> Result<ObjectKind, ErrorKind> {
  parse_path_arg!(objects, "isFile"; path_str);
  let metadata = fs::metadata(Path::new(path_str)).await;
  Ok(ObjectKind::Boolean(
    metadata.map(|m| m.is_file()).unwrap_or(false),
  ))
}

#[register_module(name = "isDirectory", module = "fs")]
async fn is_directory(objects: &[ObjectKind]) -> Result<ObjectKind, ErrorKind> {
  parse_path_arg!(objects, "isDirectory"; path_str);
  let metadata = fs::metadata(Path::new(path_str)).await;
  Ok(ObjectKind::Boolean(
    metadata.map(|m| m.is_dir()).unwrap_or(false),
  ))
}
