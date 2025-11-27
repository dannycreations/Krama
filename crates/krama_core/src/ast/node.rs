use crate::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Node<'ast, T> {
  pub kind: T,
  pub span: Span,
  _phantom: std::marker::PhantomData<&'ast ()>,
}

impl<'ast, T> Node<'ast, T> {
  pub fn new(kind: T, span: Span) -> Self {
    Self {
      kind,
      span,
      _phantom: std::marker::PhantomData,
    }
  }
}
