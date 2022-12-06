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

    println!("found tmpl {:?}", tree.get(scope).get_name());

    // Move through all tokens until we reach a semicolon:
    while let Some(tk) = iter.next() {
      match tk {

        /* < */
        Token::LessThen => {
          if let Some(tk) = iter.peek() {
            match tk {
              /* Opening Tag <tag> */
              Token::Ident(ident) => {
                *curr = tree.add_child(*curr, ident.clone(), SyntaxNode::Tag { 
                  name: ident.clone(), 
                  attributes: vec![], 
                  self_closing: false 
                });
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
                *curr = tree.add_child(*curr, ident.clone(), SyntaxNode::Comb { 
                  name: ident.clone(), 
                  attributes: vec![], 
                  self_closing: false 
                });
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
          println!("\n - found semicolon level {} - ", *curr);
          if *curr == scope { println!("\n - found template ending - "); break; }
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
  fn parse_attributes<'a>(name: String, iter: &mut PeekMoreIterator<Iter<'a, Token>>) -> SyntaxNode {
    let mut attributes = Vec::new();
    let mut self_closing = false;
    
    while let Some(tk) = iter.next() {
      match tk {

        Token::Ident(ident) => {
          if let Some(Token::Equals) = Self::next_token(iter) {
            if let Some(value) = Self::parse_string(iter) {
              // Attribute with value:
              attributes.push(Attribute { 
                name: ident.clone(), 
                value: Some(value)
              });
            } else {
              panic!("attribute is missing its value");
            }
          } else {
            // Attribute without value:
            attributes.push(Attribute { 
              name: ident.clone(), 
              value: None 
            });
          }
        }

        /* / */
        Token::Slash => {
          // Found self closing tag.
          if let Some(Token::GreaterThen) = iter.peek() {
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

    SyntaxNode::Tag {
      name, attributes, self_closing
    }
  }

  fn parse_string<'a>(iter: &mut PeekMoreIterator<Iter<'a, Token>>) -> Option<String> {
    let mut word: Vec<char> = Vec::new();
    
    /* " */
    if let Some(Token::DoubleQuote) = iter.next() {
      while let Some(tk) = iter.next() {
         /* " */
        if let Token::DoubleQuote = tk {
          return Some(word.iter().collect());
        }
      }
    }

    None
  }

  /// Get the next token skipping any whitespace tokens.
  fn next_token<'a>(iter: &mut PeekMoreIterator<Iter<'a, Token>>) -> Option<&'a Token> {
    while let Some(tk) = iter.next() {
      if let Token::Whitespace(_) = tk {
        continue;
      } else {
        return Some(tk);
      }
    }
    None
  }
}