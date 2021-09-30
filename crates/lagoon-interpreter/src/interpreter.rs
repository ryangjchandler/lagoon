use std::slice::Iter;
use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};
use hashbrown::HashMap;
use thiserror::Error;
use colored::*;
use lagoon_parser::*;

use crate::environment::*;

pub fn interpret(ast: Program) -> Result<(), InterpreterResult> {
    let mut interpreter = Interpreter::new(ast.iter());

    interpreter.define_global_function("println", crate::stdlib::println);
    interpreter.define_global_function("print", crate::stdlib::print);
    interpreter.define_global_function("type", crate::stdlib::r#type);

    interpreter.run()
}

#[derive(Error, Debug)]
pub enum InterpreterResult {
    #[error("")]
    Return(Value),

    #[error("Undefined variable: {0}.")]
    UndefinedVariable(String),

    #[error("Undefined index: {0}.")]
    UndefinedIndex(usize),

    #[error("Undefined field: {0}.{1}")]
    UndefinedField(String, String),

    #[error("Undefined method: {0}.{1}()")]
    UndefinedMethod(String, String),

    #[error("Unable to iterate over value of type {0}.")]
    InvalidIterable(String),

    #[error("Too few arguments to function {0}(), {1} passed in, {2} expected.")]
    TooFewArguments(String, usize, usize),

    #[error("Cannot append to value of type {0}.")]
    InvalidAppendTarget(String),

    #[error("Cannot assign method to static property of type {0}.")]
    InvalidMethodAssignmentTarget(String),
}

impl InterpreterResult {
    pub fn print(self) {
        eprintln!("{}", format!("{}", self).red().bold());
        std::process::exit(1);
    }
}

#[derive(Debug, Clone)]
pub struct Interpreter<'i> {
    ast: Iter<'i, Statement>,
    environment: Rc<RefCell<Environment>>,
    globals: HashMap<String, Value>,
}

impl<'i> Interpreter<'i> {
    fn new(ast: Iter<'i, Statement>) -> Self {
        Self {
            ast: ast,
            environment: Rc::new(RefCell::new(Environment::new())),
            globals: HashMap::new(),
        }
    }

    fn run_statement(&mut self, statement: Statement) -> Result<(), InterpreterResult> {
        Ok(match statement {
            Statement::LetDeclaration { name, initial } => {
                if initial.is_none() {
                    self.env_mut().set(name, Value::Null)
                } else {
                    let initial = initial.unwrap();
                    let value = self.run_expression(initial)?;

                    self.env_mut().set(name, value)
                }
            },
            Statement::FunctionDeclaration { name, params, body } => {
                self.globals.insert(name.clone(), Value::Function {
                    name, params, body, environment: None, context: None,
                });
            },
            Statement::StructDeclaration { name, fields } => {
                self.globals.insert(name.clone(), Value::Struct {
                    name, fields, methods: Rc::new(RefCell::new(HashMap::new())),
                });
            },
            Statement::For { iterable, value, index, then } => {
                let iterable = self.run_expression(iterable)?;

                let items = match iterable {
                    Value::List(items) => items,
                    _ => return Err(InterpreterResult::InvalidIterable(iterable.typestring())),
                };

                // If there aren't any items in the list, we can leave this execution
                // cycle early.
                if items.borrow().is_empty() {
                    return Ok(())
                }

                let set_index: bool = index.is_some();

                for (i, item) in items.borrow().iter().enumerate() {
                    self.env_mut().set(value.clone(), item.clone());

                    if set_index {
                        self.env_mut().set(index.clone().unwrap(), Value::Number(i as f64));
                    }

                    for statement in then.clone() {
                        self.run_statement(statement)?;
                    }
                }

                self.env_mut().drop(value);

                if set_index {
                    self.env_mut().drop(index.unwrap());
                }
            },
            Statement::If { condition, then, otherwise } => {
                let condition = self.run_expression(condition)?;

                if condition.to_bool() {
                    for statement in then {
                        self.run_statement(statement)?;
                    }
                } else if otherwise.is_some() {
                    for statement in otherwise.unwrap() {
                        self.run_statement(statement)?;
                    }
                }
            },
            Statement::Expression { expression } => {
                self.run_expression(expression)?;
            },
            Statement::Return { value } => {
                return Err(InterpreterResult::Return(self.run_expression(value)?));
            },
            _ => todo!("{:?}", statement),
        })
    }

    pub fn call(&mut self, callable: Value, arguments: Vec<Value>) -> Result<Value, InterpreterResult> {
        Ok(match callable {
            Value::NativeFunction { callback, .. } => callback(self, arguments),
            Value::NativeMethod { callback, context, .. } => {
                let context = self.run_expression(context)?;

                callback(self, context, arguments)?
            },
            Value::Function { name, params, body, environment, context } => {
                if params.first() != Some(&Parameter { name: "self".to_string() }) && params.len() != arguments.len() {
                    return Err(InterpreterResult::TooFewArguments(name.clone(), arguments.len(), params.len()));
                }

                let old_environment = Rc::clone(&self.environment);
                let new_environment = if environment.is_some() { 
                    Rc::new(RefCell::new(environment.unwrap()))
                } else {
                    Rc::new(RefCell::new(Environment::new()))
                };

                if context.is_some() && params.first() == Some(&Parameter { name: "self".to_string() }) {
                    let context = self.run_expression(context.unwrap())?;
                    new_environment.borrow_mut().set("self", context);
                }

                for (Parameter { name, .. }, value) in params.into_iter().filter(|p| p.name != "self").zip(arguments) {
                    new_environment.borrow_mut().set(name, value);
                };

                self.environment = new_environment;

                let mut return_value: Option<Value> = None;

                for statement in body {
                    match self.run_statement(statement) {
                        Err(InterpreterResult::Return(value)) => {
                            return_value = Some(value);
                            break;
                        },
                        _ => (),
                    };
                }

                self.environment = old_environment;

                if return_value.is_some() { return_value.unwrap() } else { Value::Null }
            },
            _ => todo!(),
        })
    }

    fn run_expression(&mut self, expression: Expression) -> Result<Value, InterpreterResult> {
        Ok(match expression {
            Expression::Number(n) => Value::Number(n),
            Expression::String(s) => Value::String(s),
            Expression::Bool(b) => Value::Bool(b),
            Expression::Identifier(n) => {
                if self.globals.contains_key(&n) {
                    self.globals[&n].clone()
                } else {
                    if let Some(v) = self.env().get(n.clone()) {
                        v
                    } else {
                        return Err(InterpreterResult::UndefinedVariable(n));
                    }
                }
            },
            Expression::Index(target, index) => {
                let instance = self.run_expression(*target)?;
                let index = self.run_expression(*index.expect("Expected index."))?.to_number() as usize;

                match instance {
                    Value::List(items) => {
                        match items.borrow().get(index) {
                            Some(v) => v.clone(),
                            None => return Err(InterpreterResult::UndefinedIndex(index))
                        }
                    },
                    _ => unreachable!()
                }
            },
            Expression::Get(target, field) => {
                let instance = self.run_expression(*target.clone())?;

                match instance {
                    Value::StructInstance { environment, definition, .. } => if let Some(value) = environment.borrow().get(field.clone()) {
                        match value {
                            Value::Function { name, params, body, environment, .. } => Value::Function { name, params, body, environment, context: Some(*target) },
                            _ => value,
                        }
                    } else {
                        let name = match *definition {
                            Value::Struct { name, .. } => name,
                            _ => unreachable!()
                        };

                        return Err(InterpreterResult::UndefinedField(name, field))
                    },
                    Value::Struct { name, methods, .. } => if let Some(value) = methods.borrow().get(&field.clone()) {
                        value.clone()
                    } else {
                        return Err(InterpreterResult::UndefinedMethod(name, field))
                    },
                    Value::String(..) => Value::NativeMethod { name: field.clone(), callback: crate::stdlib::StringObject::get(field), context: *target },
                    Value::Number(..) => Value::NativeMethod { name: field.clone(), callback: crate::stdlib::NumberObject::get(field), context: *target },
                    Value::List(..) => Value::NativeMethod { name: field.clone(), callback: crate::stdlib::ListObject::get(field), context: *target },
                    _ => todo!(),
                }
            },
            Expression::Infix(left, op, right) => {
                let left = self.run_expression(*left)?;
                let right = self.run_expression(*right)?;

                match (left, op, right) {
                    (Value::Number(l), Op::Add, Value::Number(r)) => Value::Number(l + r),
                    (Value::Number(l), Op::Multiply, Value::Number(r)) => Value::Number(l * r),
                    (Value::Number(l), Op::Divide, Value::Number(r)) => Value::Number(l / r),
                    (Value::Number(l), Op::Subtract, Value::Number(r)) => Value::Number(l - r),
                    (Value::Number(l), Op::Add, Value::String(r)) => {
                        let mut l = l.to_string();
                        l.push_str(r.as_str());
                        Value::String(l)
                    },
                    (Value::String(l), Op::Add, Value::Number(r)) => {
                        let mut l = l;
                        l.push_str(r.to_string().as_str());
                        Value::String(l)
                    },
                    (Value::String(l), Op::Add, Value::String(r)) => {
                        let mut l = l;
                        l.push_str(r.as_str());
                        Value::String(l)
                    },
                    (Value::String(l), Op::Equals, Value::String(r)) => Value::Bool(l == r),
                    (Value::Number(l), Op::Equals, Value::Number(r)) => Value::Bool(l == r),
                    (Value::Bool(l), Op::Equals, Value::Bool(r)) => Value::Bool(l == r),
                    (Value::String(l), Op::NotEquals, Value::String(r)) => Value::Bool(l != r),
                    (Value::Number(l), Op::NotEquals, Value::Number(r)) => Value::Bool(l != r),
                    (Value::Bool(l), Op::NotEquals, Value::Bool(r)) => Value::Bool(l != r),
                    (Value::Number(l), Op::LessThan, Value::Number(r)) => Value::Bool(l < r),
                    (Value::Number(l), Op::GreaterThan, Value::Number(r)) => Value::Bool(l > r),
                    (Value::Number(l), Op::LessThanOrEquals, Value::Number(r)) => Value::Bool(l <= r),
                    (Value::Number(l), Op::GreaterThanOrEquals, Value::Number(r)) => Value::Bool(l >= r),
                    (l, Op::And, r) => Value::Bool(l.to_bool() && r.to_bool()),
                    (l, Op::Or, r) => Value::Bool(l.to_bool() || r.to_bool()),
                    (Value::Number(l), Op::Pow, Value::Number(r)) => Value::Number(l.powf(r)),
                    (l, Op::In, Value::List(r)) => {
                        let filtered: Vec<Value> = r.borrow().clone()
                            .into_iter()
                            .filter(|v| {
                                v.clone().is(l.clone())
                            })
                            .collect();

                        Value::Bool(! filtered.is_empty())
                    },
                    (Value::String(l), Op::In, Value::String(r)) => {
                        Value::Bool(r.contains(l.as_str()))
                    },
                    (l, Op::NotIn, Value::List(r)) => {
                        let filtered: Vec<Value> = r.borrow().clone()
                            .into_iter()
                            .filter(|v| {
                                v.clone().is(l.clone())
                            })
                            .collect();

                        Value::Bool(filtered.is_empty())
                    },
                    (Value::String(l), Op::NotIn, Value::String(r)) => {
                        Value::Bool(! r.contains(l.as_str()))
                    },
                    _ => todo!()
                }
            },
            Expression::List(items) => {
                let mut values: Vec<Value> = Vec::new();

                for item in items.into_iter() {
                    values.push(self.run_expression(item)?);
                }

                Value::List(Rc::new(RefCell::new(values)))
            },
            Expression::Closure(params, body) => {
                Value::Function {
                    name: String::from("Closure"),
                    params,
                    body,
                    environment: Some(self.environment.borrow().clone()),
                    context: None,
                }
            },
            Expression::Struct(definition, fields) => {
                let definition = self.run_expression(*definition)?;

                let (name, field_definitions, methods) = match definition.clone() {
                    Value::Struct { name, fields, methods } => (name, fields, methods),
                    _ => unreachable!()
                };

                let mut environment = Environment::new();

                for (field, value) in fields {
                    if ! field_definitions.contains(&Parameter { name: field.clone() }) {
                        return Err(InterpreterResult::UndefinedField(name, field.clone()));
                    }

                    let value = self.run_expression(value)?;

                    environment.set(field, match value {
                        Value::StructInstance { environment, definition } => {
                            // This logic is needed to ensure that any nested structs
                            // that receive modifications do not apply the same side-effect
                            // to the original reference.
                            let environment = environment.borrow().clone();

                            Value::StructInstance { definition, environment: Rc::new(RefCell::new(environment)) }
                        },
                        _ => value,
                    });
                }

                let environment = Rc::new(RefCell::new(environment));

                for (name, method) in methods.borrow().clone() {
                    let method = match method {
                        Value::Function { name, body, params, .. } => Value::Function {
                            name,
                            params,
                            body,
                            environment: None,
                            context: None,
                        },
                        _ => unreachable!()
                    };
                    
                    environment.borrow_mut().set(name, method);
                }

                Value::StructInstance { environment, definition: Box::new(definition) }
            },
            Expression::Call(callable, arguments) => {
                let callable = self.run_expression(*callable)?;
                let mut argument_values: Vec<Value> = Vec::new();

                for argument in arguments.into_iter() {
                    argument_values.push(self.run_expression(argument)?);
                }

                self.call(callable, argument_values)?
            },
            Expression::Prefix(op, right) => {
                let right = self.run_expression(*right)?;

                match op {
                    Op::Bang => Value::Bool(! right.to_bool()),
                    Op::Subtract => Value::Number(- right.to_number()),
                    _ => unreachable!()
                }
            },
            Expression::Assign(target, value) => {
                let value = self.run_expression(*value)?;

                match *target.clone() {
                    Expression::Index(instance, index) => {
                        let instance = self.run_expression(*instance)?;

                        match instance {
                            Value::List(items) => {
                                match index {
                                    Some(i) => {
                                        let index = self.run_expression(*i)?.to_number();
                                        items.borrow_mut()[index as usize] = value.clone();
                                    },
                                    None => {
                                        items.borrow_mut().push(value.clone());
                                    }
                                }
                            },
                            _ => return Err(InterpreterResult::InvalidAppendTarget(instance.typestring()))
                        };
                    },
                    Expression::Get(instance, field) => {
                        let instance = self.run_expression(*instance)?;

                        match instance.clone() {
                            // TODO: Check if the field exists on the definition before
                            // actually doing the assignment.
                            Value::StructInstance { environment, .. } => {
                                environment.borrow_mut().set(field, value.clone())
                            },
                            Value::Struct { methods, .. } => {
                                if ! matches!(value.clone(), Value::Function { .. }) {
                                    return Err(InterpreterResult::InvalidMethodAssignmentTarget(instance.typestring()))
                                } else {
                                    methods.borrow_mut().insert(field, value.clone());
                                }
                            },
                            _ => return Err(InterpreterResult::InvalidMethodAssignmentTarget(instance.typestring()))
                        };
                    },
                    Expression::Identifier(i) => {
                        self.env_mut().set(i, value.clone());
                    },
                    _ => todo!()
                };

                value
            },
            _ => todo!("{:?}", expression),
        })
    }

    fn define_global_function(&mut self, name: impl Into<String>, callback: NativeFunctionCallback) {
        let name = name.into();

        self.globals.insert(name.clone(), Value::NativeFunction {
            name: name,
            callback: callback,
        });
    }

    fn env(&self) -> Ref<Environment> {
        RefCell::borrow(&self.environment)
    }

    fn env_mut(&mut self) -> RefMut<Environment> {
        RefCell::borrow_mut(&self.environment)
    }

    fn run(&mut self) -> Result<(), InterpreterResult> {
        while let Some(statement) = self.ast.next() {
            let _r = self.run_statement(statement.clone())?;
        }

        if ! ::std::env::args().filter(|a| a == "--debug").collect::<Vec<String>>().is_empty() {
            self.env().dump();
            dbg!(self.globals.clone());
        }

        Ok(())
    }
}