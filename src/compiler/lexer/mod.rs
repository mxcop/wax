mod token;

use self::token::Token;

/// Wax component lexer.
pub fn lex(input: &String) -> Result<Vec<Token>, String> {
  let mut result = Vec::new();

  let mut it = input.chars().peekable();
  while let Some(&c) = it.peek() {
    // Do lexing...
  }
  Ok(result)
}
