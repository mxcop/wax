use wax_lexer::{token::{Token, SyntaxToken}, iter::TokenIter};
use wax_logger::error::{WaxError, WaxHint};

use crate::{tree::ArenaTree, node::{SyntaxNode, Attribute}};

/// ### Parse Template
pub fn parse<'a>(
  iter: &mut TokenIter<'a>, 
  tmpl_tk: &'a SyntaxToken,
  curr: &mut usize,
  tree: &mut ArenaTree<SyntaxNode>)
-> Result<(), WaxError<'a>> {

  // Check if there is whitespace after the `tmpl` keyword:
  let Some(Token::Whitespace(_)) = iter.next() else {
    return Err(WaxError::from_token(tmpl_tk.clone(), 
      "`tmpl` must be followed by whitespace", 
      WaxHint::Example("`tmpl name:`")
    )); 
  };

  let Some((dtk, tk)) = iter.next_de() else {
    return Err(WaxError::from_token(tmpl_tk.clone(), 
      "missing name on template definition", 
      WaxHint::Example("`tmpl name:`")
    )); 
  };

  // And then there should be an indentifier for the template.
  match tk {
    /* <name> */
    Token::Ident(ident) => {
      *curr = tree.add_child(*curr, ident.clone(), dtk.get_span().clone(), SyntaxNode::Template {
        name: ident.clone()
      });
    },
    /* @ */
    Token::Atsign => {
      if let Some((dtk, tk)) = iter.next_de() {
        /* <name> */
        let Token::Ident(ident) = tk else {
          return Err(WaxError::from_token(dtk.clone(), 
            "invalid template name `@`", 
            WaxHint::Hint("you can add a name after the `@`".into())
          ));
        };

        let ident = format!("@{ident}");
        *curr = tree.add_child(*curr, ident.clone(), dtk.get_span().clone(), SyntaxNode::Template {
          name: ident.clone()
        });
      }
    },
    _ => {
      return Err(WaxError::from_token(tmpl_tk.clone(), 
        "missing name on template definition", 
        WaxHint::Example("`tmpl name:`")
      )); 
    }
  }

  // Check if there is a double dot after the name:
  let Some(Token::Colon) = peek_next_token(iter) else {
    return Err(WaxError::from_token(tmpl_tk.clone(), 
      "missing `:` on template definition", 
      WaxHint::Example("`tmpl name:`")
    )); 
  };
  iter.next_until_cursor();

  let scope = *curr;

  // Move through all tokens until we reach a semicolon:
  while let Some((dtk, tk)) = iter.next_de() {
    match tk {

      /* < */
      Token::LessThen => {
        let Some((dtk, tk)) = iter.peek_de() else {
          continue;
        };

        match tk {
          /* Opening Tag <tag> */
          Token::Ident(ident) => {
            iter.next();
            let tag = parse_attributes(ident.clone(), iter, false)?;

            let tag_idx = tree.add_child(
              *curr, 
              ident.clone(), 
              dtk.get_span_offset(1, 1).clone(),
              tag.clone()
            );

            if let SyntaxNode::Tag { self_closing: false, .. } = tag {
              *curr = tag_idx;
            }
          }
          /* Closing Tag </tag> */
          Token::Slash => {

            let Some(Token::Ident(ident)) = iter.peek_next() else {
              iter.retreat_cursor().expect("failed to move back cursor");
              continue;
            };

            // Make sure we're closing the current scope:
            if ident != tree.get(*curr).get_name() {
              let tag = tree.get(*curr);
              let hint = format!("try closing <{}> before it's parent tag is closed", tag.get_name());
              return Err(WaxError::from_span(tag.get_span().clone(), 
                "misnested tag", 
                WaxHint::Hint(hint)
              ));
            }

            *curr = tree.get_parent(*curr).expect("No parent");
          }
          _ => {}
        }
      }

      /* <- */
      Token::LeftArrow => {
        let Some((dtk, tk)) = iter.peek_de() else {
          continue;
        };

        match tk {
          /* Comb Opening Tag <-comb> */
          Token::Ident(ident) => {
            iter.next();
            let tag = parse_attributes(ident.clone(), iter, true)?;

            let tag_idx = tree.add_child(
              *curr, 
              ident.clone(), 
              dtk.get_span().clone(),
              tag.clone()
            );

            if let SyntaxNode::Comb { self_closing: false, .. } = tag {
              *curr = tag_idx;
            }
          }
          /* Comb Closing Tag <-/comb> */
          Token::Slash => {

            let Some(Token::Ident(ident)) = iter.peek_next() else {
              iter.retreat_cursor().expect("failed to move back cursor");
              continue;
            };

            // Make sure we're closing the current scope:
            if ident != tree.get(*curr).get_name() {
              let tag = tree.get(*curr);
              let hint = format!("try closing <{}> before it's parent tag is closed", tag.get_name());
              return Err(WaxError::from_span(tag.get_span().clone(), 
                "misnested tag", 
                WaxHint::Hint(hint)
              ));
            }

            *curr = tree.get_parent(*curr).expect("No parent");
          }
          _ => {}
        }
      }

      /* ; */
      Token::Semicolon => {
        if *curr == scope { break; }
        if *curr > scope {
          return Err(WaxError::from_span(tree.get(scope).get_span().clone(), 
            "overflowing template", 
            WaxHint::Hint("make sure all tags witin the template are closed".into())
          ));
        }
      }

      /* End of File */
      Token::EOF => {
        if *curr == scope {
          return Err(WaxError::from_token(dtk.clone(), 
            "dangling template", 
            WaxHint::Hint("templates should end with a `;`".into())
          ));
        }
      }

      _ => {}
    }
  }

  // Move back out of this template.
  *curr = tree.get_parent(*curr).expect("No parent");

  Ok(())
}

/// Parse the attributes of a tag.
fn parse_attributes<'a>(name: String, iter: &mut TokenIter<'a>, is_comb: bool) -> Result<SyntaxNode, WaxError<'a>> {
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

        /* = */
        let Some(Token::Equals) = peek_next_token(iter) else {
          // Attribute without value:
          attributes.push(Attribute { 
            name: ident, 
            value: None 
          });
          continue;
        };

        // Advance the cursor.
        iter.next_until_cursor(); iter.next();

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

/// Get the next token skipping any whitespace tokens.
fn peek_next_token<'a>(iter: &mut TokenIter<'a>) -> Option<&'a Token> {
  
  let tk = iter.peek();
  let Some(Token::Whitespace(_)) = tk else {
    return tk;
  };

  while let Some(tk) = iter.peek_next() {
    let Token::Whitespace(_) = tk else {
      return Some(tk);
    };
  }
  None
}