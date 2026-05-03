#[derive(Clone, Debug)]
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

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
}
