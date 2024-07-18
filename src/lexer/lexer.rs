pub struct Lexer {
    input: String,
    input_size: u32,
    pos: u32,
    read_pos: u32,
    character: char,
}

impl Lexer {
    fn new(content: String) -> Lexer {
        Lexer {
            input: content.clone(),
            input_size: content.len() as u32,
            pos: 0,
            read_pos: 0,
            character: ' ',
        }
    }

    
}