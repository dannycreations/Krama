use crate::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Node<'ast, T> {
  pub kind: T,
  pub span: Span,
  pub _marker: std::marker::PhantomData<&'ast ()>,
}

impl<'ast, T> Node<'ast, T> {
  pub fn new(kind: T, span: Span) -> Self {
    Self {
      kind,
      span,
      _marker: std::marker::PhantomData,
    }
  }
}
