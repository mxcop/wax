use wax_lexer::{token::Token, iter::TokenIter};
use wax_logger::error::{WaxError, WaxHint};

use crate::{tree::ArenaTree, node::{SyntaxNode, Attribute}};

/// Wax template parser.
pub struct TemplateParser {}

impl TemplateParser {
  /// ### Parse Template
  pub fn parse_tmpl<'a>(
    iter: &mut TokenIter<'a>, 
    curr: &mut usize,
    tree: &mut ArenaTree<SyntaxNode>)
  -> Result<(), WaxError<'a>> {
    // Make sure that the next token is whitespace.
    if let Some((dtk, tk)) = iter.next_de() {
      match tk {
        Token::Whitespace(_) => {},
        _ => { 
          return Err(WaxError::from_token(dtk.clone(), 
            "`tmpl` must be followed by whitespace", 
            WaxHint::Example("`tmpl name:`")
          )); 
        }
      }
    }

    // And then there should be an indentifier for the template.
    if let Some((dtk, tk)) = iter.next_de() {
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
            if let Token::Ident(ident) = tk {
              let ident = format!("@{ident}");
              *curr = tree.add_child(*curr, ident.clone(), dtk.get_span().clone(), SyntaxNode::Template {
                name: ident.clone()
              });
            } else {
              return Err(WaxError::from_token(dtk.clone(), 
                "template name cannot only consist of `@`", 
                WaxHint::Hint("you can add a name after the `@`".into())
              )); 
            }
          }
        },
        _ => {
          return Err(WaxError::from_token(dtk.clone(), 
            "templates must have a name", 
            WaxHint::Example("`tmpl name:`")
          )); 
        }
      }
    }

    /* TODO: check for DoubleDot after the template name */

    let scope = *curr;

    // Move through all tokens until we reach a semicolon:
    while let Some((dtk, tk)) = iter.next_de() {
      match tk {

        /* < */
        Token::LessThen => {
          if let Some(tk) = iter.peek() {
            match tk {
              /* Opening Tag <tag> */
              Token::Ident(ident) => {
                iter.next();
                let tag = Self::parse_attributes(ident.clone(), iter, false);

                let tag_idx = tree.add_child(
                  *curr, 
                  ident.clone(), 
                  dtk.get_span().clone(),
                  tag.clone()
                );

                if let SyntaxNode::Tag { self_closing: false, .. } = tag {
                  *curr = tag_idx;
                }
              }
              /* Closing Tag </tag> */
              Token::Slash => {
                if let Some(Token::Ident(ident)) = iter.peek_next() {

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
                } else {
                  iter.retreat_cursor().expect("failed to move back cursor");
                }
              }
              _ => {}
            }
          }
        }

        /* <- */
        Token::LeftArrow => {
          if let Some((dtk, tk)) = iter.peek_de() {
            match tk {
              /* Comb Opening Tag <-comb> */
              Token::Ident(ident) => {
                iter.next();
                let tag = Self::parse_attributes(ident.clone(), iter, true);

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
                if let Some(Token::Ident(ident)) = iter.peek_next() {

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
                } else {
                  iter.retreat_cursor().expect("failed to move back cursor");
                }
              }
              _ => {}
            }
          }
        }

        /* ; */
        Token::Semicolon => {
          if *curr == scope { break; }
          if *curr > scope {
            return Err(WaxError::from_span(tree.get(scope).get_span().clone(), 
              "dangling template", 
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
  fn parse_attributes<'a>(name: String, iter: &mut TokenIter<'a>, is_comb: bool) -> SyntaxNode {
    let mut attributes = Vec::new();
    let mut self_closing = false;
    let mut hashed_attrib = false;
    
    while let Some(tk) = iter.next() {
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
          if let Some(Token::Equals) = Self::peek_next_token(iter) {
            // Advance the cursor.
            iter.next_until_cursor(); iter.next();

            // Parse the value of the attribute.
            if let Some(value) = Self::parse_string(iter) {
              // Attribute with value:
              attributes.push(Attribute { 
                name: ident, 
                value: Some(value)
              });
            } else {
              panic!("attribute is missing its value");
            }
          } else {
            // Attribute without value:
            attributes.push(Attribute { 
              name: ident, 
              value: None 
            });
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
      SyntaxNode::Comb {
        name, attributes, self_closing
      }
    } else {
      SyntaxNode::Tag {
        name, attributes, self_closing
      }
    }
  }

  /// Parse a string pattern.
  fn parse_string<'a>(iter: &mut TokenIter<'a>) -> Option<String> {
    let mut word: String = String::new();
    let mut escaped: bool = false;

    /* " */
    if let Some(Token::DoubleQuote) = iter.next() {
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
          _ => {}
        }
        // Add the token to the string.
        word.push_str(&stk.get_str());
      }
    }

    None
  }

  /// Get the next token skipping any whitespace tokens.
  fn peek_next_token<'a>(iter: &mut TokenIter<'a>) -> Option<&'a Token> {
    
    if let Some(tk) = iter.peek() {
      if let Token::Whitespace(_) = tk {} else {
        return Some(tk);
      }
    }

    while let Some(tk) = iter.peek_next() {
      if let Token::Whitespace(_) = tk {
        continue;
      } else {
        return Some(tk);
      }
    }
    None
  }
}