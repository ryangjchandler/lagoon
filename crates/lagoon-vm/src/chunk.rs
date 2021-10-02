use crate::Stack;
use crate::Frame;
use crate::Code;
use crate::NativeFunction;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::{RefCell, RefMut, Ref};

pub type Refd<T> = Rc<RefCell<T>>;

#[derive(Clone)]
pub struct Chunk {
    pub stack: Stack<Value>,
    frames: Vec<Frame>,
    code: Vec<Code>,
    labels: HashMap<String, usize>,
    pub ip: usize,
}

impl Chunk {
    pub fn new(code: Vec<Code>, labels: HashMap<String, usize>) -> Self {
        let frames = vec![
            Frame::new(0),
        ];

        Self {
            frames: frames,
            stack: Stack::new(),
            code: code,
            labels: labels,
            ip: 0,
        }
    }

    pub fn frame(&self) -> &Frame {
        self.frames.last().expect("Cannot retrieve a frame from an empty stack.")
    }

    pub fn frame_mut(&mut self) -> &mut Frame {
        self.frames.last_mut().expect("Cannot retrieve a frame from an empty frame stack.")
    }
}

impl Iterator for Chunk {
    type Item = Code;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ip >= self.code.len() {
            None
        } else {
            let code = self.code.get(self.ip);

            self.ip += 1;

            Some(code.unwrap().clone())
        }
    }
}

impl ::std::fmt::Debug for Chunk {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "\n=== START PROGRAM ===")?;

        for (i, code) in self.code.iter().enumerate() {
            write!(f, "\n{}     {:?}", i, code)?;
        }

        write!(f, "\n=== END PROGRAM ===")
    }
}

#[derive(Clone)]
pub enum Value {
    String(String),
    Number(f64),
    Null,
    True,
    False,
    Label(String),
    NativeFunction(NativeFunction),
}

impl ::std::fmt::Display for Value {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", match self {
            Value::String(s) => s.clone(),
            Value::Number(n) => format!("{}", n),
            _ => format!("{:?}", self),
        })
    }
}

impl ::std::fmt::Debug for Value {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", match self {
            Value::String(s) => format!("String({})", s),
            Value::Number(n) => format!("Number({})", n),
            Value::True => "Bool(true)".to_owned(),
            Value::False => "Bool(false)".to_owned(),
            Value::Null => "Null".to_owned(),
            _ => todo!()
        })
    }
}