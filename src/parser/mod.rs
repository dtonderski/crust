use std::{fmt, iter::Peekable, slice::Iter};

use crate::{
    lexer::token::{Token, TokenKind},
    parser::ast::{
        BinaryOperator, Constant, Expression, FunctionDeclaration, Program, Statement,
        UnaryOperator,
    },
};

pub mod ast;
mod pretty;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    UnexpectedEnd,
    UnexpectedToken {
        expected: ExpectedToken,
        found: TokenKind,
    },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedEnd => write!(f, "unexpected end of input"),
            ParseError::UnexpectedToken { expected, found } => {
                write!(f, "unexpected token: expected {expected}, found {found}")
            }
        }
    }
}

// Needed because the TokenKind enum includes data
#[derive(Debug, PartialEq, Eq)]
pub enum ExpectedToken {
    Int,
    Return,
    Identifier,
    Number,
    Factor,
    LBrace,
    RBrace,
    LParen,
    RParen,
    Semicolon,
    Void,
    Minus,
    BitwiseComplement,
    LogicalNegation,
    NoToken,
    Addition,
    Multiplication,
    Division,
    Modulo,
    LogicalAnd,
    LogicalOr,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

impl From<&TokenKind> for ExpectedToken {
    fn from(kind: &TokenKind) -> Self {
        match kind {
            TokenKind::Int => ExpectedToken::Int,
            TokenKind::Return => ExpectedToken::Return,
            TokenKind::Identifier(_) => ExpectedToken::Identifier,
            TokenKind::Number(_) => ExpectedToken::Number,
            TokenKind::LBrace => ExpectedToken::LBrace,
            TokenKind::RBrace => ExpectedToken::RBrace,
            TokenKind::LParen => ExpectedToken::LParen,
            TokenKind::RParen => ExpectedToken::RParen,
            TokenKind::Semicolon => ExpectedToken::Semicolon,
            TokenKind::Void => ExpectedToken::Void,
            TokenKind::Minus => ExpectedToken::Minus,
            TokenKind::BitwiseComplement => ExpectedToken::BitwiseComplement,
            TokenKind::LogicalNegation => ExpectedToken::LogicalNegation,
            TokenKind::Addition => ExpectedToken::Addition,
            TokenKind::Multiplication => ExpectedToken::Multiplication,
            TokenKind::Division => ExpectedToken::Division,
            TokenKind::Modulo => ExpectedToken::Modulo,
            TokenKind::LogicalAnd => ExpectedToken::LogicalAnd,
            TokenKind::LogicalOr => ExpectedToken::LogicalOr,
            TokenKind::Equal => ExpectedToken::Equal,
            TokenKind::NotEqual => ExpectedToken::NotEqual,
            TokenKind::LessThan => ExpectedToken::LessThan,
            TokenKind::LessThanOrEqual => ExpectedToken::LessThanOrEqual,
            TokenKind::GreaterThan => ExpectedToken::GreaterThan,
            TokenKind::GreaterThanOrEqual => ExpectedToken::GreaterThanOrEqual,
        }
    }
}

impl fmt::Display for ExpectedToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpectedToken::Int => write!(f, "`int`"),
            ExpectedToken::Return => write!(f, "`return`"),
            ExpectedToken::Identifier => write!(f, "identifier"),
            ExpectedToken::Number => write!(f, "number"),
            ExpectedToken::Factor => {
                write!(f, "number, unary operator, or parenthesized expression")
            }
            ExpectedToken::LBrace => write!(f, "`{{`"),
            ExpectedToken::RBrace => write!(f, "`}}`"),
            ExpectedToken::LParen => write!(f, "`(`"),
            ExpectedToken::RParen => write!(f, "`)`"),
            ExpectedToken::Semicolon => write!(f, "`;`"),
            ExpectedToken::Void => write!(f, "`void`"),
            ExpectedToken::Minus => write!(f, "minus `-`"),
            ExpectedToken::BitwiseComplement => write!(f, "bitwise complement `-`"),
            ExpectedToken::LogicalNegation => write!(f, "logical negation `!`"),
            ExpectedToken::Addition => write!(f, "`+`"),
            ExpectedToken::Multiplication => write!(f, "`*`"),
            ExpectedToken::Division => write!(f, "division `/`"),
            ExpectedToken::Modulo => write!(f, "modulo `%`"),
            ExpectedToken::LogicalAnd => write!(f, "`&&`"),
            ExpectedToken::LogicalOr => write!(f, "`||`"),
            ExpectedToken::Equal => write!(f, "`==`"),
            ExpectedToken::NotEqual => write!(f, "`!=`"),
            ExpectedToken::LessThan => write!(f, "`<`"),
            ExpectedToken::LessThanOrEqual => write!(f, "`<=`"),
            ExpectedToken::GreaterThan => write!(f, "`>`"),
            ExpectedToken::GreaterThanOrEqual => write!(f, "`>=`"),

            ExpectedToken::NoToken => write!(f, "<no token>"),
        }
    }
}

type TokenIter<'a> = Peekable<Iter<'a, Token>>;

pub fn parse(tokens: &Vec<Token>) -> Result<Program, ParseError> {
    let mut tokens_iter = tokens.iter().peekable();

    let fun = parse_function(&mut tokens_iter)?;

    expect_next_end(&mut tokens_iter)?;

    return Ok(Program { function: fun });
}

pub fn parse_function(tokens: &mut TokenIter<'_>) -> Result<FunctionDeclaration, ParseError> {
    expect_next(tokens, TokenKind::Int)?;

    let name = expect_next_identifier(tokens)?;

    expect_next(tokens, TokenKind::LParen)?;
    let is_next_void = is_next(tokens, TokenKind::Void)?;
    if is_next_void {
        expect_next(tokens, TokenKind::Void)?;
    }
    expect_next(tokens, TokenKind::RParen)?;
    expect_next(tokens, TokenKind::LBrace)?;

    let statement = parse_statement(tokens)?;

    expect_next(tokens, TokenKind::RBrace)?;

    return Ok(FunctionDeclaration {
        name: name.clone(),
        statement,
    });
}

pub fn parse_statement(tokens: &mut TokenIter<'_>) -> Result<Statement, ParseError> {
    expect_next(tokens, TokenKind::Return)?;

    let expression = parse_expression(tokens)?;

    expect_next(tokens, TokenKind::Semicolon)?;
    return Ok(Statement::Return(expression));
}

pub fn parse_expression(tokens: &mut TokenIter<'_>) -> Result<Expression, ParseError> {
    let mut term = parse_term(tokens)?;
    loop {
        let operator = match tokens.peek().map(|token| &token.kind) {
            Some(TokenKind::Addition) => BinaryOperator::Addition,
            Some(TokenKind::Minus) => BinaryOperator::Subtraction,
            _ => break,
        };
        next(tokens)?;
        let next_term = parse_term(tokens)?;
        term = Expression::BinaryOperation {
            operator,
            left: Box::new(term),
            right: Box::new(next_term),
        };
    }
    return Ok(term);
}

pub fn parse_term(tokens: &mut TokenIter<'_>) -> Result<Expression, ParseError> {
    let mut factor = parse_factor(tokens)?;
    loop {
        let operator = match tokens.peek().map(|token| &token.kind) {
            Some(TokenKind::Multiplication) => BinaryOperator::Multiplication,
            Some(TokenKind::Division) => BinaryOperator::Division,
            Some(TokenKind::Modulo) => BinaryOperator::Modulo,
            _ => break,
        };
        next(tokens)?;
        let next_factor = parse_factor(tokens)?;
        factor = Expression::BinaryOperation {
            operator,
            left: Box::new(factor),
            right: Box::new(next_factor),
        };
    }
    return Ok(factor);
}

pub fn parse_factor(tokens: &mut TokenIter<'_>) -> Result<Expression, ParseError> {
    let token = next(tokens)?;

    match &token.kind {
        TokenKind::Number(value) => Ok(Expression::Constant(Constant::Int(*value))),
        TokenKind::BitwiseComplement | TokenKind::Minus | TokenKind::LogicalNegation => {
            let inner_factor = parse_factor(tokens)?;
            let operator = match &token.kind {
                TokenKind::BitwiseComplement => UnaryOperator::BinaryComplement,
                TokenKind::Minus => UnaryOperator::Negation,
                TokenKind::LogicalNegation => UnaryOperator::LogicalNegation,
                _ => unreachable!(),
            };
            return Ok(Expression::UnaryOperation {
                operator: operator,
                expression: Box::new(inner_factor),
            });
        }
        TokenKind::LParen => {
            let inner_expression = parse_expression(tokens)?;
            expect_next(tokens, TokenKind::RParen)?;
            return Ok(inner_expression);
        }
        found => Err(ParseError::UnexpectedToken {
            expected: ExpectedToken::Factor,
            found: found.clone(),
        }),
    }
}

fn expect_next_identifier<'a>(tokens: &mut TokenIter<'a>) -> Result<&'a String, ParseError> {
    let token = next(tokens)?;
    match &token.kind {
        TokenKind::Identifier(value) => Ok(value),
        found => Err(ParseError::UnexpectedToken {
            expected: ExpectedToken::Identifier,
            found: found.clone(),
        }),
    }
}

fn expect_next(tokens: &mut TokenIter<'_>, expected: TokenKind) -> Result<(), ParseError> {
    let token = next(tokens)?;
    return expect(&token, expected);
}

fn is_next(tokens: &mut TokenIter<'_>, kind: TokenKind) -> Result<bool, ParseError> {
    let token = peek_next(tokens)?;
    return Ok(token.kind == kind);
}

fn expect(token: &Token, expected: TokenKind) -> Result<(), ParseError> {
    if token.kind == expected {
        Ok(())
    } else {
        Err(ParseError::UnexpectedToken {
            expected: ExpectedToken::from(&expected),
            found: token.kind.clone(),
        })
    }
}

fn expect_next_end(tokens: &mut TokenIter<'_>) -> Result<(), ParseError> {
    match tokens.next() {
        None => Ok(()),

        Some(token) => Err(ParseError::UnexpectedToken {
            expected: ExpectedToken::NoToken,
            found: token.kind.clone(),
        }),
    }
}

fn next<'a>(tokens: &mut TokenIter<'a>) -> Result<&'a Token, ParseError> {
    let token = tokens.next().ok_or(ParseError::UnexpectedEnd)?;
    return Ok(token);
}

fn peek_next<'a>(tokens: &mut TokenIter<'a>) -> Result<&'a Token, ParseError> {
    let token = tokens.peek().ok_or(ParseError::UnexpectedEnd)?;
    return Ok(token);
}
