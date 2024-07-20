use super::token::{Token, TokenType};
use anyhow::Result;

#[derive(Debug)]
pub struct Lexer {
    input: Vec<u8>,
    input_size: usize,
    pos: usize,
    read_pos: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(content: Vec<u8>) -> Lexer {
        let mut lexer = Lexer {
            input: content.clone(),
            input_size: content.len(),
            pos: 0,
            read_pos: 0,
            ch: 0,
        };
        lexer.read_char();
        lexer
    }

    pub fn read_char(&mut self) {
        if self.read_pos >= self.input_size {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_pos];
        }
        self.pos = self.read_pos;
        self.read_pos += 1;
    }

    fn peek(&self) -> u8 {
        if self.read_pos >= self.input.len() {
            return 0;
        } else {
            return self.input[self.read_pos];
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_ident(&mut self) -> String {
        let pos = self.pos;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char();
        }
        return String::from_utf8_lossy(&self.input[pos..self.pos]).to_string();
    }

    fn read_int(&mut self) -> String {
        let pos = self.pos;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        return String::from_utf8_lossy(&self.input[pos..self.pos]).to_string();
    }

    pub fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace();

        let token = match self.ch {
            b'{' => Token::new(TokenType::LSquirly, self.ch),
            b'}' => Token::new(TokenType::RSquirly, self.ch),
            b'(' => Token::new(TokenType::Lparen, self.ch),
            b')' => Token::new(TokenType::Rparen, self.ch),
            b',' => Token::new(TokenType::Comma, self.ch),
            b';' => Token::new(TokenType::Semicolon, self.ch),
            b'+' => Token::new(TokenType::Plus, self.ch),
            b'-' => Token::new(TokenType::Dash, self.ch),
            b'!' => {
                if self.peek() == b'=' {
                    self.read_char();
                    Token::new(TokenType::NotEqual, b'!')
                } else {
                    Token::new(TokenType::Bang, self.ch)
                }
            }
            b'>' => Token::new(TokenType::GreaterThan, self.ch),
            b'<' => Token::new(TokenType::LessThan, self.ch),
            b'*' => Token::new(TokenType::Asterisk, self.ch),
            b'/' => Token::new(TokenType::ForwardSlash, self.ch),
            b'=' => {
                if self.peek() == b'=' {
                    self.read_char();
                    Token::new(TokenType::Equal, b'=')
                } else {
                    Token::new(TokenType::Assign, self.ch)
                }
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_ident();
                return Ok(match ident.as_str() {
                    "fn" => Token::new(TokenType::Function, self.ch),
                    "let" => Token::new(TokenType::Let, self.ch),
                    "if" => Token::new(TokenType::If, self.ch),
                    "false" => Token::new(TokenType::False, self.ch),
                    "true" => Token::new(TokenType::True, self.ch),
                    "return" => Token::new(TokenType::Return, self.ch),
                    "else" => Token::new(TokenType::Else, self.ch),
                    _ => Token::new(TokenType::Ident(ident), self.ch),
                });
            }
            b'0'..=b'9' => return Ok(Token::new(TokenType::Int(self.read_int()), self.ch)),
            0 => Token::new(TokenType::Eof, self.ch),
            _ => unreachable!("no monkey program should contain these characters"),
        };

        self.read_char();
        Ok(token)
    }
}

#[cfg(test)]
mod test {
    use super::{Token, TokenType};
    use super::Lexer;
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
}