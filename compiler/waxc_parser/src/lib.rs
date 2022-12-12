pub mod node;
pub mod tree;
pub mod span;
mod parser;
//mod scope;

//use std::slice::Iter;

use parser::Parser;
use tree::AST;

use waxc_errors::error::WaxError;
use waxc_lexer::token::{Token, TokenKind};

type WaxErrors = Vec<WaxError<'static>>;

/// Parse an input stream of tokens into an abstract syntax tree.
pub fn parse<'a>(file: String, input: impl Iterator<Item = Token> + Clone + 'a) -> Result<AST, WaxErrors> {
  let mut parser = Parser::new(file, input);
  let errors: WaxErrors = Vec::new();

  /* Move through all the tokens */
  loop {
    if let Ok(false) = parser.advance() {
      break;
    }
  }

  match errors.len() {
    0 => Ok(parser.get_tree()),
    _ => Err(errors)
  }
}

impl<T: Iterator<Item = Token> + Clone> Parser<T> {
  pub fn advance(&mut self) -> Result<bool, WaxError> {
    let position = self.consumed();

    let Some(tk) = self.next() else {
      return Ok(false);
    };

    match tk.kind {
      TokenKind::Ident => {
        let ident = self.read();
        match ident {
          "tmpl" => self.template(position, *tk.get_len())?,
          _ => ()
        }
      }
      _ => ()
    }

    self.reset_cursor();

    Ok(true)
  }

  fn template(&mut self, pos: usize, len: usize) -> Result<(), WaxError> {

    self.add_scope(pos, len, 
      node::NodeKind::Template { name: "".to_string() }
    );

    Ok(())
  }
}

// The Wax parser.
// pub struct Parser<'a> {
//   tokens: Iter<'a, Token>
// }

// impl<'a> Parser<'a> {
//   pub fn new(tokens: Iter<Token>) -> Self {
//     Self {
//       tokens
//     }
//   }

//   #[allow(unused_variables)]
//   /// ### Syntactic Analysis
//   /// Analize the input tokens and convert it into an abstract syntax tree.
//   pub fn parse(&mut self) -> Result<ArenaTree<SyntaxNode>, WaxError> {
//     let mut tree: ArenaTree<SyntaxNode> = ArenaTree::new();
//     let mut curr = tree.add_node("Root".to_string(), Span::new(0, 0), SyntaxNode::Root);

//     while let Some((dtk, tk)) = self.iter.next_de() {
//       match tk {
//         Token::Template => {
//           tmpl::parse(&mut self.tokens, dtk, &mut curr, &mut tree)?;
//         }
//         // Token::Slash => { return Err(WaxError::from_token(dtk.clone(), "test msg", None)); }
//         _ => {}
//       }
//     }

//     Ok(tree)
//   }
// }
