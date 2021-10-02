#[derive(Debug, Clone)]
pub enum Code {
    Push,
    Pop,
    Set(String),
    Get(String),
    MakeString(String),
    MakeNumber(f64),
    True,
    False,
    Null,
    Call(usize),
}