use crate::Value;

#[derive(Debug, Clone)]
pub enum Code {
    Push(Value),
    Pop,
    Set(String),
    Get(String),
    MakeString(String),
    MakeNumber(f64),
    True,
    False,
    Null,
    Call(usize),
    StartLabel(String),
    EndLabel(String),
    Return,
}