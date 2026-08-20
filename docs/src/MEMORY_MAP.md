# Carte Mémoire (Memory Map) - CPU 8 bits

Ce document décrit l'organisation des 256 octets de la RAM de notre processeur.
Afin de pouvoir interagir avec le monde extérieur (Clavier et Écran) sans modifier l'architecture matérielle du CPU, nous utilisons la technique du **Memory-Mapped I/O**.

La Machine Virtuelle (VM) observe en permanence certaines adresses spécifiques à la fin de la RAM pour déclencher des actions.

## Répartition Globale

| Adresses (Décimal) | Adresses (Hex) | Utilisation |
| :--- | :--- | :--- |
| **0 à 239** | `0x00 - 0xEF` | **RAM Standard (Libre)** - Espace disponible pour le code utilisateur, les variables et les tableaux (ex: le corps du Snake). |
| **240 à 251** | `0xF0 - 0xFB` | **Zone de Sécurité** - Réservée pour les variables internes critiques de l'OS ou du jeu (ex: taille du serpent, pointeurs). |
| **252 à 255** | `0xFC - 0xFF` | **Memory-Mapped I/O** - Adresses matérielles pour contrôler l'écran et lire le clavier. |

---

## Détail des Entrées/Sorties (I/O)

### Clavier (Adresse 255)
La Machine Virtuelle injecte le code ASCII de la dernière touche pressée dans cette case de la RAM.
- **Action requise :** Le programme Assembleur doit lire cette case, puis la **remettre à 0** pour éviter de lire la même touche en boucle.

### Écran (Adresses 252, 253, 254)
Pour dessiner un pixel à l'écran, le processeur doit écrire les coordonnées X et Y, puis utiliser l'adresse `252` comme une **gâchette** en y insérant un code couleur.

*   **Adresse `253` :** Coordonnée X (de 0 à la largeur du terminal).
*   **Adresse `254` :** Coordonnée Y (de 0 à la hauteur du terminal).
*   **Adresse `252` :** Code de dessin (Gâchette). Dès que cette case est différente de `0`, la VM dessine et remet automatiquement la case à `0`.

**Codes de dessin disponibles (Adresse 252) :**
*   `0` = En attente (Aucune action)
*   `1` = ██ (Bloc plein / Corps du serpent)
*   `2` = 🍎 (Pomme / Objectif)
*   `3` = Effacer (Espaces vides)