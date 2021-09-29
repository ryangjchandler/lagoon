use std::env::args;
use std::fs::read_to_string;

mod token;
mod parser;
mod ast;
mod interpreter;
mod environment;
mod stdlib;

fn main() {
    let file = args().nth(1).unwrap();
    let contents = read_to_string(file).unwrap();

    let tokens = token::generate(contents.as_str());
    let ast = parser::parse(tokens).unwrap();

    interpreter::interpret(ast);
}
