pub mod token;
mod char;

use std::{slice::Iter, iter::Peekable};
use self::char::{is_ident_start, is_ident, is_number};
use token::Token;

pub struct Lexer<'a> {
  iter: Peekable<Iter<'a, char>>
}

impl<'a> Lexer<'a> {
  pub fn new(input: Peekable<Iter<'a, char>>) -> Self {
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

  /// ### Read next number
  fn number(&mut self, first_ch: char) -> Option<String> {
    let mut word: Vec<char> = Vec::new();

    if is_number(first_ch) {
      word.push(first_ch);
      while let Some(&ch) = self.iter.peek() {
        if is_number(*ch) {
          self.next();
          word.push(*ch);
        } else {
          return Some(word.iter().collect());
        }
      }
    }

    None
  }

  /// ### Read next identity
  fn ident(&mut self, first_ch: char) -> Option<String> {
    let mut word: Vec<char> = Vec::new();

    if is_ident_start(first_ch) {
      word.push(first_ch);
      while let Some(&ch) = self.iter.peek() {
        if is_ident(*ch) {
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
  pub fn lex(&mut self) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    // Move through all the characters:
    while let Some(&ch) = self.next() {

      // Handle spaces.
      if ch == ' ' {
        tokens.push(Token::Whitespace(self.whitespace(ch)));
        continue;
      }

      // Handle newlines.
      if ch == '\n' {
        tokens.push(Token::Newline);
        continue;
      }

      // Html part of the file:
      let token: Token = match ch {
        // Systematic
        ' ' => Token::Whitespace(self.whitespace(ch)),
        '\n' => Token::Newline,
        '\r' => continue,

        // Generic
        ',' => Token::Comma,
        '.' => Token::Dot,
        '\'' => Token::SingleQuote,
        '"' => Token::DoubleQuote,
        '`' => Token::Grave,
        ':' => Token::Colon,
        ';' => Token::Semicolon,

        '+' => Token::Plus,
        '-' => { if self.cmove('>') { Token::RightArrow } else { Token::Minus } },
        '=' => Token::Equals,
        '*' => Token::Star,

        '#' => Token::Hash,
        '%' => Token::Percent,
        '&' => Token::Ampersand,
        '@' => Token::Atsign,
        '$' => Token::Dollarsign,
        '~' => Token::Tilde,
        '/' => Token::Slash,
        '\\' => Token::BackSlash,
        '!' => Token::Bang,
        '?' => Token::Quest,

        '<' => { if self.cmove('-') { Token::LeftArrow } else { Token::LessThen } },
        '>' => Token::GreaterThen,
        '(' => Token::LeftParenthesis,
        ')' => Token::RightParenthesis,
        '{' => Token::LeftCurlyBracket,
        '}' => Token::RightCurlyBracket,
        '[' => Token::LeftSquareBracket,
        ']' => Token::RightSquareBracket,

        _ => {
          // Read the next word as an identity:
          if let Some(identity) = self.ident(ch) {
            if let Some(keyword) = Token::from_string(&identity) {
              keyword // Found keyword.
            } else {
              Token::Ident(identity)
            }
          } else if let Some(number) = self.number(ch) {
            Token::Number(number)
          } else {
            // If the char wasn't matched and isn't an identity, it's illegal.
            Token::Illegal(ch)
          }
        }
      };

      // Push the token onto the stack.
      tokens.push(token.clone());
    }

    tokens.push(Token::EOF);
    tokens
  }
}
