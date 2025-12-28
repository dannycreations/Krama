use krama_core::Object;
use krama_macro::register_property;

#[register_property(name = "length", types = ["str", "array", "tuple"])]
async fn length(object: Object) -> ObjectResult {
  match object {
    Object::Array { elements, .. } => {
      let elements = elements.read();
      Ok(Object::Integer(elements.len() as i64))
    }
    Object::Tuple(elements) => Ok(Object::Integer(elements.len() as i64)),
    Object::String(s) => Ok(Object::Integer(s.len() as i64)),
    _ => unreachable!(),
  }
}
