use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run -p assembler -- <fichier.asm>");
        return;
    }

    let input_filename = &args[1];
    println!("Démarrage de l'assemblage de {}...", input_filename);

    let source_code = fs::read_to_string(input_filename)
        .expect("Erreur : Impossible de lire le fichier source !");

    let mut binary_output: Vec<u8> = Vec::new();

    for (line_number, line) in source_code.lines().enumerate() {
        let clean_line = line.trim();
        if clean_line.is_empty() || clean_line.starts_with("//") {
            continue;
        }

        let byte = parse_instruction(clean_line);
        binary_output.push(byte);

        println!("Ligne {:03} | {:<15} -> {:08b}", line_number + 1, clean_line, byte);
    }

    let output_filename = input_filename.replace(".asm", ".bin");
    fs::write(&output_filename, &binary_output)
        .expect("Erreur : Impossible de sauvegarder le fichier binaire !");

    println!("Succès ! Fichier compilé avec {} instructions : {}", binary_output.len(), output_filename);
}

fn parse_instruction(instruction: &str) -> u8 {
    let parts: Vec<&str> = instruction.split_whitespace().collect();
    if parts.is_empty() { return 0; }

    let op = parts[0];

    if op == "VAL" {
        let val: u8 = parts[1].parse().unwrap_or(0);
        return val & 0b0111_1111
    }

    let opcode: u8 = match op {
        "ADD" => 0b0000,
        "SUB" => 0b0001,
        "RSUB" => 0b0010,
        "INC" => 0b0011,
        "DEC" => 0b0100,
        "AND" => 0b0101,
        "OR" => 0b0110,
        "XOR" => 0b0111,
        "NOT" => 0b1000,
        "SHL" => 0b1001,
        "SHR" => 0b1010,
        "CMP_EQ" => 0b1011,
        "CMP_LT" => 0b1101,
        "CMP_GT" => 0b1101,
        "PASS_A" => 0b1110,
        "PASS_B" => 0b1111,
        _ => {
            eprintln!("Erreur : Opération inconnue '{}'", op);
            return 0;
        }
    };

    let mode_str = parts.get(1).unwrap_or(&"D");

    let mmm: u8 = match *mode_str {
        "D"     => 0b000, // (0) loadD
        "D_B"   => 0b001, // (1) loadD + aluBMux
        "A"     => 0b010, // (2) loadA
        "RAM"   => 0b011, // (3) writeM
        "RAM_B" => 0b100, // (4) writeM + aluBMux
        "JMP"   => 0b101, // (5) jumpEnable
        "JMP_B" => 0b110, // (6) jumpEnable + aluBMux
        "AD"    => 0b111, // (7) loadA + loadD
        _ => {
            eprintln!("Erreur : Destination/Mode inconnu '{}'", mode_str);
            0b000
        }
    };

    let b7 = 0b1000_0000;
    let opcode_shifted = opcode << 3;
    b7 | opcode_shifted | mmm
}
#[cfg(test)]
mod tests;