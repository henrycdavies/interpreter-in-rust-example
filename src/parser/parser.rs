use crate::token::Token;

pub struct Parser<'a> {
    pub tokens: &'a [Token],
    lookahead: &'a Token,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Parser<'a> {
        let lookahead = &tokens[0];
        Parser{tokens, lookahead }
    }
    pub fn parse(&self) {
        println!("Parsing {:?} {:?}", &self.tokens, &self.lookahead);
        self.expr();
        unimplemented!()
    }
    fn expr(&self) {
        unimplemented!()
    }
    fn expr_opt(&self) {
        unimplemented!()
    }
    fn eat(&self) {
        unimplemented!()
    }
    fn error(&self, msg: String) {
        unimplemented!()
    }
}
