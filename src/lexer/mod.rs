pub mod token;

use crate::lexer::token::{Token, TokenKind};
use std::num::ParseIntError;

#[derive(Debug)]
pub enum LexError {
    InvalidNumber(std::num::ParseIntError),
    UnexpectedChar(char),
}

impl From<ParseIntError> for LexError {
    fn from(value: ParseIntError) -> Self {
        return LexError::InvalidNumber(value);
    }
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, LexError> {
    let chars: Vec<char> = source.chars().collect();
    let mut tokens: Vec<Token> = Vec::new();

    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        match c {
            ' ' | '\n' | '\r' | '\t' => {
                i += 1;
            }
            '(' | ')' | '{' | '}' | ';' => {
                let kind = match c {
                    '(' => TokenKind::LParen,
                    ')' => TokenKind::RParen,
                    '{' => TokenKind::LBrace,
                    '}' => TokenKind::RBrace,
                    ';' => TokenKind::Semicolon,
                    _ => unreachable!(),
                };

                tokens.push(Token { kind });
                i += 1;
            }
            '0'..='9' => {
                let start = i;
                while i < chars.len() && chars[i].is_ascii_digit() {
                    i += 1;
                }

                let text: String = chars[start..i].iter().collect();
                let value = text.parse::<i64>()?;

                tokens.push(Token {
                    kind: TokenKind::Number(value),
                })
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let start = i;

                while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }

                let text: String = chars[start..i].iter().collect();

                let kind = match text.as_str() {
                    "int" => TokenKind::Int,
                    "return" => TokenKind::Return,
                    _ => TokenKind::Identifier(text),
                };

                tokens.push(Token { kind });
            }

            _ => return Err(LexError::UnexpectedChar(c)),
        }
    }
    return Ok(tokens);
}
