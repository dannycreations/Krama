use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Span {
  pub start: usize,
  pub end: usize,
}

impl Span {
  pub fn new(start: usize, end: usize) -> Self {
    Self { start, end }
  }

  pub fn empty() -> Self {
    Self { start: 0, end: 0 }
  }

  pub fn merge(&self, other: &Self) -> Self {
    Self {
      start: self.start.min(other.start),
      end: self.end.max(other.end),
    }
  }
}

impl Debug for Span {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("Span")
      .field("start", &self.start)
      .field("end", &self.end)
      .finish()
  }
}

impl Display for Span {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}:{}", self.start, self.end)
  }
}

// We implement Send and Sync for Span because it only contains usize
unsafe impl Send for Span {}
unsafe impl Sync for Span {}
