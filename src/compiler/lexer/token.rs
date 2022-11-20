#[derive(Debug, Clone)]
pub enum Token {
    Paren(char),
    Op(char),
    Num(u64),
}