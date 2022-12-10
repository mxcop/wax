use std::str::Chars;

use crate::token::{Token, TokenKind};

/// The Wax lexer (tokenizer)
pub struct Lexer<'a> {
  cursor: usize,
  /* Chars<'a> slightly faster than &'a str */
  chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Self {
    Self {
      cursor: 0,
      chars: input.chars()
    }
  }
}