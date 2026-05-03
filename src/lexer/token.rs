#[derive(Clone, Debug, PartialEq)]
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
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
}
