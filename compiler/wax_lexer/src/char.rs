/** Is this character the start of a new identity? */
pub fn is_ident_start(ch: char) -> bool {
  'a' <= ch && ch <= 'z' || 
  'A' <= ch && ch <= 'Z'
}

/** Is this character still part of the identity? */
pub fn is_ident(ch: char) -> bool {
  'a' <= ch && ch <= 'z' || 
  'A' <= ch && ch <= 'Z' || 
  '0' <= ch && ch <= '9' ||

  ch == '_' || ch == '-'
}

pub fn is_number(ch: char) -> bool {
  '0' <= ch && ch <= '9' || ch == '.'
}