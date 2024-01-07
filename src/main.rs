pub mod lexer;
pub mod parser;
pub mod token;
use lexer::Lexer;
use parser::Parser;

fn main() {
    let in_string = "foo = bar + baz * qux true;";
    let lexer = Lexer{input: in_string};
    let tokens = lexer.lex();
    let parser = Parser::new(&tokens);
    parser.parse();
}
