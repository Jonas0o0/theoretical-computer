use super::*;
use crate::hardware::utils::Byte;

fn byte(bits: [bool; 8]) -> Byte {
    Byte(
        bits[0], bits[1], bits[2], bits[3],
        bits[4], bits[5], bits[6], bits[7],
    )
}

#[test]
fn instruction_a_force_load_a_et_rien_dautre() {
    for r0 in [false, true] {
        for r1 in [false, true] {
            for r2 in [false, true] {
                let rom = byte([r0, r1, r2, true, false, true, false, false]);
                let (opcode, load_a, load_d, alu_b_mux, write_m, jump_enable) = cu(rom);

                assert!(load_a, "loadA doit être true en instruction A");
                assert!(!load_d);
                assert!(!alu_b_mux);
                assert!(!write_m);
                assert!(!jump_enable);
                assert_eq!(opcode, (true, false, true, false));
            }
        }
    }
}

fn check_group(r0: bool, r1: bool, r2: bool, expected: (bool, bool, bool, bool, bool)) {
    let opcode_bits = [true, false, false, true];
    let rom = byte([
        r0, r1, r2,
        opcode_bits[0], opcode_bits[1], opcode_bits[2], opcode_bits[3],
        true,
    ]);

    let (opcode, load_a, load_d, alu_b_mux, write_m, jump_enable) = cu(rom);

    assert_eq!(opcode, (true, false, false, true), "opcode doit être transmis tel quel");
    assert_eq!(
        (load_a, load_d, alu_b_mux, write_m, jump_enable),
        expected,
        "groupe rom0={} rom1={} rom2={}", r0, r1, r2
    );
}

#[test]
fn groupe_000_load_d_seul() {
    check_group(false, false, false, (false, true, false, false, false));
}

#[test]
fn groupe_001_load_d_et_alu_b_mux() {
    check_group(true, false, false, (false, true, true, false, false));
}

#[test]
fn groupe_010_load_a_seul() {
    check_group(false, true, false, (true, false, false, false, false));
}

#[test]
fn groupe_011_write_m_seul() {
    check_group(true, true, false, (false, false, false, true, false));
}

#[test]
fn groupe_100_alu_b_mux_et_write_m() {
    check_group(false, false, true, (false, false, true, true, false));
}

#[test]
fn groupe_101_jump_enable_seul() {
    check_group(true, false, true, (false, false, false, false, true));
}

#[test]
fn groupe_110_alu_b_mux_et_jump_enable() {
    check_group(false, true, true, (false, false, true, false, true));
}

#[test]
fn groupe_111_load_a_et_load_d() {
    check_group(true, true, true, (true, true, false, false, false));
}

#[test]
fn signaux_de_controle_jamais_incoherents() {
    for r0 in [false, true] {
        for r1 in [false, true] {
            for r2 in [false, true] {
                let rom = byte([r0, r1, r2, false, false, false, false, true]);
                let (_, load_a, _, _, write_m, jump_enable) = cu(rom);
                assert!(!(load_a && write_m));
                assert!(!(load_a && jump_enable));
            }
        }
    }
}