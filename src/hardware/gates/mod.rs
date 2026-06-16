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

pub fn mux(a: bool, b: bool, sel: bool) -> bool {
    or(and(not(sel), a), and(sel, b))
}

pub fn dmux(d: bool, sel: bool) -> (bool, bool) {
    (and(d, not(sel)), and(d, sel))
}

#[cfg(test)]
mod tests;
