#[allow(unused_imports)]
use super::token::Token;

#[allow(unused_imports)]
use super::lexer::Lexer;

#[allow(unused_imports)]
use anyhow::Result;

#[test]
fn get_next_token() -> Result<()> {
    let input = "=+(){},;";
    let mut lexer = Lexer::new(input.into());

    let tokens = vec![
        Token::Assign,
        Token::Plus,
        Token::Lparen,
        Token::Rparen,
        Token::LSquirly,
        Token::RSquirly,
        Token::Comma,
        Token::Semicolon,
    ];

    for token in tokens {
        let next_token = lexer.next_token()?;
        println!("expected: {:?}, received {:?}", token, next_token);
        assert_eq!(token, next_token);
    }


    return Ok(());
}

#[test]
fn get_next_complete() -> Result<()> {
    let input = r#"let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);
    !-/*5;
    5 < 10 > 5;
    if (5 < 10) {
        return true;
    } else {
        return false;
    }

    10 == 10;
    10 != 9;
    "#;


    let mut lex = Lexer::new(input.into());

    let tokens = vec![
        Token::Let,
        Token::Ident(String::from("five")),
        Token::Assign,
        Token::Int(String::from("5")),
        Token::Semicolon,
        Token::Let,
        Token::Ident(String::from("ten")),
        Token::Assign,
        Token::Int(String::from("10")),
        Token::Semicolon,
        Token::Let,
        Token::Ident(String::from("add")),
        Token::Assign,
        Token::Function,
        Token::Lparen,
        Token::Ident(String::from("x")),
        Token::Comma,
        Token::Ident(String::from("y")),
        Token::Rparen,
        Token::LSquirly,
        Token::Ident(String::from("x")),
        Token::Plus,
        Token::Ident(String::from("y")),
        Token::Semicolon,
        Token::RSquirly,
        Token::Semicolon,
        Token::Let,
        Token::Ident(String::from("result")),
        Token::Assign,
        Token::Ident(String::from("add")),
        Token::Lparen,
        Token::Ident(String::from("five")),
        Token::Comma,
        Token::Ident(String::from("ten")),
        Token::Rparen,
        Token::Semicolon,


        Token::Bang,
        Token::Dash,
        Token::ForwardSlash,
        Token::Asterisk,
        Token::Int(String::from("5")),
        Token::Semicolon,
        Token::Int(String::from("5")),
        Token::LessThan,
        Token::Int(String::from("10")),
        Token::GreaterThan,
        Token::Int(String::from("5")),
        Token::Semicolon,
        Token::If,
        Token::Lparen,
        Token::Int(String::from("5")),
        Token::LessThan,
        Token::Int(String::from("10")),
        Token::Rparen,
        Token::LSquirly,
        Token::Return,
        Token::True,
        Token::Semicolon,
        Token::RSquirly,
        Token::Else,
        Token::LSquirly,
        Token::Return,
        Token::False,
        Token::Semicolon,
        Token::RSquirly,

        Token::Int(String::from("10")),
        Token::Equal,
        Token::Int(String::from("10")),
        Token::Semicolon,
        Token::Int(String::from("10")),
        Token::NotEqual,
        Token::Int(String::from("9")),
        Token::Semicolon,

        Token::Eof,
    ];

    for token in tokens {
        let next_token = lex.next_token()?;
        println!("expected: {:?}, received {:?}", token, next_token);
        assert_eq!(token, next_token);
    }

    return Ok(());
}