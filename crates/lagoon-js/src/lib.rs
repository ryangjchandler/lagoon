use lagoon_parser::*;
use thiserror::Error;
use colored::*;

const POLYFILL: &'static [u8] = include_bytes!("./polyfill.js");

#[derive(Error, Debug)]
pub enum TranspilerError {
    #[error("Failed to write file to output.")]
    FailedToWriteFile,

    #[error("Unable to transpile statement: {0:?}")]
    NotImplementedStatement(Statement),

    #[error("Unable to transpile expression: {0:?}")]
    NotImplementedExpression(Expression),

    #[error("Unable to transpile operator: {0:?}")]
    NotImplementedOperator(Op),
}

impl TranspilerError {
    pub fn print(self) {
        eprintln!("{}", format!("{}", self).red().bold());
    }
}

pub fn transpile(ast: Program) -> Result<String, TranspilerError> {
    let mut js = String::new();

    js.push_str(
        std::str::from_utf8(POLYFILL).unwrap()
    );

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
        Statement::FunctionDeclaration { name, params, body } => {
            js.push_str("function ");
            js.push_str(&name);
            js.push('(');
            js.push_str(&params.iter().map(|p| p.clone().name).collect::<Vec<String>>().join(", "));
            js.push(')');
            transpile_block(js, body)?;
        },
        Statement::If { condition, then, otherwise } => {
            js.push_str("if (");
            transpile_expression(js, condition)?;
            js.push_str(")");
            transpile_block(js, then)?;
            
            if otherwise.is_some() {
                js.push_str(" else ");
                transpile_block(js, otherwise.unwrap())?;
            }
        },
        Statement::Return { value } => {
            js.push_str("return ");
            transpile_expression(js, value)?;
        },
        Statement::Expression { expression } => {
            transpile_expression(js, expression)?
        },
        _ => return Err(TranspilerError::NotImplementedStatement(statement)),
    };

    js.push(';');

    Ok(())
}

fn transpile_block(js: &mut String, block: Block) -> Result<(), TranspilerError> {
    js.push_str("{ \n");
    for statement in block {
        transpile_statement(js, statement)?;
    }
    js.push_str("\n}");
    Ok(())
}

fn transpile_expression(js: &mut String, expression: Expression) -> Result<(), TranspilerError> {
    match expression {
        Expression::String(s) => {
            js.push('"');
            js.push_str(&s);
            js.push('"');
        },
        Expression::Number(n) => {
            js.push_str(&n.to_string());
        },
        Expression::Bool(b) => {
            js.push_str(if b { "true" } else { "false" });
        },
        Expression::Null => js.push_str("null"),
        Expression::Identifier(i) => {
            js.push_str(&i)
        },
        Expression::List(items) => {
            js.push_str("[");
            for (i, item) in items.clone().into_iter().enumerate() {
                transpile_expression(js, item)?;

                if i != items.len() - 1 {
                    js.push_str(", ");
                }
            }
            js.push_str("]");
        },
        Expression::Call(identifier, arguments) => {
            transpile_expression(js, *identifier)?;
            js.push('(');
            for (i, argument) in arguments.iter().enumerate() {
                transpile_expression(js, argument.clone())?;
            
                if i != arguments.len() - 1 {
                    js.push_str(", ");
                }
            }
            js.push(')');
        },
        Expression::Infix(left, op, right) => {
            transpile_expression(js, *left)?;
            js.push_str(op_to_string(op)?);
            transpile_expression(js, *right)?;
        },
        _ => return Err(TranspilerError::NotImplementedExpression(expression))
    };

    Ok(())
}

fn op_to_string(op: Op) -> Result<&'static str, TranspilerError> {
    Ok(match op {
        Op::Add => "+",
        Op::Subtract => "-",
        Op::Multiply => "*",
        Op::Divide => "/",
        Op::LessThan => "<",
        Op::LessThanOrEquals => "<=",
        Op::GreaterThan => ">",
        Op::GreaterThanOrEquals => ">=",
        _ => return Err(TranspilerError::NotImplementedOperator(op)),
    })
}