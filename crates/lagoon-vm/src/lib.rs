mod stack;
mod builder;
mod compiler;
mod code;
mod chunk;
mod frame;

use lagoon_parser::*;
pub(crate) use stack::Stack;
pub(crate) use builder::Builder;
pub(crate) use compiler::Compiler;
pub(crate) use code::Code;
pub(crate) use chunk::Chunk;
pub(crate) use chunk::Value;
pub(crate) use frame::Frame;

pub fn execute(program: Program) {
    let mut compiler = Compiler::new(program.into_iter());
    let mut chunk = compiler.compile();

    while let Some(code) = chunk.next() {
        match code {
            Code::MakeString(s) => {
                chunk.stack.push(Value::String(s));
            },
            Code::Set(name) => {
                let value = chunk.stack.pop();
                chunk.frame_mut().env_mut().insert(name, value);
            },
            _ => todo!("Code: {:?}", code),
        }
    }
}