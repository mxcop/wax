pub mod node;
pub mod tree;
pub mod span;
mod parser;
//mod scope;

use std::slice::Iter;

use parser::Parser;
use tree::AST;

use waxc_errors::error::WaxError;
use waxc_lexer::token::Token;

type WaxErrors = Vec<WaxError<'static>>;

/// Parse an input stream of tokens into an abstract syntax tree.
fn parse(input: Iter<Token>) -> Result<AST, WaxErrors> {
  let mut parser = Parser::new(input);
  match parser.build() {
    Ok(ast) => Ok(ast),
    Err(e) => Err(e)
  }
}

impl Parser<'_> {
  pub fn build(&mut self) -> Result<AST, WaxErrors> {
    todo!();
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
