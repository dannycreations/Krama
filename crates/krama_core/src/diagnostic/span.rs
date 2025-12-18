#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Span<'a> {
  pub start: usize,
  pub end: usize,
  pub source: Option<&'a str>,
  pub file: Option<&'a str>,
}

impl<'a> Span<'a> {
  pub fn new(
    start: usize,
    end: usize,
    source: Option<&'a str>,
    file: Option<&'a str>,
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

  pub fn merge(&self, other: &Span) -> Self {
    Self {
      start: self.start,
      end: other.end,
      source: self.source,
      file: self.file,
    }
  }

  pub fn into_static(self) -> Span<'static> {
    Span {
      start: self.start,
      end: self.end,
      source: None,
      file: None,
    }
  }
}
