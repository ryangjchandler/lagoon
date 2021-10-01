use crate::Op;

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Number(f64),
    NativeFunction(fn (Vec<Value>) -> Value),
    Function(Vec<Op>, usize),
}