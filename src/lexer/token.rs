use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Int,
    Return,

    Identifier(String),
    Number(i64),

    LBrace,
    RBrace,

    LParen,
    RParen,

    Semicolon,

    Void,

    Negation,
    BitwiseComplement,
    LogicalNegation,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Int => write!(f, "`int`"),
            TokenKind::Return => write!(f, "`return`"),
            TokenKind::Identifier(value) => write!(f, "identifier `{value}`"),
            TokenKind::Number(value) => write!(f, "number `{value}`"),
            TokenKind::LBrace => write!(f, "`{{`"),
            TokenKind::RBrace => write!(f, "`}}`"),
            TokenKind::LParen => write!(f, "`(`"),
            TokenKind::RParen => write!(f, "`)`"),
            TokenKind::Semicolon => write!(f, "`;`"),
            TokenKind::Void => write!(f, "`void`"),
            TokenKind::Negation => write!(f, "negation `-`"),
            TokenKind::BitwiseComplement => write!(f, "bitwise complement `-`"),
            TokenKind::LogicalNegation => write!(f, "logical negation `!`"),
        }
    }
}
