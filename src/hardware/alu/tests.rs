use super::*;
use crate::hardware::utils::verify;

#[test]
fn test_half_adder() {
    verify(
        |(a, b)| half_adder(a, b),
        &[
            ((false, false), (false, false)),
            ((true, false), (true, false)),
            ((false, true), (true, false)),
            ((true, true), (false, true)),
        ],
    );
}

#[test]
fn test_full_adder() {
    verify(
        |(a, b, c)| full_adder(a, b, c),
        &[
            ((false, false, false), (false, false)),
            ((true, false, false), (true, false)),
            ((false, true, false), (true, false)),
            ((false, false, true), (true, false)),
            ((true, true, false), (false, true)),
            ((true, false, true), (false, true)),
            ((false, true, true), (false, true)),
            ((true, true, true), (true, true)),
        ],
    );
}

#[test]
fn test_cmp() {
    verify(
        |(a, b, eq_in, ct_in)| cmp(a, b, eq_in, ct_in),
        &[
            ((false, false, false, false), (false, false)),
            ((false, false, false, true), (false, true)),
            ((false, false, true, false), (true, false)),
            ((false, false, true, true), (true, true)),
            ((false, true, false, false), (false, false)),
            ((false, true, false, true), (false, false)),
            ((false, true, true, false), (false, false)),
            ((false, true, true, true), (false, false)),
            ((true, false, false, false), (false, false)),
            ((true, false, false, true), (false, false)),
            ((true, false, true, false), (false, true)),
            ((true, false, true, true), (false, true)),
            ((true, true, false, false), (false, false)),
            ((true, true, false, true), (false, true)),
            ((true, true, true, false), (true, false)),
            ((true, true, true, true), (true, true)),
        ],
    );
}

#[test]
fn test_lu() {
    verify(
        |(a, b, opcode)| lu(a, b, opcode),
        &[
            ((false, false, (false, false, false, false)), false),
            ((true, false, (false, false, false, false)), false),
            ((false, true, (false, false, false, false)), false),
            ((true, true, (false, false, false, false)), true),
            ((false, false, (false, true, true, false)), false),
            ((true, false, (false, true, true, false)), true),
            ((false, true, (false, true, true, false)), true),
            ((true, true, (false, true, true, false)), true),
            ((false, false, (true, true, true, false)), false),
            ((true, false, (true, true, true, false)), true),
            ((false, true, (true, true, true, false)), true),
            ((true, true, (true, true, true, false)), false),
            ((false, false, (false, false, false, true)), true),
            ((true, false, (false, false, false, true)), false),
            ((false, true, (false, true, true, true)), false),
            ((true, false, (false, true, true, true)), true),
            ((false, true, (true, true, true, true)), true),
            ((true, false, (true, true, true, true)), false),
        ],
    );
}

#[test]
fn test_lu8() {
    let a: Byte = Byte(false, true, false, true, false, true, false, true);
    let b: Byte = Byte(false, true, true, false, false, true, true, false);

    verify(
        |(a, b, opcode): (Byte, Byte, (bool, bool, bool, bool))| lu8(a, b, opcode),
        &[
            (
                (a, b, (false, false, false, false)),
                Byte(false, true, false, false, false, true, false, false),
            ),
            (
                (a, b, (false, true, true, false)),
                Byte(false, true, true, true, false, true, true, true),
            ),
            (
                (a, b, (true, true, true, false)),
                Byte(false, false, true, true, false, false, true, true),
            ),
            ((a, b, (false, true, true, true)), a),
            ((a, b, (true, true, true, true)), b),
        ],
    );
}

#[test]
fn test_au8() {
    let cinq: Byte = Byte(true, false, true, false, false, false, false, false);
    let trois: Byte = Byte(true, true, false, false, false, false, false, false);
    let un: Byte = Byte(true, false, false, false, false, false, false, false);
    let zero: Byte = Byte(false, false, false, false, false, false, false, false);
    let deux: Byte = Byte(false, true, false, false, false, false, false, false);
    let quatre: Byte = Byte(false, false, true, false, false, false, false, false);
    let six: Byte = Byte(false, true, true, false, false, false, false, false);
    let huit: Byte = Byte(false, false, false, true, false, false, false, false);
    let deux_cinq_cinq: Byte = Byte(true, true, true, true, true, true, true, true);

    verify(
        |(a, b, opcode): (Byte, Byte, (bool, bool, bool))| au8(a, b, opcode),
        &[
            ((cinq, trois, (false, false, false)), (huit, false)),
            ((deux_cinq_cinq, un, (false, false, false)), (zero, true)),
            ((cinq, trois, (true, false, false)), (deux, true)),
            (
                (trois, cinq, (true, false, false)),
                (Byte(false, true, true, true, true, true, true, true), false),
            ),
            ((cinq, zero, (true, true, false)), (six, false)),
            ((cinq, zero, (false, false, true)), (quatre, true)),
            ((zero, zero, (false, false, true)), (deux_cinq_cinq, false)),
        ],
    );
}

#[test]
fn test_su8() {
    let un: Byte = Byte(true, false, false, false, false, false, false, false);
    let deux: Byte = Byte(false, true, false, false, false, false, false, false);
    let zero: Byte = Byte(false, false, false, false, false, false, false, false);
    let alterne: Byte = Byte(false, true, false, true, false, true, false, true);
    let alterne_gauche: Byte = Byte(false, false, true, false, true, false, true, false);
    let alterne_droite: Byte = Byte(true, false, true, false, true, false, true, false);
    let tous_uns: Byte = Byte(true, true, true, true, true, true, true, true);
    let tous_uns_gauche: Byte = Byte(false, true, true, true, true, true, true, true);
    let tous_uns_droite: Byte = Byte(true, true, true, true, true, true, true, false);

    verify(
        |(a, opcode): (Byte, bool)| su8(a, opcode),
        &[
            ((un, false), deux),
            ((alterne, false), alterne_gauche),
            ((tous_uns, false), tous_uns_gauche),
            ((zero, false), zero),
            ((deux, true), un),
            ((alterne, true), alterne_droite),
            ((tous_uns, true), tous_uns_droite),
            ((zero, true), zero),
        ],
    );
}

#[test]
fn test_cmp8() {
    let zero: Byte = Byte(false, false, false, false, false, false, false, false);
    let un: Byte = Byte(true, false, false, false, false, false, false, false);
    let grand: Byte = Byte(false, false, false, false, false, false, false, true);

    verify(
        |(a, b, opcode): (Byte, Byte, (bool, bool))| cmp8(a, b, opcode),
        &[
            ((zero, zero, (false, true)), Byte(true, true, true, true, true, true, true, true)),
            ((un, un, (false, true)), Byte(true, true, true, true, true, true, true, true)),
            ((un, zero, (false, true)), Byte(false, false, false, false, false, false, false, false)),
            ((un, zero, (false, false)), Byte(false, false, false, false, false, false, false, false)), // 1 < 0 -> false
            ((zero, un, (false, false)), Byte(true, true, true, true, true, true, true, true)),         // 0 < 1 -> true
            ((zero, zero, (false, false)), Byte(false, false, false, false, false, false, false, false)), // 0 < 0 -> false
            ((grand, zero, (true, false)), Byte(true, true, true, true, true, true, true, true)),  // 128 > 0 -> true
            ((zero, grand, (true, false)), Byte(false, false, false, false, false, false, false, false)), // 0 > 128 -> false
            ((zero, zero, (true, false)), Byte(false, false, false, false, false, false, false, false)),  // 0 > 0 -> false
        ],
    );
}

#[test]
fn test_alu8() {
    let cinq: Byte = Byte(true, false, true, false, false, false, false, false);
    let trois: Byte = Byte(true, true, false, false, false, false, false, false);
    let zero: Byte = Byte(false, false, false, false, false, false, false, false);
    let un: Byte = Byte(true, false, false, false, false, false, false, false);
    let cinq_onze: Byte = Byte(true, true, true, true, true, true, true, true);

    verify(
        |(a, b, opcode): (Byte, Byte, (bool, bool, bool, bool))| alu8(a, b, opcode),
        &[
            (
                (cinq_onze, un, (false, false, false, false)),
                (zero, false),
            ),
            (
                (cinq, trois, (false, true, true, false)),
                (Byte(true, true, true, false, false, false, false, false), false),
            ),
        ],
    );
}