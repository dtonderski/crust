use std::{fmt, iter::Peekable, slice::Iter};

use crate::{
    lexer::token::{Token, TokenKind},
    parser::ast::{Constant, Expression, FunctionDeclaration, Program, Statement, UnaryOperator},
};

pub mod ast;

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
    LBrace,
    RBrace,
    LParen,
    RParen,
    Semicolon,
    Void,
    Negation,
    BitwiseComplement,
    LogicalNegation,
    NoToken,
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
            TokenKind::Negation => ExpectedToken::Negation,
            TokenKind::BitwiseComplement => ExpectedToken::BitwiseComplement,
            TokenKind::LogicalNegation => ExpectedToken::LogicalNegation,
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
            ExpectedToken::LBrace => write!(f, "`{{`"),
            ExpectedToken::RBrace => write!(f, "`}}`"),
            ExpectedToken::LParen => write!(f, "`(`"),
            ExpectedToken::RParen => write!(f, "`)`"),
            ExpectedToken::Semicolon => write!(f, "`;`"),
            ExpectedToken::Void => write!(f, "`void`"),
            ExpectedToken::Negation => write!(f, "negation `-`"),
            ExpectedToken::BitwiseComplement => write!(f, "bitwise complement `-`"),
            ExpectedToken::LogicalNegation => write!(f, "logical negation `!`"),

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
    let token = next(tokens)?;

    match &token.kind {
        TokenKind::Number(value) => Ok(Expression::Constant(Constant::Int(*value))),
        TokenKind::BitwiseComplement | TokenKind::Negation | TokenKind::LogicalNegation => {
            let inner_expression = parse_expression(tokens)?;
            let operator = match &token.kind {
                TokenKind::BitwiseComplement => UnaryOperator::BinaryComplement,
                TokenKind::Negation => UnaryOperator::Negation,
                TokenKind::LogicalNegation => UnaryOperator::LogicalNegation,
                _ => unreachable!(),
            };
            return Ok(Expression::UnaryOperation {
                operator: operator,
                expression: Box::new(inner_expression),
            });
        }
        TokenKind::LParen => {
            let inner_expression = parse_expression(tokens)?;
            expect_next(tokens, TokenKind::RParen)?;
            return Ok(inner_expression);
        }
        found => Err(ParseError::UnexpectedToken {
            expected: ExpectedToken::Number,
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
    let token = tokens.peek().ok_or(ParseError::UnexpectedEnd)?;
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
