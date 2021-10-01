use crate::Value;

#[derive(Debug)]
pub enum Op {
    Push(Value),
    Set(String),
    Get(String),
    Call(usize),
}