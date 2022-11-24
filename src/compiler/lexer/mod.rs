pub mod token;
pub mod html;

use std::slice::Iter;
use peekmore::PeekMoreIterator;
use token::Token;


fn is_tag_name(ch: char) -> bool {
  'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_' || ch == '-'
}

// fn is_letter(ch: char) -> bool {
//   'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_' || ch == '-' || ch == '#'
// }

// fn is_digit(ch: char) -> bool {
//   '0' <= ch && ch <= '9'
// }


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

  pub fn lex(&mut self) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    // Move through all the characters:
    while let Some(ch) = self.iter.next() {
      match ch {
        '<' => {
          if self.cmove('/') {

            // Read the next word as the tag name...
            if let Some(word) = self.rtag() {
              tokens.push(Token::ClosingTag(word.clone()));
            } else {
              // If there's no word then panic!
              panic!("Closing tag missing tag name");
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
                tokens.push(Token::OpeningTag(name));
                break;
              }

              // Or until we reach the '/'
              if *ch == '/' {
                tokens.push(Token::ClosedTag(name));
                self.iter.next();
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

  // pub fn read_char(&mut self) {
  //   if self.read_position >= self.input.len() {
  //     self.ch = '0';
  //   } else {
  //     self.ch = self.input[self.read_position];
  //   }
  //   self.position = self.read_position;
  //   self.read_position = self.read_position + 1;
  // }

  // pub fn skip_whitespace(&mut self) {
  //   let ch = self.ch;
  //   if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
  //     self.read_char();
  //     self.skip_whitespace();
  //   }
  // }

  // pub fn next_token(&mut self) -> Token {
  //   let read_identifier = |l: &mut Lexer| -> String {
  //     let position = l.position;
  //     while l.position < l.input.len() && is_letter(l.ch) {
  //       l.read_char();
  //     }
  //     l.input[position..l.position].into_iter().collect()
  //   };

  //   let read_tag = |l: &mut Lexer| -> String {
  //     let position = l.position + 1;
  //     while l.position < l.input.len() && l.input[l.position + 1] != '>' && (is_letter(l.ch) || l.ch == '<') {
  //       l.read_char();
  //     }
  //     if l.input[l.position + 1] != '>' {
  //       l.input[position..l.position].into_iter().collect()
  //     } else {
  //       l.input[position..l.position + 1].into_iter().collect()
  //     }
  //   };

  //   let read_number = |l: &mut Lexer| -> Vec<char> {
  //     let position = l.position;
  //     while l.position < l.input.len() && is_digit(l.ch) {
  //       l.read_char();
  //     }
  //     l.input[position..l.position].to_vec()
  //   };

  //   let tok: Token;
  //   self.skip_whitespace();
  //   match self.ch {
  //     '=' => {
  //       tok = Token::EQUALS(self.ch);
  //     }
  //     '+' => {
  //       tok = Token::PLUS(self.ch);
  //     }
  //     '-' => {
  //       tok = Token::MINUS(self.ch);
  //     }
  //     '!' => {
  //       tok = Token::BANG(self.ch);
  //     }
  //     '/' => {
  //       tok = Token::SLASH(self.ch);
  //     }
  //     '*' => {
  //       tok = Token::AST(self.ch);
  //     }
  //     '<' => {
  //       if is_letter(self.input[self.position + 1]) {
  //         let ident = read_tag(self);
  //         tok = Token::TAG(ident);
  //       } else {
  //         tok = Token::LT(self.ch);
  //       }
  //     }
  //     '>' => {
  //       tok = Token::GT(self.ch);
  //     }
  //     '?' => {
  //       tok = Token::QUEST(self.ch);
  //     }
  //     ';' => {
  //       tok = Token::SEMI(self.ch);
  //     }
  //     ':' => {
  //       tok = Token::COLON(self.ch);
  //     }
  //     '(' => {
  //       tok = Token::LPAREN(self.ch);
  //     }
  //     ')' => {
  //       tok = Token::RPAREN(self.ch);
  //     }
  //     ',' => {
  //       tok = Token::COMMA(self.ch);
  //     }
  //     '{' => {
  //       tok = Token::LBRACE(self.ch);
  //     }
  //     '}' => {
  //       tok = Token::RBRACE(self.ch);
  //     }
  //     '"' => {
  //       tok = Token::QUOT(self.ch);
  //     }
  //     '\'' => {
  //       tok = Token::APOS(self.ch);
  //     }
  //     '.' => {
  //       tok = Token::DOT(self.ch);
  //     }
  //     '@' => {
  //       tok = Token::COMMAT(self.ch);
  //     }
  //     '0' => {
  //       tok = Token::EOF;
  //     }
  //     _ => {
  //       if is_letter(self.ch) {
  //         let ident: String = read_identifier(self);
  //         match token::get_keyword_token(&ident) {
  //           Ok(keywork_token) => {
  //             return keywork_token;
  //           }
  //           Err(_err) => {
  //             return Token::IDENT(ident);
  //           }
  //         }
  //       } else if is_digit(self.ch) {
  //         let ident: Vec<char> = read_number(self);
  //         return Token::INT(ident);
  //       } else {
  //         println!("illegal token: '{}' ({})", self.ch, self.ch as u32);
  //         return Token::ILLEGAL;
  //       }
  //     }
  //   }
  //   self.read_char();
  //   tok
  // }
}
