use hashbrown::HashMap;
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::rc::Rc;
use std::cell::RefCell;
use std::iter::Iterator;

use crate::interpreter::Interpreter;
use crate::ast::{Block, Parameter, Expression};
use crate::interpreter::InterpreterResult;

pub type NativeFunctionCallback = fn (&mut Interpreter, Vec<Value>) -> Value;
pub type NativeMethodCallback = fn (&mut Interpreter, Value, Vec<Value>) -> Result<Value, InterpreterResult>;

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: impl Into<String>, value: Value) {
        self.values.insert(name.into(), value);
    }

    pub fn get(&self, name: impl Into<String>) -> Option<Value> {
        if let Some(value) = self.values.get(&name.into()) {
            Some(value.clone())
        } else {
            None
        }
    }

    pub fn drop(&mut self, name: impl Into<String>) {
        self.values.remove(&name.into());
    }

    pub fn dump(&self) {
        dbg!(self.values.clone());
    }
}

#[derive(Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Null,
    Bool(bool),
    Struct {
        name: String,
        fields: Vec<Parameter>,
        methods: Rc<RefCell<HashMap<String, Value>>>,
    },
    StructInstance {
        environment: Rc<RefCell<Environment>>,
        definition: Box<Value>,
    },
    List(Rc<RefCell<Vec<Value>>>),
    Function {
        name: String,
        params: Vec<Parameter>,
        body: Block,
        environment: Option<Environment>,
        context: Option<Expression>,
    },
    NativeFunction {
        name: String,
        callback: NativeFunctionCallback,
    },
    NativeMethod {
        name: String,
        callback: NativeMethodCallback,
        context: Expression,
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", match self {
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.to_string(),
            Value::Null => "null".to_string(),
            Value::NativeFunction { name, .. } => format!("<{}>", name),
            Value::Function { name, params, .. } => format!("<{}>({})", name, params.into_iter().map(|p| p.name.clone()).collect::<Vec<String>>().join(", ")),
            Value::StructInstance { definition, .. } => {
                let name = match *definition.clone() {
                    Value::Struct { name, .. } => name,
                    _ => unreachable!()
                };

                format!("<{}>", name)
            },
            Value::Struct { name, methods, fields, .. } => {
                let name = format!("<struct:{}>", name);
                let mut fields = fields.into_iter().map(|p| p.name.clone()).collect::<Vec<String>>();
                let mut methods = methods.borrow().keys().into_iter().map(|p| {
                    let mut p = p.clone();
                    p.push_str("()");
                    p
                }).collect::<Vec<String>>(); 

                fields.append(&mut methods);

                let fields = format!("{} {{ {} }}", name, fields.join(", "));

                fields
            },
            Value::List(items) => {
                let mut buffer = String::from("[");
                let items = items.borrow();

                for (i, item) in items.iter().enumerate() {
                    buffer.push_str(&item.clone().to_string());

                    if i != items.len() - 1 {
                        buffer.push_str(", ");
                    }
                }

                buffer.push_str("]");
                buffer
            },
            Value::Bool(true) => "true".to_string(),
            Value::Bool(false) => "false".to_string(),
            _ => todo!(),
        })
    }
}

impl Value {
    pub fn to_vec(self) -> Rc<RefCell<Vec<Value>>> {
        match self {
            Value::List(list) => list,
            _ => unreachable!()
        }
    }

    pub fn to_number(self) -> f64 {
        match self {
            Value::Number(n) => n,
            Value::Bool(true) => 1.0,
            Value::Null | Value::Bool(false) => 0.0,
            Value::String(s) => {
                match s.trim().parse::<f64>() {
                    Ok(f) => f,
                    Err(_) => 0.0
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn to_string(self) -> String {
        match self {
            Value::String(s) => s,
            Value::Number(n) => n.to_string(),
            Value::Bool(_) => self.to_number().to_string(),
            Value::Null => "".to_string(),
            v @ Value::Function { .. } | v @ Value::StructInstance { .. } | v @ Value::List(..) => format!("{:?}", v),
            _ => todo!(),
        }
    }

    pub fn to_bool(self) -> bool {
        match self {
            Value::Bool(true) | Value::Function { .. } => true,
            Value::String(s) => !s.is_empty(),
            Value::Number(n) => n > 0.0,
            _ => false,
        }
    }

    pub fn is(self, other: Value) -> bool {
        match (self, other) {
            (Value::String(l), r) => l == r.to_string(),
            (Value::Number(n), r) => n == r.to_number(),
            (Value::Bool(true), r) => r.to_bool() == true,
            (Value::Bool(false), r) => r.to_bool() == false,
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }
}