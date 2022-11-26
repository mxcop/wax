#[derive(PartialEq, Debug, Clone)]
pub enum Token {
  Text(String),

  OpeningTag(String),
  ClosedTag(String),
  ClosingTag(String),

  DefaultImport{ name: String, path: String }
}

// pub fn get_keyword_token(ident: &String) -> Result<Token, String> {
//   match &ident[..] {
//     "import" => Ok(Token::IMPORT),
//     "export" => Ok(Token::EXPORT),
//     "from" => Ok(Token::FROM),
//     "function" => Ok(Token::FUNCTION),
//     "let" => Ok(Token::LET),
//     "const" => Ok(Token::CONST),
//     "true" => Ok(Token::TRUE),
//     "false" => Ok(Token::FALSE),
//     "if" => Ok(Token::IF),
//     "else" => Ok(Token::ELSE),
//     "return" => Ok(Token::RETURN),
//     _ => Err(String::from("Not a keyword")),
//   }
// }
