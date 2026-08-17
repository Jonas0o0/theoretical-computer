# Guide d'utilisation : De l'Assembleur à la Machine Virtuelle

Ce projet contient la chaîne de compilation complète pour exécuter des programmes sur notre architecture processeur.

Voici les 3 étapes pour écrire, compiler et lancer un programme.

## Étape 1 : Écrire le code source (.asm)

Crée un fichier texte avec l'extension `.asm` (par exemple dans un dossier `examples/helloworld.asm`).

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

## Étape 2 : Compiler avec l'Assembleur

L'assembleur lit ton fichier texte et le traduit en octets (des `0` et des `1`) pour le processeur. Depuis la racine du projet, lance :

```bash
cargo run -p assembler -- examples/helloworld.asm
```

**Résultat :** Cela va générer un fichier binaire `helloworld.bin` dans le même dossier.

## Étape 3 : Lancer sur la Machine Virtuelle (VM)

La VM charge le fichier binaire dans la ROM du CPU et lance l'horloge. À la fin de l'exécution, elle affiche le contenu de la RAM (utile pour voir le texte ASCII).

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