use crate::span::Span;
use crate::token::TokenKind;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
  pub span: Span,
  pub kind: ErrorKind,
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.kind)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
  ParserError(&'static str),
  ParserErrorOwned(String),
  IdentifierNotFound(String),
  NotAFunction(&'static str),
  TypeMismatch(String),
  InvalidOperator(String),
  WrongNumberOfArguments {
    expected: usize,
    got: usize,
  },
  InvalidExpression(String),
  RuntimeError(String),
  UnexpectedToken {
    expected: TokenKind<'static>,
    found: TokenKind<'static>,
  },
}

impl fmt::Display for ErrorKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ErrorKind::ParserError(msg) => write!(f, "Parser Error: {}", msg),
      ErrorKind::ParserErrorOwned(msg) => {
        write!(f, "Parser Error: {}", msg)
      }
      ErrorKind::IdentifierNotFound(name) => {
        write!(f, "Identifier not found: {}", name)
      }
      ErrorKind::NotAFunction(name) => {
        write!(f, "Not a function: {}", name)
      }
      ErrorKind::TypeMismatch(msg) => write!(f, "Type mismatch: {}", msg),
      ErrorKind::InvalidOperator(op) => {
        write!(f, "Invalid operator: {}", op)
      }
      ErrorKind::WrongNumberOfArguments { expected, got } => {
        write!(f, "Expected {} arguments, but got {}", expected, got)
      }
      ErrorKind::InvalidExpression(msg) => {
        write!(f, "Invalid expression: {}", msg)
      }
      ErrorKind::RuntimeError(msg) => write!(f, "Runtime error: {}", msg),
      ErrorKind::UnexpectedToken { expected, found } => {
        write!(f, "Expected token {:?}, but got {:?}", expected, found)
      }
    }
  }
}
