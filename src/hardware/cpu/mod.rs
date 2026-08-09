use crate::hardware::gates::{and, mux, mux8, not, or, xor};
use crate::hardware::utils::{Byte, apply8, splat8, U16};

/// Unité de contrôle 8 bits.
///
///
/// # Paramètres
/// - `rom` : ROM.
///
/// # Retour
/// Retourne `(opcode, loadA, loadD, aluBMux, writeM, jumpEnable)`.
pub fn cu(rom: Byte) -> ((bool, bool, bool, bool), bool, bool, bool, bool, bool){
    let is_000 = and(and(not(ram.0), not(ram.1)), not(ram.2));
    let is_001 = and(and(ram.0, not(ram.1)), not(ram.2));
    let is_010 = and(and(not(ram.0), ram.1), not(ram.2));
    let is_011 = and(and(ram.0, ram.1), not(ram.2));
    let is_100 = and(and(not(ram.0), not(ram.1)), ram.2);
    let is_101 = and(and(ram.0, not(ram.1)), ram.2);
    let is_110 = and(and(not(ram.0), ram.1), ram.2);
    let is_111 = and(and(ram.0, ram.1), ram.2);

    ((ram.6, ram.5, ram.4, ram.3), or(or(ram.7, is_010), is_111), or(or(is_000, is_001), is_111), or(or(is_001, is_100), is_110), or(is_011, is_100), or(is_101, is_110))
}