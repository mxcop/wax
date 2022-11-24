use super::{token::Token, Lexer};

trait ScriptLexer {
  fn next_script_token(&mut self, ch: char) -> Token;
}

impl ScriptLexer for Lexer<'_> {
  fn next_script_token(&mut self, ch: char) -> Token {
    // Do script stuff...
    // Look for script end tag... 
    Token::ClosedTag(String::new())
  }
}
