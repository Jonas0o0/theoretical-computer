use std::{env, process};
use std::fs;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::codegen::CodeGen;

mod lexer;
mod ast;
mod parser;
mod codegen;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <fichier.jmp>");
        return;
    }

    let input_filename = &args[1];
    println!("Démarrage de la compilation de {}...", input_filename);

    let code = match fs::read_to_string(input_filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Erreur : Impossible de lire le fichier '{}' : {}", input_filename, e);
            return;
        }
    };

    println!("\n=== 1. ANALYSE LEXICALE (Tokens) ===");
    let mut lexer = Lexer::new(&code);
    let tokens = lexer.tokenize();

    println!("{:#?}", tokens);
    println!("\n=== 2. ANALYSE SYNTAXIQUE (AST) ===");
    let mut parser = Parser::new(tokens);

    let ast = match parser.parse_program() {
        Ok(ast) => ast,
        Err(erreur) => {
            eprintln!("ERREUR DE SYNTAXE :\n{}", erreur);
            return;
        }
    };
    println!("{:#?}", ast);
    println!("\n=== 3. GÉNÉRATION DE CODE (Assembleur) ===");
    let mut compiler = CodeGen::new();

    let assembleur = match compiler.compile(&ast) {
        Ok(asm) => asm,
        Err(erreur) => {
            eprintln!("ERREUR DE COMPILATION :\n{}", erreur);
            return;
        }
    };
    println!("{}", assembleur);
    let asm_filename = input_filename.replace(".jmp", ".asm");

    match fs::write(&asm_filename, &assembleur) {
        Ok(_) => println!("\nSuccès ! Fichier assembleur sauvegardé sous : {}", asm_filename),
        Err(e) => {eprintln!("\nErreur lors de la sauvegarde du fichier assembleur : {}", e); return},
    }

    println!("\n=== 4. APPEL DE L'ASSEMBLEUR ===");
    let status = process::Command::new("cargo")
        .arg("run")
        .arg("-p")
        .arg("assembler")
        .arg("--")
        .arg(&asm_filename)
        .status();

    match status {
        Ok(exit_status) if exit_status.success() => {
            println!("\nPipeline complet terminé avec succès !");
        }
        _ => {
            eprintln!("\nErreur : L'assemblage a échoué.");
            process::exit(1);
        }
    }
}