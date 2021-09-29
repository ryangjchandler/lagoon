use std::slice::Iter;
use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};
use hashbrown::HashMap;
use regex::{Regex, Replacer};

use crate::ast::*;
use crate::environment::*;

pub fn interpret(ast: Program) {
    let mut interpreter = Interpreter::new(ast.iter());

    interpreter.define_global_function("println", crate::stdlib::println);
    interpreter.define_global_function("print", crate::stdlib::print);
    interpreter.define_global_function("type", crate::stdlib::r#type);

    interpreter.run();
}

enum InterpreterResult {
    Return(Value),
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
                    let value = self.run_expression(initial);

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
            Statement::If { condition, then, otherwise } => {
                let condition = self.run_expression(condition);

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
                self.run_expression(expression);
            },
            Statement::Return { value } => {
                return Err(InterpreterResult::Return(self.run_expression(value)));
            },
            _ => todo!("{:?}", statement),
        })
    }

    fn run_expression(&mut self, expression: Expression) -> Value {
        match expression {
            Expression::Number(n) => Value::Number(n),
            Expression::String(s) => Value::String(s),
            Expression::InterpolatedString(s) => {
                let re = Regex::new(r"\{((.*|\n)*)\}").unwrap();
                let replaced = re.replace_all(&s, |captures: &regex::Captures| {
                    let raw: &str = &captures[1];
                    let tokens = crate::token::generate(raw);
                    let expression = match crate::parser::parse(tokens) {
                        Ok(p) => {
                            match p.first() {
                                Some(Statement::Expression { expression }) => expression.clone(),
                                _ => panic!("Unable to parse interpolated expression.")
                            }
                        },
                        _ => panic!("Unable to parse interpolated expression.")
                    };
                    
                    let value = self.run_expression(expression);

                    value.to_string()
                }).to_string();

                Value::String(replaced)
            },
            Expression::Bool(b) => Value::Bool(b),
            Expression::Identifier(n) => {
                if self.globals.contains_key(&n) {
                    self.globals[&n].clone()
                } else {
                    if let Some(v) = self.env().get(n.clone()) {
                        v
                    } else {
                        panic!("Undefined variable: {}", n);
                    }
                }
            },
            Expression::Index(target, index) => {
                let instance = self.run_expression(*target);
                let index = self.run_expression(*index.expect("Expected index.")).to_number() as usize;

                match instance {
                    Value::List(items) => {
                        match items.borrow().get(index) {
                            Some(v) => v.clone(),
                            None => panic!("Undefined index: {}", index)
                        }
                    },
                    _ => unreachable!()
                }
            },
            Expression::Get(target, field) => {
                let instance = self.run_expression(*target.clone());

                match instance {
                    Value::StructInstance { environment, .. } => if let Some(value) = environment.borrow().get(field.clone()) {
                        match value {
                            Value::Function { name, params, body, environment, .. } => Value::Function { name, params, body, environment, context: Some(*target) },
                            _ => value,
                        }
                    } else {
                        panic!("Undefined field / method: {}", field)
                    },
                    Value::Struct { methods, .. } => if let Some(value) = methods.borrow().get(&field.clone()) {
                        value.clone()
                    } else {
                        panic!("Undefined static method: {}", field)
                    },
                    _ => unreachable!("{:?}", instance)
                }
            },
            Expression::Infix(left, op, right) => {
                let left = self.run_expression(*left);
                let right = self.run_expression(*right);

                match (left, op, right) {
                    (Value::Number(l), Op::Add, Value::Number(r)) => Value::Number(l + r),
                    (Value::Number(l), Op::Multiply, Value::Number(r)) => Value::Number(l * r),
                    (Value::Number(l), Op::Divide, Value::Number(r)) => Value::Number(l / r),
                    (Value::Number(l), Op::Subtract, Value::Number(r)) => Value::Number(l - r),
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
                    _ => todo!()
                }
            },
            Expression::List(items) => {
                let items = items.into_iter().map(|i| self.run_expression(i)).collect::<Vec<Value>>();

                Value::List(Rc::new(RefCell::new(items)))
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
                let definition = self.run_expression(*definition);

                let (name, field_definitions, methods) = match definition.clone() {
                    Value::Struct { name, fields, methods } => (name, fields, methods),
                    _ => unreachable!()
                };

                let mut environment = Environment::new();

                for (field, value) in fields {
                    if ! field_definitions.contains(&Parameter { name: field.clone() }) {
                        panic!("The definition of structure {} does not contain a field named {}.", name, field.clone());
                    }

                    let value = self.run_expression(value);

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
                let callable = self.run_expression(*callable);
                let arguments: Vec<Value> = arguments.into_iter().map(|a| self.run_expression(a)).collect();

                match callable {
                    Value::NativeFunction { callback, .. } => return callback(self, arguments),
                    Value::Function { name, params, body, environment, context } => {
                        if params.first() != Some(&Parameter { name: "self".to_string() }) && params.len() != arguments.len() {
                            panic!("Function {} expects {} arguments, only received {}", name, params.len(), arguments.len());
                        }

                        let old_environment = Rc::clone(&self.environment);
                        let new_environment = if environment.is_some() { 
                            Rc::new(RefCell::new(environment.unwrap()))
                        } else {
                            Rc::new(RefCell::new(Environment::new()))
                        };

                        if context.is_some() && params.first() == Some(&Parameter { name: "self".to_string() }) {
                            let context = self.run_expression(context.unwrap());
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
                }
            },
            Expression::Prefix(op, right) => {
                let right = self.run_expression(*right);

                match op {
                    Op::Bang => Value::Bool(! right.to_bool()),
                    Op::Subtract => Value::Number(- right.to_number()),
                    _ => unreachable!()
                }
            },
            Expression::Assign(target, value) => {
                let value = self.run_expression(*value);

                match *target.clone() {
                    Expression::Index(instance, index) => {
                        match self.run_expression(*instance) {
                            Value::List(items) => {
                                match index {
                                    Some(i) => {
                                        let index = self.run_expression(*i).to_number();
                                        items.borrow_mut().insert(index as usize, value.clone());
                                    },
                                    None => {
                                        items.borrow_mut().push(value.clone());
                                    }
                                }
                            },
                            _ => panic!("You can only assign and append items to lists.")
                        };
                    },
                    Expression::Get(instance, field) => {
                        match self.run_expression(*instance) {
                            // TODO: Check if the field exists on the definition before
                            // actually doing the assignment.
                            Value::StructInstance { environment, .. } => {
                                environment.borrow_mut().set(field, value.clone())
                            },
                            Value::Struct { methods, .. } => {
                                if ! matches!(value.clone(), Value::Function { .. }) {
                                    panic!("Can only assign methods to static properties on a structure.")
                                } else {
                                    methods.borrow_mut().insert(field, value.clone());
                                }
                            },
                            _ => todo!()
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
        }
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

    fn run(&mut self) {
        while let Some(statement) = self.ast.next() {
            let _r = self.run_statement(statement.clone());
        }

        if ! ::std::env::args().filter(|a| a == "--debug").collect::<Vec<String>>().is_empty() {
            self.env().dump();
            dbg!(self.globals.clone());
        }
    }
}