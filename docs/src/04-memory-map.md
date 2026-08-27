# Carte Mémoire - CPU 8 bits (V2)

Ce document décrit l'organisation de la RAM de notre processeur.
Afin de pouvoir interagir avec le monde extérieur (Clavier et Écran) sans modifier l'architecture matérielle du CPU, nous utilisons la technique du **Memory-Mapped I/O**.

> **Note d'Architecture (Le mur des 7 bits) :**
> Dans notre ISA, le bit de poids fort de l'instruction sert à définir le type (`T=0` pour une adresse/valeur, `T=1` pour un calcul). L'instruction `VAL` ne dispose donc que de 7 bits, limitant l'accès direct en écriture (`poke`) aux adresses de `0` à `127`. La carte mémoire a été optimisée ("Zero Page") pour placer les I/O dans cette zone hautement accessible.

## Répartition Globale

| Adresses (Décimal) | Adresses (Hex) | Utilisation |
| --- | --- | --- |
| **0 à 119** | `0x00 - 0x77` | **RAM Standard (Libre)** - Espace disponible pour le code utilisateur, les variables et les tableaux (ex: le corps du Snake). |
| **120 à 123** | `0x78 - 0x7B` | **Zone de Sécurité (OS)** - Réservée pour les variables internes critiques du compilateur (ex: cache mathématique) ou du jeu. |
| **124 à 127** | `0x7C - 0x7F` | **Memory-Mapped I/O** - Adresses matérielles pour contrôler l'écran et lire le clavier. |
| **128 à 255** | `0x80 - 0xFF` | **RAM Supérieure (Shadow RAM)** - Zone mémoire difficilement accessible en écriture directe. Réservée pour de futures extensions (lecture seule, etc.). |

---

## Détail des Entrées/Sorties (I/O)

### Clavier (Adresse 127)

La Machine Virtuelle injecte le code ASCII de la dernière touche pressée dans cette case de la RAM.

* **Action requise :** Le programme Assembleur doit lire cette case, puis la **remettre à 0** pour éviter de lire la même touche en boucle.

### Écran (Adresses 124, 125, 126)

Pour dessiner un pixel à l'écran, le processeur doit écrire les coordonnées X et Y, puis utiliser l'adresse `124` comme une **gâchette** en y insérant un code couleur.

* **Adresse `125` :** Coordonnée X (de 0 à la largeur du terminal).
* **Adresse `126` :** Coordonnée Y (de 0 à la hauteur du terminal).
* **Adresse `124` :** Code de dessin (Gâchette). Dès que cette case est différente de `0`, la VM dessine et remet automatiquement la case à `0`.

**Codes de dessin disponibles (Adresse 124) :**

* `0` = En attente (Aucune action)
* `1` = ██ (Bloc plein / Corps du serpent)
* `2` = 🍎 (Pomme / Objectif)
* `3` = Effacer (Espaces vides)