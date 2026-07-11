use crate::hardware::gates::{and, mux, not, or, xor};
use crate::hardware::utils::{Byte, apply8, splat8};

/// Réalise un demi-additionneur (Half Adder).
///
/// # Paramètres
/// - `a` : premier bit.
/// - `b` : second bit.
///
/// # Retour
/// Retourne `(somme, retenue)`.
pub fn half_adder(a: bool, b: bool) -> (bool, bool){ (xor(a, b), and(a, b)) }

/// Réalise un additionneur complet (Full Adder).
///
/// Additionne les bits `a` et `b` avec une retenue entrante `c`.
///
/// # Paramètres
/// - `a` : premier bit.
/// - `b` : second bit.
/// - `c` : retenue entrante.
///
/// # Retour
/// Retourne `(somme, retenue_sortante)`.
pub fn full_adder(a: bool, b: bool, c: bool) -> (bool, bool){
    let (s0, c0) = half_adder(a, b);
    let (s1, c1) = half_adder(c, s0);
    (s1, or(c1, c0))
}

/// Unité arithmétique 8 bits.
///
/// Exécute une opération arithmétique entre deux opérandes de 8 bits.
/// L'opération réalisée dépend de `opcode`.
///
/// # Paramètres
/// - `a` : premier opérande.
/// - `b` : second opérande.
/// - `opcode` : code de l'opération arithmétique.
///
/// # Retour
/// Retourne `(résultat, retenue_sortante)`.
pub fn au8(a: Byte, b: Byte, opcode: (bool, bool, bool)) -> (Byte, bool) {
    let is_001 = and(not(opcode.2), and(not(opcode.1), opcode.0));
    let is_010 = and(not(opcode.2), and(opcode.1, not(opcode.0)));
    let is_011 = and(not(opcode.2), and(opcode.1, opcode.0));
    let is_100 = and(opcode.2, and(not(opcode.1), not(opcode.0)));

    let sel_b    = or(is_001, is_100);           // inverse b (soustraction / incrément par complément)
    let carry_in = or(is_001, or(is_010, is_100));

    let (s0, c0) = full_adder(xor(a.0, is_010), xor(or(b.0, or(is_011, is_100)), sel_b), carry_in);
    let (s1, c1) = full_adder(xor(a.1, is_010), xor(b.1, sel_b), c0);
    let (s2, c2) = full_adder(xor(a.2, is_010), xor(b.2, sel_b), c1);
    let (s3, c3) = full_adder(xor(a.3, is_010), xor(b.3, sel_b), c2);
    let (s4, c4) = full_adder(xor(a.4, is_010), xor(b.4, sel_b), c3);
    let (s5, c5) = full_adder(xor(a.5, is_010), xor(b.5, sel_b), c4);
    let (s6, c6) = full_adder(xor(a.6, is_010), xor(b.6, sel_b), c5);
    let (s7, c7) = full_adder(xor(a.7, is_010), xor(b.7, sel_b), c6);

    ((s0, s1, s2, s3, s4, s5, s6, s7), c7)
}

/// Unité logique 1 bit.
///
/// Réalise une opération logique entre deux bits en fonction de `opcode`.
///
/// # Paramètres
/// - `a` : premier bit.
/// - `b` : second bit.
/// - `opcode` : code de l'opération logique.
///
/// # Retour
/// Le résultat de l'opération logique.
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

/// Unité logique 8 bits.
///
/// Applique l'unité logique `lu` indépendamment sur chacun des 8 bits.
///
/// # Paramètres
/// - `a` : premier opérande.
/// - `b` : second opérande.
/// - `opcode` : code de l'opération logique.
///
/// # Retour
/// Le résultat de l'opération logique sur les 8 bits.
pub fn lu8(a: Byte, b: Byte, opcode: (bool, bool, bool, bool)) -> Byte {
    apply8(a, b, |x, y| lu(x, y, opcode))
}

/// Unité de décalage 8 bits.
///
/// Effectue un décalage logique d'un bit.
///
/// - `opcode = false` : décalage à gauche.
/// - `opcode = true` : décalage à droite.
///
/// # Paramètres
/// - `a` : valeur à décaler.
/// - `opcode` : sens du décalage.
///
/// # Retour
/// La valeur décalée.
pub fn su8(a: Byte, opcode: bool) -> Byte {
    let s = |lo: bool, hi: bool| mux(lo, hi, opcode);
    (s(false, a.1), s(a.0, a.2), s(a.1, a.3), s(a.2, a.4), s(a.3, a.5), s(a.4, a.6), s(a.5, a.7), s(a.6, false))
}

/// Comparateur 1 bit.
///
/// Compare deux bits en tenant compte des résultats des bits précédents.
///
/// # Paramètres
/// - `a` : premier bit.
/// - `b` : second bit.
/// - `eq_in` : indique que tous les bits précédents sont égaux.
/// - `ct_in` : indique que `a` est déjà supérieur à `b`.
///
/// # Retour
/// Retourne `(egal, superieur)`.
pub fn cmp (a: bool, b: bool, eq_in: bool, ct_in: bool) -> (bool, bool) {
    let a_b = not(xor(a, b));
    (and(eq_in, a_b), or(and(a_b, ct_in), and(and(a, not(b)), eq_in)))
}

/// Comparateur 8 bits.
///
/// Compare deux valeurs de 8 bits.
///
/// L'opération effectuée dépend de `opcode` (égalité, supérieur, inférieur,
/// selon l'encodage choisi).
///
/// # Paramètres
/// - `a` : premier opérande.
/// - `b` : second opérande.
/// - `opcode` : sélection de la comparaison.
///
/// # Retour
/// Le résultat de la comparaison, répliqué sur les 8 bits.
pub fn cmp8(a: Byte, b: Byte, opcode: (bool, bool)) -> Byte {
    let a = [a.0, a.1, a.2, a.3, a.4, a.5, a.6, a.7];
    let b = [b.0, b.1, b.2, b.3, b.4, b.5, b.6, b.7];

    let mut eq = true;
    let mut ct = false;
    for i in 0..8 {
        let (e, c) = cmp(a[i], b[i], eq, ct);
        eq = e;
        ct = c;
    }

    let result = mux(mux(eq, ct, not(opcode.1)), and(not(eq), not(ct)), and(opcode.0, not(opcode.1)));
    splat8(result)
}
