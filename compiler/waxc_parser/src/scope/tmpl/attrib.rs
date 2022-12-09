use waxc_errors::error::{WaxError, WaxHint};
use waxc_lexer::{iter::TokenIter, token::Token};

use crate::node::{SyntaxNode, Attribute};

use super::void::is_void;

/// Parse the next attributes inside a token iterator.
pub fn parse_attributes<'a>(name: String, iter: &mut TokenIter<'a>, is_comb: bool) -> Result<SyntaxNode, WaxError<'a>> {
  let mut attributes = Vec::new();
  let mut self_closing = false;
  let mut hashed_attrib = false;
  
  while let Some((dtk, tk)) = iter.next_de() {
    match tk {

      /* # */
      Token::Hash => {
        hashed_attrib = true;
      }

      /* name */
      Token::Ident(ident) => {
        let mut ident = ident.clone();

        /* # */
        if hashed_attrib {
          ident.insert(0, '#');
          hashed_attrib = false;
        }

        iter.eat_whitespace();

        /* = */
        let Some(Token::Equals) = iter.peek() else {
          // Attribute without value:
          attributes.push(Attribute { 
            name: ident, 
            value: None 
          });
          continue;
        };
        iter.next();

        // Parse the value of the attribute.
        if let Some(value) = parse_string(iter) {
          // Attribute with value:
          attributes.push(Attribute { 
            name: ident, 
            value: Some(value)
          });
        } else {
          return Err(WaxError::from_token(dtk.clone(), 
            "attribute missing value", 
            WaxHint::Hint("attributes should have a value after the `=`".into())
          ));
        }
      }

      /* / */
      Token::Slash => {
        // Found self closing tag.
        if let Some(Token::GreaterThen) = iter.peek() {
          iter.next();
          self_closing = true;
          break;
        }
      }

      /* > */
      Token::GreaterThen => {
        break;
      }

      _ => {}
    }
  }

  /* Void elements */
  if is_void(&name) {
    self_closing = true;
  }

  if is_comb {
    Ok(SyntaxNode::Comb {
      name, attributes, self_closing
    })
  } else {
    Ok(SyntaxNode::Tag {
      name, attributes, self_closing
    })
  }
}

/// Parse a string pattern.
fn parse_string<'a>(iter: &mut TokenIter<'a>) -> Option<String> {
  let mut word: String = String::new();
  let mut escaped: bool = false;
  let double_quoted: bool;

  iter.eat_whitespace();

  /* " or ' */
  match iter.next() {
    Some(Token::DoubleQuote) => double_quoted = true,
    Some(Token::SingleQuote) => double_quoted = false,
    _ => return None,
  }

  while let Some((stk, tk)) = iter.next_de() {
    match tk {
      /* Be aware of escape chars */
      Token::BackSlash => escaped = true,
      /* " */
      Token::DoubleQuote => {
        if !double_quoted || escaped {
          escaped = false;
        } else {
          return Some(word);
        }
      }
      /* ' */
      Token::SingleQuote => {
        if double_quoted || escaped {
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