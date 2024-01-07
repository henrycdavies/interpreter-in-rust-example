#[derive(Debug)]
pub enum TokenType {
    Num,
    Plus,
    Times,
    Identifier,
    True,
    False,
    Equals,
    SemiColon,
    EOF,
}   

#[derive(Debug)]
pub struct Token {
    pub tpe: TokenType,
    pub text: String,
    pub start_pos: usize,
}
