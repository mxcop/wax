pub mod token;
pub mod script;
mod char;

use std::slice::Iter;
use peekmore::PeekMoreIterator;
use script::ScriptLexer;
use token::Token;

use self::char::{is_tag_name, is_whitespace};

pub struct Lexer<'a> {
  iter: PeekMoreIterator<Iter<'a, char>>,
}

impl<'a> Lexer<'a> {
  pub fn new(input: PeekMoreIterator<Iter<'a, char>>) -> Self {
    Self {
      iter: input,
    }
  }

  /// ### Read next Tag
  fn rtag(&mut self) -> Option<String> {
    let mut word: Vec<char> = Vec::new();
    while let Some(&ch) = self.iter.peek() {
      if is_tag_name(*ch) {
        self.iter.next();
        word.push(*ch);
      } else {
        break;
      }
    }
    if word.len() > 0 {
      Some(word.into_iter().collect())
    } else {
      None
    }
  }

  /// ### Conditional Move
  /// Returns whether the next character is equal to the given character.<br>
  /// If true then moves the iterator forward by one.
  fn cmove(&mut self, ch: char) -> bool {
    match self.iter.peek() {
      Some(&ch2) if *ch2 == ch => {
        self.iter.next();
        true
      }
      _ => false
    }
  }

  /// ### Lexical Analysis
  /// Analize the input and convert it into an array of tokens.
  pub fn lex(&mut self) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut script: bool = false;

    // Move through all the characters:
    while let Some(ch) = self.iter.next() {

      // Script part of the file:
      if script {
        script = self.next_script_token(*ch, &mut tokens);
        continue;
      }

      // Html part of the file:
      match ch {
        '<' => {
          if self.cmove('/') {

            // Read the next word as the tag name...
            if let Some(word) = self.rtag() {
              if word == "script" {
                script = false;
              }
              tokens.push(Token::ClosingTag(word.clone()));
            } else {
              // If there's no word then panic!
              panic!("Closing tag missing tag name");
            }

            while let Some(&ch) = self.iter.peek() {
              if !is_whitespace(*ch) {
                break;
              }
              self.iter.next();
            }

            // Read the next char which should be '>' if not panic!
            if !self.cmove('>') {
              panic!("Closing tag missing closing bracket");
            }

          } else {
            let name: String;

            // Read the next word as the tag name...
            if let Some(word) = self.rtag() {
              name = word;
            } else {
              // If there's no word then panic!
              panic!("Closing tag missing tag name");
            }

            // Read the next words as attributes...
            while let Some(&ch) = self.iter.peek() {

              // Until we reach the '>'
              if *ch == '>' {
                if name == "script" {
                  self.iter.next();
                  script = true;
                }
                tokens.push(Token::OpeningTag(name));
                break;
              }

              // Or until we reach the '/'
              if *ch == '/' {
                tokens.push(Token::ClosedTag(name));
                self.iter.next();
                
                // Read the next char which should be '>' if not panic!
                if !self.cmove('>') {
                  panic!("Closing tag missing closing bracket");
                }

                break;
              }

              self.iter.next();
            }
          }
        }
        _ => {}
      }
    }

    tokens
  }
}
