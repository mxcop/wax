use super::{token::Token, Lexer, char::{is_letter, is_whitespace, is_string}};

pub trait ScriptLexer {
  fn rword(&mut self) -> Option<String>;
  fn rstr(&mut self) -> Option<String>;
  fn next_script_token(&mut self, ch: char, tokens: &mut Vec<Token>) -> bool;
}

impl<'a> ScriptLexer for Lexer<'a> {

  /// ### Read next Word
  fn rword(&mut self) -> Option<String> {
    let mut word: Vec<char> = vec![];
    while let Some(&ch) = self.iter.peek() {
      if word.len() == 0 && is_whitespace(*ch) {
        self.next();
      } else if is_letter(*ch) {
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

  /// ### Read next String
  fn rstr(&mut self) -> Option<String> {
    let mut word: Vec<char> = vec![];
    while let Some(&ch) = self.iter.peek() {
      if word.len() == 0 && is_whitespace(*ch) {
        self.next();
      } else if word.len() == 0 && *ch == '"' {
        self.next();
      } else if is_string(*ch) {
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

  /// Parse the next token.
  fn next_script_token(&mut self, ch: char, tokens: &mut Vec<Token>) -> bool {
    match ch {
      '<' => {
        if self.cmove('/') {
          // Read the next word as the tag name...
          if let Some(word) = self.rtag() {
            tokens.push(Token::ClosingTag(word.clone()));
            if word == "script" {
              return false;
            }
          } else {
            // If there's no word then panic!
            panic!("Closing tag missing tag name");
          }
        }
      }
      _ => {
        if is_letter(ch) {
          let keyword = self.rword();
          if let Some(mut keyword) = keyword {
            // Add the first char to the string.
            keyword.insert(0, ch);

            match keyword.as_str() {
              "import" => {

                let mut import_name = String::from("unknown");
                if let Some(name) = self.rword() {
                  import_name = name;
                }

                if let Some(from) = self.rword() {
                  if from.as_str() != "from" {
                    panic!("import missing path");
                  }
                }

                let mut import_path = String::from(".");
                if let Some(path) = self.rstr() {
                  import_path = path;
                }

                tokens.push(Token::DefaultImport { name: import_name, path: import_path });
              }
              _ => {
                //panic!("unknown keyword ({})", keyword);
              }
            }
          } else {
            panic!("illegal character ({})", ch);
          }
        }
      }
    }

    true
  }
}
