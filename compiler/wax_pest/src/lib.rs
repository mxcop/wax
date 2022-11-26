use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "wax.pest"]
pub struct WaxParser;