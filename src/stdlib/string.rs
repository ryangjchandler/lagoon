use crate::environment::{Value, NativeMethodCallback};
use crate::interpreter::Interpreter;

pub struct StringObject;

impl StringObject {
    pub fn get(name: String) -> NativeMethodCallback {
        match name.as_str() {
            "contains" => string_contains,
            "startsWith" => string_starts_with,
            "endsWith" => string_ends_with,
            _ => panic!("Undefined method: {}", name),
        }
    }
}

fn string_contains(_: &mut Interpreter, context: Value, arguments: Vec<Value>) -> Value {
    let string = context.to_string();

    for argument in arguments {
        if string.contains(&argument.to_string()) {
            return Value::Bool(true);
        }
    }

    Value::Bool(false)
}

fn string_starts_with(_: &mut Interpreter, context: Value, arguments: Vec<Value>) -> Value {
    let string = context.to_string();

    for argument in arguments {
        if string.starts_with(&argument.to_string()) {
            return Value::Bool(true);
        }
    }

    Value::Bool(false)
}

fn string_ends_with(_: &mut Interpreter, context: Value, arguments: Vec<Value>) -> Value {
    let string = context.to_string();

    for argument in arguments {
        if string.ends_with(&argument.to_string()) {
            return Value::Bool(true);
        }
    }

    Value::Bool(false)
}