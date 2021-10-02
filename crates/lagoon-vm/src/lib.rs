mod stack;
mod builder;
mod compiler;
mod code;
mod chunk;
mod frame;
mod native;

use lagoon_parser::*;
pub(crate) use stack::Stack;
pub(crate) use builder::Builder;
pub(crate) use compiler::Compiler;
pub(crate) use code::Code;
pub(crate) use chunk::Chunk;
pub(crate) use chunk::Value;
pub(crate) use frame::Frame;
pub(crate) use native::NativeFunction;

pub fn execute(program: Program) {
    let mut compiler = Compiler::new(program.into_iter());
    let mut chunk = compiler.compile();

    while let Some(code) = chunk.next() {
        match code {
            Code::Pop => {
                chunk.stack.pop();
            },
            Code::MakeString(s) => {
                chunk.stack.push(Value::String(s));
            },
            Code::Set(name) => {
                let value = chunk.stack.pop();
                chunk.frame_mut().env_mut().insert(name, value);
            },
            Code::Get(name) => {
                let value = {
                    let env = chunk.frame().env();
                    env.get(&name).cloned()
                };

                chunk.stack.push(value.unwrap());
            },
            Code::Call(arity) => {
                let callable = chunk.stack.pop();

                let result: Value = match callable {
                    Value::NativeFunction(callback) => {
                        let mut args = Vec::new();

                        if arity > 0 {
                            for _ in 0..arity {
                                args.push(chunk.stack.pop());
                            }
                        }

                        callback(&mut chunk, &args)
                    },
                    _ => todo!()
                };

                chunk.stack.push(result);
            },
            _ => todo!("Code: {:?}", code),
        }
    }
}