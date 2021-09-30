use lagoon_parser::Program;
use lagoon_js::TranspilerError;
use std::fs::write;

pub fn js(ast: Program, output: &str) -> Result<(), TranspilerError> {
    let js = lagoon_js::transpile(ast)?;

    match write(output, js) {
        Err(_) => return Err(TranspilerError::FailedToWriteFile),
        _ => Ok(())
    }
}