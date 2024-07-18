#[derive(Debug)]
pub struct Lexer {
    input: String,
    pos: usize,
    read_pos: usize,
    character: char,
}

impl Lexer {
    pub fn new(content: String) -> Lexer {
        Lexer {
            input: content.clone(),
            pos: 0,
            read_pos: 0,
            character: ' ',
        }
    }
}