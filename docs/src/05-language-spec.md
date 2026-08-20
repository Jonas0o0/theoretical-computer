# JUMP : Guide du Développeur et Manuel de Référence

**Langage :** JUMP (Just Uncomplicated Microprocessor Programming)
**Cible :** CPU 8-bits (256 octets RAM)
**Version :** 1.0.0

JUMP est un langage impératif de haut niveau conçu pour l'écosystème matériel `theoretical-computeur`. Ce document sert de guide de développement et explique les choix architecturaux du compilateur.

---

## 1. Architecture et Modèle Mémoire

JUMP est conçu pour des environnements extrêmement contraints, dépourvus de système d'exploitation (Bare-Metal). 

*   **Espace d'adressage :** Le processeur dispose d'exactement 256 octets de RAM.
*   **Types de données :** Il n'existe qu'un seul type de donnée natif : l'entier non signé 8-bits (`u8`), allant de `0` à `255`.
*   **Allocation statique :** Le compilateur alloue la mémoire linéairement. Les variables classiques sont stockées dans les adresses `0` à `239`. Les adresses `240` à `255` sont réservées au système et au Memory-Mapped I/O.

## 2. Syntaxe et Mots-Clés

La grammaire de JUMP s'inspire des langages modernes (Rust, C) avec une approche stricte basée sur des blocs.

### Déclaration et Affectation
L'allocation initiale en mémoire exige le mot-clé `let`. Les mutations ultérieures s'en passent.
```jump
let compteur = 0;
compteur = compteur + 1;

```

### Structures de Contrôle de Flux

Les blocs d'instructions sont obligatoirement délimités par des accolades `{}`.

```jump
// Boucle de jeu principale (infinie)
loop {
    let actif = 1;
}

// Boucle conditionnelle
while (compteur < 10) {
    compteur = compteur + 1;
}

// Condition simple
if (actif == 1) {
    compteur = 0;
}

```

### Macros d'Inlining (Fonctions)

JUMP ne possède pas de pile d'exécution (Call Stack). Le mot-clé `fn` définit une macro. Lors de la compilation, l'Arbre Syntaxique Abstrait (AST) injecte le code de la fonction directement à l'emplacement de son appel (Inlining).

```jump
fn reset_game() {
    compteur = 0;
}

reset_game(); // Le code est copié-collé ici par le compilateur

```

## 3. Entrées/Sorties Matérielles (I/O)

JUMP ne possède pas de bibliothèque standard (`std`). L'interaction avec le matériel se fait via des manipulations directes des pointeurs mémoire (Memory-Mapped I/O).

* **`poke(adresse, valeur)`** : Injecte une valeur dans une adresse physique.
* **`peek(adresse)`** : Lit le bus de données à l'adresse indiquée.

> **Exemple de routine graphique (Écran) :**
> L'écran écoute les adresses `252` (Ordre/Couleur), `253` (X) et `254` (Y).

```jump
let x = 10;
let y = 10;
// Dessine un pixel (Code couleur 1) en (10, 10)
poke(253, x);
poke(254, y);
poke(252, 1);

```

## 4. Le Compilateur JUMP (Sous le capot)

Pour comprendre comment le texte JUMP devient un programme exécutable, voici le pipeline du compilateur :

1. **Lexer (Analyseur Lexical) :** Convertit le texte brut en une séquence de "Tokens" (Jeton `LET`, Jeton `IDENTIFIER`, etc.).
2. **Parser (Analyseur Syntaxique) :** Assemble les Tokens en un Arbre Syntaxique Abstrait (AST) pour vérifier la logique (ex: s'assurer qu'une accolade est fermée).
3. **Allocateur Mémoire :** Parcourt l'AST et attribue une adresse RAM fixe à chaque `let` rencontré.
4. **Générateur de Code :** Traduit l'AST en instructions assembleur brutes (`VAL`, `PASS_A`, etc.), prêt à être chargé dans la ROM du processeur.
