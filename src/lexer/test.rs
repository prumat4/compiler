use super::token::{Token, TokenType};
use super::lexer::Lexer;
use anyhow::Result;

#[test]
fn test_next_token() -> Result<()> {
    let input = "=+(){},;";
    let mut lexer = Lexer::new(input.into());

    let tokens = vec![
        Token::new(TokenType::Assign, b'='),
        Token::new(TokenType::Plus, b'+'),
        Token::new(TokenType::Lparen, b'('),
        Token::new(TokenType::Rparen, b')'),
        Token::new(TokenType::LSquirly, b'{'),
        Token::new(TokenType::RSquirly, b'}'),
        Token::new(TokenType::Comma, b','),
        Token::new(TokenType::Semicolon, b';'),
    ];

    for token in tokens {
        let next_token = lexer.next_token()?;
        println!("expected: {:?}, received {:?}", token, next_token);
        assert_eq!(token, next_token);
    }

    return Ok(());
}