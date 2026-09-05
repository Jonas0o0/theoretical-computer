# Documentation de l'Architecture ISA 8-bits

## 1. Le Problème de l'Espace (Contrainte des 8 bits)
Dans une architecture strictement limitée à 8 bits, intégrer une ALU complète à 16 opérations exige 4 bits, et l'identification de l'instruction prend 1 bit. Il ne restait que **3 bits libres** pour contrôler 5 signaux physiques indispensables : la source de l'entrée B de l'ALU (Registre A ou RAM), les autorisations d'écriture (Load A, Load D, Write M) et les instructions de Saut (Jump).

## 2. La Solution : L'Encodage par Modes
Pour contourner cette limite, les 3 derniers bits ne contrôlent pas des fils individuels, mais définissent un **Mode d'Exécution**. Ces bits agissent comme un sélecteur qui préconfigure le processeur selon 8 profils matériels précis (lecture, écriture, saut conditionnel), permettant de réaliser toutes les opérations nécessaires en compressant intelligemment les signaux de contrôle.

---

## 3. Format Général de l'Instruction

Chaque instruction de 8 bits est structurée de la manière suivante :

| Bit 7 | Bit 6 | Bit 5 | Bit 4 | Bit 3 | Bit 2 | Bit 1 | Bit 0 |
| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **T** | **O** | **O** | **O** | **O** | **M** | **M** | **M** |
| Type | \- | \- | ALU | \- | \- | Mode | \- |

* **T (1 bit)** : Type d'instruction (Adresse ou Calcul)
* **OOOO (4 bits)** : Opcode de l'ALU
* **MMM (3 bits)** : Mode d'exécution (Sources, Destinations, Jumps)

---

## 4. Décodage du Bit de Type (`T`)

Ce bit de poids fort définit le comportement global du processeur pour le cycle en cours.

| `T` | Nom | Action du CPU |
| :---: | :--- | :--- |
| **0** | **Instruction Type A (Adresse / Valeur)** | L'ALU est désactivée. Le CPU prend les 7 bits restants (`0vvv_vvvv`) et stocke directement cette valeur dans le **Registre A**. |
| **1** | **Instruction Type C (Calcul)** | L'ALU est activée. Le CPU lit l'Opcode (`OOOO`) pour le calcul et le Mode (`MMM`) pour diriger les données. |

---

## 5. Table des Opcodes de l'ALU (`OOOO`)

*(Actif uniquement si `T = 1`. L'entrée A de l'ALU est toujours reliée au Registre D).*

| Opcode (`OOOO`) | Opération | Description |
| :---: | :--- | :--- |
| `0000` | **ADD** | A + B |
| `0001` | **SUB** | A - B |
| `0010` | **RSUB** | B - A |
| `0011` | **INC** | A + 1 |
| `0100` | **DEC** | A - 1 |
| `0101` | **AND** | A AND B (Logique) |
| `0110` | **OR** | A OR B (Logique) |
| `0111` | **XOR** | A XOR B (Logique) |
| `1000` | **NOT** | NOT A (Logique) |
| `1001` | **SHL** | A << 1 (Décalage gauche) |
| `1010` | **SHR** | A >> 1 (Décalage droite) |
| `1011` | **CMP_EQ**| A == B (Sort `11111111` si vrai, `00000000` si faux) |
| `1100` | **CMP_LT**| A < B (Sort `11111111` si vrai, `00000000` si faux) |
| `1101` | **CMP_GT**| A > B (Sort `11111111` si vrai, `00000000` si faux) |
| `1110` | **PASS_A**| Renvoie l'entrée A intacte |
| `1111` | **PASS_B**| Renvoie l'entrée B intacte |

---

## 6. Table des Modes d'Exécution (`MMM`)

*(Actif uniquement si `T = 1`. Définit l'entrée B de l'ALU, les registres modifiés, et la logique de saut).*

| Mode (`MMM`) | Entrée B (ALU) | Destination (Sauvegarde) | Saut (Jump) | Description de l'usage |
| :---: | :--- | :--- | :---: | :--- |
| `000` | Registre A | **Registre D** | Non | Calcul pur (ex: `D = D + A`) |
| `001` | RAM[A] | **Registre D** | Non | Lecture mémoire (ex: `D = D + RAM[A]`) |
| `010` | Registre A | **Registre A** | Non | Mise à jour adresse (ex: `A = D + A`) |
| `011` | Registre A | **RAM[A]** | Non | Écriture mémoire (ex: `RAM[A] = D op A`) |
| `100` | RAM[A] | **RAM[A]** | Non | Modif. mémoire directe (ex: `RAM[A] = D op RAM`) |
| `101` | Registre A | Aucune | **OUI, si sortie ALU != 0** | Condition : Comparer D et A, puis sauter à A |
| `110` | RAM[A] | Aucune | **OUI, si sortie ALU != 0** | Condition : Comparer D et RAM, puis sauter à A |
| `111` | Registre A | **Reg D** ET **Reg A** | Non | Clonage : Copier le résultat dans D et A |

> **Note sur le Saut Conditionnel (JUMP) :**
> Les modes `101` et `110` évaluent la sortie de l'ALU. Si le résultat renvoyé par l'ALU n'est pas strictement égal à `00000000`, le PC charge l'adresse contenue dans le Registre A. Ces modes sont prévus pour être combinés avec les Opcodes de comparaison (`CMP`).