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

fn run_chunk(chunk: &mut Chunk) {
    while let Some(code) = chunk.next() {
        match code {
            Code::Pop => {
                chunk.stack.pop();
            },
            Code::MakeString(s) => {
                chunk.stack.push(Value::String(s));
            },
            Code::MakeNumber(n) => {
                chunk.stack.push(Value::Number(n));
            },
            Code::True => {
                chunk.stack.push(Value::True);
            },
            Code::False => {
                chunk.stack.push(Value::False);
            },
            Code::Null => {
                chunk.stack.push(Value::Null);
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
            Code::StartLabel(name) => {
                let mut c = chunk.clone();

                while ! matches!(c.next(), Some(Code::EndLabel(..))) {
                    // noop.
                }

                chunk.ip = c.ip
            },
            Code::Return => break,
            Code::Push(value) => {
                chunk.stack.push(value);
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

                        callback(chunk, &args)
                    },
                    Value::Label(ip) => {
                        let original = chunk.ip;
                        
                        chunk.ip = ip;
                        chunk.start_frame();

                        run_chunk(chunk);

                        chunk.ip = original + 1;

                        Value::Null
                    },
                    _ => todo!()
                };

                chunk.stack.push(result);
            },
            _ => todo!("Code: {:?}", code),
        }
    }
}

pub fn execute(program: Program) {
    let mut compiler = Compiler::new(program.into_iter());
    let mut chunk = compiler.compile();

    dbg!(&chunk);

    run_chunk(&mut chunk);
}