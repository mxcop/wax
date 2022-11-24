#[derive(PartialEq, Debug, Clone)]
pub enum Token {
  OpeningTag(String),
  ClosedTag(String),
  ClosingTag(String),
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
