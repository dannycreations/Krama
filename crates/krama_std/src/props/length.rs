use krama_core::ObjectKind;
use krama_macro::register_property;

#[register_property(name = "length", types = ["string", "array", "tuple"])]
async fn length(object: ObjectKind) -> ObjectResult {
  match object {
    ObjectKind::Array { elements, .. } => {
      let elements = elements.read();
      Ok(ObjectKind::Integer(elements.len() as i64))
    }
    ObjectKind::Tuple { elements } => {
      Ok(ObjectKind::Integer(elements.len() as i64))
    }
    ObjectKind::String(s) => Ok(ObjectKind::Integer(s.len() as i64)),
    _ => unreachable!(),
  }
}
