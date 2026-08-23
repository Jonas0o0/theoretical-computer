use std::collections::HashMap;
use crate::ast::{Program, Stmt};

pub struct SymbolTable {
    table: HashMap<String, u8>,
    next_address: u8,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
            next_address: 0,
        }
    }

    pub fn define(&mut self, name: String) -> Result<u8, String> {
        if self.next_address > 239 {
            return Err(format!(
                "Erreur de compilation : Dépassement de la RAM utilisateur ! Trop de variables déclarées. La variable '{}' ne peut pas dépasser l'adresse 239.",
                name
            ));
        }

        let addr = self.next_address;
        self.table.insert(name, addr);
        self.next_address += 1;
        Ok(addr)
    }

    pub fn lookup(&self, name: &str) -> Option<u8> {
        self.table.get(name).copied()
    }
}


pub struct CodeGen {
    symbols: SymbolTable,
    functions: HashMap<String, Vec<Stmt>>,
    output: Vec<String>,
}

impl CodeGen {
    pub fn new() -> Self {
        Self {
            symbols: SymbolTable::new(),
            functions: HashMap::new(),
            output: Vec::new(),
        }
    }

    fn emit(&mut self, instruction: &str) {
        self.output.push(instruction.to_string());
    }

    pub fn compile(&mut self, program: &Program) -> Result<String, String> {
        for stmt in &program.statements {
            self.compile_statement(stmt)?;
        }
        Ok(self.output.join("\n"))
    }

    fn compile_statement(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Let { name, value } => self.compile_let(name, value),
            Stmt::Assign { name, value } => self.compile_assign(name, value),
            Stmt::Poke { address, value } => self.compile_poke(address, value),
            Stmt::If { condition, body } => self.compile_if(condition, body),
            Stmt::While { condition, body } => self.compile_while(condition, body),
            Stmt::Loop { body } => self.compile_loop(body),
            Stmt::InlineFn {name, body} => self.compile_InlineFn(name, body),
            Stmt::FnCall(name) => self.compileFnCall(name),
            _ => Err(format!("Compilation non implémentée pour cette instruction : {:?}", stmt)),
        }
    }
}