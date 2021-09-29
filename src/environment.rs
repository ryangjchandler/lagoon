use hashbrown::HashMap;
use std::fmt::{Debug, Formatter, Result};
use std::rc::Rc;
use std::cell::RefCell;

use crate::interpreter::Interpreter;
use crate::ast::{Block, Parameter, Expression};

pub type NativeFunctionCallback = fn (&mut Interpreter, Vec<Value>) -> Value;

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
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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
            Value::Bool(true) => "true".to_string(),
            Value::Bool(false) => "false".to_string(),
            _ => todo!(),
        })
    }
}

impl Value {
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
            v @ Value::Function { .. } | v @ Value::StructInstance { .. } => format!("{:?}", v),
            _ => todo!(),
        }
    }

    pub fn to_bool(self) -> bool {
        match self {
            Value::Bool(true) => true,
            Value::String(s) => !s.is_empty(),
            Value::Function { .. } => true,
            Value::Number(n) => n > 0.0,
            _ => false,
        }
    }
}