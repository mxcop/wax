use std::str::Chars;

use crate::token::{Token, EOF_TOKEN, TokenKind};

/// The Wax lexer (tokenizer)
#[derive(Clone)]
pub(crate) struct Lexer<'a> {
  remaining: usize,
  /* Chars<'a> slightly faster than &'a str */
  chars: Chars<'a>,
}

pub(crate) const EOF_CHAR: char = '\0';

impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Self {
    Self {
      remaining: input.len(),
      chars: input.chars()
    }
  }

  /// Moves to the next character.
  pub fn next(&mut self) -> Option<char> {
    self.chars.next()
  }

  /// Peeks the next symbol from the input stream without consuming it.
  /// Returns `EOF_CHAR` if the next char was None.
  pub fn peek(&self) -> char {
    // `.next()` optimizes better than `.nth(0)`
    self.chars.clone().next().unwrap_or(EOF_CHAR)
  }

  /// Returns amount of chars consumed since last reset.
  pub fn len_since_last_reset(&self) -> usize {
    (self.remaining - self.chars.as_str().len()) as usize
  }

  /// Resets the number of chars consumed to 0.
  pub fn reset_len(&mut self) {
    self.remaining = self.chars.as_str().len();
  }

  /// Are we at the end of the file?
  pub fn is_eof(&self) -> bool {
    self.chars.as_str().is_empty()
  }

  /// Eats symbols while predicate returns true or until the end of file is reached.
  pub fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
    while predicate(self.peek()) && !self.is_eof() {
      self.next();
    }
  }
}

/// Lexical iterator abstraction.
/// Makes interacting with the lexical iterator easier.
pub struct LexIter<I: Iterator<Item=Token> + Clone> {
  iter: I,
}

impl<I: Iterator<Item=Token> + Clone> LexIter<I> {
  pub fn new(iter: I) -> Self {
    Self { iter }
  }
}

impl<I: Iterator<Item=Token> + Clone> LexIter<I> {  
  /// Move to the next token in the iterator consuming it.
  pub fn next(&mut self) -> Option<Token> {
    self.iter.next()
  }

  /// Get the type of the next token without consuming it.
  pub fn first(&mut self) -> TokenKind {
    self.iter.clone().next().unwrap_or(EOF_TOKEN).kind
  }

  /// Get the type of the second token without consuming it.
  pub fn second(&mut self) -> TokenKind {
    let mut clone = self.iter.clone();
    clone.next();
    clone.next().unwrap_or(EOF_TOKEN).kind
  }

  /// Eat tokens while they match a kind of token.
  pub fn eat_while(&mut self, kind: TokenKind) -> usize {
    let mut clone = self.iter.clone();
    let mut count = 0usize;
    while let Some(tk) = clone.next() {
      if tk.kind != kind {
        break;
      } count += 1;
    }
    let mut len = 0usize;
    for _ in 0..count {
      len += self.next().unwrap().len();
    }
    len
  }

  /// Eat tokens until one matches a kind of token.
  pub fn eat_until(&mut self, kind: TokenKind) -> usize {
    let mut clone = self.iter.clone();
    let mut count = 0usize;
    while let Some(tk) = clone.next() {
      if tk.kind == kind {
        break;
      } count += 1;
    }
    let mut len = 0usize;
    for _ in 0..count {
      len += self.next().unwrap().len();
    }
    len
  }

  /// Eat a scope of opening and closing tokens.
  pub fn eat_scope(&mut self, open: TokenKind, close: TokenKind) -> Result<usize, ()> {
    let mut clone = self.iter.clone();
    let mut count: usize = 0;
    let mut depth: usize = 0;
    while let Some(tk) = clone.next() {
      count += 1;
      match tk.kind {
        kind if kind == open => {
          depth += 1;
        }
        kind if kind == close => {
          depth -= 1;
          if depth == 0 {
            break;
          }
        }
        _ => ()
      }
    }

    if depth != 0 {
      /* Scope wasn't closed properly */
      return Err(());
    }

    let mut len = 0usize;
    for _ in 0..count {
      len += self.next().unwrap().len();
    }
    Ok(len)
  }
}