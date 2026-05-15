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

    Minus,
    BitwiseComplement,
    LogicalNegation,

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
            TokenKind::Minus => write!(f, "negation `-`"),
            TokenKind::BitwiseComplement => write!(f, "bitwise complement `-`"),
            TokenKind::LogicalNegation => write!(f, "logical negation `!`"),
            TokenKind::Addition => write!(f, "`+`"),
            TokenKind::Multiplication => write!(f, "`*`"),
            TokenKind::Division => write!(f, "`/`"),
            TokenKind::Modulo => write!(f, "`%`"),
            TokenKind::LogicalAnd => write!(f, "`&&`"),
            TokenKind::LogicalOr => write!(f, "`||`"),
            TokenKind::Equal => write!(f, "`==`"),
            TokenKind::NotEqual => write!(f, "`!=`"),
            TokenKind::LessThan => write!(f, "`<`"),
            TokenKind::LessThanOrEqual => write!(f, "`<=`"),
            TokenKind::GreaterThan => write!(f, "`>`"),
            TokenKind::GreaterThanOrEqual => write!(f, "`>=`"),
        }
    }
}
