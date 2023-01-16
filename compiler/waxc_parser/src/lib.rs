pub mod node;
mod parser;
mod scopes;
pub mod tree;

use node::NodeKind;
use parser::Parser;
use scopes::tmpl;
use tree::AST;

use waxc_errors::error::WaxError;
use waxc_lexer::{token::{Token, TokenKind}, lexer::LexIter};

/// Parse an input stream of tokens into an abstract syntax tree.
pub fn parse<'a, I: Iterator<Item = Token> + Clone>(
  file: String,
  iter: LexIter<I>,
) -> Result<AST, WaxError> {

  let mut parser 
    = Parser::new(file, iter);

  /* Move through all the tokens */
  loop {
    if parser.advance()? {
      continue;
    }
    break;
  }

  Ok(parser.get_tree())
}

impl<I: Iterator<Item = Token> + Clone> Parser<I> {
  /// Parse the next token from the lexer.
  pub fn advance(&mut self) -> Result<bool, WaxError> {
    use TokenKind::*;
    self.update_cursor();
    
    // Read the next token:
    let tk = self.next();

    match tk.kind {
      
      Ident => match self.read().as_str() {
        "use"  => self.using()?,
        "tmpl" => self.template()?,
        "impl" => self.implementation()?,
        "styl" => self.stylesheet()?,
        _ => (),
      },

      EOF => return Ok(false),

      _ => (),
    };

    Ok(true)
  }

  /// Try parsing a using statement. (`use "...";`)
  fn using(&mut self) -> Result<(), WaxError> {
    let path = self.string()?;

    self.add_node(NodeKind::Using { path });

    self.eat_while(TokenKind::Whitespace);
    if self.next().kind != TokenKind::Semi {
      return Err(self.err_example(
        "missing semicolon on use", 
        "use \"/path\";"
      ));
    }

    Ok(())
  }

  fn string(&mut self) -> Result<String, WaxError> {
    /* Eat whitespace */
    self.eat_while(TokenKind::Whitespace);
    self.update_cursor();

    if self.next().kind != TokenKind::DoubleQuote {
      return Err(self.err_example(
        "unformatted import path", 
        "use \"/path\";"
      ));
    }

    /* File names cannot include double quotes 
      so we can just eat until the next one */
    self.eat_until(TokenKind::DoubleQuote);
    self.next();
    
    Ok(self.read())
  }

  /// Check if an identity is really the start of a template.
  /// If so then start the template parser.
  fn template(&mut self) -> Result<(), WaxError> {
    let Some(name) = self.declaration()? else {
      return Err(self.err_example(
        "missing template name", 
        "tmpl <name>: <html>;"
      ));
    };

    /* Create the template node */
    self.add_scope(NodeKind::Template {
      name: name.to_string()
    });

    tmpl::parse(self)?;
    self.retreat_scope();

    Ok(())
  }

  /// Check if an identity is really the start of a implementation.
  /// If so then just parse the entire contents as text.
  fn implementation(&mut self) -> Result<(), WaxError> {
    let Some(name) = self.declaration()? else {
      return Err(self.err_example(
        "missing implementation name", 
        "impl <name>() { <js> }"
      ));
    };

    /* Create the implementation node */
    self.add_scope(NodeKind::Implementation {
      name: name.to_string()
    });

    /* Eat until we reach the end of the implementation */
    self.eat_until(TokenKind::OpenBrace);
    self.update_cursor();
    self.eat_scope(TokenKind::OpenBrace, TokenKind::CloseBrace)?;

    /* Read contents and remove the outer braces */
    let content = self.read().trim_matches(|c| c == '{' || c == '}').to_string();

    /* Put the contents into a text node and retreat */
    self.add_node(NodeKind::Text(content));
    self.retreat_scope();

    Ok(())
  }

  /// Check if an identity is really the start of a stylesheet.
  /// If so then just parse the entire contents as text.
  fn stylesheet(&mut self) -> Result<(), WaxError> {
    let Some(name) = self.declaration()? else {
      return Err(self.err_example(
        "missing stylesheet name", 
        "styl <name>() { <css> }"
      ));
    };

    /* Create the stylesheet node */
    self.add_scope(NodeKind::Stylesheet {
      name: name.to_string()
    });

    /* Eat until we reach the end of the stylesheet */
    self.eat_until(TokenKind::OpenBrace);
    self.update_cursor();
    self.eat_scope(TokenKind::OpenBrace, TokenKind::CloseBrace)?;

    /* Read contents and remove the outer braces */
    let mut content = self.read().trim_matches(|c| c == '{' || c == '}').to_string();
    content.retain(|c| !c.is_whitespace());

    /* Put the contents into a text node and retreat */
    self.add_node(NodeKind::Text(content));
    self.retreat_scope();

    Ok(())
  }

  /// Try to parse a declaration, e.g. `tmpl, styl, impl`
  fn declaration(&mut self) -> Result<Option<String>, WaxError> {
    /* Eat whitespace */
    self.eat_while(TokenKind::Whitespace);
    self.update_cursor();

    /* Match the declared name */
    match self.first() {
      TokenKind::Ident => (),

      /* Special names */
      TokenKind::Atsign => {
        self.next();
        let TokenKind::Ident = self.first() else {
          self.next_with_cursor();
          return Err(self.err_hint(
            "name cannot be `@`", 
            "did you mean `@html`?"
          ));
        };
      }

      _ => return Ok(None)
    }

    /* Read the declared name */
    self.next();
    Ok(Some(self.read()))
  }
}
