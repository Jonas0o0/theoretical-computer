# Spécification des Portes Logiques (Couche 0)

Ce document détaille l'implémentation de la logique combinatoire de base de notre architecture. Dans une démarche de rigueur, toutes les portes élémentaires sont dérivées d'une porte universelle : la porte **NAND**.

## 1. La Porte Universelle : NAND

La porte **NAND** (NON-ET) renvoie faux (`0`) si et seulement si toutes ses entrées sont vraies (`1`). Elle est l'axiome de notre processeur.

**Équation logique :**
$$ NAND(A, B) = \overline{A \cdot B} $$

**Table de vérité :**

| A | B | Sortie |
|---|---|---|
| 0 | 0 | 1 |
| 0 | 1 | 1 |
| 1 | 0 | 1 |
| 1 | 1 | 0 |

## 2. Dérivation des portes fondamentales

### Porte NOT (NON)
La porte NOT inverse le signal d'entrée. On l'obtient en connectant la même entrée aux deux broches du NAND.

**Équation :**
$$ NOT(A) = NAND(A, A) = \overline{A \cdot A} = \overline{A} $$

### Porte AND (ET)
La porte AND renvoie vrai si toutes ses entrées sont vraies. C'est simplement un NAND dont on a inversé la sortie.

**Équation :**
$$ AND(A, B) = NOT(NAND(A, B)) = \overline{\overline{A \cdot B}} = A \cdot B $$

### Porte OR (OU)
La porte OR renvoie vrai si au moins une entrée est vraie. On applique ici le théorème de De Morgan : inverser les entrées d'un NAND équivaut à faire un OR.

**Équation :**
$$ OR(A, B) = NAND(NOT(A), NOT(B)) = \overline{\overline{A} \cdot \overline{B}} = A + B $$

### Porte XOR (OU Exclusif)
La porte XOR renvoie vrai si les entrées sont strictement différentes (l'une ou l'autre, mais pas les deux).

**Équation standard :**
$$ XOR(A, B) = (A \cdot \overline{B}) + (\overline{A} \cdot B) = A \oplus B $$

## 3. Logique de Routage (Multiplexage)

### Multiplexeur (MUX)
Le Multiplexeur agit comme un aiguillage : il sélectionne entre deux entrées $A$ et $B$ en fonction d'un signal de sélection $S$.
- Si $S = 0$, il sort $A$.
- Si $S = 1$, il sort $B$.

**Équation :**
$$ MUX(A, B, S) = (A \cdot \overline{S}) + (B \cdot S) $$

### Démultiplexeur (DMUX)
Le Démultiplexeur fait l'inverse : il prend un signal d'entrée $E$ et le route vers la sortie $Out_0$ ou $Out_1$ en fonction du sélecteur $S$.

**Équations :**
$$ Out_0 = E \cdot \overline{S} $$
$$ Out_1 = E \cdot S $$
