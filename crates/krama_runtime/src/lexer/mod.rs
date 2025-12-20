use krama_core::{Span, Token, TokenKind};
use logos::Logos;

#[derive(Clone)]
pub struct Lexer<'a> {
  logos: logos::Lexer<'a, TokenKind<'a>>,
  file: Option<&'a str>,
}

impl<'a> Lexer<'a> {
  pub fn new(source: &'a str, file: Option<&'a str>) -> Self {
    Self {
      logos: TokenKind::lexer(source),
      file,
    }
  }

  #[inline(always)]
  pub fn source_len(&self) -> usize {
    self.logos.source().len()
  }

  pub fn span(&self) -> Span<'a> {
    let span = self.logos.span();
    Span::new(span.start, span.end, Some(self.logos.slice()), self.file)
  }
}

impl<'a> Iterator for Lexer<'a> {
  type Item = Token<'a>;

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
