# Theoretical Computeur

> Un ordinateur complet construit **from scratch** : des portes NAND jusqu'à un langage de programmation de haut niveau, en passant par l'ALU, la mémoire, le CPU, une machine virtuelle et un assembleur.

[![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Logisim](https://img.shields.io/badge/Logisim-Evolution-blue)](http://www.cburch.com/logisim/)

**[Documentation complète (mdBook)](https://tc.jonas-facon.dev)**

---

## Pourquoi ce projet ?

Plutôt que d'utiliser un ordinateur comme une boîte noire, ce projet le reconstruit couche par couche, en partant d'un unique postulat : **tout est dérivable d'une porte NAND**. L'objectif n'est pas la performance, mais la compréhension : chaque abstraction (arithmétique, mémoire, jeu d'instructions, compilation) est conçue, prouvée et testée soi-même avant d'être utilisée comme brique pour la couche suivante.

## Architecture — 7 couches

| # | Couche | Ce qui a été construit | Statut |
|---|---|---|---|
| 0 | **Logique booléenne** | NAND, NOT, AND, OR, XOR, MUX, DMUX | Terminé |
| 1 | **Arithmétique (ALU)** | Additionneur, unité logique, décaleur, comparateur — ALU 8 bits, 16 opérations | Terminé |
| 2 | **Mémoire** | Registre, RAM (256 octets), Program Counter (16 bits) | Terminé |
| 3 | **CPU** | ISA 8 bits maison, Control Unit, cycle Fetch/Decode/Execute | Terminé |
| 4 | **Machine Virtuelle** | Émulation haute performance, Memory-Mapped I/O (clavier, écran) | Terminé |
| 5 | **Assembleur** | Parseur mnémoniques → binaire | Terminé |
| 6 | **Compilateur (JUMP)** | Langage haut niveau maison — Lexer terminé, Parser en cours | En cours |

Chaque couche est développée selon une approche **Double-Track** : conception physique dans **Logisim** (source de vérité matérielle) et émulation logicielle en **Rust**, validée par des tests unitaires exhaustifs (tables de vérité, cas limites).

Le détail technique de chaque couche (équations, schémas, choix de conception) est documenté dans les [Notes de Thèse](./docs/src/06-these.md), et le suivi chronologique dans le [Journal de Bord](./docs/src/07-journal.md).

## Démarrage rapide

```bash
# Compiler un programme assembleur
cargo run -p assembler -- examples/helloworld.asm

# L'exécuter sur la machine virtuelle
cargo run -p vm -- examples/helloworld.bin
```

Guide détaillé : [docs/src/02-quickstart.md](./docs/src/02-quickstart.md)

## Stack technique

- **Rust** — émulation matérielle, VM, assembleur, compilateur
- **Logisim Evolution** — conception et simulation des circuits
- **mdBook** — documentation technique

## À propos

Projet personnel mené en solo, avec pour objectif de démontrer une compréhension bas niveau de l'informatique (électronique numérique, architecture des ordinateurs, compilation) — développé dans le cadre d'une recherche de stage.

**Jonas Facon**
Email : [jonas.facon@proton.me](mailto:jonas.facon@proton.me)
LinkedIn : [linkedin.com/in/jonas-facon](https://www.linkedin.com/in/jonas-facon)
Portfolio : [jonas-facon.dev](https://jonas-facon.dev)
GitHub : [@Jonas0o0](https://github.com/Jonas0o0)