use crate::Value;
use lagoon_parser::Op as InfixOp;

#[derive(Debug, Clone)]
pub enum Op {
    Push(Value),
    Set(String),
    Get(String),
    Call(usize),
    Infix(InfixOp),
    If(Vec<Op>, Vec<Op>),
    Return,
}