use std::rc::Rc;
use std::cell::RefCell;

use crate::environment::{Value, NativeMethodCallback};
use crate::interpreter::{Interpreter, InterpreterResult};

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
            "first" => list_first,
            _ => panic!("Undefined method: {}", name),
        }
    }
}

fn list_is_empty(_: &mut Interpreter, context: Value, arguments: Vec<Value>) -> Result<Value, InterpreterResult> {
    super::arity("List.isEmpty()", 0, &arguments);

    Ok(Value::Bool(context.to_vec().borrow().is_empty()))
}

fn list_is_not_empty(_: &mut Interpreter, context: Value, arguments: Vec<Value>) -> Result<Value, InterpreterResult> {
    super::arity("List.isNotEmpty()", 0, &arguments);

    Ok(Value::Bool(! context.to_vec().borrow().is_empty()))
}

fn list_reverse(_: &mut Interpreter, context: Value, arguments: Vec<Value>) -> Result<Value, InterpreterResult> {
    super::arity("List.reverse()", 0, &arguments);

    let mut list = context.to_vec().borrow().clone();
    list.reverse();

    Ok(Value::List(Rc::new(RefCell::new(list))))
}

fn list_join(_: &mut Interpreter, context: Value, arguments: Vec<Value>) -> Result<Value, InterpreterResult> {
    super::arity("List.join()", 1, &arguments);

    let list = context.to_vec().borrow().clone();
    let separator = arguments.get(0).unwrap().clone().to_string();
    let result = list.into_iter().map(|a| a.to_string()).collect::<Vec<String>>().join(&separator);
    
    Ok(Value::String(result))
}

fn list_filter(interpreter: &mut Interpreter, context: Value, arguments: Vec<Value>) -> Result<Value, InterpreterResult> {
    super::arity("List.filter()", 1, &arguments);

    let callback = arguments.get(0).unwrap().clone();
    let mut new_list: Vec<Value> = Vec::new();

    for item in context.to_vec().borrow().clone().into_iter() {
        if interpreter.call(callback.clone(), vec![item.clone()])?.to_bool() {
            new_list.push(item);
        }
    }

    Ok(Value::List(Rc::new(RefCell::new(new_list))))
}

fn list_each(interpreter: &mut Interpreter, context: Value, arguments: Vec<Value>) -> Result<Value, InterpreterResult> {
    super::arity("List.each()", 1, &arguments);

    let callback = arguments.get(0).unwrap().clone();

    for v in context.clone().to_vec().borrow().iter() {
        interpreter.call(callback.clone(), vec![v.clone()])?.to_bool();   
    }

    Ok(context)
}

fn list_map(interpreter: &mut Interpreter, context: Value, arguments: Vec<Value>) -> Result<Value, InterpreterResult> {
    super::arity("List.map()", 1, &arguments);

    let callback = arguments.get(0).unwrap().clone();
    let mut list = context.clone().to_vec().borrow().clone();

    for (i, v) in list.clone().iter().enumerate() {
        let result = interpreter.call(callback.clone(), vec![v.clone()])?;

        list[i] = result;
    }

    Ok(Value::List(Rc::new(RefCell::new(list))))
}

fn list_first(interpreter: &mut Interpreter, context: Value, arguments: Vec<Value>) -> Result<Value, InterpreterResult> {
    let list = context.clone().to_vec().borrow().clone();

    if list.is_empty() {
        return Ok(Value::Null)
    }

    if arguments.len() == 1 {
        let callback = arguments.get(0).unwrap().clone();

        for v in list.iter() {
            let result = interpreter.call(callback.clone(), vec![v.clone()])?;

            if result.clone().to_bool() {
                return Ok(v.clone());
            }
        }
    }

    Ok(list.first().unwrap().clone())
}