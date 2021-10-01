use crate::Value;

#[derive(Debug, Clone)]
pub enum Op {
    Push(Value),
    Set(String),
    Get(String),
    Call(usize),
}