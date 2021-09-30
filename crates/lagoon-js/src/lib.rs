use lagoon_parser::*;
use thiserror::Error;
use colored::*;

#[derive(Error, Debug)]
pub enum TranspilerError {
    #[error("Failed to write file to output.")]
    FailedToWriteFile,

    #[error("Unable to transpile statement: {0:?}")]
    NotImplementedStatement(Statement),

    #[error("Unable to transpile expression: {0:?}")]
    NotImplementedExpression(Expression),
}

impl TranspilerError {
    pub fn print(self) {
        eprintln!("{}", format!("{}", self).red().bold());
    }
}

pub fn transpile(ast: Program) -> Result<String, TranspilerError> {
    let mut js = String::new();

    for statement in ast {
        transpile_statement(&mut js, statement)?;
    }

    Ok(js)
}

fn transpile_statement(js: &mut String, statement: Statement) -> Result<(), TranspilerError> {
    match statement {
        Statement::LetDeclaration { name, initial } => {
            js.push_str("let ");
            js.push_str(&name);
            if initial.is_some() {
                js.push_str(" = ");
                transpile_expression(js, initial.unwrap())?;
            }
            js.push_str(";");
        },
        Statement::Expression { expression } => transpile_expression(js, expression)?,
        _ => return Err(TranspilerError::NotImplementedStatement(statement)),
    };

    Ok(())
}

fn transpile_expression(js: &mut String, expression: Expression) -> Result<(), TranspilerError> {
    match expression {
        Expression::String(s) => {
            js.push('"');
            js.push_str(&s);
            js.push('"');
        },
        _ => return Err(TranspilerError::NotImplementedExpression(expression))
    };

    Ok(())
}