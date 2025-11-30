use krama_core::object::Object;
use krama_macro::register_property;

#[register_property(name = "length", types = ["string", "array", "tuple"])]
async fn length<'ast>(
  object: Object<'ast>,
) -> Result<Object<'ast>, krama_core::error::ErrorKind> {
  match object {
    Object::Array { elements, .. } => {
      Ok(Object::Integer(elements.len() as i64))
    }
    Object::Tuple { elements } => Ok(Object::Integer(elements.len() as i64)),
    Object::String(s) => Ok(Object::Integer(s.len() as i64)),
    _ => unreachable!(),
  }
}
