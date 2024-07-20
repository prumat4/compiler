#[derive(PartialEq, Debug)]
pub enum TokenType {
    Ident(String),
    Int(String),

    Illegal,
    Eof,
    Assign,

    Bang,
    Dash,
    ForwardSlash,
    Asterisk,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,

    Plus,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    LSquirly,
    RSquirly,

    Function,
    Let,

    If,
    Else,
    Return,
    True,
    False,
}

#[derive(PartialEq, Debug)]
pub struct Token {
    token_type: TokenType,
    literal: u8,
}

impl Token {
    pub fn new(token_type: TokenType, literal: u8) -> Token {
        Token {
            token_type,
            literal,
        }
    }
}