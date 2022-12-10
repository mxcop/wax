/** Is this character part of an identity? */
pub fn is_ident(is_first: bool, ch: &char) -> bool {
  if is_first {
    'a' <= *ch && *ch <= 'z' || 'A' <= *ch && *ch <= 'Z'
  } else {
    'a' <= *ch && *ch <= 'z'
      || 'A' <= *ch && *ch <= 'Z'
      || '0' <= *ch && *ch <= '9'
      || *ch == '_'
      || *ch == '-'
  }
}

/* Is this character part of a number? */
pub fn is_number(_: bool, ch: &char) -> bool {
  '0' <= *ch && *ch <= '9' || *ch == '.'
}

/// Is this character a space according to the HTML language definition?
///
/// See [HTML language reference](https://www.w3.org/TR/2011/WD-html-markup-20110113/syntax.html)
/// for definitions of these classes.
pub fn is_space(ch: &char) -> bool {
  matches!(
    ch,
      '\u{0020}' // Space
    | '\u{0009}' // Character Tabulation (tab)
    | '\u{000A}' // Line Feed (LF)
    | '\u{000C}' // Form Feed (FF)
    | '\u{000D}' // Carriage Return (CR)
  )
}
