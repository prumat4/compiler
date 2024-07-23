/*

programm is a set of statements

here we will need 3 nodes:
- root
- statement 
- expr


to parse the let statement we need to parse 3 parts: let <ident> = <expr>;

we will use currToken and peekToken to traverse the tokens
we nee peekToken ot decide what to do with currToken, cause in most cases the info that currToken is

*/

pub trait Node {
    fn token_literal(&self) -> String; 
}

trait Statement: Node {
    fn statement_node(&self);
}

trait Expression: Node {
    fn expression_node(&self);
}

pub struct Programm {
    statements: Vec<Statement>
}

impl Programm {
    pub fn new() -> Programm {
        todo!("?");
    }

    pub fn token_literal(&self) -> u8 {
        if self.statements.len() > 0 {
            return self.statements[0].token_literal();
        } else {
            return 0;
        }
    }
}

struct Identifier {
    token: Token,
    value: Vec<u8> 

}

impl Identifier {
    fn token_literal(&self) -> u8 {
        self.token.token_literal()
    } 
}

struct LetStatement {
    token: Token,
    name: Identifier.
    expression: Expression
} 

impl LetStatement {
    fn token_literal(&self) -> u8 {
        self.token.token_literal()
    } 
}