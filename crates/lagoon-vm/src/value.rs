#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Number(f64),
    NativeFunction(fn (Vec<Value>) -> Value),
}