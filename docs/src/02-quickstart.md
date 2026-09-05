# Guide d'utilisation : Du Code à la Machine Virtuelle

Ce projet contient la chaîne de compilation complète pour exécuter des programmes sur notre architecture processeur. Vous pouvez programmer la machine de deux manières : avec notre langage de haut niveau (**JUMP**) ou directement en **Assembleur**.

---

## Méthode 1 : Le langage JUMP (Haut Niveau)

C'est la méthode recommandée. Le compilateur JUMP gère automatiquement l'allocation de la mémoire, génère l'assembleur, et appelle l'outil d'assemblage de manière transparente.

### Étape 1 : Écrire le code source (.jmp)

Crée un fichier texte avec l'extension `.jmp` (par exemple `examples/dessin.jmp`).

**Exemple d'un mini-programme de dessin interactif :**

```rust
// On initialise les coordonnées
let x = 7;
let y = 7;

loop {
    // On dessine le pixel (Gâchette d'affichage = 4)
    poke(125, x);
    poke(126, y);
    poke(124, 4); 
}

```

### Étape 2 : Compiler le programme

Le compilateur va transformer votre code JUMP en Assembleur (`.asm`), puis invoquer automatiquement l'Assembleur pour générer le binaire exécutable (`.bin`).

Depuis la racine du projet, lance :

```bash
cargo run -p compiler -- examples/dessin.jmp

```

**Résultat :** Les fichiers `dessin.asm` et `dessin.bin` sont générés automatiquement.

### Étape 3 : Lancer sur la Machine Virtuelle (VM)

La VM charge le fichier binaire généré et lance l'exécution.

```bash
cargo run -p vm -- examples/dessin.bin

```

Vous pouvez maintenant interagir avec le programme directement dans le terminal !

---

## Méthode 2 : L'Assembleur (Bas Niveau)

Si vous souhaitez écrire les instructions matérielles à la main pour contrôler directement les registres et l'ALU.

### Étape 1 : Écrire le code source (.asm)

Crée un fichier texte avec l'extension `.asm` (par exemple `examples/helloworld.asm`).

**Exemple pour écrire "Hi" en mémoire :**

```text
// Lettre 'H' (Code ASCII 72) dans la case RAM 100
VAL 72
PASS_B D
VAL 100
PASS_A RAM

// Lettre 'i' (Code ASCII 105) dans la case RAM 101
VAL 105
PASS_B D
VAL 101
PASS_A RAM

```

### Étape 2 : Compiler avec l'Assembleur

L'assembleur lit ton fichier texte et le traduit en octets (des `0` et des `1`) pour le processeur. Depuis la racine du projet, lance :

```bash
cargo run -p assembler -- examples/helloworld.asm

```

**Résultat :** Cela va générer un fichier binaire `helloworld.bin` dans le même dossier.

### Étape 3 : Lancer sur la Machine Virtuelle (VM)

La VM charge le fichier binaire dans la ROM du CPU et lance l'horloge. À la fin de l'exécution, elle affiche le contenu de la RAM (utile pour voir le texte ASCII ou le résultat des calculs).

Depuis la racine du projet, lance :

```bash
cargo run -p vm -- examples/helloworld.bin

```

**Résultat :** Un tableau s'affichera dans le terminal avec l'adresse mémoire, la valeur numérique, et le caractère ASCII correspondant.

```bash
--- ÉTAT DE LA RAM (Valeurs non nulles) ---
Adresse    | Valeur (Num) | ASCII
------------------------------------------
100        | 72           | H
101        | 105          | i
------------------------------------------
Arrêt de la machine.

```