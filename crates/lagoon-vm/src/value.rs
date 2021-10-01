use crate::Op;

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Number(f64),
    String(String),
    Bool(bool),
    NativeFunction(fn (Vec<Value>) -> Value),
    Function(Vec<Op>, usize),
}

impl Value {
    pub fn to_bool(self) -> bool {
        match self {
            Value::Bool(b) => b,
            _ => todo!(),
        }
    }
}