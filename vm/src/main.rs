use std::{env, fs, process, thread};
use std::io::{stdout, Stdout};
use std::time::Duration;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use theoretical_computer::hardware::cpu::Cpu;
use theoretical_computer::hardware::utils::{u8_to_byte, byte_to_u8, U16};

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

    let mut stdout = stdout();

    execute!(
        stdout,
        EnterAlternateScreen,
        Hide,
        Clear(ClearType::All)
    ).expect("Erreur lors de l'initialisation de l'écran");

    loop {
        if !key_input(&mut cpu) {
            break;
        }
        print_ram(&mut cpu, &mut stdout);
        cpu.clock_tick(false);
        thread::sleep(Duration::from_millis(5));
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

fn key_input(cpu: &mut Cpu) -> bool {
    if poll(Duration::from_millis(0)).unwrap() {
        if let Event::Key(key_event) = read().unwrap() {
            match key_event.code {
                KeyCode::Esc => {
                    return false;
                }
                KeyCode::Up | KeyCode::Char('z') => cpu.ram.write(usize::from(u8_to_byte(KEY_MEMORY_CASE)), u8_to_byte(b'z')),
                KeyCode::Down | KeyCode::Char('s') => cpu.ram.write(usize::from(u8_to_byte(KEY_MEMORY_CASE)), u8_to_byte(b's')),
                KeyCode::Left | KeyCode::Char('q') => cpu.ram.write(usize::from(u8_to_byte(KEY_MEMORY_CASE)), u8_to_byte(b'q')),
                KeyCode::Right | KeyCode::Char('d') => cpu.ram.write(usize::from(u8_to_byte(KEY_MEMORY_CASE)), u8_to_byte(b'd')),
                _ => {}
            }
        }
    }
    true
}

fn print_ram(cpu: &mut Cpu,stdout: &mut Stdout) {
    let color_byte = cpu.ram.read_output(u8_to_byte(COLOR_BTE_CASE));
    let x_byte = cpu.ram.read_output(u8_to_byte(X_BYTE_CASE));
    let y_byte = cpu.ram.read_output(u8_to_byte(Y_BYTE_CASE));

    let color = byte_to_u8(&color_byte);
    let x = byte_to_u8(&x_byte) as u16;
    let y = byte_to_u8(&y_byte) as u16;

    if color != 0 {

        match color {
            1 => execute!(stdout, MoveTo(x, y), Print("██")).unwrap(),
            2 => execute!(stdout, MoveTo(x, y), Print("🍎")).unwrap(),
            3 => execute!(stdout, MoveTo(x, y), Print("  ")).unwrap(),
            _ => {}
        }

        cpu.ram.write(usize::from(u8_to_byte(COLOR_BTE_CASE)), u8_to_byte(0));
    }
}