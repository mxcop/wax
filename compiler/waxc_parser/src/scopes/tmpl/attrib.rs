use waxc_errors::error::WaxError;
use waxc_lexer::{token::{Token, TokenKind}};

use crate::{node::{Attribute, NodeKind}, parser::Parser};

use super::void::is_void;

/// Parse the next attributes inside a token iterator.
pub fn parse_attributes<I: Iterator<Item = Token> + Clone>(name: String, pars: &mut Parser<I>, is_comb: bool) -> Result<NodeKind, WaxError> {
  let mut attributes: Vec<Attribute> = Vec::new();
  let mut self_closing = false;
  let mut hashed_attrib = false;
  
  while let Some(tk) = pars.next_with_cursor() {
    match tk.kind {

      /* # */
      TokenKind::Hash => {
        hashed_attrib = true;
      }

      /* name */
      TokenKind::Ident => {
        let mut ident = pars.read().to_string();

        /* # */
        if hashed_attrib {
          ident.insert(0, '#');
          hashed_attrib = false;
        }

        // Check for duplicate attributes:
        if attributes.iter().any(|attrib| attrib.name == ident) {
          // return Err(WaxError::from_token(dtk.clone(), 
          //   "duplicate attribute", 
          //   WaxHint::Hint("attribute names should be unique".into())
          // ));
          todo!();
        }

        pars.eat_while(TokenKind::Whitespace);

        /* = */
        let TokenKind::Eq = pars.first() else {
          // Attribute without value:
          attributes.push(Attribute { 
            name: ident, 
            value: None 
          });
          continue;
        };
        pars.next();

        // Parse the value of the attribute.
        if let Some(value) = parse_string(pars) {
          // Attribute with value:
          attributes.push(Attribute { 
            name: ident, 
            value: Some(value)
          });
        } else {
          // return Err(WaxError::from_token(dtk.clone(), 
          //   "attribute missing value", 
          //   WaxHint::Hint("attributes should have a value after the `=`".into())
          // ));
          todo!();
        }
      }

      /* / */
      TokenKind::Slash => {
        // Found self closing tag.
        if let TokenKind::Gt = pars.first() {
          pars.next();
          self_closing = true;
          break;
        }
      }

      /* > */
      TokenKind::Gt => {
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
    Ok(NodeKind::Comb {
      name, attributes, self_closing
    })
  } else {
    Ok(NodeKind::Tag {
      name, attributes, self_closing
    })
  }
}

/// Parse a string pattern.
fn parse_string<I: Iterator<Item = Token> + Clone>(pars: &mut Parser<I>) -> Option<String> {
  let mut word: String = String::new();
  let mut escaped: bool = false;
  let double_quoted: bool;

  pars.eat_while(TokenKind::Whitespace);

  /* " or ' */
  match pars.first() {
    TokenKind::DoubleQuote => double_quoted = true,
    TokenKind::Quote => double_quoted = false,
    _ => return None,
  }
  pars.next();

  while let Some(tk) = pars.next_with_cursor() {
    match tk.kind {
      /* Be aware of escape chars */
      TokenKind::BackSlash => escaped = true,
      /* " */
      TokenKind::DoubleQuote => {
        if !double_quoted || escaped {
          escaped = false;
        } else {
          return Some(word);
        }
      }
      /* ' */
      TokenKind::Quote => {
        if double_quoted || escaped {
          escaped = false;
        } else {
          return Some(word);
        }
      }
      TokenKind::EOF => return None,
      _ => ()
    }
    // Add the token to the string.
    word.push_str(&pars.read());
  }

  None
}