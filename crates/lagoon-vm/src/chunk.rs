use crate::Stack;
use crate::Frame;
use crate::Code;
use std::collections::HashMap;

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

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
}