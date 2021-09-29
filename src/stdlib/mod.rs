use crate::interpreter::Interpreter;
use crate::environment::Value;

pub fn println(_: &mut Interpreter, args: Vec<Value>) -> Value {
    let arg = args.get(0).unwrap().clone();

    println!("{}", arg.to_string());

    Value::Null
}

pub fn print(_: &mut Interpreter, args: Vec<Value>) -> Value {
    let arg = args.get(0).unwrap().clone();

    print!("{}", arg.to_string());

    Value::Null
}

pub fn r#type(_: &mut Interpreter, args: Vec<Value>) -> Value {
    if args.is_empty() || args.len() > 1 {
        panic!("Function {} expects {} argument, received {}", "type", 1, args.len());
    }

    let arg = args.first().unwrap();

    Value::String(match arg {
        Value::String(..) => "string".into(),
        Value::Number(..) => "number".into(),
        Value::Bool(..) => "bool".into(),
        Value::Null => "null".into(),
        Value::Function { .. } | Value::NativeFunction { .. } => "function".into(),
        Value::StructInstance { definition, .. } => match *definition.clone() {
            Value::Struct { name, .. } => name,
            _ => unreachable!()
        },
        Value::Struct { .. } => "struct".into(),
        _ => unreachable!()
    })
}