use std::collections::HashMap;
use crate::ast::{Expr, Program, Stmt};

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

    fn compile_let(&mut self, name: &str, value: &Expr) -> Result<(), String> {
        let address = self.symbols.define(name.to_string())?;

    }

    fn compile_expression(&mut self, expr: &Expr) -> Result<(), String> {
        match expr {
            Expr::Number(n) => {
                self.emit(&format!("VAL {}", n));
                self.emit("PASS_B D");
                Ok(())
            }

            Expr::Identifier(name) => {
                let address = self.symbols.lookup(name)
                    .ok_or(format!("Erreur : Variable '{}' non définie.", name))?;

                self.emit(&format!("VAL {}", address));
                self.emit("PASS_B D_B");
                Ok(())
            }

            Expr::BinaryOp { left, operator, right } => {
                self.compile_expression(right)?;
                let temp_addr = 240;
                self.emit(&format!("// --- Mise en cache de la partie droite (adresse {}) ---", temp_addr));
                self.emit(&format!("VAL {}", temp_addr));
                self.emit("PASS_A RAM");

                self.compile_expression(left)?;

                self.emit(&format!("// --- Calcul de l'opération ({:?}) ---", operator));
                self.emit(&format!("VAL {}", temp_addr));

                let asm_op = match operator {
                    crate::ast::BinaryOperator::Add => "ADD",
                    crate::ast::BinaryOperator::Sub => "SUB",
                    crate::ast::BinaryOperator::Equal => "CMP_EQ",
                    crate::ast::BinaryOperator::NotEqual => {
                        self.emit("CMP_EQ D_B");
                        self.emit("NOT D");
                        return Ok(());
                    }
                    crate::ast::BinaryOperator::LessThan => "CMP_LT",
                    crate::ast::BinaryOperator::GreaterThan => "CMP_GT",
                    _ => return Err(format!("L'opérateur {:?} n'est pas encore géré.", operator)),
                };
                self.emit(&format!("{} D_B", asm_op));

                Ok(())
            }
            _ => Err(format!("Compilation non implémentée pour cette expression : {:?}", expr)),
        }
    }
}