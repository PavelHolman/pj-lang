pub use lexer::Lexer;
use tokens::{Token, TokenKind};

pub mod lexer;
pub mod tokens;


#[test]
fn test_simple_lexing() {
    let source_code =
        r#"def foo(x <- Int, y <-> String) -> Int | String:
    return if x > 5 then x else y;

let x = 2 + 2 - 1;
let y = "str";
let z = foo(x, y);

print(x, y, z);"#;

    let tokens = Lexer::new(source_code)
        .map(Result::unwrap)
        .collect::<Vec<Token>>();

    let expected = vec![
        Token::new(TokenKind::Def),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::Identifier("foo".to_string())),
        Token::new(TokenKind::LParen),
        Token::new(TokenKind::Identifier("x".to_string())),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::LArrow),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::Identifier("Int".to_string())),
        Token::new(TokenKind::Comma),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::Identifier("y".to_string())),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::LRArrow),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::Identifier("String".to_string())),
        Token::new(TokenKind::RParen),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::RArrow),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::Identifier("Int".to_string())),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::Pipe),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::Identifier("String".to_string())),
        Token::new(TokenKind::Colon),
        Token::new(TokenKind::NewLine),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::Return),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::If),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::Identifier("x".to_string())),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::Greater),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::Int(5)),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::Then),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::Identifier("x".to_string())),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::Else),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::Identifier("y".to_string())),
        Token::new(TokenKind::Semicolon),
        Token::new(TokenKind::NewLine),
        Token::new(TokenKind::NewLine),
        Token::new(TokenKind::Let),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::Identifier("x".to_string())),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::Equal),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::Int(2)),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::Plus),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::Int(2)),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::Minus),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::Int(1)),
        Token::new(TokenKind::Semicolon),
        Token::new(TokenKind::NewLine),
        Token::new(TokenKind::Let),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::Identifier("y".to_string())),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::Equal),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::String("str".to_string())),
        Token::new(TokenKind::Semicolon),
        Token::new(TokenKind::NewLine),
        Token::new(TokenKind::Let),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::Identifier("z".to_string())),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::Equal),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::Identifier("foo".to_string())),
        Token::new(TokenKind::LParen),
        Token::new(TokenKind::Identifier("x".to_string())),
        Token::new(TokenKind::Comma),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::Identifier("y".to_string())),
        Token::new(TokenKind::RParen),
        Token::new(TokenKind::Semicolon),
        Token::new(TokenKind::NewLine),
        Token::new(TokenKind::NewLine),
        Token::new(TokenKind::Identifier("print".to_string())),
        Token::new(TokenKind::LParen),
        Token::new(TokenKind::Identifier("x".to_string())),
        Token::new(TokenKind::Comma),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::Identifier("y".to_string())),
        Token::new(TokenKind::Comma),
        Token::new(TokenKind::Whitespace),
        Token::new(TokenKind::Identifier("z".to_string())),
        Token::new(TokenKind::RParen),
        Token::new(TokenKind::Semicolon),
    ];

    assert_eq!(tokens, expected);
}
