/** Is this character part of an identity? */
pub fn is_ident(is_first: bool, ch: &char) -> bool {
  if is_first {
    'a' <= *ch && *ch <= 'z' || 
    'A' <= *ch && *ch <= 'Z'
  } else {
    'a' <= *ch && *ch <= 'z' || 
    'A' <= *ch && *ch <= 'Z' || 
    '0' <= *ch && *ch <= '9' ||

    *ch == '_' || *ch == '-'
  }
}

pub fn is_number(_: bool, ch: &char) -> bool {
  '0' <= *ch && *ch <= '9' || *ch == '.'
}