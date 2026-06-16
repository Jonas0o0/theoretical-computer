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
