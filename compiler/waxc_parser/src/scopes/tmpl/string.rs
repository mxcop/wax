/// Parse a string pattern.
pub fn parse_string<'a>(iter: &mut TokenIter<'a>) -> Option<String> {
  let mut word: String = String::new();
  let mut escaped: bool = false;

  /* " */
  let Some(Token::DoubleQuote) = iter.next() else {
    return None;
  };

  while let Some((stk, tk)) = iter.next_de() {
    match tk {
      /* Be aware of escape chars */
      Token::BackSlash => escaped = true,
      /* " */
      Token::DoubleQuote => {
        if escaped {
          escaped = false;
        } else {
          return Some(word);
        }
      }
      Token::EOF => return None,
      _ => ()
    }
    // Add the token to the string.
    word.push_str(&stk.get_str());
  }

  None
}