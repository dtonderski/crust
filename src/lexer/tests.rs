use super::{
    LexError,
    token::{Token, TokenKind},
    tokenize,
};

fn tokens(source: &str) -> Vec<Token> {
    tokenize(source).expect("source should tokenize")
}

fn kinds(source: &str) -> Vec<TokenKind> {
    tokens(source).into_iter().map(|token| token.kind).collect()
}

#[test]
fn tokenizes_simple_return_program() {
    assert_eq!(
        kinds("int main() { return 2; }"),
        vec![
            TokenKind::Int,
            TokenKind::Identifier("main".to_string()),
            TokenKind::LParen,
            TokenKind::RParen,
            TokenKind::LBrace,
            TokenKind::Return,
            TokenKind::Number(2),
            TokenKind::Semicolon,
            TokenKind::RBrace,
        ]
    );
}

#[test]
fn tokenizes_void_parameter_list() {
    assert_eq!(
        kinds("int main(void) { return 0; }"),
        vec![
            TokenKind::Int,
            TokenKind::Identifier("main".to_string()),
            TokenKind::LParen,
            TokenKind::Void,
            TokenKind::RParen,
            TokenKind::LBrace,
            TokenKind::Return,
            TokenKind::Number(0),
            TokenKind::Semicolon,
            TokenKind::RBrace,
        ]
    );
}

#[test]
fn skips_line_and_block_comments() {
    assert_eq!(
        kinds("int /* function name */ main() { // body\n return 42; }"),
        vec![
            TokenKind::Int,
            TokenKind::Identifier("main".to_string()),
            TokenKind::LParen,
            TokenKind::RParen,
            TokenKind::LBrace,
            TokenKind::Return,
            TokenKind::Number(42),
            TokenKind::Semicolon,
            TokenKind::RBrace,
        ]
    );
}

#[test]
fn tokenizes_identifier_characters_after_first_character() {
    assert_eq!(
        kinds("int _main2() { return 7; }")[1],
        TokenKind::Identifier("_main2".to_string())
    );
}

#[test]
fn rejects_identifier_immediately_after_number() {
    let err = tokenize("int main() { return 123abc; }").expect_err("lexer should fail");

    assert!(matches!(err, LexError::UnexpectedChar('a')));
}

#[test]
fn rejects_unterminated_block_comment() {
    let err = tokenize("int main() { /*").expect_err("lexer should fail");

    assert!(matches!(err, LexError::UnterminatedBlockComment));
}

#[test]
fn rejects_unknown_character() {
    let err = tokenize("int main() { return @; }").expect_err("lexer should fail");

    assert!(matches!(err, LexError::UnexpectedChar('@')));
}
