use crate::hardware::gates::{and, mux, not, or, xor};

pub fn half_adder(a: bool, b: bool) -> (bool, bool){ (xor(a, b), and(a, b)) }

pub fn full_adder(a: bool, b: bool, c: bool) -> (bool, bool){
    let (s0, c0) = half_adder(a, b);
    let (s1, c1) = half_adder(c, s0);
    (s1, or(c1, c0))
}

pub fn lu(a: bool, b: bool, opcode: (bool, bool, bool, bool)) -> bool {
    let (o0, o1, o2, o3) = opcode;

    let is_1000 = and(and(o3, not(o2)), and(not(o1), not(o0)));
    let is_1111 = and(and(o3, o2), and(o1, o0));
    let is_0110 = and(and(not(o3), o2), and(o1, not(o0)));
    let is_0111 = and(and(o1, o0), and(not(o3), o2));
    let is_1110 = and(and(o1, not(o0)), and(o2, o3));

    let mux0 = or(or(is_1000, is_1111), is_0110);
    let mux1 = or(or(is_0111, is_1110), or(is_1000, is_1111));
    let mux2 = or(is_1110, is_1111);

    mux(
        mux(
            mux(and(a, b), or(a, b), mux0),
            mux(xor(a, b), not(a), mux0),
            mux1,
        ),
        mux(a, b, mux0),
        mux2,
    )
}

pub fn lu8(a: (bool, bool, bool, bool, bool, bool, bool, bool), b: (bool, bool, bool, bool, bool, bool, bool, bool), opcode: (bool, bool, bool, bool)) -> (bool, bool, bool, bool, bool, bool, bool, bool) {
    (lu(a.0, b.0, opcode), lu(a.1, b.1, opcode), lu(a.2, b.2, opcode), lu(a.3, b.3, opcode), lu(a.4, b.4, opcode), lu(a.5, b.5, opcode), lu(a.6, b.6, opcode), lu(a.7, b.7, opcode))
}