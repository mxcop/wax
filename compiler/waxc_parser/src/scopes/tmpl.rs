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
    // return Err(WaxError::from_token(tmpl_tk.clone(), 
    //   "missing `:` on template definition", 
    //   WaxHint::Example("`tmpl name:`".into())
    // )); 
    todo!();
  };
  pars.next();

  let scope = pars.get_scope();

  // Move through all tokens until we reach a semicolon:
  while let Some(tk) = pars.next() {
    match tk.kind {

      /* < */
      TokenKind::Lt => {
        pars.update_cursor();
        let Some(tk) = pars.next() else {
          break;
        };

        match tk.kind {
          /* Opening Tag <tag> */
          TokenKind::Ident => {
            let ident = pars.read();
            let tag = parse_attributes(ident.to_string(), pars, false)?;

            if let NodeKind::Tag { self_closing: false, .. } = tag {
              pars.add_scope(tag);
            } else {
              pars.add_node(tag);
            }
          }
          /* Closing Tag </tag> */
          TokenKind::Slash => {

            pars.eat_while(TokenKind::Whitespace);

            let TokenKind::Ident = pars.first() else {
              // return Err(WaxError::from_token(dtk.clone(), 
              //   "invalid end tag", 
              //   WaxHint::Example("</name>".into())
              // ));
              todo!();
            };
            pars.next();
            let ident = pars.read();

            pars.eat_while(TokenKind::Whitespace);

            let TokenKind::Gt = pars.first() else {
              // return Err(WaxError::from_token(dtk.clone(), 
              //   "invalid end tag", 
              //   WaxHint::Example("</name>".into())
              // ));
              todo!();
            };

            /* Void Tag */
            if is_void(&ident) {
              // let example = format!("void elements should only have a start tag `<{}>`", ident);
              // return Err(WaxError::from_token(dtk.clone(), 
              //   "void element with end tag", 
              //   WaxHint::Example(example)
              // ));
              todo!();
            }

            // Make sure we're closing the current scope:
            //if ident != tree.get(*curr).get_name() {
              // let tag = tree.get(*curr);
              // let hint = format!("try closing <{}> before it's parent tag is closed", tag.get_name());
              // return Err(WaxError::from_span(tag.get_span(), 
              //   "misnested tag", 
              //   WaxHint::Hint(hint)
              // ));
              //todo!();
            //}

            pars.retreat_scope();
            //*curr = tree.get_parent(*curr).expect("No parent");
          }
          _ => {}
        }
      }

      /* <- */
      TokenKind::LeftArrow => {
        pars.update_cursor();
        let Some(tk) = pars.next() else {
          break;
        };

        match tk.kind {
          /* Comb Opening Tag <-comb> */
          TokenKind::Ident => {
            let ident = pars.read();
            let tag = parse_attributes(ident.to_string(), pars, false)?;

            if let NodeKind::Comb { self_closing: false, .. } = tag {
              pars.add_scope(tag);
            } else {
              pars.add_node(tag);
            }
          }
          /* Comb Closing Tag <-/comb> */
          TokenKind::Slash => {

            pars.eat_while(TokenKind::Whitespace);

            let TokenKind::Ident = pars.first() else {
              // return Err(WaxError::from_token(dtk.clone(), 
              //   "invalid end tag", 
              //   WaxHint::Example("</name>".into())
              // ));
              todo!();
            };
            pars.next();
            let ident = pars.read();

            pars.eat_while(TokenKind::Whitespace);

            let TokenKind::Gt = pars.first() else {
              // return Err(WaxError::from_token(dtk.clone(), 
              //   "invalid end tag", 
              //   WaxHint::Example("</name>".into())
              // ));
              todo!();
            };

            /* Void Tag */
            if is_void(&ident) {
              // let example = format!("void elements should only have a start tag `<{}>`", ident);
              // return Err(WaxError::from_token(dtk.clone(), 
              //   "void element with end tag", 
              //   WaxHint::Example(example)
              // ));
              todo!();
            }

            // Make sure we're closing the current scope:
            //if ident != tree.get(*curr).get_name() {
              // let tag = tree.get(*curr);
              // let hint = format!("try closing <{}> before it's parent tag is closed", tag.get_name());
              // return Err(WaxError::from_span(tag.get_span(), 
              //   "misnested tag", 
              //   WaxHint::Hint(hint)
              // ));
              //todo!();
            //}

            pars.retreat_scope();
            //*curr = tree.get_parent(*curr).expect("No parent");
          }
          _ => {}
        }
      }

      /* ; */
      TokenKind::Semi => {
        if pars.get_scope() == scope { break; }
        // if *curr > scope {
        //   return Err(WaxError::from_span(tree.get(scope).get_span(), 
        //     "overflowing template", 
        //     WaxHint::Hint("make sure all tags witin the template are closed".into())
        //   ));
        // }
      }

      /* End of File */
      TokenKind::EOF => {
        // if *curr == scope {
        //   return Err(WaxError::from_token(dtk.clone(), 
        //     "dangling template", 
        //     WaxHint::Hint("templates should end with a `;`".into())
        //   ));
        // }
        break;
      }

      /* Whitespace */
      TokenKind::Whitespace => (),

      /* Unknown */
      TokenKind::Unknown => {
        // return Err(WaxError::from_token(dtk.clone(), 
        //   "unknown character",
        //   WaxHint::None
        // ));
        todo!();
      }

      /* Text */
      _ => { 
        // let mut text = pars.read();

        // while let tk = pars.first() {
        //   match tk {
        //     TokenKind::Lt | TokenKind::LeftArrow => break,
        //     TokenKind::Whitespace => { pars.next(); continue; },
        //     _ => { text.push_str(pars.read()); pars.next(); }
        //   }
        // }

        // tree.add_child(*curr, "text".into(), dtk.get_span(), SyntaxNode::Text(text));
      }
    }
  }

  // Move back out of this template.
  //*curr = tree.get_parent(*curr).expect("No parent");

  Ok(())
}
