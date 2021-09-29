use std::rc::Rc;
use std::cell::RefCell;

use crate::environment::{Value, NativeMethodCallback};
use crate::interpreter::Interpreter;

pub struct ListObject;

impl ListObject {
    pub fn get(name: String) -> NativeMethodCallback {
        match name.as_str() {
            "isEmpty" => list_is_empty,
            "isNotEmpty" => list_is_not_empty,
            "reverse" => list_reverse,
            "join" => list_join,
            _ => panic!("Undefined method: {}", name),
        }
    }
}

fn list_is_empty(_: &mut Interpreter, context: Value, arguments: Vec<Value>) -> Value {
    super::arity("List.isEmpty()", 0, &arguments);

    Value::Bool(context.to_vec().borrow().is_empty())
}

fn list_is_not_empty(_: &mut Interpreter, context: Value, arguments: Vec<Value>) -> Value {
    super::arity("List.isNotEmpty()", 0, &arguments);

    Value::Bool(! context.to_vec().borrow().is_empty())
}

fn list_reverse(_: &mut Interpreter, context: Value, arguments: Vec<Value>) -> Value {
    super::arity("List.reverse()", 0, &arguments);

    let mut list = context.to_vec().borrow().clone();
    list.reverse();

    Value::List(Rc::new(RefCell::new(list)))
}

fn list_join(_: &mut Interpreter, context: Value, arguments: Vec<Value>) -> Value {
    super::arity("List.join()", 1, &arguments);

    let list = context.to_vec().borrow().clone();
    let separator = arguments.get(0).unwrap().clone().to_string();
    let result = list.into_iter().map(|a| a.to_string()).collect::<Vec<String>>().join(&separator);
    
    Value::String(result)
}