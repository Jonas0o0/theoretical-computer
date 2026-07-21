use super::*;
use crate::hardware::utils::verify;

#[test]
fn test_nand() {
    verify(|(a, b)| nand(a, b), &[
        ((false, false), true),
        ((false, true), true),
        ((true, false), true),
        ((true, true), false),
    ]);
}

#[test]
fn test_not() {
    verify(not, &[
        (false, true),
        (true, false),
    ]);
}

#[test]
fn test_and() {
    verify(|(a, b)| and(a, b), &[
        ((false, false), false),
        ((false, true), false),
        ((true, false), false),
        ((true, true), true),
    ]);
}

#[test]
fn test_or() {
    verify(|(a, b)| or(a, b), &[
        ((false, false), false),
        ((false, true), true),
        ((true, false), true),
        ((true, true), true),
    ]);
}

#[test]
fn test_xor() {
    verify(|(a, b)| xor(a, b), &[
        ((false, false), false),
        ((false, true), true),
        ((true, false), true),
        ((true, true), false),
    ]);
}

#[test]
fn test_mux() {
    verify(|(a, b, s)| mux(a, b, s), &[
        ((false, false, false), false),
        ((false, true, false), false),
        ((true, false, false), true),
        ((true, true, false), true),
        ((false, false, true), false),
        ((false, true, true), true),
        ((true, false, true), false),
        ((true, true, true), true),
    ]);
}

#[test]
fn test_dmux() {
    verify(|(d, s)| dmux(d, s), &[
        ((true, false), (true, false)),
        ((true, true), (false, true)),
        ((false, false), (false, false)),
        ((false, true), (false, false)),
    ]);
}

#[test]
fn test_mux8() {
    let cinq: Byte = Byte(true, false, true, false, false, false, false, false);
    let trois: Byte = Byte(true, true, false, false, false, false, false, false);

    verify(
        |(a, b, opcode): (Byte, Byte, bool)| mux8(a, b, opcode),
        &[
            ((cinq, trois, false), cinq),
            ((cinq, trois, true), trois),
        ],
    );
}