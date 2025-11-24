#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Literal<'ast> {
  Integer(i64),
  Float(f64),
  String(&'ast str),
  Boolean(bool),
  Null,
}
