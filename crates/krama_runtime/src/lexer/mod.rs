use krama_core::{Span, Token, TokenKind};
use logos::Logos;

#[derive(Clone)]
pub struct Lexer<'a> {
  logos: logos::Lexer<'a, TokenKind>,
  file: Option<String>,
}

impl<'a> Lexer<'a> {
  pub fn new(source: &'a str, file: Option<String>) -> Self {
    Self {
      logos: TokenKind::lexer(source),
      file,
    }
  }

  #[inline(always)]
  pub fn source_len(&self) -> usize {
    self.logos.source().len()
  }

  pub fn span(&self) -> Span {
    let span = self.logos.span();
    Span::new(span.start, span.end)
  }

  pub fn file(&self) -> Option<&str> {
    self.file.as_deref()
  }

  pub fn source(&self) -> &'a str {
    self.logos.source()
  }
}

impl<'a> Iterator for Lexer<'a> {
  type Item = Token;

  fn next(&mut self) -> Option<Self::Item> {
    let kind = self.logos.next()?;

    // Convert Result<TokenKind, ()> to TokenKind
    // Logos returns Err(()) for invalid tokens, which we map to Unknown
    let kind = match kind {
      Ok(k) => k,
      Err(_) => TokenKind::Unknown,
    };

    Some(Token::new(kind, self.span()))
  }
}
