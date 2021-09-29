use hashbrown::HashMap;

use crate::token::Token;

pub type Program = Vec<Statement>;
pub type Block = Vec<Statement>;
pub type Identifier = String;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Return {
        value: Expression,
    },
    FunctionDeclaration {
        name: Identifier,
        params: Vec<Parameter>,
        body: Block,
    },
    StructDeclaration {
        name: Identifier,
        fields: Vec<Parameter>,
    },
    LetDeclaration {
        name: Identifier,
        initial: Option<Expression>,
    },
    If {
        condition: Expression,
        then: Block,
        otherwise: Option<Block>
    },
    Expression {
        expression: Expression,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(f64),
    String(String),
    InterpolatedString(String),
    Bool(bool),
    Null,
    Identifier(Identifier),
    Assign(Box<Expression>, Box<Expression>),
    Infix(Box<Expression>, Op, Box<Expression>),
    Prefix(Op, Box<Expression>),
    Call(Box<Expression>, Vec<Expression>),
    Struct(Box<Expression>, HashMap<Identifier, Expression>),
    Closure(Vec<Parameter>, Vec<Statement>),
    Get(Box<Expression>, Identifier),
    List(Vec<Expression>),
}

impl Expression {
    pub fn some(self) -> Option<Self> {
        Some(self)
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Bang,
    Equals,
    NotEquals,
    Assign,
    LessThan,
    GreaterThan,
    LessThanOrEquals,
    GreaterThanOrEquals,
    And,
    Or,
    Pow,
}

impl Op {
    pub fn token(token: Token) -> Self {
        match token {
            Token::Plus => Self::Add,
            Token::Minus => Self::Subtract,
            Token::Asterisk => Self::Multiply,
            Token::Slash => Self::Divide,
            Token::Bang => Self::Bang,
            Token::Percent => Self::Modulo,
            Token::Equals => Self::Equals,
            Token::NotEquals => Self::NotEquals,
            Token::Assign => Self::Assign,
            Token::LessThan => Self::LessThan,
            Token::GreaterThan => Self::GreaterThan,
            Token::LessThanOrEquals => Self::LessThanOrEquals,
            Token::GreaterThanOrEquals => Self::GreaterThanOrEquals,
            Token::And => Self::And,
            Token::Or => Self::Or,
            Token::Pow => Self::Pow,
            _ => unreachable!("{:?}", token)
        }
    }
}