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
        if self.next_address > 119 {
            return Err(format!(
                "Erreur de compilation : Dépassement de la RAM utilisateur ! La variable '{}' ne peut pas dépasser l'adresse 119.",
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

    fn current_address(&self) -> usize {
        self.output.iter()
            .filter(|line| {
                let trimmed = line.trim();
                !trimmed.is_empty() && !trimmed.starts_with("//")
            })
            .count()
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
            Stmt::InlineFn { name, body } => self.compile_inline_fn(name, body),
            Stmt::FnCall(name) => self.compile_fn_call(name),
            _ => Err(format!("Compilation non implémentée pour cette instruction : {:?}", stmt)),
        }
    }

    fn compile_let(&mut self, name: &str, value: &Expr) -> Result<(), String> {
        let address = self.symbols.define(name.to_string())?;
        self.emit(&format!("// let {} = ...", name));
        self.compile_expression(value)?;
        self.emit(&format!("VAL {}", address));
        self.emit("PASS_A RAM");
        Ok(())
    }

    fn compile_assign(&mut self, name: &str, value: &Expr) -> Result<(), String> {
        let address = self.symbols.lookup(name)
            .ok_or(format!("Erreur : Variable '{}' non déclarée.", name))?;

        // OPTIMISATION : Détect x = x + 1 ou x = x - 1
        if let Expr::BinaryOp { left, operator, right } = value {
            if let Expr::Identifier(left_name) = &**left {
                if left_name == name {
                    if let Expr::Number(1) = &**right {
                        if *operator == crate::ast::BinaryOperator::Add {
                            self.emit(&format!("// {} += 1 (Opti INC)", name));
                            self.emit(&format!("VAL {}", address));
                            self.emit("PASS_B D_B");
                            self.emit(&format!("VAL {}", address));
                            self.emit("INC RAM");
                            return Ok(());
                        } else if *operator == crate::ast::BinaryOperator::Sub {
                            self.emit(&format!("// {} -= 1 (Opti DEC)", name));
                            self.emit(&format!("VAL {}", address));
                            self.emit("PASS_B D_B");
                            self.emit(&format!("VAL {}", address));
                            self.emit("DEC RAM");
                            return Ok(());
                        }
                    }
                }
            }
        }

        self.emit(&format!("// {} = ...", name));
        self.compile_expression(value)?;
        self.emit(&format!("VAL {}", address));
        self.emit("PASS_A RAM");
        Ok(())
    }

    fn compile_poke(&mut self, address: &Expr, value: &Expr) -> Result<(), String> {
        match address {
            Expr::Number(addr_val) => {
                self.emit(&format!("// poke({}, ...)", addr_val));
                self.compile_expression(value)?;
                self.emit(&format!("VAL {}", addr_val));
                self.emit("PASS_A RAM");
                Ok(())
            }
            _ => Err("L'adresse de 'poke' doit être un nombre brut.".to_string()),
        }
    }

    fn compile_inline_fn(&mut self, name: &str, body: &Vec<Stmt>) -> Result<(), String> {
        self.functions.insert(name.to_string(), body.clone());
        Ok(())
    }

    fn compile_fn_call(&mut self, name: &str) -> Result<(), String> {
        let body = self.functions.get(name).cloned()
            .ok_or(format!("Erreur : Fonction '{}()' inconnue.", name))?;
        self.emit(&format!("// --- APPEL FONCTION : {} ---", name));
        for stmt in body {
            self.compile_statement(&stmt)?;
        }
        self.emit(&format!("// --- FIN FONCTION : {} ---", name));
        Ok(())
    }

    fn compile_loop(&mut self, body: &Vec<Stmt>) -> Result<(), String> {
        let start_address = self.current_address();

        self.emit("// --- LOOP ---");
        for stmt in body {
            self.compile_statement(stmt)?;
        }
        self.emit("VAL 255");
        self.emit("PASS_B D");
        self.emit_jump_target(start_address);
        self.emit("PASS_A JMP");

        Ok(())
    }

    fn compile_if(&mut self, condition: &Expr, body: &Vec<Stmt>) -> Result<(), String> {
        // OPTIMISATION DE LA CONDITION : a == b devient (a - b != 0)
        let mut optimized = false;
        if let Expr::BinaryOp { left, operator: crate::ast::BinaryOperator::Equal, right } = condition {
            self.compile_expression(left)?;
            match &**right {
                Expr::Number(n) => { self.emit(&format!("VAL {}", n)); self.emit("SUB D"); optimized = true; },
                Expr::Identifier(var) => {
                    let addr = self.symbols.lookup(var).unwrap();
                    self.emit(&format!("VAL {}", addr));
                    self.emit("SUB D_B");
                    optimized = true;
                },
                _ => {}
            }
        }

        self.emit("// --- IF ---");
        self.compile_expression(condition)?;
        self.emit("VAL 0");
        self.emit("CMP_EQ D");

        let jump_index = self.output.len();
        self.emit("VAL 0");
        self.emit("VAL 0");
        self.emit("VAL 0");
        self.emit("PASS_A JMP");

        for stmt in body {
            self.compile_statement(stmt)?;
        }

        let end_address = self.current_address();
        self.patch_jump_target(jump_index, end_address);

        Ok(())
    }

    fn compile_while(&mut self, condition: &Expr, body: &Vec<Stmt>) -> Result<(), String> {
        let start_address = self.current_address();

        self.emit("// --- WHILE ---");
        self.compile_expression(condition)?;
        self.emit("VAL 0");
        self.emit("CMP_EQ D");

        let jump_index = self.output.len();
        self.emit("VAL 0");
        self.emit("VAL 0");
        self.emit("VAL 0");
        self.emit("PASS_A JMP");

        for stmt in body {
            self.compile_statement(stmt)?;
        }

        self.emit("VAL 255");
        self.emit("PASS_B D");
        self.emit_jump_target(start_address);
        self.emit("PASS_A JMP");

        let end_address = self.current_address();
        self.patch_jump_target(jump_index, end_address);

        Ok(())
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
                let temp_addr = 120;
                self.emit(&format!("// --- Cache droite (adresse {}) ---", temp_addr));
                self.emit(&format!("VAL {}", temp_addr));
                self.emit("PASS_A RAM");

                self.compile_expression(left)?;
                self.emit(&format!("// --- Opération ({:?}) ---", operator));
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
                    _ => return Err(format!("L'opérateur {:?} n'est pas géré.", operator)),
                };
                self.emit(&format!("{} D_B", asm_op));
                Ok(())
            }
            _ => Err(format!("Compilation non implémentée pour cette expression : {:?}", expr)),
        }
    }

    fn emit_jump_target(&mut self, target: usize) {
        if target <= 127 {
            self.emit(&format!("VAL {}", target));
        } else {
            panic!("ERREUR ARCHITECTURE : Impossible de sauter à la ligne {}. La limite stricte de la ROM est de 127 instructions !", target);
        }
    }

    fn patch_jump_target(&mut self, index: usize, target: usize) {
        if target <= 127 {
            self.output[index] = format!("VAL {}", target);
            self.output[index + 1] = format!("VAL {}", target);
            self.output[index + 2] = format!("VAL {}", target);
        } else {
            panic!("ERREUR ARCHITECTURE : Impossible de sauter à la ligne {}. La limite stricte de la ROM est de 127 instructions !", target);
        }
    }
}