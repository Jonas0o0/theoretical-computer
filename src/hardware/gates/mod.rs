use crate::hardware::utils::Byte;

pub fn nand(a: bool, b: bool) -> bool {
    !(a && b)
}

pub fn not(a: bool) -> bool {
    nand(a, a)
}

pub fn and(a: bool, b: bool) -> bool {
    not(nand(a, b))
}

pub fn or(a: bool, b: bool) -> bool {
    nand(not(a), not(b))
}

pub fn xor(a: bool, b: bool) -> bool {
    or(and(not(a), b), and(a, not(b)))
}

pub fn mux(a: bool, b: bool, sel: bool) -> bool {
    or(and(not(sel), a), and(sel, b))
}

pub fn dmux(d: bool, sel: bool) -> (bool, bool) {
    (and(d, not(sel)), and(d, sel))
}

pub fn mux8(a: Byte, b: Byte, opcode: bool) -> Byte {
    (mux(a.0, b.0, opcode), mux(a.1, b.1, opcode), mux(a.2, b.2, opcode), mux(a.3, b.3, opcode), mux(a.4, b.4, opcode), mux(a.5, b.5, opcode), mux(a.6, b.6, opcode), mux(a.7, b.7, opcode))
}

#[cfg(test)]
mod tests;
