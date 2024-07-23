use super::token::Token;
use super::lexer::Lexer;
use super::parser::Parser;

#[test]
fn get_next_token() -> Result<()> {
    let input = "
        let x = 5;
        let y = 10;
        let foobar = 838383;
    ";
    
    let mut lexer = Lexer::new(input.into());
    let mut parser = Parser::new(&mut lexer);

    let programm = parser.parse_programm();

    

    // let tokens = vec![
    //     Token::Assign,
    //     Token::Plus,
    //     Token::Lparen,
    //     Token::Rparen,
    //     Token::LSquirly,
    //     Token::RSquirly,
    //     Token::Comma,
    //     Token::Semicolon,
    // ];

    // for token in tokens {
    //     let next_token = lexer.next_token();
    //     println!("expected: {:?}, received {:?}", token, next_token);
    //     assert_eq!(token, next_token);
    // }

    return Ok(());
}
