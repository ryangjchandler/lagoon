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
            "filter" => list_filter,
            "each" => list_each,
            "map" => list_map,
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

fn list_filter(interpreter: &mut Interpreter, context: Value, arguments: Vec<Value>) -> Value {
    super::arity("List.filter()", 1, &arguments);

    let callback = arguments.get(0).unwrap().clone();

    let list: Vec<Value> = context.to_vec().borrow().clone().into_iter().filter(|v| {
        interpreter.call(callback.clone(), vec![v.clone()]).to_bool()
    }).collect();

    Value::List(Rc::new(RefCell::new(list)))
}

fn list_each(interpreter: &mut Interpreter, context: Value, arguments: Vec<Value>) -> Value {
    super::arity("List.each()", 1, &arguments);

    let callback = arguments.get(0).unwrap().clone();

    for v in context.clone().to_vec().borrow().iter() {
        interpreter.call(callback.clone(), vec![v.clone()]).to_bool();   
    }

    context
}

fn list_map(interpreter: &mut Interpreter, context: Value, arguments: Vec<Value>) -> Value {
    super::arity("List.map()", 1, &arguments);

    let callback = arguments.get(0).unwrap().clone();
    let mut list = context.clone().to_vec().borrow().clone();

    for (i, v) in list.clone().iter().enumerate() {
        let result = interpreter.call(callback.clone(), vec![v.clone()]);

        list[i] = result;
    }

    Value::List(Rc::new(RefCell::new(list)))
}