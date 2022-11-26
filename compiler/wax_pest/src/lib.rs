use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"] // relative to src
pub struct WaxParser;