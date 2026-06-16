# Stack complète à installer sur Debian

Le but :

```txt id="fq9js4"
Logisim → CPU
Rust → assembleur/compilateur
Git → versionning
Graphviz → visualisation
VS Code/Neovim → dev
```

Tu vas construire progressivement :

```txt id="x1yv4x"
portes logiques
→ ALU
→ CPU
→ assembleur
→ mini langage
→ compilateur
→ exécution
```

---

# 1. Les logiciels à installer

## A. Java (obligatoire pour Logisim)

OpenJDK

Commande Debian :

```bash
sudo apt update
sudo apt install openjdk-21-jdk
```

Vérifie :

```bash
java --version
```

---

# B. Logisim Evolution

Le simulateur hardware principal.

Logisim Evolution

Téléchargement :
[Logisim Evolution GitHub Releases](https://github.com/logisim-evolution/logisim-evolution/releases?utm_source=chatgpt.com)

Prends le `.jar`.

Puis :

```bash
java -jar logisim-evolution-*.jar
```

---

# C. Rust

Le meilleur choix pour :

* assembleur,
* compilateur,
* VM,
* outils systèmes.

Rust

Installation officielle :
[Rustup officiel](https://rustup.rs/?utm_source=chatgpt.com)

Commande :

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Puis :

```bash
source $HOME/.cargo/env
```

Vérifie :

```bash
rustc --version
cargo --version
```

---

# D. Git

Git

```bash
sudo apt install git
```

---

# E. Graphviz

Pour afficher :

* automates,
* graphes,
* AST,
* circuits.

Graphviz

```bash
sudo apt install graphviz
```

---

# F. Un éditeur de code

## Option simple :

Visual Studio Code

[VS Code officiel](https://code.visualstudio.com/?utm_source=chatgpt.com)

Extensions :

* rust-analyzer,
* Error Lens,
* CodeLLDB.

---

## Option “élite terminal”

Neovim

Mais VS Code est très bien pour commencer.

---

# 2. Organisation de ton projet

Fais un dossier :

```bash
mkdir theoretical-computer
cd theoretical-computer
```

Structure future :

```txt id="3tjlwm"
theoretical-computer/
│
├── hardware/
│   ├── gates/
│   ├── alu/
│   ├── cpu/
│
├── assembler/
│
├── vm/
│
├── compiler/
│
├── docs/
│
└── examples/
```

---

# 3. GRANDES ÉTAPES DU PROJET

---

# PHASE 1 — Comprendre les portes logiques

## Objectif

Construire :

* NOT,
* AND,
* OR,
* XOR,
* NAND.

Dans :
Logisim Evolution

---

# Ce que tu apprends

* logique booléenne,
* circuits,
* représentation binaire.

---

# Puis construire

## Half Adder

Addition 1 bit.

## Full Adder

Addition avec retenue.

---

# Résultat final

Tu peux additionner des nombres binaires.

---

# PHASE 2 — Construire une ALU

## Objectif

Créer une unité de calcul.

Fonctions :

```txt id="t4y0pn"
ADD
SUB
AND
OR
XOR
COMPARE
```

---

# Concepts appris

* arithmétique binaire,
* multiplexeurs,
* architecture CPU.

---

# PHASE 3 — Registres + mémoire

## Construire :

* registres,
* RAM simple,
* compteur programme (PC).

---

# Concepts

* stockage,
* cycles,
* état machine.

---

# PHASE 4 — Construire TON CPU

Le gros cap.

---

# Tu définis TON ISA

(Instruction Set Architecture)

Exemple :

```txt id="tw6x5w"
LOAD
STORE
ADD
SUB
JMP
JZ
PRINT
HALT
```

---

# Exemple programme

```asm id="onr2fv"
LOAD R1, 5
LOAD R2, 10
ADD R1, R2
PRINT R1
HALT
```

---

# Là tu construis :

* unité de contrôle,
* fetch/decode/execute,
* horloge.

---

# PHASE 5 — Assembleur en Rust

## Objectif

Transformer :

```asm id="4wjlwm"
ADD R1, R2
```

en :

```txt id="o2mrc4"
01010110
```

---

# Tu apprends

* parsing,
* tokenisation,
* encodage binaire.

---

# Librairies Rust utiles

## CLI

[Clap crate](https://crates.io/crates/clap?utm_source=chatgpt.com)

## Parsing

[Nom crate](https://crates.io/crates/nom?utm_source=chatgpt.com)

---

# PHASE 6 — VM / émulateur

Très important.

---

# Pourquoi ?

Parce que :

* Logisim est lent,
* debugger un CPU réel est compliqué.

Donc tu fais :

## un émulateur logiciel de ton CPU.

En Rust.

---

# Ton VM :

* lit les opcodes,
* simule les registres,
* exécute les instructions.

---

# Avantage énorme

Tu peux :

* tester vite,
* debugger,
* afficher l’état CPU.

---

# PHASE 7 — Mini langage compilé

Le niveau “wow”.

---

# Exemple

```txt id="tpxj4m"
let x = 5;
let y = 2;
print(x + y);
```

---

# Tu construis

## Lexer

Automates finis.

## Parser

AST.

## Génération de code

assembleur → bytecode.

---

# Concepts théoriques

* langages formels,
* automates,
* grammaires,
* compilation.

---

# PHASE 8 — Optimisations

Exemple :

```txt id="dlmnfr"
2 + 3
```

devient directement :

```txt id="wvw3zh"
5
```

---

# Concepts

* optimisation,
* compilation moderne.

---

# 4. Ressources PRINCIPALES

---

# Ressource n°1 (obligatoire)

## The Elements of Computing Systems

Site :
[Nand2Tetris officiel](https://www.nand2tetris.org/?utm_source=chatgpt.com)

---

# Ressource n°2

## Computer Systems: A Programmer's Perspective

Très fort pour comprendre :

* CPU,
* mémoire,
* assembleur.

---

# Ressource n°3

## Compilation

Crafting Interpreters

[Crafting Interpreters gratuit](https://craftinginterpreters.com/?utm_source=chatgpt.com)

---

# 5. Ce que JE te conseille vraiment

Ne cherche pas :

> “faire un énorme ordinateur”.

Cherche :

> “comprendre chaque couche profondément”.

---

# Ton ordre idéal

## Mois 1

* Logique booléenne
* Additionneurs
* ALU

## Mois 2

* CPU simple
* ISA
* registres

## Mois 3

* Assembleur Rust
* VM

## Mois 4

* Mini langage
* Lexer/parser

---

# Et surtout

Documente TOUT.

Dans :

```txt id="m5p0uv"
docs/
```

Avec :

* schémas,
* explications,
* théorie,
* architecture.

Ça transformera ton projet étudiant en vrai projet d’ingénierie informatique fondamentale.
