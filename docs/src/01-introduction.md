# Introduction

Bienvenue dans la documentation du projet **Theoretical Computeur**.

## Le projet

Ce projet a pour ambition de construire un ordinateur complet **from scratch**, en partant des portes logiques les plus élémentaires (NAND) pour aboutir à un langage de programmation de haut niveau propre, **JUMP**. L'objectif n'est pas de produire un système optimisé pour un usage réel, mais de comprendre en profondeur *comment* un ordinateur fonctionne, en construisant chaque abstraction soi-même plutôt que de la considérer comme acquise.

## Approche

Le projet suit une construction **"bottom-up"** : chaque couche s'appuie exclusivement sur les composants validés dans la couche précédente.

| Couche | Contenu | Statut |
|---|---|---|
| 0 | Logique booléenne (NAND, AND, OR, MUX...) | Terminé |
| 1 | Arithmétique et ALU | Terminé |
| 2 | Mémoire et registres | Terminé |
| 3 | Architecture du processeur (CPU) | Terminé |
| 4 | Machine virtuelle et émulation | Terminé |
| 5 | Langage d'assemblage | Terminé |
| 6 | Compilation et langage haut niveau (JUMP) | En cours |

Chaque composant matériel est développé selon une méthodologie **"Double-Track"** :
- **Logisim**, pour la conception visuelle et la simulation physique des circuits logiques (source de vérité hardware).
- **Rust**, pour l'émulation logicielle des mêmes composants, validée par des tests unitaires rigoureux (tables de vérité, cas limites, etc.).

## Comment naviguer cette documentation

- Le [Guide de Démarrage](02-quickstart.md) explique comment écrire, assembler et exécuter un programme sur la machine simulée.
- La section **Architecture & Matériel** détaille la conception du CPU et sa carte mémoire.
- La section **Le Langage JUMP** documente le langage de haut niveau du projet.
- La section **Recherche & Suivi** contient les [notes de thèse](06-these.md) (analyse technique détaillée de chaque couche) et le [journal de bord](07-journal.md) (suivi chronologique des sprints).