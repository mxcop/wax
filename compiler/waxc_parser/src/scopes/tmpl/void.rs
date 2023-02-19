/// Const array of all void element names.
const VOIDS: [&str; 16] = [
  "area", "base", "br", "col", 
  "command", "embed", "hr", "img", 
  "input", "keygen", "link", 
  "meta", "param", "source", 
  "track", "wbr"
];

/// Is an element a void element?
pub fn is_void(ident: &str) -> bool {
  VOIDS.contains(&ident)
}