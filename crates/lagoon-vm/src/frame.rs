use std::collections::HashMap;
use std::rc::Rc;
use std::cell::{RefCell, RefMut, Ref};
use crate::Value;
use crate::native;

pub type Refd<T> = Rc<RefCell<T>>;

#[derive(Clone)]
pub struct Frame {
    values: Refd<HashMap<String, Value>>,
    ret: usize,
}

fn register_natives(env: &mut HashMap<String, Value>) {
    env.insert("println".to_owned(), Value::NativeFunction(native::native_println));
}

impl Frame {
    pub fn new(ret: usize) -> Self {
        let mut values = HashMap::new();

        register_natives(&mut values);

        Self {
            values: Rc::new(RefCell::new(values)),
            ret: ret
        }
    }

    pub fn env_mut(&mut self) -> RefMut<HashMap<String, Value>> {
        RefCell::borrow_mut(&self.values)
    }

    pub fn env(&self) -> Ref<HashMap<String, Value>> {
        RefCell::borrow(&self.values)
    }
}