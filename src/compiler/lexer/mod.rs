pub mod token;

use token::Token;

pub struct Lexer {
  input: Vec<char>,
  pub position: usize,
  pub read_position: usize,
  pub ch: char,
}

fn is_letter(ch: char) -> bool {
  'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_' || ch == '-' || ch == '#'
}

fn is_digit(ch: char) -> bool {
  '0' <= ch && ch <= '9'
}

impl Lexer {
  pub fn new(input: Vec<char>) -> Self {
    Self {
      input: input,
      position: 0,
      read_position: 0,
      ch: '0',
    }
  }

  pub fn read_char(&mut self) {
    if self.read_position >= self.input.len() {
      self.ch = '0';
    } else {
      self.ch = self.input[self.read_position];
    }
    self.position = self.read_position;
    self.read_position = self.read_position + 1;
  }

  pub fn skip_whitespace(&mut self) {
    let ch = self.ch;
    if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
      self.read_char();
      self.skip_whitespace();
    }
  }

  pub fn next_token(&mut self) -> Token {
    let read_identifier = |l: &mut Lexer| -> String {
      let position = l.position;
      while l.position < l.input.len() && is_letter(l.ch) {
        l.read_char();
      }
      l.input[position..l.position].into_iter().collect()
    };

    let read_number = |l: &mut Lexer| -> Vec<char> {
      let position = l.position;
      while l.position < l.input.len() && is_digit(l.ch) {
        l.read_char();
      }
      l.input[position..l.position].to_vec()
    };

    let tok: Token;
    self.skip_whitespace();
    match self.ch {
      '=' => {
        tok = Token::EQUALS(self.ch);
      }
      '+' => {
        tok = Token::PLUS(self.ch);
      }
      '-' => {
        tok = Token::MINUS(self.ch);
      }
      '!' => {
        tok = Token::BANG(self.ch);
      }
      '/' => {
        tok = Token::SLASH(self.ch);
      }
      '*' => {
        tok = Token::AST(self.ch);
      }
      '<' => {
        tok = Token::LT(self.ch);
      }
      '>' => {
        tok = Token::GT(self.ch);
      }
      '?' => {
        tok = Token::QUEST(self.ch);
      }
      ';' => {
        tok = Token::SEMI(self.ch);
      }
      ':' => {
        tok = Token::COLON(self.ch);
      }
      '(' => {
        tok = Token::LPAREN(self.ch);
      }
      ')' => {
        tok = Token::RPAREN(self.ch);
      }
      ',' => {
        tok = Token::COMMA(self.ch);
      }
      '{' => {
        tok = Token::LBRACE(self.ch);
      }
      '}' => {
        tok = Token::RBRACE(self.ch);
      }
      '"' => {
        tok = Token::QUOT(self.ch);
      }
      '\'' => {
        tok = Token::APOS(self.ch);
      }
      '.' => {
        tok = Token::DOT(self.ch);
      }
      '@' => {
        tok = Token::COMMAT(self.ch);
      }
      '0' => {
        tok = Token::EOF;
      }
      _ => {
        if is_letter(self.ch) {
          let ident: String = read_identifier(self);
          match token::get_keyword_token(&ident) {
            Ok(keywork_token) => {
              return keywork_token;
            }
            Err(_err) => {
              return Token::IDENT(ident);
            }
          }
        } else if is_digit(self.ch) {
          let ident: Vec<char> = read_number(self);
          return Token::INT(ident);
        } else {
          println!("illegal token: '{}' ({})", self.ch, self.ch as u32);
          return Token::ILLEGAL;
        }
      }
    }
    self.read_char();
    tok
  }
}
