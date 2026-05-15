use super::{
    ExpectedToken, ParseError,
    ast::{BinaryOperator, Constant, Expression, FunctionDeclaration, Program, Statement},
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
fn parses_logical_and_comparison_precedence() {
    let program = parse_source("int main() { return 1 == 2 || 3 != 4 && 5 <= 6; }")
        .expect("parser should succeed");

    assert_eq!(
        program.function.statement,
        Statement::Return(Expression::BinaryOperation {
            operator: BinaryOperator::LogicalOr,
            left: Box::new(Expression::BinaryOperation {
                operator: BinaryOperator::Equal,
                left: Box::new(Expression::Constant(Constant::Int(1))),
                right: Box::new(Expression::Constant(Constant::Int(2))),
            }),
            right: Box::new(Expression::BinaryOperation {
                operator: BinaryOperator::LogicalAnd,
                left: Box::new(Expression::BinaryOperation {
                    operator: BinaryOperator::NotEqual,
                    left: Box::new(Expression::Constant(Constant::Int(3))),
                    right: Box::new(Expression::Constant(Constant::Int(4))),
                }),
                right: Box::new(Expression::BinaryOperation {
                    operator: BinaryOperator::LessThanOrEqual,
                    left: Box::new(Expression::Constant(Constant::Int(5))),
                    right: Box::new(Expression::Constant(Constant::Int(6))),
                }),
            }),
        })
    );
}

#[test]
fn parses_relational_operators_below_addition() {
    let program =
        parse_source("int main() { return 1 + 2 < 3 * 4; }").expect("parser should succeed");

    assert_eq!(
        program.function.statement,
        Statement::Return(Expression::BinaryOperation {
            operator: BinaryOperator::LessThan,
            left: Box::new(Expression::BinaryOperation {
                operator: BinaryOperator::Addition,
                left: Box::new(Expression::Constant(Constant::Int(1))),
                right: Box::new(Expression::Constant(Constant::Int(2))),
            }),
            right: Box::new(Expression::BinaryOperation {
                operator: BinaryOperator::Multiplication,
                left: Box::new(Expression::Constant(Constant::Int(3))),
                right: Box::new(Expression::Constant(Constant::Int(4))),
            }),
        })
    );
}

#[test]
fn parses_relational_operators_left_associatively() {
    let program =
        parse_source("int main() { return 1 < 2 > 3 >= 4; }").expect("parser should succeed");

    assert_eq!(
        program.function.statement,
        Statement::Return(Expression::BinaryOperation {
            operator: BinaryOperator::GreaterThanOrEqual,
            left: Box::new(Expression::BinaryOperation {
                operator: BinaryOperator::GreaterThan,
                left: Box::new(Expression::BinaryOperation {
                    operator: BinaryOperator::LessThan,
                    left: Box::new(Expression::Constant(Constant::Int(1))),
                    right: Box::new(Expression::Constant(Constant::Int(2))),
                }),
                right: Box::new(Expression::Constant(Constant::Int(3))),
            }),
            right: Box::new(Expression::Constant(Constant::Int(4))),
        })
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
            expected: ExpectedToken::Factor,
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
