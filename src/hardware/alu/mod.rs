use crate::hardware::gates::{and, mux, not, or, xor};

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
pub fn au8(a: (bool, bool, bool, bool, bool, bool, bool, bool), b: (bool, bool, bool, bool, bool, bool, bool, bool), opcode: (bool, bool, bool)) -> ((bool, bool, bool, bool, bool, bool, bool, bool), bool) {
    let is_001 = and(not(opcode.2), and(not(opcode.1), opcode.0));
    let is_010 = and(not(opcode.2), and(opcode.1, not(opcode.0)));
    let is_011 = and(not(opcode.2), and(opcode.1, opcode.0));
    let is_100 = and(opcode.2, and(not(opcode.1), not(opcode.0)));

    let (s0, c0) = full_adder(xor(a.0, is_010), xor(or(b.0, or(is_011, is_100)), or(is_001, is_100)), or(is_001, or(is_010, is_100)));
    let (s1, c1) = full_adder(xor(a.1, is_010), xor(b.1,  or(is_001, is_100)), c0);
    let (s2, c2) = full_adder(xor(a.2, is_010), xor(b.2,  or(is_001, is_100)), c1);
    let (s3, c3) = full_adder(xor(a.3, is_010), xor(b.3,  or(is_001, is_100)), c2);
    let (s4, c4) = full_adder(xor(a.4, is_010), xor(b.4,  or(is_001, is_100)), c3);
    let (s5, c5) = full_adder(xor(a.5, is_010), xor(b.5,  or(is_001, is_100)), c4);
    let (s6, c6) = full_adder(xor(a.6, is_010), xor(b.6,  or(is_001, is_100)), c5);
    let (s7, c7) = full_adder(xor(a.7, is_010), xor(b.7,  or(is_001, is_100)), c6);

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
pub fn lu8(a: (bool, bool, bool, bool, bool, bool, bool, bool), b: (bool, bool, bool, bool, bool, bool, bool, bool), opcode: (bool, bool, bool, bool)) -> (bool, bool, bool, bool, bool, bool, bool, bool) {
    (lu(a.0, b.0, opcode), lu(a.1, b.1, opcode), lu(a.2, b.2, opcode), lu(a.3, b.3, opcode), lu(a.4, b.4, opcode), lu(a.5, b.5, opcode), lu(a.6, b.6, opcode), lu(a.7, b.7, opcode))
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
pub fn su8(a: (bool, bool, bool, bool, bool, bool, bool, bool), opcode: bool) -> (bool, bool, bool, bool, bool, bool, bool, bool) {
    (mux(false, a.1, opcode), mux(a.0, a.2, opcode), mux(a.1, a.3, opcode), mux(a.2, a.4, opcode), mux(a.3, a.5, opcode), mux(a.4, a.6, opcode), mux(a.5, a.7, opcode), mux(a.6, false, opcode))
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
pub fn cmp8 (a: (bool, bool, bool, bool, bool, bool, bool, bool), b: (bool, bool, bool, bool, bool, bool, bool, bool), opcode: (bool, bool)) -> (bool, bool, bool, bool, bool, bool, bool, bool) {
    let (eq_out0, ct_out0) = cmp(a.0, b.0, true, false);
    let (eq_out1, ct_out1) = cmp(a.1, b.1, eq_out0, ct_out0);
    let (eq_out2, ct_out2) = cmp(a.2, b.2, eq_out1, ct_out1);
    let (eq_out3, ct_out3) = cmp(a.3, b.3, eq_out2, ct_out2);
    let (eq_out4, ct_out4) = cmp(a.4, b.4, eq_out3, ct_out3);
    let (eq_out5, ct_out5) = cmp(a.5, b.5, eq_out4, ct_out4);
    let (eq_out6, ct_out6) = cmp(a.6, b.6, eq_out5, ct_out5);
    let (eq_out7, ct_out7) = cmp(a.7, b.7, eq_out6, ct_out6);

    let result = mux(mux(eq_out7, ct_out7, not(opcode.1)), and(not(eq_out7), not(ct_out7)), and(opcode.0, not(opcode.1)));

    (result, result, result, result, result, result, result, result)
}
