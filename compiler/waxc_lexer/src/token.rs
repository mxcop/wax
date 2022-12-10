use super::span::Span;

#[derive(Debug, Clone)]
pub struct Token {
  pub kind: TokenKind,
  span: Span,
}

impl Token {
  pub fn new(kind: TokenKind, idx: usize, len: usize) -> Self {
    Self {
      kind,
      span: Span::new(idx, len)
    }
  }

  /// Get the span of the token.
  pub fn get_span(&self) -> &Span {
    &self.span
  }

  /// Get the span of the token with an offset.
  pub fn get_span_offset(&self, left: usize, right: usize) -> Span {
    let mut span = self.span.clone();
    span.start_index += left;
    span.length += right + left;
    span
  }

  /// Get the string representation of the token.
  pub fn get_str(&self) -> String {
    self.kind.to_string()
  }
}

#[derive(PartialEq, Debug, Clone)]
pub enum TokenKind {
  // Systematic
  Illegal(char),
  Whitespace(usize),
  Newline,
  EOF,

  // Generic
  Ident(String),       // ([a-zA-Z][a-zA-Z0-9_-]+)
  Number(String),      // ([.0-9]+)
  Comma,               // ,
  Dot,                 // .
  SingleQuote,         // '
  DoubleQuote,         // "
  Grave,               // `
  Colon,               // :
  Semicolon,           // ;
  Plus,                // +
  Minus,               // -
  Equals,              // =
  Star,                // *
  Hash,                // #
  Percent,             // %
  Ampersand,           // &
  Atsign,              // @
  Dollarsign,          // $
  Tilde,               // ~
  Slash,               // /
  BackSlash,           // \
  Bang,                // !
  Quest,               // ?
  LessThen,            // <
  GreaterThen,         // >
  LeftArrow,           // <-
  RightArrow,          // ->
  LeftParenthesis,     // (
  RightParenthesis,    // )
  LeftCurlyBracket,    // {
  RightCurlyBracket,   // }
  LeftSquareBracket,   // [
  RightSquareBracket,  // ]

  // Special Keywords
  Template,            // tmpl
  Implementation,      // impl
  Stylesheet,          // styl

  // Keywords
  Let,                 // let
  Const,               // const
  Import,              // import
  Export,              // export
  From,                // from
  Function,            // function
  True,                // true
  False,               // false
  If,                  // if
  Else,                // else
  Return,              // return
}

use TokenKind::*;

impl TokenKind {
  pub fn to_string(&self) -> String {
    match &self {
      Whitespace(_) => " ",
      Newline => "\n",
      Ident(ident) => ident,
      Number(num) => num,
      Comma => ",",
      Dot => ".",
      SingleQuote => "'",
      DoubleQuote => r#"""#,
      Grave => "`",
      Colon => ":",
      Semicolon => ";",
      Plus => "+",
      Minus => "-",
      Equals => "=",
      Star => "*",
      Hash => "#",
      Percent => "%",
      Ampersand => "&",
      Atsign => "@",
      Dollarsign => "$",
      Tilde => "~",
      Slash => "/",
      BackSlash => r"\",
      Bang => "!",
      Quest => "?",
      LessThen => "<",
      GreaterThen => ">",
      LeftArrow => "<-",
      RightArrow => "->",
      LeftParenthesis => "(",
      RightParenthesis => ")",
      LeftCurlyBracket => "{",
      RightCurlyBracket => "}",
      LeftSquareBracket => "[",
      RightSquareBracket => "]",
      Template => "tmpl",
      Implementation => "impl",
      Stylesheet => "styl",
      Let => "let",
      Const => "const",
      Import => "import",
      Export => "export",
      From => "from",
      Function => "function",
      True => "true",
      False => "false",
      If => "if",
      Else => "else",
      Return => "return",
      _ => { panic!("unknown token ({:?})", &self); }
    }.into()
  }

  /** Try to convert a string to a keyword. */
  pub fn from_string(s: &str) -> Option<Self> {
    match s {
      "tmpl"     => Some(Template),
      "impl"     => Some(Implementation),
      "styl"     => Some(Stylesheet),
  
      "let"      => Some(Let),
      "const"    => Some(Const),
      "import"   => Some(Import),
      "export"   => Some(Export),
      "from"     => Some(From),
      "function" => Some(Function),
      "true"     => Some(True),
      "false"    => Some(False),
      "if"       => Some(If),
      "else"     => Some(Else),
      "return"   => Some(Return),
      _ => None,
    }
  }
}