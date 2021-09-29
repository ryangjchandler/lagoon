use crate::environment::{Value, NativeMethodCallback};
use crate::interpreter::Interpreter;

pub struct ListObject;

impl ListObject {
    pub fn get(name: String) -> NativeMethodCallback {
        match name.as_str() {
            "isEmpty" => list_is_empty,
            "isNotEmpty" => list_is_not_empty,
            _ => panic!("Undefined method: {}", name),
        }
    }
}

fn list_is_empty(_: &mut Interpreter, context: Value, arguments: Vec<Value>) -> Value {
    super::arity("List.isEmpty()", 0, &arguments);

    Value::Bool(context.to_vec().borrow().is_empty())
}

fn list_is_not_empty(_: &mut Interpreter, context: Value, arguments: Vec<Value>) -> Value {
    super::arity("List.isEmpty()", 0, &arguments);

    Value::Bool(! context.to_vec().borrow().is_empty())
}