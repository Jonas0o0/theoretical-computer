# Spécifications de l'Architecture

## Vision Globale
L'approche retenue est une **Architecture Double-Piste** :
1. **Piste Structurelle (Logisim)** : Conception physique des circuits.
2. **Piste Comportementale (Rust)** : Émulation logique pour validation et performance.

## Structure du Dossier `hardware/`
Pour chaque composant majeur, nous aurons :
- `composant.circ` : Le schéma Logisim (Source de vérité hardware).
- `composant.rs` : L'implémentation Rust (Module d'émulation).
- `README.md` : Spécification technique du composant.

## Hiérarchie des Composants
1. **Gates** : NAND, AND, OR, XOR, MUX, DMUX.
2. **ALU** : Additionneur, Unité Logique, Comparateurs.
3. **Storage** : Flip-Flops, Registres, RAM, PC (Program Counter).
4. **CPU** : Unité de Contrôle, Décodeur d'instructions.
