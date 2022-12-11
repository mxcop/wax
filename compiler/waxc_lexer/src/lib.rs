mod lexer;
pub mod token;

use lexer::Lexer;
use token::{Token, TokenKind, LiteralKind};
use TokenKind::*;
use LiteralKind::*;

/// Tokenize an input file token by token using an iterater from fn.
pub fn lex(input: &str) -> impl Iterator<Item = Token> + '_ {
  let mut lexer = Lexer::new(input);
  std::iter::from_fn(move || {
    let token = lexer.advance();
    if token.kind != EOF {
      Some(token)
    } else {
      None
    }
  })
}

impl Lexer<'_> {
  /// Determine the next token from the input file.
  /// Will return an EOF token when the end of the file is reached.
  pub fn advance(&mut self) -> Token {
    let Some(first_char) = self.next() else {
      return Token::new(EOF, 0);
    };

    let token = match first_char {
      // Slash, comment or block comment.
      '/' => match self.peek() {
        '/' => self.line_comment(),
        '*' => self.block_comment(),
        _ => Slash,
      },

      // Whitespace sequence.
      c if is_whitespace(c) => self.whitespace(),

      // Identifier.
      c if is_id_start(c) => self.ident(),

      // Number literal.
      '0'..='9' | '.' => {
        let literal_kind = self.number();
        TokenKind::Literal {
          kind: literal_kind
        }
      }

      // Single Character Tokens:
      ';' => Semi,
      ',' => Comma,
      '(' => OpenParen,
      ')' => CloseParen,
      '{' => OpenBrace,
      '}' => CloseBrace,
      '[' => OpenBracket,
      ']' => CloseBracket,
      '@' => Atsign,
      '#' => Hash,
      '~' => Tilde,
      '?' => Quest,
      ':' => Colon,
      '$' => Dollar,
      '=' => Eq,
      '!' => Bang,
      '>' => Gt,
      '&' => And,
      '|' => Or,
      '+' => Plus,
      '*' => Star,
      '^' => Caret,
      '%' => Percent,
      '\\' => BackSlash,

      // Double Character Tokens:
      '-' => match self.peek() {
        '>' => { self.next(); RightArrow },
        _ => Minus,
      },
      '<' => match self.peek() {
        '-' => { self.next(); LeftArrow },
        _ => Lt,
      },

      // String literal.
      '"' => DoubleQuote,
      '\'' => Quote,
      '`' => Grave,
      // '"' => {
      //   let terminated = self.double_quoted_string();
      //   if terminated {
      //     self.eat_literal_suffix();
      //   }
      //   let kind = Str { terminated };
      //   Literal { kind }
      // }
      // '\'' => {
      //   let terminated = self.single_quoted_string();
      //   if terminated {
      //     self.eat_literal_suffix();
      //   }
      //   let kind = Str { terminated };
      //   Literal { kind }
      // }

      _ => Unknown,
    };

    let token = Token::new(token, self.len_since_last_reset());
    self.reset_len();
    token
  }

  fn line_comment(&mut self) -> TokenKind {
    self.next();
    self.eat_while(|ch| ch != '\n');
    LineComment
  }

  fn block_comment(&mut self) -> TokenKind {
    self.next();

    // Keep track of the depth of the comment.
    // We only want to close it once the depth is zero.
    let mut depth: usize = 0;
    while let Some(ch) = self.next() {
      match ch {
        '/' if self.peek() == '*' => {
          self.next();
          depth += 1;
        }
        '*' if self.peek() == '/' => {
          self.next();
          if depth == 0 {
            break;
          }
          depth -= 1;
        }
        _ => (),
      }
    }

    BlockComment
  }

  fn whitespace(&mut self) -> TokenKind {
    self.eat_while(is_whitespace);
    Whitespace
  }

  fn ident(&mut self) -> TokenKind {
    self.eat_while(is_ident);
    Ident
  }

  // fn double_quoted_string(&mut self) -> bool {
  //   self.next();
  //   self.eat_while(|ch| !matches!(ch, '"'));
  //   !self.is_eof()
  // }

  // fn single_quoted_string(&mut self) -> bool {
  //   self.next();
  //   self.eat_while(|ch| !matches!(ch, '\''));
  //   !self.is_eof()
  // }

  fn number(&mut self) -> LiteralKind {
    self.eat_while(is_number);
    Number
  }

  // Just eat the next char for now.
  // fn eat_literal_suffix(&mut self) {
  //   self.next();
  // }
}

/// Is this character whitespace according to the HTML language definition?
///
/// See [HTML language reference](https://www.w3.org/TR/2011/WD-html-markup-20110113/syntax.html)
/// for definitions of these classes.
fn is_whitespace(ch: char) -> bool {
  matches!(
    ch,
      '\u{0020}' // Space
    | '\u{0009}' // Character Tabulation (tab)
    | '\u{000A}' // Line Feed (LF)
    | '\u{000C}' // Form Feed (FF)
    | '\u{000D}' // Carriage Return (CR)
  )
}

/// Is this character the start of an identity?
fn is_id_start(ch: char) -> bool {
  'a' <= ch && ch <= 'z' || 
  'A' <= ch && ch <= 'Z'
}

/// Is this character a part of an identity?
fn is_ident(ch: char) -> bool {
  'a' <= ch && ch <= 'z' || 
  'A' <= ch && ch <= 'Z' || 
  '0' <= ch && ch <= '9' || 
  ch == '_' || ch == '-'
}

/// Is this character a part of a number?
fn is_number(ch: char) -> bool {
  '0' <= ch && ch <= '9' || ch == '.'
}