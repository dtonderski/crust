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
fn skips_preprocessor_directive_lines() {
    assert_eq!(
        kinds("#include <stdio.h>\n  #define VALUE 2\nint main() { return 2; }"),
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
fn tokenizes_identifier_characters_after_first_character() {
    assert_eq!(
        kinds("int _main2() { return 7; }")[1],
        TokenKind::Identifier("_main2".to_string())
    );
}

#[test]
fn tokenizes_comparison_and_logical_operators() {
    assert_eq!(
        kinds("return a&&b||c==d!=e<f<=g>h>=i;"),
        vec![
            TokenKind::Return,
            TokenKind::Identifier("a".to_string()),
            TokenKind::LogicalAnd,
            TokenKind::Identifier("b".to_string()),
            TokenKind::LogicalOr,
            TokenKind::Identifier("c".to_string()),
            TokenKind::Equal,
            TokenKind::Identifier("d".to_string()),
            TokenKind::NotEqual,
            TokenKind::Identifier("e".to_string()),
            TokenKind::LessThan,
            TokenKind::Identifier("f".to_string()),
            TokenKind::LessThanOrEqual,
            TokenKind::Identifier("g".to_string()),
            TokenKind::GreaterThan,
            TokenKind::Identifier("h".to_string()),
            TokenKind::GreaterThanOrEqual,
            TokenKind::Identifier("i".to_string()),
            TokenKind::Semicolon,
        ]
    );
}

#[test]
fn tokenizes_logical_negation_separately_from_not_equal() {
    assert_eq!(
        kinds("return !1 != 0;"),
        vec![
            TokenKind::Return,
            TokenKind::LogicalNegation,
            TokenKind::Number(1),
            TokenKind::NotEqual,
            TokenKind::Number(0),
            TokenKind::Semicolon,
        ]
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

#[test]
fn rejects_hash_in_code() {
    let err = tokenize("int main() { return 1 # 2; }").expect_err("lexer should fail");

    assert!(matches!(err, LexError::UnexpectedChar('#')));
}

#[test]
fn rejects_single_logical_and_or_assignment_tokens() {
    assert!(matches!(
        tokenize("return 1 & 2;"),
        Err(LexError::UnexpectedChar('&'))
    ));
    assert!(matches!(
        tokenize("return 1 | 2;"),
        Err(LexError::UnexpectedChar('|'))
    ));
    assert!(matches!(
        tokenize("return a = 1;"),
        Err(LexError::UnexpectedChar('='))
    ));
}
