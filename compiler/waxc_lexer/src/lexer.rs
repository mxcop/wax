use std::str::Chars;

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