use crate::lexer::Lexer;
use crate::parser::Parser; // N'oublie pas d'importer ton Parser !

mod lexer;
mod ast;
mod parser;

fn main() {
    let code = "
        let x = 10;
        if x == 10 {
            poke(252, 1);
        }
    ";

    println!("=== 1. ANALYSE LEXICALE (Tokens) ===");
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize();

    println!("{:#?}\n", tokens);

    println!("=== 2. ANALYSE SYNTAXIQUE (AST) ===");
    let mut parser = Parser::new(tokens);

    match parser.parse_program() {
        Ok(ast) => {
            println!("Compilation réussie ! Voici l'AST :");
            println!("{:#?}", ast);
        }
        Err(erreur) => {
            println!("ERREUR DE SYNTAXE :\n{}", erreur);
        }
    }
}