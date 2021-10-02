use crate::Code;
use std::collections::HashMap;
use crate::Chunk;

#[derive(Clone)]
pub struct Builder {
    instructions: Vec<Code>,
    labels: HashMap<String, usize>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            labels: HashMap::new(),
        }
    }

    pub fn emit(&mut self, code: Code) {
        self.instructions.push(code)
    }

    pub fn label(&mut self, name: impl Into<String>) -> usize {
        let index = self.instructions.len();
        self.labels.insert(name.into(), index);
        index
    }
}

impl From<Builder> for Chunk {
    fn from(builder: Builder) -> Chunk {
        Chunk::new(builder.instructions, builder.labels)
    }
}