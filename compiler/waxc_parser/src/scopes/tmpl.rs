mod void;
mod attrib;

use waxc_errors::error::{WaxError, WaxHint};

use void::is_void;
use attrib::parse_attributes;
use waxc_lexer::token::{Token, TokenKind};

use crate::{parser::Parser, node::NodeKind};

/// Parse a template
pub fn parse<I: Iterator<Item = Token> + Clone>(
  pars: &mut Parser<I>,
) -> Result<(), WaxError> {

  pars.eat_while(TokenKind::Whitespace);

  // Check if there is a double dot after the name:
  let TokenKind::Colon = pars.first() else {
    return Err(pars.err_example(
      "missing `:` on template definition", 
      "tmpl <name>: <html>;"
    ));
  };
  pars.next();

  let scope = pars.get_scope();

  // Move through all tokens until we reach a semicolon:
  loop {
    let tk = pars.next_with_cursor();
    
    match tk.kind {

      /* < */
      TokenKind::Lt => {
        pars.checkpoint();
        pars.update_cursor();
        let tk = pars.next();
        
        match tk.kind {
          /* Opening Tag <tag> */
          TokenKind::Ident => {
            let ident = pars.read();
            let tag = parse_attributes(ident.to_string(), pars, false)?;

            if let NodeKind::Tag { self_closing: false, .. } = tag {
              pars.add_scope_with_checkpoint(tag);
            } else {
              pars.add_node_with_checkpoint(tag);
            }
          }
          /* Closing Tag </tag> */
          TokenKind::Slash => {

            pars.eat_while(TokenKind::Whitespace);
            pars.update_cursor();

            let TokenKind::Ident = pars.first() else {
              return Err(pars.err_example(
                "invalid end tag", 
                "</tag>"
              ));
            };
            pars.next();
            let ident = pars.read();

            pars.eat_while(TokenKind::Whitespace);

            let TokenKind::Gt = pars.first() else {
              return Err(pars.err_example(
                "invalid end tag", 
                "</tag>"
              ));
            };

            /* Void Tag */
            if is_void(&ident) {
              return Err(pars.err_hint(
                "void element with closing tag", 
                &format!("void elements only have an opening tag `<{}>`", ident)
              ));
            }
            pars.next();

            // Make sure we're closing the current scope:
            if ident != pars.scoped_node().get_name() {
              let tag = pars.scoped_node();
              let hint;
              if let NodeKind::Comb { .. } = tag.kind {
                hint = format!("try closing <-{}> before it's parent tag is closed", tag.get_name());
              } else {
                hint = format!("try closing <{}> before it's parent tag is closed", tag.get_name());
              }
              return Err(pars.err_node(tag, 
                "misnested tag", 
                WaxHint::Hint(hint)
              ));
            }

            pars.retreat_scope();
          }
          _ => {}
        }
      }

      /* <- */
      TokenKind::LeftArrow => {
        pars.checkpoint();
        pars.update_cursor();
        let tk = pars.next();

        match tk.kind {
          /* Comb Opening Tag <-comb> */
          TokenKind::Ident => {
            let ident = pars.read();
            let tag = parse_attributes(ident.to_string(), pars, true)?;

            if let NodeKind::Comb { self_closing: false, .. } = tag {
              pars.add_scope_with_checkpoint(tag);
            } else {
              pars.add_node_with_checkpoint(tag);
            }
          }
          /* Comb Closing Tag <-/comb> */
          TokenKind::Slash => {
            pars.eat_while(TokenKind::Whitespace);

            let TokenKind::Ident = pars.first() else {
              return Err(pars.err_example(
                "invalid end comb tag", 
                "<-/comb>"
              ));
            };
            pars.next_with_cursor();
            let ident = pars.read();

            pars.eat_while(TokenKind::Whitespace);

            let TokenKind::Gt = pars.first() else {
              return Err(pars.err_example(
                "invalid end comb tag", 
                "<-/comb>"
              ));
            };
            pars.next();

            // Make sure we're closing the current scope:
            if ident != pars.scoped_node().get_name() {
              let tag = pars.scoped_node();
              let hint;
              if let NodeKind::Comb { .. } = tag.kind {
                hint = format!("try closing <-{}> before it's parent tag is closed", tag.get_name());
              } else {
                hint = format!("try closing <{}> before it's parent tag is closed", tag.get_name());
              }
              return Err(pars.err_node(tag, 
                "misnested tag", 
                WaxHint::Hint(hint)
              ));
            }

            pars.retreat_scope();
          }
          _ => {}
        }
      }

      /* ; */
      TokenKind::Semi => {
        if pars.get_scope() == scope { break; }
        if pars.get_scope() > scope {
          let tag = pars.scoped_node();
          return Err(pars.err_hint(
            "overflowing template", 
            &format!("try closing <{}> before the `;`", tag.get_name())
          ));
        }
      }

      /* End of File */
      TokenKind::EOF => {
        if pars.get_scope() == scope {
          return Err(pars.err_hint(
            "dangling template", 
            "try closing the template with a `;`"
          ));
        }
        break;
      }

      /* Whitespace */
      TokenKind::Whitespace => (),

      /* Unknown */
      TokenKind::Unknown => {
        return Err(pars.err(
          "unknown character"
        ));
      }

      /* Text */
      _ => { 
        let mut text = pars.read();

        loop {
          match pars.first() {
            TokenKind::Lt | TokenKind::LeftArrow | TokenKind::EOF => break,
            _ => { pars.next_with_cursor(); text.push_str(&pars.read()); }
          }
        }

        pars.add_node(NodeKind::Text(text));
      }
    }
  }

  Ok(())
}
