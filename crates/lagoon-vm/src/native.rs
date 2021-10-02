use crate::{Chunk, Value};

pub type NativeFunction = fn (&mut Chunk, &[Value]) -> Value;

pub fn native_println(_: &mut Chunk, args: &[Value]) -> Value {
    for arg in args {
        println!("{}", arg);
    }

    Value::Null
}