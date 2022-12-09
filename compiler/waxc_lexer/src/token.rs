use super::span::Span;

#[derive(Debug, Clone)]
pub struct SyntaxToken {
  pub kind: Token,
  span: Span,
}

impl SyntaxToken {
  pub fn new(kind: Token, idx: usize, len: usize) -> Self {
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
pub enum Token {
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

impl Token {
  pub fn to_string(&self) -> String {
    match &self {
      Token::Whitespace(_) => " ",
      Token::Newline => "\n",
      Token::Ident(ident) => ident,
      Token::Number(num) => num,
      Token::Comma => ",",
      Token::Dot => ".",
      Token::SingleQuote => "'",
      Token::DoubleQuote => r#"""#,
      Token::Grave => "`",
      Token::Colon => ":",
      Token::Semicolon => ";",
      Token::Plus => "+",
      Token::Minus => "-",
      Token::Equals => "=",
      Token::Star => "*",
      Token::Hash => "#",
      Token::Percent => "%",
      Token::Ampersand => "&",
      Token::Atsign => "@",
      Token::Dollarsign => "$",
      Token::Tilde => "~",
      Token::Slash => "/",
      Token::BackSlash => r"\",
      Token::Bang => "!",
      Token::Quest => "?",
      Token::LessThen => "<",
      Token::GreaterThen => ">",
      Token::LeftArrow => "<-",
      Token::RightArrow => "->",
      Token::LeftParenthesis => "(",
      Token::RightParenthesis => ")",
      Token::LeftCurlyBracket => "{",
      Token::RightCurlyBracket => "}",
      Token::LeftSquareBracket => "[",
      Token::RightSquareBracket => "]",
      Token::Template => "tmpl",
      Token::Implementation => "impl",
      Token::Stylesheet => "styl",
      Token::Let => "let",
      Token::Const => "const",
      Token::Import => "import",
      Token::Export => "export",
      Token::From => "from",
      Token::Function => "function",
      Token::True => "true",
      Token::False => "false",
      Token::If => "if",
      Token::Else => "else",
      Token::Return => "return",
      _ => { panic!("unknown token ({:?})", &self); }
    }.into()
  }

  /** Try to convert a string to a keyword. */
  pub fn from_string(s: &str) -> Option<Self> {
    match s {
      "tmpl"     => Some(Token::Template),
      "impl"     => Some(Token::Implementation),
      "styl"     => Some(Token::Stylesheet),
  
      "let"      => Some(Token::Let),
      "const"    => Some(Token::Const),
      "import"   => Some(Token::Import),
      "export"   => Some(Token::Export),
      "from"     => Some(Token::From),
      "function" => Some(Token::Function),
      "true"     => Some(Token::True),
      "false"    => Some(Token::False),
      "if"       => Some(Token::If),
      "else"     => Some(Token::Else),
      "return"   => Some(Token::Return),
      _ => None,
    }
  }
}