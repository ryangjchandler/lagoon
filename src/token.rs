use logos::{Lexer, Logos};

pub fn generate(input: &str) -> Vec<Token> {
    Token::lexer(input).collect()
}

fn to_string(lex: &mut Lexer<Token>) -> Option<String> {
    let mut string: String = lex.slice().to_string();

    if string.starts_with("$") {
        string.remove(0);
    }

    if string.starts_with("\"") {
        string.remove(0);
    }

    if string.ends_with('"') {
        string.remove(string.len() - 1);
    }

    Some(string)
}

fn to_float(lex: &mut Lexer<Token>) -> Option<f64> {
    Some(lex.slice().parse().ok()?)
}

#[derive(Debug, Clone, Logos, PartialEq)]
pub enum Token {
    #[token("fn")]
    Fn,
    #[token("let")]
    Let,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("struct")]
    Struct,
    #[token("while")]
    While,
    #[token("return")]
    Return,
    #[token("for")]
    For,
    #[token("in")]
    In,

    #[regex(r"[a-zA-Z_?]+", to_string)]
    Identifier(String),

    #[regex(r"([0-9]+[.])?[0-9]+", to_float)]
    Number(f64),
    #[regex(r##""(?:[^"\\]|\\.)*""##, to_string)]
    String(String),
    #[regex(r##"\$"(?:[^"\\]|\\.)*""##, to_string)]
    InterpolatedString(String),

    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Asterisk,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("**")]
    Pow,

    #[token("=")]
    Assign,
    #[token("==")]
    Equals,
    #[token("!=")]
    NotEquals,
    #[token("<")]
    LessThan,
    #[token(">")]
    GreaterThan,
    #[token("<=")]
    LessThanOrEquals,
    #[token(">=")]
    GreaterThanOrEquals,

    #[token("&&")]
    And,
    #[token("||")]
    Or,

    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("null")]
    Null,

    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token("!")]
    Bang,
    #[token(".")]
    Dot,

    Eof,

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

impl Into<String> for Token {
    fn into(self) -> String {
        match self {
            Token::Identifier(s) => s,
            Token::String(s) => s,
            _ => unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_recognise_reserved_keywords() {
        let mut lexer = Token::lexer("fn let true false if else while struct");

        assert_eq!(lexer.next(), Some(Token::Fn));
        assert_eq!(lexer.next(), Some(Token::Let));
        assert_eq!(lexer.next(), Some(Token::True));
        assert_eq!(lexer.next(), Some(Token::False));
        assert_eq!(lexer.next(), Some(Token::If));
        assert_eq!(lexer.next(), Some(Token::Else));
        assert_eq!(lexer.next(), Some(Token::While));
        assert_eq!(lexer.next(), Some(Token::Struct));
    }

    #[test]
    fn it_can_recognise_symbols() {
        let mut lexer = Token::lexer("( ) { } +-*/ = == != : .");

        assert_eq!(lexer.next(), Some(Token::LeftParen));
        assert_eq!(lexer.next(), Some(Token::RightParen));
        assert_eq!(lexer.next(), Some(Token::LeftBrace));
        assert_eq!(lexer.next(), Some(Token::RightBrace));
        assert_eq!(lexer.next(), Some(Token::Plus));
        assert_eq!(lexer.next(), Some(Token::Minus));
        assert_eq!(lexer.next(), Some(Token::Asterisk));
        assert_eq!(lexer.next(), Some(Token::Slash));
        assert_eq!(lexer.next(), Some(Token::Assign));
        assert_eq!(lexer.next(), Some(Token::Equals));
        assert_eq!(lexer.next(), Some(Token::NotEquals));
        assert_eq!(lexer.next(), Some(Token::Colon));
        assert_eq!(lexer.next(), Some(Token::Dot));
    }

    #[test]
    fn it_can_recognise_identifiers() {
        let mut lexer = Token::lexer("hello_world HelloWorld hello_world? helloWorld");

        assert_eq!(lexer.next(), Some(Token::Identifier("hello_world".to_owned())));
        assert_eq!(lexer.next(), Some(Token::Identifier("HelloWorld".to_owned())));
        assert_eq!(lexer.next(), Some(Token::Identifier("hello_world?".to_owned())));
        assert_eq!(lexer.next(), Some(Token::Identifier("helloWorld".to_owned())));
    }

    #[test]
    fn it_can_recognise_numbers() {
        let mut lexer = Token::lexer("12345 6789.01");

        assert_eq!(lexer.next(), Some(Token::Number(12345.0)));
        assert_eq!(lexer.next(), Some(Token::Number(6789.01)));
    }

    #[test]
    fn it_can_recognise_strings() {
        let mut lexer = Token::lexer(r##""testing" "testing with \"" "testing \n""##);

        assert_eq!(lexer.next(), Some(Token::String(r##"testing"##.to_owned())));
        assert_eq!(lexer.next(), Some(Token::String(r##"testing with \""##.to_owned())));
        assert_eq!(lexer.next(), Some(Token::String(r##"testing \n"##.to_owned())));
    }

    #[test]
    fn it_can_recognise_interpolated_strings() {
        let mut lexer = Token::lexer(r##"$"testing""##);

        assert_eq!(lexer.next(), Some(Token::InterpolatedString(r##"testing"##.to_owned())));
    }
}