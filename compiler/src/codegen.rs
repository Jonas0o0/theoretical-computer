use std::collections::HashMap;

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