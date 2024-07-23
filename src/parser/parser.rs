use crate::lexer::lexer::Lexer;
use crate::lexer::token::Token;

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    curr: Token,
    peek: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Parser<'a> {
        let curr = lexer.next_token().clone();
        let peek = lexer.next_token().clone();
        
        Parser {
            lexer,
            curr,
            peek,
        }
    }

    pub fn next_token(&mut self) {
        self.curr = self.peek.clone();
        self.peek = self.lexer.next_token();
    }

    pub fn parse_programm(&self) {
        todo!("parse_programm is not implemented yet");
    }

}
