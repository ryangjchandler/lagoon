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
}

impl CallFrame {
    pub fn env_mut(&mut self) -> RefMut<Environment> {
        self.environment.borrow_mut()
    }

    pub fn env(&self) -> Ref<Environment> {
        self.environment.borrow()
    }
}

pub fn execute(ast: Program, _: PathBuf) -> Result<(), MachineResult> {
    let mut codes: Vec<Op> = Vec::new();

    for statement in ast.into_iter() {
        compile(&mut codes, statement)?;
    }

    let mut stack: Vec<Value> = Vec::new();
    let mut frames: Vec<CallFrame> = vec![
        CallFrame::default(),
    ];

    for code in codes {
        match code {
            Op::Push(v) => stack.push(v),
            Op::Set(name) => {
                let value = stack.pop().unwrap();
                frames.last_mut().unwrap().env_mut().set(name, value);
            },
            Op::Get(name) => {
                stack.push(frames.last().unwrap().env().get(&name).expect(&format!("{} is not defined.", name)));
            },
            Op::Call(count) => {
                let callable = stack.pop().unwrap();

                match callable {
                    Value::NativeFunction(callback) => {
                        let mut arguments: Vec<Value> = Vec::new();

                        if count > 0 {
                            for _ in 0..count {
                                arguments.push(stack.pop().unwrap());
                            }
                        }

                        callback(arguments);
                    },
                    _ => todo!("callable {:?}", callable),
                }
            },
            _ => todo!("{:?}", code),
        };
    }

    dbg!(stack);
    dbg!(frames);

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
        Statement::Expression { expression } => compile_expression(code, expression)?,
        _ => todo!("{:?}", statement),
    }

    Ok(())
}

fn compile_expression(code: &mut Vec<Op>, expression: Expression) -> Result<(), MachineResult> {
    match expression {
        Expression::Number(n) => code.push(Op::Push(Value::Number(n))),
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