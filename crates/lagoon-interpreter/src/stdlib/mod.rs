use crate::interpreter::{Interpreter, InterpreterResult};
use crate::environment::Value;
use lagoon_parser::{generate, parse};

mod string;
mod number;
mod list;

pub use string::StringObject;
pub use number::NumberObject;
pub use list::ListObject;

pub fn arity(name: &str, arity: usize, arguments: &Vec<Value>) {
    if arity != arguments.len() {
        panic!("Method {} expected {} arguments, received {}.", name, arity, arguments.len());
    }
}

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

    Value::String(arg.clone().typestring())
}

pub fn require(interpreter: &mut Interpreter, args: Vec<Value>) -> Value {
    arity("require", 1, &args);
    
    let path = args.first().unwrap().clone().to_string();
    let directory = interpreter.path().parent().unwrap().to_path_buf();

    // Handle relative paths.
    if path.starts_with(".") {
        let mut module_path = directory.clone();
        if path.ends_with(".lag") {
            module_path.push(path);
        } else {
            module_path.push(path + ".lag");
        }

        let module_path = module_path.canonicalize().unwrap();
        let contents = ::std::fs::read_to_string(&module_path).unwrap();

        let tokens = generate(&contents);
        let ast = parse(tokens).unwrap(); // TODO: Handle errors here.

        let value = match interpreter.exec(ast) {
            Ok(_) => Value::Null,
            Err(e) => {
                e.print();
                std::process::exit(1);
            },
        };

        return value;
    }

    panic!("Cannot find module.")
}