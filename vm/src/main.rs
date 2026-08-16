use std::{env, fs, process};
use theoretical_computer::hardware::cpu::Cpu;
use theoretical_computer::hardware::utils::{u8_to_byte, byte_to_u8};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: vm <filename.bin>");
    }

    let chemin_fichier = &args[1];
    println!("Démarrage de la Machine Virtuelle...");
    println!("Fichier : {}", chemin_fichier);

    let mut cpu = Cpu::new();

    let bytecode = fs::read(chemin_fichier).unwrap_or_else(|_| {
        eprintln!("Erreur: Impossible de lire le fichier '{}'", chemin_fichier);
        process::exit(1);
    });

    println!("Flash de la ROM ({} octets)...", bytecode.len());

    for (i, &octet) in bytecode.iter().enumerate() {
        cpu.rom.write(i, u8_to_byte(octet));
    }

    println!("Exécution du programme en cours...\n");
    let nombre_instructions = bytecode.len();

    for _ in 0..nombre_instructions {
        cpu.clock_tick(false);
    }

    println!("--- RÉSULTATS ---");

    let addr_preuve = u8_to_byte(3);
    let resultat_math = byte_to_u8(&cpu.ram.read_output(addr_preuve));
    println!("Preuve de vie (RAM[3]) : 17 + 3 = {}", resultat_math);

    print!("Écran (RAM[100..110]) : ");
    for adresse in 100..=110 {
        let ram_val_byte = cpu.ram.read_output(u8_to_byte(adresse));
        let ascii_val = byte_to_u8(&ram_val_byte);

        if ascii_val != 0 {
            print!("{}", ascii_val as char);
        }
    }

    println!("\n-----------------");
    println!("Arrêt de la machine.");
}