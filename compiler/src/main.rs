use crate::lexer::Lexer;

mod lexer;
mod ast;
mod parser;

fn main() {
    let code = "let x = 10; poke(252, 1);";
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize();

    println!("{:#?}", tokens);
}