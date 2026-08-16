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

        if b.0 { index += 1; }
        if b.1 { index += 2; }
        if b.2 { index += 4; }
        if b.3 { index += 8; }
        if b.4 { index += 16; }
        if b.5 { index += 32; }
        if b.6 { index += 64; }
        if b.7 { index += 128; }

        index
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct U16(
    pub bool, pub bool, pub bool, pub bool,
    pub bool, pub bool, pub bool, pub bool,
    pub bool, pub bool, pub bool, pub bool,
    pub bool, pub bool, pub bool, pub bool,
);

impl From<U16> for usize {
    fn from(b: U16) -> Self {
        let mut index: usize = 0;

        if b.0 { index += 1; }
        if b.1 { index += 2; }
        if b.2 { index += 4; }
        if b.3 { index += 8; }
        if b.4 { index += 16; }
        if b.5 { index += 32; }
        if b.6 { index += 64; }
        if b.7 { index += 128; }
        if b.8 { index += 256; }
        if b.9 { index += 512; }
        if b.10 { index += 1024; }
        if b.11 { index += 2048; }
        if b.12 { index += 4096; }
        if b.13 { index += 8192; }
        if b.14 { index += 16384; }
        if b.15 { index += 32768; }

        index
    }
}

/// Applique une fonction bit à bit sur deux Byte (a op b).
pub(crate) fn apply8<F: Fn(bool, bool) -> bool>(a: Byte, b: Byte, f: F) -> Byte {
    Byte(f(a.0, b.0), f(a.1, b.1), f(a.2, b.2), f(a.3, b.3),
     f(a.4, b.4), f(a.5, b.5), f(a.6, b.6), f(a.7, b.7))
}

/// Duplique une valeur sur les 8 bits.
pub fn splat8(v: bool) -> Byte {
    Byte(v, v, v, v, v, v, v, v)
}

/// Fonction utilitaire pour convertir un u8 (venant du fichier) en ta structure Byte
fn u8_to_byte(val: u8) -> Byte {
    Byte(
        (val & 0b0000_0001) != 0,
        (val & 0b0000_0010) != 0,
        (val & 0b0000_0100) != 0,
        (val & 0b0000_1000) != 0,
        (val & 0b0001_0000) != 0,
        (val & 0b0010_0000) != 0,
        (val & 0b0100_0000) != 0,
        (val & 0b1000_0000) != 0,
    )
}

/// Fonction inverse pour lire la RAM et reconvertir ton Byte en u8 (ASCII)
fn byte_to_u8(b: &Byte) -> u8 {
    let mut val = 0;
    if b.0 { val |= 0b0000_0001; }
    if b.1 { val |= 0b0000_0010; }
    if b.2 { val |= 0b0000_0100; }
    if b.3 { val |= 0b0000_1000; }
    if b.4 { val |= 0b0001_0000; }
    if b.5 { val |= 0b0010_0000; }
    if b.6 { val |= 0b0100_0000; }
    if b.7 { val |= 0b1000_0000; }
    val
}