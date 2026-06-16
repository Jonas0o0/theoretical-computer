# Thèse : Construction d'un Ordinateur à partir de Zéro (from Scratch)

## Résumé
*Ce document retrace la conception et l'implémentation complète d'une architecture informatique, des portes logiques jusqu'au langage de haut niveau.*

## Table des Matières
1. [Introduction](#introduction)
2. [Couche 0 : Logique Booléenne (Hardware)](#couche-0)
3. [Couche 1 : Arithmétique et ALU](#couche-1)
4. [Couche 2 : Mémoire et Registres](#couche-2)
5. [Couche 3 : Architecture du Processeur (CPU)](#couche-3)
6. [Couche 4 : Machine Virtuelle et Émulation](#couche-4)
7. [Couche 5 : Langage d'Assemblage](#couche-5)
8. [Couche 6 : Compilation et Langage Haut Niveau](#couche-6)
9. [Conclusion](#conclusion)

---

## Introduction <a name="introduction"></a>
*Expliquer ici la motivation du projet, l'approche "bottom-up" (du bas vers le haut) et les outils utilisés (Logisim, Rust).*

## Couche 0 : Logique Booléenne <a name="couche-0"></a>
L'implémentation de la couche 0 a permis de valider l'universalité de la porte **NAND**. 

### Résultats de la recherche :
1.  **Universalité** : Nous avons prouvé par la pratique que les portes fondamentales (NOT, AND, OR) peuvent être exclusivement construites à partir de portes NAND.
2.  **Routage** : La conception du Multiplexeur (MUX) et du Démultiplexeur (DMUX) a posé les bases de l'aiguillage des signaux, essentiel pour la future Unité de Contrôle.
3.  **Méthodologie** : L'approche "Double-Track" (Logisim + Rust) a permis de confirmer la justesse des circuits avant leur intégration. Les tests unitaires en Rust ont validé 100% des tables de vérité mathématiques.

---
