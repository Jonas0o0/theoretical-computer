use std::{env, fs, process};
use std::io::stdout;
use std::time::Duration;
use crossterm::cursor::MoveTo;
use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use theoretical_computer::hardware::cpu::Cpu;
use theoretical_computer::hardware::utils::{u8_to_byte, byte_to_u8};

const COLOR_BTE_CASE: u8 = 124;
const X_BYTE_CASE: u8 = 125;
const Y_BYTE_CASE: u8 = 126;
const KEY_MEMORY_CASE: u8 = 127;

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

    enable_raw_mode().unwrap();

    loop {
        key_input(&mut cpu);
        print_ram(&mut cpu);
        cpu.clock_tick(false);
    }

    println!("--- ÉTAT DE LA RAM (Valeurs non nulles) ---");

    println!("{:<10} | {:<12} | {}", "Adresse", "Valeur (Num)", "ASCII");
    println!("------------------------------------------");

    for adresse in 0..=255 {
        let ram_val_byte = cpu.ram.read_output(u8_to_byte(adresse));
        let val = byte_to_u8(&ram_val_byte);

        if val != 0 {
            let ascii_char = if val >= 32 && val <= 126 {
                (val as char).to_string()
            } else {
                ".".to_string()
            };

            println!("{:<10} | {:<12} | {}", adresse, val, ascii_char);
        }
    }

    println!("------------------------------------------");
    println!("Arrêt de la machine.");
}

fn key_input(cpu: &mut Cpu) {
    if poll(Duration::from_millis(0)).unwrap() {
        if let Event::Key(key_event) = read().unwrap() {
            match key_event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode().unwrap();
                    println!("\nArrêt du programme.");
                    process::exit(0);
                }
                KeyCode::Up | KeyCode::Char('z') => cpu.ram.write(usize::from(u8_to_byte(KEY_MEMORY_CASE)), u8_to_byte(b'z')),
                KeyCode::Down | KeyCode::Char('s') => cpu.ram.write(usize::from(u8_to_byte(KEY_MEMORY_CASE)), u8_to_byte(b's')),
                KeyCode::Left | KeyCode::Char('q') => cpu.ram.write(usize::from(u8_to_byte(KEY_MEMORY_CASE)), u8_to_byte(b'q')),
                KeyCode::Right | KeyCode::Char('d') => cpu.ram.write(usize::from(u8_to_byte(KEY_MEMORY_CASE)), u8_to_byte(b'd')),
                _ => {}
            }
        }
    }
}

fn print_ram(cpu: &mut Cpu) {
    let color_byte = cpu.ram.read_output(u8_to_byte(COLOR_BTE_CASE));
    let x_byte = cpu.ram.read_output(u8_to_byte(X_BYTE_CASE));
    let y_byte = cpu.ram.read_output(u8_to_byte(Y_BYTE_CASE));

    let color = byte_to_u8(&color_byte);
    let x = byte_to_u8(&x_byte);
    let y = byte_to_u8(&y_byte);

    if color != 0 {
        let mut stdout = stdout();

        let _ = execute!(stdout, MoveTo(x as u16, y as u16));

        match color {
            1 => print!("██"),
            2 => print!("🍎"),
            3 => print!("  "),
            _ => {}
        }

        cpu.ram.write(usize::from(u8_to_byte(252)), u8_to_byte(0));
    }
}