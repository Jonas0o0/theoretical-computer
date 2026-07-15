# Journal de Bord (Lab Notebook)

## Format d'entrée :
- **Date** : [Date]
- **Objectif** : [Ce que je veux faire]
- **Réalisation** : [Ce que j'ai fait]
- **Difficultés** : [Bloquages rencontrés]
- **Apprentissages** : [Ce que j'ai compris aujourd'hui]

---

## [2026-06-14] - Initialisation du Projet et Sprint 1
- **Objectif** : Mise en place de la structure de projet et réalisation de la couche 0 (Portes Logiques).
- **Réalisation** : 
    - Configuration de l'environnement (Installation de Rust/Cargo via rustup).
    - Initialisation du dépôt GitHub et des Issues de suivi (Sprints).
    - Création des schémas Logisim (`myGates.circ`) utilisant des sous-circuits.
    - Développement de l'émulateur Rust avec une architecture Cargo modulaire.
    - Implémentation d'un moteur de test générique (`verify`) pour valider les tables de vérité.
- **Difficultés** : Configuration du PATH pour Rust et gestion des types génériques en Rust pour les tests unitaires.
- **Apprentissages** : Maîtrise de la logique NAND, structuration de projets Rust complexes et utilisation du GitHub CLI pour la gestion de projet.
- **Statut** : Sprint 1 validé (6/6 tests réussis).

## [2026-06-21] - Arithmétique - Construction de l'ALU et Sprint 2
- **Objectif** : Construire une Unité Arithmétique et Logique (ALU) capable d'effectuer des opérations binaires.
- **Réalisation** :
  - Création des schémas Logisim (myAlu.circ)
  - Émulation de l'ALU et de ses composants en Rust
- **Difficultés** :
  - Pour le CMP, mon erreur était de comparer les bits entre eux indépendamment, sans tenir compte des résultats précédents. De plus, mon CMP 1 bit gérait directement les signaux EQ et GT avec des MUX, alors que le MUX final n'aurait dû être placé qu'au niveau du CMP 8 bits. J'ai corrigé mon circuit en conséquence, et le CMP 1 bit s'est finalement avéré possible à réaliser.
  - Pour l'ALU, la construction en 1 bit ne fonctionnait pas, donc j'ai directement conçu l'ALU en 8 bits en assemblant tous les full adders en chaîne, et j'ai géré les bonnes opérations via l'opcode.
  - Le dépassement à 3 semaines (au lieu d'une semaine initialement prévue) s'explique par le fait que je pensais que la méthode "construire en 1 bit puis assembler pour obtenir le 8 bits" serait simple à généraliser. Le CMP et l'ALU m'ont donc ralenti, tout comme la phase de recherche nécessaire pour comprendre et trouver les bonnes solutions.
- **Apprentissages** :
  - La table de Karnaugh m'a permis de relier facilement mes MUX à l'opcode, en trouvant simplement la suite logique de sortie correspondant à chaque opération.
  - Le code de Gray m'a également aidé dans cette démarche.