#[derive(Debug, Clone)]
pub enum Token {
  // Systematic
  Illegal(char),
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

/** Try to convert an identity to a keyword. */
pub fn get_keyword_token(ident: &String) -> Option<Token> {
  match &ident[..] {
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
