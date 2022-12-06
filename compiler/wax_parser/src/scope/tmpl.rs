use peekmore::PeekMoreIterator;
use wax_lexer::token::Token;
use std::slice::Iter;

use crate::{tree::ArenaTree, node::{SyntaxNode, Attribute}};

/// Wax template parser.
pub struct TemplateParser {}

impl TemplateParser {
  /// ### Parse Template
  pub fn parse_tmpl<'a>(
    iter: &mut PeekMoreIterator<Iter<'a, Token>>, 
    curr: &mut usize,
    tree: &mut ArenaTree<SyntaxNode>)
  {
    // Make sure that the next token is whitespace.
    if let Some(Token::Whitespace(_)) = iter.next() {} 
    else { panic!("No whitespace after `tmpl`"); }

    // And then there should be an indentifier for the template.
    if let Some(Token::Ident(ident)) = iter.peek() {
      *curr = tree.add_child(*curr, ident.clone(), SyntaxNode::Template {
        name: ident.clone()
      });
      iter.next();
    } 
    // Also check if the identifier starts with an atsign.
    else if let Some(Token::Atsign) = iter.next() {
      if let Some(Token::Ident(ident)) = iter.next() {
        let ident = format!("@{ident}");
        *curr = tree.add_child(*curr, ident.clone(), SyntaxNode::Template {
          name: ident.clone()
        });
      } 
    }
    else { panic!("No name after `tmpl`"); }
    let scope = *curr;

    // Move through all tokens until we reach a semicolon:
    while let Some(tk) = iter.next() {
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
                  tag.clone()
                );

                if let SyntaxNode::Tag { self_closing: false, .. } = tag {
                  *curr = tag_idx;
                }
              }
              /* Closing Tag </tag> */
              Token::Slash => {
                if let Some(Token::Ident(_)) = iter.peek_next() {
                  *curr = tree.get_parent(*curr).expect("No parent");
                } else {
                  iter.move_cursor_back().expect("failed to move back cursor");
                }
              }
              _ => {}
            }
          }
        }

        /* <- */
        Token::LeftArrow => {
          if let Some(tk) = iter.peek() {
            match tk {
              /* Comb Opening Tag <-comb> */
              Token::Ident(ident) => {
                iter.next();
                let tag = Self::parse_attributes(ident.clone(), iter, true);

                let tag_idx = tree.add_child(
                  *curr, 
                  ident.clone(), 
                  tag.clone()
                );

                if let SyntaxNode::Comb { self_closing: false, .. } = tag {
                  *curr = tag_idx;
                }
              }
              /* Comb Closing Tag <-/comb> */
              Token::Slash => {
                if let Some(Token::Ident(_)) = iter.peek_next() {
                  *curr = tree.get_parent(*curr).expect("No parent");
                } else {
                  iter.move_cursor_back().expect("failed to move back cursor");
                }
              }
              _ => {}
            }
          }
        }

        /* ; */
        Token::Semicolon => {
          if *curr == scope { break; }
        }

        /* End of File */
        Token::EOF => {
          if *curr != scope { panic!("template overflow, forgot to close a tag within <{}>", tree.get(*curr).get_name()); }
          else { panic!("dangling template, (don't forget to close your templates using `;`)"); }
        }

        _ => {}
      }
    }

    // Move back out of this template.
    *curr = tree.get_parent(*curr).expect("No parent");
  }

  /// Parse the attributes of a tag.
  fn parse_attributes<'a>(name: String, iter: &mut PeekMoreIterator<Iter<'a, Token>>, is_comb: bool) -> SyntaxNode {
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
            iter.truncate_iterator_to_cursor(); iter.next();

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
  fn parse_string<'a>(iter: &mut PeekMoreIterator<Iter<'a, Token>>) -> Option<String> {
    let mut word: String = String::new();
    let mut escaped: bool = false;

    /* " */
    if let Some(Token::DoubleQuote) = iter.next() {
      while let Some(tk) = iter.next() {
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
        word.push_str(&tk.to_string());
      }
    }

    None
  }

  /// Get the next token skipping any whitespace tokens.
  fn peek_next_token<'a>(iter: &mut PeekMoreIterator<Iter<'a, Token>>) -> Option<&'a Token> {
    
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