pub fn verify<I, O, F>(f: F, table: &[(I, O)]) 
where 
    F: Fn(I) -> O, 
    I: std::fmt::Debug + Copy, 
    O: std::fmt::Debug + PartialEq + Copy 
{
    for &(inputs, expected) in table {
        assert_eq!(f(inputs), expected, "Échec pour l'entrée {:?}", inputs);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Byte(pub bool, pub bool, pub bool, pub bool, pub bool, pub bool, pub bool, pub bool);

impl From<Byte> for usize {
    fn from(b: Byte) -> Self {
        let mut index: usize = 0;

        if b.0 { index += 128; }
        if b.1 { index += 64; }
        if b.2 { index += 32; }
        if b.3 { index += 16; }
        if b.4 { index += 8; }
        if b.5 { index += 4; }
        if b.6 { index += 2; }
        if b.7 { index += 1; }

        index
    }
}

#[derive(Clone, Copy)]
pub struct U16(
    pub bool, pub bool, pub bool, pub bool,
    pub bool, pub bool, pub bool, pub bool,
    pub bool, pub bool, pub bool, pub bool,
    pub bool, pub bool, pub bool, pub bool,
);

/// Applique une fonction bit à bit sur deux Byte (a op b).
pub(crate) fn apply8<F: Fn(bool, bool) -> bool>(a: Byte, b: Byte, f: F) -> Byte {
    Byte(f(a.0, b.0), f(a.1, b.1), f(a.2, b.2), f(a.3, b.3),
     f(a.4, b.4), f(a.5, b.5), f(a.6, b.6), f(a.7, b.7))
}

/// Duplique une valeur sur les 8 bits.
pub fn splat8(v: bool) -> Byte {
    Byte(v, v, v, v, v, v, v, v)
}