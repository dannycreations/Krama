use std::{
  fmt::{Display, Formatter, Result as FmtResult},
  ops::Deref,
};

use crate::Span;

/// A generic node in the AST, wrapping a kind with its source span.
/// Optimized with #[repr(C)] to ensure predictable layout and minimal padding.
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Node<T> {
  /// The specific data for this node (e.g., ExpressionKind or StatementKind).
  pub kind: T,
  /// The source code range this node covers.
  pub span: Span,
}

impl<T> Node<T> {
  /// Creates a new AST node.
  #[inline(always)]
  pub fn new(kind: T, span: Span) -> Self {
    Self { kind, span }
  }

  /// Returns the span of this node.
  #[inline(always)]
  pub fn span(&self) -> Span {
    self.span
  }
}

// Simplified IntoNode trait to reduce boilerplate during AST creation.
pub trait IntoNode: Sized {
  #[inline(always)]
  fn into_node(self, span: Span) -> Node<Self> {
    Node::new(self, span)
  }
}

impl<T> IntoNode for T {}

impl<T> Display for Node<T>
where
  T: Display,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}", self.kind)
  }
}

impl<T> Deref for Node<T> {
  type Target = T;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    &self.kind
  }
}

/// Allows transparent conversion from Node<T> to T in patterns.
impl<T> AsRef<T> for Node<T> {
  fn as_ref(&self) -> &T {
    &self.kind
  }
}
