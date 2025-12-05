use std::fmt;

use phf::{phf_map, Map};
use strum_macros::AsRefStr;

pub static KEYWORDS: Map<&'static str, TokenKind> = phf_map! {
    "const" => TokenKind::Const,
    "fn" => TokenKind::Fn,
    "pub" => TokenKind::Pub,
    "let" => TokenKind::Let,
    "if" => TokenKind::If,
    "elif" => TokenKind::Elif,
    "else" => TokenKind::Else,
    "match" => TokenKind::Match,
    "return" => TokenKind::Return,
    "while" => TokenKind::While,
    "break" => TokenKind::Break,
    "continue" => TokenKind::Continue,
    "test" => TokenKind::Test,
    "true" => TokenKind::True,
    "false" => TokenKind::False,
    "import" => TokenKind::Import,
    "as" => TokenKind::As,
    "null" => TokenKind::Null,
    "i8" => TokenKind::I8,
    "i16" => TokenKind::I16,
    "i32" => TokenKind::I32,
    "i64" => TokenKind::I64,
    "i128" => TokenKind::I128,
    "isize" => TokenKind::Isize,
    "u8" => TokenKind::U8,
    "u16" => TokenKind::U16,
    "u32" => TokenKind::U32,
    "u64" => TokenKind::U64,
    "u128" => TokenKind::U128,
    "usize" => TokenKind::Usize,
    "f32" => TokenKind::F32,
    "f64" => TokenKind::F64,
    "bool" => TokenKind::Bool,
    "str" => TokenKind::Str,
};

macro_rules! define_token_enum {
    (
        enum $name:ident <'a> {
            keywords: {
                $( $kw_variant:ident, )*
            },
            units: {
                $(
                    $unit_variant:ident,
                )*
            },
            data: {
                $(
                    $data_variant:ident(&'a str),
                )*
            }
        }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, AsRefStr)]
        #[strum(serialize_all = "lowercase")]
        pub enum $name<'a> {
            $( $kw_variant, )*
            $(
                $unit_variant,
            )*
            $( $data_variant(&'a str), )*
        }

        impl<'a> $name<'a> {
            pub fn into_static(self) -> $name<'static> {
                match self {
                    $( $name::$data_variant(_) => $name::$data_variant("..."), )*
                    $( $name::$kw_variant => $name::$kw_variant, )*
                    $( $name::$unit_variant => $name::$unit_variant, )*
                }
            }

            pub fn is_keyword(&self) -> bool {
                matches!(self, $( $name::$kw_variant )|*)
            }
        }
    };
}

define_token_enum! {
    enum TokenKind<'a> {
        keywords: {
            Const, Fn, Pub, Let, If, Elif, Else, Match, While, Return, Break, Continue, Test, True, False, Import, As, Null,
            I8, I16, I32, I64, I128, Isize, U8, U16, U32, U64, U128, Usize, F32, F64, Bool, Str,
        },
        units: {
            Plus,
            PlusPlus,
            Minus,
            MinusMinus,
            Star,
            StarStar,
            Slash,
            Percent,
            Equal,
            EqualEqual,
            Bang,
            BangEqual,
            GreaterThan,
            GreaterThanEqual,
            LessThan,
            LessThanEqual,
            PlusEqual,
            MinusEqual,
            StarEqual,
            SlashEqual,
            PercentEqual,
            Ampersand,
            AmpersandAmpersand,
            Pipe,
            PipePipe,
            Caret,
            Tilde,
            LessLess,
            GreaterGreater,
            AmpersandEqual,
            PipeEqual,
            CaretEqual,
            LessLessEqual,
            GreaterGreaterEqual,

            // Delimiters
            LParen,
            RParen,
            LBrace,
            RBrace,
            LBracket,
            RBracket,
            Comma,
            Dot,
            DotDot,
            Arrow,
            Colon,
            Semicolon,
            Newline,

            // Other
            Unknown,
            Eof,
        },
        data: {
            Integer(&'a str),
            Float(&'a str),
            String(&'a str),
            Identifier(&'a str),
        }
    }
}

impl<'a> fmt::Display for TokenKind<'a> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.is_keyword() {
      return f.write_str(self.as_ref());
    }
    let s: &dyn fmt::Display = match self {
      TokenKind::Plus => &"+",
      TokenKind::PlusPlus => &"++",
      TokenKind::Minus => &"-",
      TokenKind::MinusMinus => &"--",
      TokenKind::Star => &"*",
      TokenKind::StarStar => &"**",
      TokenKind::Slash => &"/",
      TokenKind::Percent => &"%",
      TokenKind::Equal => &"=",
      TokenKind::EqualEqual => &"==",
      TokenKind::Bang => &"!",
      TokenKind::BangEqual => &"!=",
      TokenKind::GreaterThan => &">",
      TokenKind::GreaterThanEqual => &">=",
      TokenKind::LessThan => &"<",
      TokenKind::LessThanEqual => &"<=",
      TokenKind::PlusEqual => &"+=",
      TokenKind::MinusEqual => &"-=",
      TokenKind::StarEqual => &"*=",
      TokenKind::SlashEqual => &"/=",
      TokenKind::PercentEqual => &"%=",
      TokenKind::Ampersand => &"&",
      TokenKind::AmpersandAmpersand => &"&&",
      TokenKind::Pipe => &"|",
      TokenKind::PipePipe => &"||",
      TokenKind::Caret => &"^",
      TokenKind::Tilde => &"~",
      TokenKind::LessLess => &"<<",
      TokenKind::GreaterGreater => &">>",
      TokenKind::AmpersandEqual => &"&=",
      TokenKind::PipeEqual => &"|=",
      TokenKind::CaretEqual => &"^=",
      TokenKind::LessLessEqual => &"<<=",
      TokenKind::GreaterGreaterEqual => &">>=",
      TokenKind::LParen => &"(",
      TokenKind::RParen => &")",
      TokenKind::LBrace => &"{",
      TokenKind::RBrace => &"}",
      TokenKind::LBracket => &"[",
      TokenKind::RBracket => &"]",
      TokenKind::Comma => &",",
      TokenKind::Dot => &".",
      TokenKind::DotDot => &"..",
      TokenKind::Arrow => &"=>",
      TokenKind::Colon => &":",
      TokenKind::Semicolon => &";",
      TokenKind::Newline => &"\n",
      TokenKind::Unknown => &"Unknown",
      TokenKind::Eof => &"Eof",
      TokenKind::Integer(s) => s,
      TokenKind::Float(s) => s,
      TokenKind::String(s) => s,
      TokenKind::Identifier(s) => s,
      _ => unreachable!(),
    };
    write!(f, "{}", s)
  }
}
