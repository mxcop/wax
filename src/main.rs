use peekmore::PeekMore;
use wax_lexer::{Lexer, token::Token};
use wax_logger::{bail, warn};
use wax_parser::{tree::ArenaTree, node::NodeType};

mod args;
mod build;
mod create;
mod utils;

fn main() {
  // Enable colors in the command prompt.
  colored::control::set_virtual_terminal(true).unwrap();

  let input = std::fs::read_to_string("./example/src/pages/hive.wx").expect("failed to load file");
  let input: Vec<char> = input.chars().collect();

  // Tokenize :
  let mut l = Lexer::new(input.iter().peekmore());
  let tk: Vec<Token> = l.lex();

  // Parse :
  let mut tree: ArenaTree<NodeType> = ArenaTree::new();
  let mut curr = tree.add_node("Root".into(), NodeType::Root);

  for (index, token) in tk.iter().enumerate() {
    match token {
      Token::OpeningTag(tag) => {
        curr = tree.add_child(curr, tag.to_string(), NodeType::Tag { attributes: vec![] });
      },
      Token::ClosingTag(_) => {
        //tree.add_child(curr, tag.to_string(), NodeType::ClosingTag);
        curr = tree.get_parent(curr);
      },
      Token::ClosedTag(tag) => {
        tree.add_child(curr, tag.to_string(), NodeType::Tag { attributes: vec![] });
      },
      Token::DefaultImport{ name, path } => {
        tree.add_child(
          curr, 
          name.to_string(), 
          NodeType::DefaultImport { 
            specifier: name.to_string(), 
            source: path.to_string()
          }
        );
      }
    }
    println!("{} : {:?}", index, token);
  }

  println!("\nAST : \n{}", tree);

  warn!(
    "non-default component import!",
    "src/pages/hive.wx",
    "<script>",
    2,
    r#"import { comp } from "../lib/comp.wx";"#,
    r#"try using `import <name> from "...";`"#
  );

  bail!(
    "non-default component import!",
    "src/pages/hive.wx",
    "<script>",
    2,
    r#"import { comp } from "../lib/comp.wx";"#,
    r#"try using `import <name> from "...";`"#
  );
}
