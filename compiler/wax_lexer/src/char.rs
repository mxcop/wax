pub fn is_tag_name(ch: char) -> bool {
  'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_' || ch == '-'
}

pub fn is_whitespace(ch: char) -> bool {
  ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r'
}

pub fn is_letter(ch: char) -> bool {
  'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z'
}

pub fn is_string(ch: char) -> bool {
  '0' <= ch && ch <= '9' || 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '.' || ch == '/' || ch == '-' || ch == '_' || ch == '#' || ch == '@'
}

// fn is_digit(ch: char) -> bool {
//   '0' <= ch && ch <= '9'
// }