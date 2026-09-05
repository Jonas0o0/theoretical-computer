# Construction d'un ordinateur à partir de zéro : de la porte NAND au langage JUMP

## Résumé
*Ce document retrace la conception et l'implémentation complète d'une architecture informatique, des portes logiques jusqu'au langage de haut niveau.*

## Introduction

Ce projet a pour ambition de construire un ordinateur complet **from scratch**, en partant des portes logiques les plus élémentaires (NAND) pour aboutir, couche après couche, à un langage de programmation de haut niveau. L'objectif n'est pas de produire un système optimisé pour un usage réel, mais de comprendre en profondeur *comment* un ordinateur fonctionne, en construisant chaque abstraction soi-même plutôt que de la considérer comme acquise.

L'approche suivie est **"bottom-up"** : chaque couche s'appuie exclusivement sur les composants validés dans la couche précédente. Ainsi, la logique booléenne (couche 0) sert de fondation à l'arithmétique et à l'ALU (couche 1), qui elle-même servira de base à la mémoire et aux registres (couche 2), et ainsi de suite jusqu'au compilateur.

Deux outils complémentaires sont utilisés tout au long du projet, selon une méthodologie **"Double-Track"** :
- **Logisim**, pour la conception visuelle et la simulation des circuits logiques.
- **Rust**, pour l'émulation logicielle des mêmes composants, permettant de valider leur comportement via des tests unitaires rigoureux (tables de vérité, cas limites, etc.).
