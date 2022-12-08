pub mod token;
pub mod span;
pub mod iter;
mod char;

use self::char::{is_ident, is_number};
use iter::TrackingIter;
use token::{Token, SyntaxToken};

pub struct Lexer<'a> {
  iter: TrackingIter<'a, char>
}

impl<'a> Lexer<'a> {
  pub fn new(input: TrackingIter<'a, char>) -> Self {
    Self {
      iter: input
    }
  }

  fn next(&mut self) -> Option<&char> {
    self.iter.next()
  }

  /// ### Get length of next whitespace
  fn whitespace(&mut self, first_ch: char) -> usize {
    let mut space: Vec<char> = vec![first_ch];

    while let Some(&ch) = self.iter.peek() {
      if *ch == ' ' {
        self.next();
        space.push(*ch);
      } else {
        break;
      }
    }

    space.len()
  }

  /// Read the next `n` amount of chars that all match the predicate.<br>
  /// ```
  /// predicate = fn(is_first: bool, ch: &char) -> bool
  /// ```
  fn read_next(&mut self, 
    first_ch: &char,
    predicate: fn(bool, &char) -> bool) 
  -> Option<String> {

    let mut word: Vec<char> = Vec::new();

    if predicate(true, first_ch) {
      word.push(*first_ch);
      while let Some(&ch) = self.iter.peek() {
        if predicate(false, ch) {
          self.next();
          word.push(*ch);
        } else {
          return Some(word.iter().collect());
        }
      }
    }

    None
  }

  /// ### Conditional Move
  /// Returns whether the next character is equal to the given character.<br>
  /// If true then moves the iterator forward by one.
  fn cmove(&mut self, ch: char) -> bool {
    match self.iter.peek() {
      Some(&ch2) if *ch2 == ch => {
        self.next();
        true
      }
      _ => false
    }
  }

  /// ### Lexical Analysis
  /// Analize the input and convert it into an array of tokens.
  pub fn lex(&mut self, char_count: usize) -> Vec<SyntaxToken> {
    let mut tokens: Vec<SyntaxToken> = Vec::with_capacity(char_count);

    // Move through all the characters:
    while let Some(&ch) = self.next() {

      let Some(token) = self.evaluate(&ch) else {
        continue;
      };

      // Push the token onto the stack.
      tokens.push(token);
    }

    // End of File reached.
    tokens.push(SyntaxToken::new(
      Token::EOF,
      self.iter.current_pos() - 1, 0
    ));
    tokens
  }

  /// Evaluate the next character.
  fn evaluate(&mut self, ch: &char) -> Option<SyntaxToken> {
    /* Spaces */
    if *ch == ' ' {
      let len = self.whitespace(*ch);
      return Some(SyntaxToken::new(
        Token::Whitespace(len), 
        self.iter.current_pos(), len
      ));
    }

    /* Newlines */
    if *ch == '\n' {
      return Some(SyntaxToken::new(
        Token::Newline, 
        self.iter.current_pos(), 1
      ));
    }

    // Evaluate which token this char is:
    let Some(token) = self.token_from_char(ch) else {
      return None;
    };

    // Return the evaluated token:
    Some(SyntaxToken::new(
      token.clone(),
      self.iter.current_pos(), token.to_string().len()
    ))
  }

  /// Evaluate which token a char should become.
  fn token_from_char(&mut self, ch: &char) -> Option<Token> {

    // Ignore return chars:
    if *ch == '\r' {
      return None;
    }

    let token = match ch {
      /* Systematic */
      ' ' => Token::Whitespace(self.whitespace(*ch)),
      '\n' => Token::Newline,

      /* Generic */
      ',' => Token::Comma,
      '.' => Token::Dot,
      '\'' => Token::SingleQuote,
      '"' => Token::DoubleQuote,
      '`' => Token::Grave,
      ':' => Token::Colon,
      ';' => Token::Semicolon,

      /* Math Symbols */
      '+' => Token::Plus,
      '-' => { if self.cmove('>') { Token::RightArrow } else { Token::Minus } },
      '=' => Token::Equals,
      '*' => Token::Star,
      '/' => Token::Slash,

      /* Special Symbols */
      '#' => Token::Hash,
      '%' => Token::Percent,
      '&' => Token::Ampersand,
      '@' => Token::Atsign,
      '$' => Token::Dollarsign,
      '~' => Token::Tilde,
      '\\' => Token::BackSlash,
      '!' => Token::Bang,
      '?' => Token::Quest,

      /* Closures */
      '<' => { if self.cmove('-') { Token::LeftArrow } else { Token::LessThen } },
      '>' => Token::GreaterThen,
      '(' => Token::LeftParenthesis,
      ')' => Token::RightParenthesis,
      '{' => Token::LeftCurlyBracket,
      '}' => Token::RightCurlyBracket,
      '[' => Token::LeftSquareBracket,
      ']' => Token::RightSquareBracket,

      /* Words */
      _ => {
        self.token_from_ident(ch)
      }
    };

    Some(token)
  }

  /// Evaluate which token an identity should be.
  fn token_from_ident(&mut self, first_ch: &char) -> Token {
    /* Identity [a-zA-Z] */
    if let Some(identity) 
      = self.read_next(first_ch, is_ident) 
    {
      /* Keywords */
      if let Some(keyword) = Token::from_string(&identity) {
        keyword
      } else {
        Token::Ident(identity)
      }
    } 
    /* Numbers [0-9] */
    else if let Some(number) 
      = self.read_next(first_ch, is_number) 
    {
      Token::Number(number)
    } 
    else {
      // If the char wasn't matched and isn't an identity, it's illegal.
      Token::Illegal(*first_ch)
    }
  }
}
