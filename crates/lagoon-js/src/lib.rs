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
        Statement::StructDeclaration { name, fields } => {
            js.push_str("class ");
            js.push_str(&name);
            js.push_str(" {\n");
            
            for field in fields.clone() {
                js.push_str(&field.name);
                js.push_str(";\n");
            }

            struct_constructor(js, "constructor", &fields.into_iter().map(|p| p.name).collect::<Vec<String>>()[..])?;
            
            js.push_str("\n}");
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

    js.push_str(";\n");

    Ok(())
}

fn struct_constructor(js: &mut String, method: &str, parameters: &[String]) -> Result<(), TranspilerError> {
    js.push_str(method);
    js.push_str(" ({");
    js.push_str(&parameters.join(", "));
    js.push_str("}) {\n");

    for parameter in parameters {
        js.push_str("this.");
        js.push_str(parameter);
        js.push_str("=");
        js.push_str(parameter);
        js.push_str(";\n");
    }

    js.push_str("}");

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
            if is_native_op(&op) {
                transpile_expression(js, *left)?;
                js.push_str(op_to_string(op)?);
                transpile_expression(js, *right)?;
            } else {
                match op {
                    Op::In => {
                        js.push_str("__lagoon_in(");
                        transpile_expression(js, *left)?;
                        js.push_str(", ");
                        transpile_expression(js, *right)?;
                        js.push_str(")");
                    },
                    Op::NotIn => {
                        js.push_str("! ");
                        js.push_str("__lagoon_in(");
                        transpile_expression(js, *left)?;
                        js.push_str(", ");
                        transpile_expression(js, *right)?;
                        js.push_str(")");
                    },
                    _ => unreachable!(),
                }
            }
        },
        Expression::Struct(target, fields) => {
            js.push_str("new ");
            transpile_expression(js, *target)?;
            js.push_str("({\n");

            for (field, value) in fields {
                js.push_str(&field);
                js.push_str(": ");
                transpile_expression(js, value)?;
                js.push_str(",\n");
            }

            js.push_str("\n})");
        },
        Expression::Get(instance, field) => {
            transpile_expression(js, *instance)?;
            js.push_str(".");
            js.push_str(&field);
        },
        _ => return Err(TranspilerError::NotImplementedExpression(expression))
    };

    Ok(())
}

fn is_native_op(op: &Op) -> bool {
    match op {
        Op::In | Op::NotIn => false,
        _ => true,
    }
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
        Op::Modulo => "%",
        Op::Pow => "**",
        Op::Bang => "!",
        Op::Equals => "==",
        Op::NotEquals => "!=",
        Op::Assign => "=",
        Op::And => "&&",
        Op::Or => "||",
        _ => return Err(TranspilerError::NotImplementedOperator(op)),
    })
}