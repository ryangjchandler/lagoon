use std::collections::HashMap;
use std::rc::Rc;
use std::cell::{RefCell, RefMut, Ref};
use crate::Value;

pub type Refd<T> = Rc<RefCell<T>>;

pub struct Frame {
    values: Refd<HashMap<String, Value>>,
    ret: usize,
}

impl Frame {
    pub fn new(ret: usize) -> Self {
        Self {
            values: Rc::new(RefCell::new(HashMap::new())),
            ret: ret
        }
    }

    pub fn env_mut(&mut self) -> RefMut<HashMap<String, Value>> {
        self.values.borrow_mut()
    }

    pub fn env(&self) -> Ref<HashMap<String, Value>> {
        self.values.borrow()
    }
}