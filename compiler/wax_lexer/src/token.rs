//[derive(PartialEq, Debug, Clone)]
pub enum Token {
  // Generic
  Ident(String),       // ([a-zA-Z][a-zA-Z0-9_-]+)
  Comma,               // ,
  Dot,                 // .
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
pub fn get_keyword_token(ident: &String) -> Result<Token, String> {
  match &ident[..] {
    "tmpl"     => Ok(Token::Template),
    "impl"     => Ok(Token::Implementation),
    "styl"     => Ok(Token::Stylesheet),

    "let"      => Ok(Token::Let),
    "const"    => Ok(Token::Const),
    "import"   => Ok(Token::Import),
    "export"   => Ok(Token::Export),
    "from"     => Ok(Token::From),
    "function" => Ok(Token::Function),
    "true"     => Ok(Token::True),
    "false"    => Ok(Token::False),
    "if"       => Ok(Token::If),
    "else"     => Ok(Token::Else),
    "return"   => Ok(Token::Return),
    _ => Err(String::from("Not a keyword")),
  }
}
