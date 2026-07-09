use crate::hardware::gates::{and, mux, not, or, xor};

pub fn half_adder(a: bool, b: bool) -> (bool, bool){ (xor(a, b), and(a, b)) }

pub fn full_adder(a: bool, b: bool, c: bool) -> (bool, bool){
    let (s0, c0) = half_adder(a, b);
    let (s1, c1) = half_adder(c, s0);
    (s1, or(c1, c0))
}