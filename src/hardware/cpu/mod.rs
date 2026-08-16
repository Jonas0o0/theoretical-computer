use crate::hardware::gates::{and, mux8, not, or};
use crate::hardware::utils::{Byte};
use crate::hardware::memory::{Register, PC, Ram, Rom};


/// Unité de contrôle 8 bits.
///
/// # Paramètres
/// - `rom` : ROM (L'instruction courante).
///
/// # Retour
/// Retourne `(opcode, loadA, loadD, aluBMux, writeM, jumpEnable)`.
pub fn cu(rom: Byte) -> ((bool, bool, bool, bool), bool, bool, bool, bool, bool) {
    let is_calc = rom.7;

    let is_000 = and(and(not(rom.0), not(rom.1)), not(rom.2));
    let is_001 = and(and(rom.0, not(rom.1)), not(rom.2));
    let is_010 = and(and(not(rom.0), rom.1), not(rom.2));
    let is_011 = and(and(rom.0, rom.1), not(rom.2));
    let is_100 = and(and(not(rom.0), not(rom.1)), rom.2);
    let is_101 = and(and(rom.0, not(rom.1)), rom.2);
    let is_110 = and(and(not(rom.0), rom.1), rom.2);
    let is_111 = and(and(rom.0, rom.1), rom.2);
    let opcode = (rom.3, rom.4, rom.5, rom.6);

    let loadA = or(not(is_calc), and(is_calc, or(is_010, is_111)));
    let loadD = and(is_calc, or(or(is_000, is_001), is_111));
    let aluBMux = and(is_calc, or(or(is_001, is_100), is_110));
    let writeM = and(is_calc, or(is_011, is_100));
    let jumpEnable = and(is_calc, or(is_101, is_110));

    (opcode, loadA, loadD, aluBMux, writeM, jumpEnable)
}

pub struct Cpu {
    pub reg_a: Register,
    pub reg_d: Register,
    pub pc: PC,
    pub ram: Ram,
    pub rom: Rom,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            reg_a: Register::new(),
            reg_d: Register::new(),
            pc: PC::new(),
            ram: Ram::new(),
            rom: Rom::new(),
        }
    }

    pub fn tick(&mut self, reset: bool) {
        let current_pc = self.pc.read_output();
        let instruction = self.rom.read_output(current_pc);

        let out_a = self.reg_a.read_output();
        let out_d = self.reg_d.read_output();
        let out_ram = self.ram.read_output(out_a);

        let (opcode, load_a, load_d, alu_b_mux, write_m, jump_enable) = cu(instruction);

        let alu_in_y = mux8(out_a, out_ram, alu_b_mux);

        let (alu_out, _carry_out) = alu8(out_d, alu_in_y, opcode);

        let Byte(r0, r1, r2, r3, r4, r5, r6, r7) = alu_out;
        let alu_not_zero = or(or(or(or(or(or(or(r0, r1), r2), r3), r4), r5), r6), r7);

        let is_calc = instruction.7;
        let in_a = mux8(instruction, alu_out, is_calc);

        self.reg_a.clock_tick(in_a, load_a, reset);
        self.reg_d.clock_tick(alu_out, load_d, reset);
        self.ram.clock_tick(out_a, alu_out, write_m);

        let do_jump = jump_enable && alu_not_zero;

        let Byte(a0, a1, a2, a3, a4, a5, a6, a7) = out_a;
        let jump_addr = U16(
            a0, a1, a2, a3, a4, a5, a6, a7,
            false, false, false, false, false, false, false, false
        );

        self.pc.clock_tick(jump_addr, do_jump, reset);
    }
}

#[cfg(test)]
mod tests;