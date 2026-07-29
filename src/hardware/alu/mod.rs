use crate::hardware::gates::{and, mux, mux8, not, or, xor};
use crate::hardware::utils::{Byte, apply8, splat8, U16};

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

    (Byte(s0, s1, s2, s3, s4, s5, s6, s7), c7)
}

/// Unité arithmétique 16 bits.
///
/// Exécute une opération arithmétique entre deux opérandes de 16 bits.
/// L'opération réalisée dépend de `opcode`.
///
/// # Paramètres
/// - `a` : premier opérande.
/// - `b` : second opérande.
/// - `opcode` : code de l'opération arithmétique.
///
/// # Retour
/// Retourne `(résultat, retenue_sortante)`.
pub fn au16(a: U16, b: U16, opcode: (bool, bool, bool)) -> (U16, bool) {
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
    let (s8, c8) = full_adder(xor(a.8, is_010), xor(b.8, sel_b), c7);
    let (s9, c9) = full_adder(xor(a.9, is_010), xor(b.9, sel_b), c8);
    let (s10, c10) = full_adder(xor(a.10, is_010), xor(b.10, sel_b), c9);
    let (s11, c11) = full_adder(xor(a.11, is_010), xor(b.11, sel_b), c10);
    let (s12, c12) = full_adder(xor(a.12, is_010), xor(b.12, sel_b), c11);
    let (s13, c13) = full_adder(xor(a.13, is_010), xor(b.13, sel_b), c12);
    let (s14, c14) = full_adder(xor(a.14, is_010), xor(b.14, sel_b), c13);
    let (s15, c15) = full_adder(xor(a.15, is_010), xor(b.15, sel_b), c14);

    (U16(s0, s1, s2, s3, s4, s5, s6, s7, s8, s9, s10, s11, s12, s13, s14, s15), c15)
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
    Byte(s(false, a.1), s(a.0, a.2), s(a.1, a.3), s(a.2, a.4), s(a.3, a.5), s(a.4, a.6), s(a.5, a.7), s(a.6, false))
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

    let result = mux(mux(eq, ct, not(opcode.1)), and(not(eq), not(ct)), and(not(opcode.0), not(opcode.1)));
    splat8(result)
}

/// Unité Arithmétique et Logique (ALU) 8 bits.
///
/// Combine les quatre sous-unités (AU, LU, SU, CMP) et sélectionne le résultat
/// à renvoyer en fonction de `opcode`, selon la table d'encodage suivante :
///
/// | Opcode | Opération | Unité |
/// |--------|-----------|-------|
/// | 0000   | ADD (A + B)      | AU |
/// | 0001   | SUB (A - B)      | AU |
/// | 0010   | RSUB (B - A)     | AU |
/// | 0011   | INC (A + 1)      | AU |
/// | 0100   | DEC (A - 1)      | AU |
/// | 0101   | AND (A AND B)    | LU |
/// | 0110   | OR (A OR B)      | LU |
/// | 0111   | XOR (A XOR B)    | LU |
/// | 1000   | NOT (NOT A)      | LU |
/// | 1001   | SHL (A << 1)     | SU |
/// | 1010   | SHR (A >> 1)     | SU |
/// | 1011   | CMP_EQ (A == B)  | CMP |
/// | 1100   | CMP_LT (A < B)   | CMP |
/// | 1101   | CMP_GT (A > B)   | CMP |
/// | 1110   | PASS_A (A)       | LU |
/// | 1111   | PASS_B (B)       | LU |
///
/// Les bits `m0` et `m1` sont dérivés de l'opcode et pilotent les MUX
/// d'aiguillage : `m1` sélectionne entre (AU, LU) et (SU, CMP), puis `m0`
/// sélectionne le résultat final entre ces deux paires.
///
/// # Paramètres
/// - `a` : premier opérande.
/// - `b` : second opérande.
/// - `opcode` : code de l'opération à exécuter (voir table ci-dessus).
///
/// # Retour
/// Retourne `(résultat, retenue_sortante)`. La retenue sortante n'est
/// significative que pour les opérations issues de l'unité arithmétique (AU) ;
/// elle est masquée (mise à `false`) pour les autres opérations.
pub fn alu8(a: Byte, b: Byte, opcode: (bool, bool, bool, bool)) -> (Byte, bool) {
    let m0 = and(opcode.3, or(xor(opcode.1, opcode.2), and(opcode.0, not(opcode.2))));
    let m1 = or(or(and(and(opcode.0, opcode.1), opcode.3), and(opcode.0, opcode.2)), or(and(opcode.1, opcode.2), and(opcode.3, and(not(opcode.1), not(opcode.0)))));

    let au8 = au8(a, b, (opcode.0, opcode.1, opcode.2));
    let lu8 = lu8(a, b, opcode);
    let su8 = su8(a, opcode.1);
    let cmp8 = cmp8(a, b, (opcode.0, opcode.1));

    (mux8(mux8(au8.0, lu8,m1), mux8(su8, cmp8, m1), m0), and(au8.1, or(m0, m1)))
}

#[cfg(test)]
mod tests;