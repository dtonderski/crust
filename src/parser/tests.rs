use super::{
    ExpectedToken, ParseError,
    ast::{Constant, Expression, FunctionDeclaration, Program, Statement},
    parse,
};
use crate::lexer::{token::TokenKind, tokenize};

fn parse_source(source: &str) -> Result<Program, ParseError> {
    let tokens = tokenize(source).expect("source should tokenize");
    parse(&tokens)
}

#[test]
fn parses_simple_return_program() {
    let program = parse_source("int main() { return 2; }").expect("parser should succeed");

    assert_eq!(
        program,
        Program {
            function: FunctionDeclaration {
                name: "main".to_string(),
                statement: Statement::Return(Expression::Constant(Constant::Int(2))),
            },
        }
    );
}

#[test]
fn parses_void_parameter_list() {
    let program = parse_source("int main(void) { return 3; }").expect("parser should succeed");

    assert_eq!(
        program,
        Program {
            function: FunctionDeclaration {
                name: "main".to_string(),
                statement: Statement::Return(Expression::Constant(Constant::Int(3))),
            },
        }
    );
}

#[test]
fn rejects_missing_function_name() {
    let err = parse_source("int () { return 2; }").expect_err("parser should fail");

    assert_eq!(
        err,
        ParseError::UnexpectedToken {
            expected: ExpectedToken::Identifier,
            found: TokenKind::LParen
        }
    );
}

#[test]
fn rejects_missing_return_expression() {
    let err = parse_source("int main() { return; }").expect_err("parser should fail");

    assert_eq!(
        err,
        ParseError::UnexpectedToken {
            expected: ExpectedToken::Number,
            found: TokenKind::Semicolon
        }
    );
}

#[test]
fn rejects_missing_semicolon() {
    let err = parse_source("int main() { return 2 }").expect_err("parser should fail");

    assert_eq!(
        err,
        ParseError::UnexpectedToken {
            expected: ExpectedToken::Semicolon,
            found: TokenKind::RBrace
        }
    );
}

#[test]
fn rejects_extra_tokens_after_function() {
    let err = parse_source("int main() { return 2; } int").expect_err("parser should fail");

    assert_eq!(
        err,
        ParseError::UnexpectedToken {
            expected: ExpectedToken::NoToken,
            found: TokenKind::Int
        }
    );
}

#[test]
fn rejects_unexpected_end_of_input() {
    let err = parse_source("int main() { return 2;").expect_err("parser should fail");

    assert_eq!(err, ParseError::UnexpectedEnd);
}
