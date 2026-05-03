pub mod token;

use crate::lexer::token::{Token, TokenKind};
use std::num::ParseIntError;

#[derive(Debug)]
pub enum LexError {
    InvalidNumber(std::num::ParseIntError),
    UnexpectedChar(char),
    UnterminatedBlockComment,
}

impl From<ParseIntError> for LexError {
    fn from(value: ParseIntError) -> Self {
        return LexError::InvalidNumber(value);
    }
}

fn is_identifier_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_identifier_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
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
            '/' => {
                if i + 1 >= chars.len() {
                    return Err(LexError::UnexpectedChar(c));
                }

                match chars[i + 1] {
                    '/' => {
                        i += 2;
                        while i < chars.len() && chars[i] != '\n' {
                            i += 1;
                        }
                    }
                    '*' => {
                        i += 2;

                        while i + 1 < chars.len() && !(chars[i] == '*' && chars[i + 1] == '/') {
                            i += 1;
                        }

                        if i + 1 >= chars.len() {
                            return Err(LexError::UnterminatedBlockComment);
                        }

                        i += 2;
                    }
                    _ => return Err(LexError::UnexpectedChar(c)),
                }
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

                if i < chars.len() && is_identifier_start(chars[i]) {
                    return Err(LexError::UnexpectedChar(chars[i]));
                }

                let text: String = chars[start..i].iter().collect();
                let value = text.parse::<i64>()?;

                tokens.push(Token {
                    kind: TokenKind::Number(value),
                })
            }
            c if is_identifier_start(c) => {
                let start = i;

                while i < chars.len() && is_identifier_char(chars[i]) {
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
