use crate::environment::{Value, NativeMethodCallback};
use crate::interpreter::{Interpreter, InterpreterResult};

pub struct StringObject;

impl StringObject {
    pub fn get(name: String) -> NativeMethodCallback {
        match name.as_str() {
            "contains" => string_contains,
            "startsWith" => string_starts_with,
            "endsWith" => string_ends_with,
            "finish" => string_finish,
            "append" => string_append,
            "tap" => string_tap,
            "toUpper" => string_to_upper,
            "toLower" => string_to_lower,
            _ => panic!("Undefined method: {}", name),
        }
    }
}

fn string_contains(_: &mut Interpreter, context: Value, arguments: Vec<Value>) -> Result<Value, InterpreterResult> {
    super::arity("String.contains", 1, &arguments);

    let string = context.to_string();

    for argument in arguments {
        if string.contains(&argument.to_string()) {
            return Ok(Value::Bool(true));
        }
    }

    Ok(Value::Bool(false))
}

fn string_starts_with(_: &mut Interpreter, context: Value, arguments: Vec<Value>) -> Result<Value, InterpreterResult> {
    super::arity("String.startsWith", 1, &arguments);

    let string = context.to_string();

    for argument in arguments {
        if string.starts_with(&argument.to_string()) {
            return Ok(Value::Bool(true));
        }
    }

    Ok(Value::Bool(false))
}

fn string_ends_with(_: &mut Interpreter, context: Value, arguments: Vec<Value>) -> Result<Value, InterpreterResult> {
    super::arity("String.endsWith", 1, &arguments);

    let string = context.to_string();

    for argument in arguments {
        if string.ends_with(&argument.to_string()) {
            return Ok(Value::Bool(true));
        }
    }

    Ok(Value::Bool(false))
}

fn string_finish(_: &mut Interpreter, context: Value, arguments: Vec<Value>) -> Result<Value, InterpreterResult> {
    super::arity("String.finish", 1, &arguments);

    let mut string = context.to_string();
    let append = arguments[0].clone().to_string();

    if ! string.ends_with(&append) {
        string.push_str(append.as_str());
    }

    Ok(Value::String(string))
}

fn string_append(_: &mut Interpreter, context: Value, arguments: Vec<Value>) -> Result<Value, InterpreterResult> {
    super::arity("String.append", 1, &arguments);

    let mut string = context.to_string();
    let append = arguments[0].clone().to_string();

    string.push_str(append.as_str());

    Ok(Value::String(string))
}

fn string_tap(interpreter: &mut Interpreter, context: Value, arguments: Vec<Value>) -> Result<Value, InterpreterResult> {
    super::arity("String.tap", 1, &arguments);

    let string = context.clone();

    // TODO: Add some better error handling here. Maybe check that
    // the argument being passed is actually a function.
    let callback = match arguments.get(0) {
        Some(f) => f.clone(),
        _ => unreachable!()
    };

    interpreter.call(callback, vec![string])?;

    Ok(context)
}

fn string_to_upper(_: &mut Interpreter, context: Value, arguments: Vec<Value>) -> Result<Value, InterpreterResult> {
    super::arity("String.toUpper", 0, &arguments);

    Ok(Value::String(context.to_string().to_uppercase()))
}

fn string_to_lower(_: &mut Interpreter, context: Value, arguments: Vec<Value>) -> Result<Value, InterpreterResult> {
    super::arity("String.toLower", 0, &arguments);

    Ok(Value::String(context.to_string().to_lowercase()))
}