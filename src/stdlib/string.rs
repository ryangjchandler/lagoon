use crate::environment::{Value, NativeMethodCallback};
use crate::interpreter::Interpreter;

pub struct StringObject;

impl StringObject {
    pub fn get(name: String) -> NativeMethodCallback {
        match name.as_str() {
            "contains" => string_contains,
            "startsWith" => string_starts_with,
            "endsWith" => string_ends_with,
            "finish" => string_finish,
            "append" => string_append,
            _ => panic!("Undefined method: {}", name),
        }
    }
}

fn string_contains(_: &mut Interpreter, context: Value, arguments: Vec<Value>) -> Value {
    super::arity("String.contains", 1, &arguments);

    let string = context.to_string();

    for argument in arguments {
        if string.contains(&argument.to_string()) {
            return Value::Bool(true);
        }
    }

    Value::Bool(false)
}

fn string_starts_with(_: &mut Interpreter, context: Value, arguments: Vec<Value>) -> Value {
    super::arity("String.startsWith", 1, &arguments);

    let string = context.to_string();

    for argument in arguments {
        if string.starts_with(&argument.to_string()) {
            return Value::Bool(true);
        }
    }

    Value::Bool(false)
}

fn string_ends_with(_: &mut Interpreter, context: Value, arguments: Vec<Value>) -> Value {
    super::arity("String.endsWith", 1, &arguments);

    let string = context.to_string();

    for argument in arguments {
        if string.ends_with(&argument.to_string()) {
            return Value::Bool(true);
        }
    }

    Value::Bool(false)
}

fn string_finish(_: &mut Interpreter, context: Value, arguments: Vec<Value>) -> Value {
    super::arity("String.finish", 1, &arguments);

    let mut string = context.to_string();
    let append = arguments[0].clone().to_string();

    if ! string.ends_with(&append) {
        string.push_str(append.as_str());
    }

    Value::String(string)
}

fn string_append(_: &mut Interpreter, context: Value, arguments: Vec<Value>) -> Value {
    super::arity("String.append", 1, &arguments);

    let mut string = context.to_string();
    let append = arguments[0].clone().to_string();

    string.push_str(append.as_str());

    Value::String(string)
}