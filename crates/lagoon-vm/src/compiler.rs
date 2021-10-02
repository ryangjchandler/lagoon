use lagoon_parser::*;
use crate::Builder;
use crate::Code;
use crate::Chunk;
use std::vec::IntoIter;

pub struct Compiler {
    program: IntoIter<Statement>,
    builder: Builder,
}

impl Compiler {

    pub fn new(program: IntoIter<Statement>) -> Self {
        Self { program: program, builder: Builder::new() }
    }

    pub fn compile(&mut self) -> Chunk {
        while let Some(statement) = self.program.next() {
            self.compile_statement(statement);
        }

        self.builder.clone().into()
    }

    fn compile_statement(&mut self, statement: Statement) {
        match statement {
            Statement::LetDeclaration { name, initial } => {
                // We first need to check if an initial value is present for the
                // variable.
                if initial.is_some() {
                    // If it is present, then we can compile the expression.
                    self.compile_expression(initial.unwrap());
                } else {
                    // If no initial value is present, we use `null` as a default value.
                    self.compile_expression(Expression::Null);
                }

                // Then we need to tell the machine to set the value for this variable
                // to the previous value on the stack.
                self.builder.emit(Code::Set(name))
            },
            Statement::Expression { expression } => {
                // First we compile the expression.
                self.compile_expression(expression);

                // Since we don't care about the result of the expression, we can
                // pop the value off of the stack to prevent it causing issues later on.
                self.builder.emit(Code::Pop);
            },
            _ => todo!("Statement: {:?}", statement),
        }
    }

    fn compile_expression(&mut self, expression: Expression) {
        match expression {
            // If we encounter a string or number, we won't explicitly pass the value straight to
            // the op-code. Instead we'll tell the machine to make a new string. This is going
            // to be slightly more optimised for erroneous programs as we won't be constructing
            // lots of values.
            Expression::String(s) => self.builder.emit(Code::MakeString(s)),
            Expression::Number(n) => self.builder.emit(Code::MakeNumber(n)),
            // `true`, `false` and `null` are all constant values. We can send separate instructions
            // to the machine and handle these in a more optimised way, instead of doing the evaluation later on.
            Expression::Bool(b) => if b { self.builder.emit(Code::True) } else { self.builder.emit(Code::False) },
            Expression::Null => self.builder.emit(Code::Null),
            _ => todo!("Expression: {:?}", expression),
        }
    }
}