use super::*;

const ZERO: Byte = Byte(false, false, false, false, false, false, false, false);
const ONE: Byte  = Byte(true, false, false, false, false, false, false, false);
const TWO: Byte  = Byte(false, true, false, false, false, false, false, false);
const VAL_A: Byte = Byte(true, true, false, false, true, false, false, false);
const VAL_B: Byte = Byte(false, false, true, true, false, true, false, true);

#[test]
fn test_register() {
    let mut reg = Register::new();
    assert_eq!(reg.read_output(), ZERO, "Le registre doit s'initialiser à 0");
    reg.clock_tick(VAL_A, true, false);
    assert_eq!(reg.read_output(), VAL_A, "Le registre doit stocker la valeur quand load est à true");
    reg.clock_tick(VAL_B, false, false);
    assert_eq!(reg.read_output(), VAL_A, "Le registre doit ignorer l'entrée si load est à false");
    reg.clock_tick(VAL_B, true, true);
    assert_eq!(reg.read_output(), ZERO, "Le registre doit forcer 0 quand reset est à true");
}

#[test]
fn test_ram() {
    let mut ram = Ram::new();
    let addr1 = ONE;
    let addr2 = TWO;
    assert_eq!(ram.read_output(addr1), ZERO, "La RAM doit être initialisée à 0");
    ram.clock_tick(addr1, VAL_A, true);
    assert_eq!(ram.read_output(addr1), VAL_A, "La RAM doit enregistrer la valeur à la bonne adresse");
    assert_eq!(ram.read_output(addr2), ZERO, "Écrire à une adresse ne doit pas modifier les autres");
    ram.clock_tick(addr1, VAL_B, false);
    assert_eq!(ram.read_output(addr1), VAL_A, "La RAM ne doit pas être modifiée si load est à false");
}

#[test]
fn test_pc_increment_and_jump() {
    let mut pc = PC::new();
    assert_eq!(pc.read_output(), ZERO, "Le PC doit démarrer à 0");
    pc.clock_tick(ZERO, false, false);
    assert_eq!(pc.read_output(), ONE, "Le PC doit s'incrémenter de 1 au premier cycle");
    pc.clock_tick(ZERO, false, false);
    assert_eq!(pc.read_output(), TWO, "Le PC doit s'incrémenter à 2 au cycle suivant");
    pc.clock_tick(VAL_A, true, false);
    assert_eq!(pc.read_output(), VAL_A, "Le PC doit charger l'adresse de saut quand load est à true");
    pc.clock_tick(VAL_B, false, true);
    assert_eq!(pc.read_output(), ZERO, "Le PC doit retomber à 0 quand reset est activé");
}