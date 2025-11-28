use std::fmt::{Display, Formatter, Result};

use strum_macros::AsRefStr;

use crate::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
  pub span: Span,
  pub kind: ErrorKind,
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", self.kind)
  }
}

macro_rules! define_error_kind {
    (
        enum $name:ident {
            $( $variant:ident(String), )*
        }
    ) => {
        #[derive(Debug, Clone, PartialEq, AsRefStr)]
        #[strum(serialize_all = "PascalCase")]
        pub enum $name {
            $( $variant(String), )*
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
                let msg = match self {
                    $( Self::$variant(msg) => msg, )*
                };
                write!(f, "{}: {}", self.as_ref(), msg)
            }
        }

        impl $name {
            pub fn message(&self) -> &str {
                match self {
                    $( Self::$variant(msg) => msg, )*
                }
            }
        }
    }
}

define_error_kind! {
    enum ErrorKind {
        RuntimeError(String),
        SyntaxError(String),
        TypeError(String),
        ReferenceError(String),
        ArgumentError(String),
    }
}
