mod value;
mod op;

use lagoon_parser::{Statement, Expression, Program};
use thiserror::Error;
use colored::*;
use std::path::PathBuf;
use hashbrown::HashMap;
use std::rc::Rc;
use std::cell::{RefCell, RefMut, Ref};

pub(crate) use value::Value;
pub(crate) use op::Op;

#[derive(Error, Debug)]
pub enum MachineResult {

}

impl MachineResult {
    pub fn print(self) {
        eprintln!("{}", format!("{}", self).red().bold());
        std::process::exit(1);
    }
}

#[macro_export]
macro_rules! arity {
    () => {
        
    };
}

fn std_println(arguments: Vec<Value>) -> Value {
    assert!(arguments.len() > 0);

    for argument in arguments {
        // TODO: Convert values into their _real_ string equivalents.
        println!("{:?}", argument);
    }

    Value::Null
}

fn register_globals(environment: &mut HashMap<String, Value>) {
    environment.insert("println".into(), Value::NativeFunction(std_println));
}

#[derive(Debug)]
struct Environment {
    values: HashMap<String, Value>,
}

impl Default for Environment {
    fn default() -> Self {
        let mut values = HashMap::new();
        register_globals(&mut values);
        Self { values }
    }
}

impl Environment {
    pub fn set(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &String) -> Option<Value> {
        if let Some(value) = self.values.get(name) {
            Some(value.clone())
        } else {
            None
        }
    }
}

#[derive(Debug, Default)]
struct CallFrame {
    environment: Rc<RefCell<Environment>>,
    stack: Vec<Value>,
}

impl CallFrame {
    pub fn env_mut(&mut self) -> RefMut<Environment> {
        self.environment.borrow_mut()
    }

    pub fn env(&self) -> Ref<Environment> {
        self.environment.borrow()
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value)
    }

    pub fn pop(&mut self) -> Option<Value> {
        self.stack.pop()
    }
}

pub fn execute(ast: Program, _: PathBuf) -> Result<(), MachineResult> {
    let mut codes: Vec<Op> = Vec::new();

    for statement in ast.into_iter() {
        compile(&mut codes, statement)?;
    }

    let mut frames: Vec<CallFrame> = vec![
        CallFrame::default(),
    ];

    run(codes, &mut frames)?;

    Ok(())
}

fn run(codes: Vec<Op>, frames: &mut Vec<CallFrame>) -> Result<(), MachineResult> {
    for code in codes {
        match code {
            Op::Push(v) => frames.last_mut().unwrap().push(v),
            Op::Set(name) => {
                let value = frames.last_mut().unwrap().pop().unwrap();
                frames.last_mut().unwrap().env_mut().set(name, value);
            },
            Op::Get(name) => {
                let value = frames.last().unwrap().env().get(&name).expect(&format!("{} is not defined.", name));
                frames.last_mut().unwrap().push(value);
            },
            Op::Call(count) => {
                let callable = frames.last_mut().unwrap().pop().unwrap();

                match callable {
                    Value::NativeFunction(callback) => {
                        let mut arguments: Vec<Value> = Vec::new();

                        if count > 0 {
                            for _ in 0..count {
                                arguments.push(frames.last_mut().unwrap().pop().unwrap());
                            }
                        }

                        callback(arguments);
                    },
                    Value::Function(chunk, arity) => {
                        let mut args: Vec<Value> = Vec::new();

                        // Use the arity to figure out how many values we need
                        // to pop from the current stack and push into the call frame.
                        if arity > 0 {
                            for i in 0..arity {
                                args.push(frames.last_mut().unwrap().pop().unwrap());
                            }
                        }

                        frames.push(CallFrame::default());

                        for arg in args {
                            frames.last_mut().unwrap().push(arg);
                        }

                        run(chunk, frames)?;

                        // Remove the last frame as we've exited the function
                        // and no longer need it.
                        frames.pop();
                    },
                    _ => todo!("callable {:?}", callable),
                }
            },
            _ => todo!("{:?}", code),
        };
    };

    Ok(())
}

fn compile(code: &mut Vec<Op>, statement: Statement) -> Result<(), MachineResult> {
    match statement {
        Statement::LetDeclaration { name, initial } => {
            if initial.is_some() {
                compile_expression(code, initial.unwrap())?;
            } else {
                code.push(Op::Push(Value::Null));
            }

            code.push(Op::Set(name))
        },
        Statement::FunctionDeclaration { name, params, body } => {
            // Create a new chunk of `Op`s for this function.
            let mut chunk: Vec<Op> = Vec::new();

            // Keeping track of the parameter count.
            let mut count: usize = 0;

            // Loop through each of the parameters and load their
            // values into the current frame's environment.
            for param in params {
                chunk.push(Op::Set(param.name));
                count += 1;
            }

            // Loop through each of the statement in the function
            // and compile them into the current chunk.
            for statement in body {
                compile(&mut chunk, statement)?;
            }

            // Push the function to the stack.
            code.push(Op::Push(Value::Function(chunk, count)));

            // Define the function in the current environment.
            code.push(Op::Set(name));
        },
        Statement::Expression { expression } => compile_expression(code, expression)?,
        _ => todo!("{:?}", statement),
    }

    Ok(())
}

fn compile_expression(code: &mut Vec<Op>, expression: Expression) -> Result<(), MachineResult> {
    match expression {
        Expression::Number(n) => code.push(Op::Push(Value::Number(n))),
        Expression::String(s) => code.push(Op::Push(Value::String(s))),
        Expression::Bool(b) => code.push(Op::Push(Value::Bool(b))),
        Expression::Null => code.push(Op::Push(Value::Null)),
        Expression::Call(callable, arguments) => {
            // Push all of the arguments to the stack and keep
            // track of how many arguments were pushed.
            let mut count = 0;
            for argument in arguments {
                compile_expression(code, argument)?;
                count += 1;
            }

            // Evaluate the callable and push it to the stack.
            compile_expression(code, *callable)?;

            // Tell the machine to call the previous stack value
            // and how many arguments there are.
            code.push(Op::Call(count));
        },
        Expression::Identifier(i) => code.push(Op::Get(i)),
        _ => todo!("{:?}", expression),
    };

    Ok(())
}