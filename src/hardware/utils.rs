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

pub type Byte = (bool, bool, bool, bool, bool, bool, bool, bool);

/// Applique une fonction bit à bit sur deux Byte (a op b).
fn apply8<F: Fn(bool, bool) -> bool>(a: Byte, b: Byte, f: F) -> Byte {
    (f(a.0, b.0), f(a.1, b.1), f(a.2, b.2), f(a.3, b.3),
     f(a.4, b.4), f(a.5, b.5), f(a.6, b.6), f(a.7, b.7))
}

/// Duplique une valeur sur les 8 bits.
fn splat8(v: bool) -> Byte {
    (v, v, v, v, v, v, v, v)
}