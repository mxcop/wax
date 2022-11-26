pub mod token;
pub mod script;
mod char;

use std::slice::Iter;
use peekmore::PeekMoreIterator;
use script::ScriptLexer;
use token::Token;

use self::char::{is_tag_name, is_string};

pub struct Lexer<'a> {
  file: String,
  filename: String,
  iter: PeekMoreIterator<Iter<'a, char>>,
  index: usize,
}

impl<'a> Lexer<'a> {
  pub fn new(file: String, filename: String, input: PeekMoreIterator<Iter<'a, char>>) -> Self {
    Self {
      file,
      filename,
      iter: input,
      index: 0
    }
  }

  fn next(&mut self) -> Option<&char> {
    self.index += 1;
    self.iter.next()
  }

  /// ### Read next Tag
  fn rtag(&mut self) -> Option<String> {
    let mut word: Vec<char> = Vec::new();
    while let Some(&ch) = self.iter.peek() {
      if is_tag_name(*ch) {
        self.next();
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

  /// ### Read next Text
  fn rtext(&mut self) -> Option<String> {
    let mut word: Vec<char> = Vec::new();
    while let Some(&ch) = self.iter.peek() {
      if (*ch == ' ' && word.len() > 0) || is_string(*ch) {
        self.next();
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
    let mut script: bool = false;

    // Move through all the characters:
    while let Some(ch) = self.iter.next() {
      self.index += 1;

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
            }

          } else {
            let name: String;

            // Read the next word as the tag name...
            if let Some(word) = self.rtag() {
              name = word;
            } else {
              continue;
            }

            // Read the next words as attributes...
            while let Some(&ch) = self.iter.peek() {

              // Until we reach the '>'
              if *ch == '>' {
                if name == "script" {
                  self.next();
                  script = true;
                }
                tokens.push(Token::OpeningTag(name));
                break;
              }

              // Or until we reach the '/'
              if *ch == '/' {
                tokens.push(Token::ClosedTag(name));
                self.next();
                
                break;
              }

              self.next();
            }
          }
        }
        _ => {
          // Read the next word as the tag name...
          if let Some(text) = self.rtext() {
            tokens.push(Token::Text(text));
          }
        }
      }
    }

    tokens
  }

  fn bail(&self, desc: &str, idx: usize, tip: Option<&str>) {
    use wax_logger::bail;

    let line_num = self.file[..idx].chars().filter(|x| *x == '\n').count();
    let line = find_line_start(&self.file, idx)..find_line_end(&self.file, idx);
  
    bail(desc, &self.filename, None, line_num, &self.file[line], tip);
  }
}

// Functions below were sourced from `https://github.com/vallentin/line-span/blob/master/src/lib.rs`

fn find_line_start(text: &str, index: usize) -> usize {
  text[..index].rfind('\n').map_or(0, |i| i + 1)
}

fn find_line_end(text: &str, index: usize) -> usize {
  let end = text[index..]
      .find('\n')
      .map_or_else(|| text.len(), |i| index + i);

  if (end > 0) && (text.as_bytes()[end - 1] == b'\r') {
      end - 1
  } else {
      end
  }
}
