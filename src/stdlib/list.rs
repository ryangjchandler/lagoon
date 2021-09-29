use std::rc::Rc;

use crate::environment::{Value, NativeMethodCallback};
use crate::interpreter::Interpreter;

pub struct ListObject;

impl ListObject {
    pub fn get(name: String) -> NativeMethodCallback {
        match name.as_str() {
            "isEmpty" => list_is_empty,
            "isNotEmpty" => list_is_not_empty,
            "reverse" => list_reverse,
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

    let rc = Rc::clone(&context.to_vec());
    rc.borrow_mut().reverse();

    Value::List(rc)
}