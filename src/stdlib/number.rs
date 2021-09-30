use crate::environment::{Value, NativeMethodCallback};
use crate::interpreter::{Interpreter, InterpreterResult};

pub struct NumberObject;

impl NumberObject {
    pub fn get(name: String) -> NativeMethodCallback {
        match name.as_str() {
            "isInteger" => number_is_integer,
            "isFloat" => number_is_float,
            "toFixed" => number_to_fixed,
            _ => panic!("Undefined method: {}", name),
        }
    }
}

fn number_is_integer(_: &mut Interpreter, context: Value, arguments: Vec<Value>) -> Result<Value, InterpreterResult> {
    super::arity("Number.isInteger", 0, &arguments);
    
    let number = context.to_number(); 
    
    Ok(Value::Bool(number == number.trunc()))
}

fn number_is_float(_: &mut Interpreter, context: Value, arguments: Vec<Value>) -> Result<Value, InterpreterResult> {
    super::arity("Number.isFloat", 0, &arguments);
    
    let number = context.to_number(); 
    
    Ok(Value::Bool(number != number.trunc()))
}

fn number_to_fixed(_: &mut Interpreter, context: Value, arguments: Vec<Value>) -> Result<Value, InterpreterResult> {
    let number = context.to_number();
    let precision = if arguments.is_empty() { 0 } else { arguments.get(0).unwrap().clone().to_number() as usize };

    if precision == 0 {
        return Ok(Value::Number(number.trunc()))
    }

    let rounded: f64 = format!("{:.1$}", number, precision).parse().unwrap();

    Ok(Value::Number(rounded))
}