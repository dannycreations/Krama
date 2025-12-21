use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Span<'ast> {
  pub start: usize,
  pub end: usize,
  pub source: Option<&'ast str>,
  pub file: Option<&'ast str>,
}

impl<'ast> Span<'ast> {
  pub fn new(
    start: usize,
    end: usize,
    source: Option<&'ast str>,
    file: Option<&'ast str>,
  ) -> Self {
    Self {
      start,
      end,
      source,
      file,
    }
  }

  pub fn empty() -> Self {
    Self {
      start: 0,
      end: 0,
      source: None,
      file: None,
    }
  }

  pub fn merge(&self, other: &Self) -> Self {
    Self {
      start: self.start.min(other.start),
      end: self.end.max(other.end),
      source: self.source, // TODO: handle source merging if needed
      file: self.file,
    }
  }
}

impl<'ast> Debug for Span<'ast> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("Span")
      .field("start", &self.start)
      .field("end", &self.end)
      .field("source", &self.source)
      .field("file", &self.file)
      .finish()
  }
}

impl<'ast> Display for Span<'ast> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}:{}", self.start, self.end)
  }
}

// We implement Send and Sync for Span because it only contains usize and &'ast str
// which are safe to send and sync across threads.
unsafe impl<'ast> Send for Span<'ast> {}
unsafe impl<'ast> Sync for Span<'ast> {}
