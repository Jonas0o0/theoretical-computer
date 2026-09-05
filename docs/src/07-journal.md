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

## [2026-07-19] - Mémoire - Registres et RAM et Sprint 3
- **Objectif** : Gérer l'état du processeur via des bascules (Flip-Flops), des registres et de la RAM.
- **Réalisation** :
  - Implémentation en Rust d'un `Register` (registre à chargement conditionnel avec reset), d'une `Ram` (65 536 cases adressables), et d'un `PC` (Program Counter avec chargement d'adresse de saut et auto-incrémentation).
  - Tests unitaires couvrant les trois composants : `Register` (initialisation, chargement, maintien, reset), `Ram` (initialisation, écriture ciblée, isolation entre adresses, non-modification si `load = false`), et `PC` (incrémentation séquentielle, saut, reset).
- **Difficultés** :
  - La bascule D repose sur une boucle combinatoire (une sortie rebouclée sur une entrée qui dépend elle-même de cette sortie), ce qui en fait un circuit séquentiel non représentable par les fonctions pures utilisées jusqu'ici (sans notion de temps ni de bouclage). Il n'était donc pas possible de la simuler en Rust à partir des portes de la Couche 0 ; l'état est simplement stocké dans une variable (value: Byte), et clock_tick reproduit le comportement attendu (charger, garder, réinitialiser) sans reconstruire le circuit séquentiel sous-jacent. 
- **Apprentissages** :
  - Distinction entre logique combinatoire (couches précédentes) et logique séquentielle (bascules, registres) : cette dernière nécessite une notion de temps/horloge (clock_tick), qu'un enchaînement de fonctions pures ne peut pas représenter.
  - Construction d'une bascule D à partir de verrous (latch) : un verrou SR se construit à partir de 2 portes NOR croisées ; un verrou D en est une version sécurisée, où une porte AND est ajoutée devant chaque NOR pour contrôler l'écriture.

## [2026-08-03] - Architecture - Le CPU complet et Sprint 4
- **Objectif** : Intégrer l'ALU, les registres et l'unité de contrôle pour former le CPU.
- **Tâches** :
  - Définir l'ISA (Instruction Set Architecture)
  - Créer l'unité de contrôle (Control Unit) dans Logisim
  - Finaliser le cycle Fetch/Decode/Execute
- **Réalisation** :
  - Conception de mon propre ISA 8 bits, où le bit 7 sert de sélecteur entre un nombre brut chargé dans le registre A (`bit7 = 0`) et une instruction de calcul activant l'ALU (`bit7 = 1`).
  - Pour les instructions de calcul, les 3 bits de poids faible (`rom.0`, `rom.1`, `rom.2`, notés MMM) servent de mode d'adressage : ils indiquent où va le résultat (registre D, RAM via `writeM`, etc.), et sont décodés dans la Control Unit (`cu`) pour produire les signaux de contrôle (`loadA`, `loadD`, `writeM`, `aluBMux`, `jumpEnable`).
  - Câblage du MUX en entrée du registre A directement piloté par le bit 7 de la ROM (extrait via un splitter), sans passer par la Control Unit, pour ne pas la surcharger inutilement.
  - Architecture mémoire à deux espaces distincts : une **ROM** (programme) adressée par le PC sur 16 bits, et une **RAM** (données) adressée par le registre A sur 8 bits (256 cases).
  - Partie Rust entièrement terminée : simulation structurée en séparant l'état (registres, RAM, ROM, PC) de la logique combinatoire (Control Unit, MUX, ALU), le tout exécuté séquentiellement dans une méthode `tick()` pour imiter un front d'horloge.
- **Difficultés** :
  - Utilisation d'une RAM de petite taille (256 cases, adressée sur 8 bits), ce qui m'a obligé à revoir mes prévisions pour la gestion de la RAM et de la ROM du programme : le fait d'être resté en 8 bits pour la RAM ajoute des contraintes (notamment sur les sauts `JUMP`, limités aux 256 premières lignes de la ROM), mais ne bloque pas le projet.
- **Apprentissages** :
  - L'ISA est le jeu d'instructions complet compris par le processeur : chaque ligne de code assembleur est traduite en opcode binaire, décodé par la Control Unit ; une instruction hors de l'ISA n'est simplement pas reconnue.
  - Le bit 7 comme sélecteur nombre/calcul est une technique classique pour économiser un bit de mode sans instruction dédiée.
  - Réserver le bit 7 au choix nombre/calcul réduit la plage de nombres directement chargeables dans le registre A à 7 bits (0-127) ; au-delà, il faut passer par des opérations arithmétiques successives.
  - Le `JUMP` ne modifie que le Program Counter (donc la lecture de la ROM), il ne sert pas à positionner une adresse en RAM. Pour écrire en RAM, le registre A sert directement de broche d'adresse : il suffit de charger l'adresse dans A, la donnée dans D, et d'activer `writeM`.
  - La ROM (16 bits, via le PC) et la RAM (8 bits, via le registre A) sont deux espaces mémoire séparés avec des capacités différentes : le programme peut être long (jusqu'à 65 536 lignes), mais les données manipulables directement en RAM restent limitées à 256 cases.

## [2026-08-16] - Software Stack - Assembleur et VM et Sprint 5
- **Objectif** : Créer les outils permettant de programmer le CPU.
- **Tâches** :
  - Développer l'assembleur en Rust (Parsing ASM -> Binaire)
  - Finaliser la Machine Virtuelle haute performance
  - Exécuter un premier programme 'Hello World' sur le CPU simulé
    Tu as totalement raison, si c'est allé tout seul et que l'apprentissage principal c'est d'avoir découvert la logique de l'assembleur, il ne faut pas inventer des galères artificielles !
* **Réalisation** :
* Implémentation d'un parseur d'assembleur en Rust capable de traduire des mnémoniques textuels en binaire.
* Conception de la Machine Virtuelle (VM) simulant le cycle d'horloge, les registres et les 256 octets de RAM.
* Exécution réussie d'un premier programme bas niveau (*Hello World* en ASCII en mémoire).
* **Difficultés** :
* Aucune difficulté majeure notable ; la logique séquentielle de traduction et d'exécution s'est mise en place naturellement.
* **Apprentissages** :
* Découverte concrète et manipulation directe du fonctionnement de l'assembleur et du cycle d'exécution d'un processeur de bas niveau.

## [2026-08-20] - Compilateur - Analyse Lexicale et I/O VM et Sprint 6
- **Objectif** : Préparer la VM pour le jeu et créer la première brique du compilateur : le Lexer (Analyseur Lexical).
- **Tâches** :
  - Implémenter le Memory-Mapped I/O dans la VM (lecture du clavier via la RAM et affichage graphique).
  - Définir la grammaire et la syntaxe exacte du mini-langage de programmation (**JUMP**).
  - Initialiser le nouveau projet Rust pour le compilateur.
  - Développer le Lexer pour transformer le code source brut en liste de Tokens structurés.
- **Réalisation** :
  - Intégration du Memory-Mapped I/O et des graphismes textuels/emojis dans la Machine Virtuelle.
  - Rédaction de la spécification complète du langage JUMP et intégration dans le mdBook.
  - Implémentation en Rust du Lexer capable de découper un fichier source JUMP en un vecteur de `Tokens` typés.
- **Difficultés** :
  - Réfléxion rigoureusement au découpage et à l'optimisation des 256 octets de RAM disponibles : il a fallu arbitrer précisément entre l'espace alloué aux variables du programme, la pile et les adresses réservées au Memory-Mapped I/O pour ne rien saturer.
- **Apprentissages** :
  - Compréhension du fonctionnement du Memory-Mapped I/O pour piloter des périphériques via la RAM.
  - Découverte des bases de la compilation et de la première phase d'analyse lexicale.

## [2026-08-21] - Compilateur - Arbre Syntaxique (Parser) et Sprint 7
- **Objectif** : Construire la structure logique du code en transformant la suite de Tokens en un Arbre Syntaxique Abstrait (AST).
- **Tâches** :
  - Définir les structures de l'AST en Rust (enum pour les assignations, boucles, conditions).
  - Développer le Parser (Analyseur Syntaxique) pour construire l'AST en mémoire.
  - Gérer la priorité des opérations mathématiques et logiques.
  - Implémenter une gestion d'erreurs basique (signaler les erreurs de syntaxe à la compilation).
- **Réalisation** :
  - Création des énumérations de l'AST (`Program`, `Stmt`, `Expr`, `BinaryOperator`) pour le langage JUMP.
  - Implémentation d'un Parser à descente récursive fonctionnel.
  - Factorisation du code avec la création de `parse_block()` pour lire les contenus entre accolades et l'optimisation du mapping des opérateurs mathématiques.
  - Gestion de la grammaire sans parenthèses obligatoires pour les `if` et `while` (style Rust).
  - Validation de la chaîne : le Lexer génère les tokens, et le Parser construit un arbre correctement imbriqué.
- **Difficultés** :
  - Le changement de paradigme m'a demandé un temps d'adaptation : il a fallu basculer d'une analyse linéaire (liste de tokens du Lexer) à la construction d'une structure hiérarchique et imbriquée (AST), ce qui nécessite de bien maîtriser les appels de fonctions récursives.
  - La résolution des ambiguïtés syntaxiques a été un point délicat. J'ai dû appréhender la subtilité entre la lecture définitive d'un jeton (`advance`/`consume`) et l'anticipation du jeton suivant (`peek`) pour orienter l'analyseur (par exemple, déterminer si un identifiant est suivi d'un `=` pour une assignation, ou d'une `(` pour un appel de fonction).
  - Il a fallu délimiter strictement la frontière entre les instructions (`Statements`, qui exécutent une action) et les évaluations (`Expressions`, qui produisent une valeur) pour structurer le parser proprement et éviter les conflits logiques.
- **Apprentissages** :
  - J'ai bien assimilé la stricte séparation des rôles : le Parser ne fait que vérifier la grammaire, c'est le Compilateur qui gérera la logique par la suite.
  - L'AST ne stocke que ce qui a du sens logique. Les notions de texte comme les points-virgules, les parenthèses ou la fin du fichier (`EOF`) n'ont pas leur place dans l'arbre.
  - J'ai dû adapter le design du langage aux contraintes matérielles. C'est ce qui a motivé le choix de ne pas gérer les arguments de fonctions dans la V1, à cause de l'absence de pile (stack) matérielle sur mon CPU 8-bits.

## [2026-08-22] - Compilateur - Génération de Code Assembleur et Sprint 8
- **Objectif** : Traduire l'AST de manière automatisée en instructions assembleur brutes, et valider la chaîne de compilation complète de bout en bout.
- **Tâches** :
  - Développer un gestionnaire basique d'adresses mémoire (allocation automatique des variables dans la RAM).
  - Développer le générateur de code (parcourir l'AST et recracher les instructions ASM).
  - Relier le compilateur à l'assembleur pour automatiser la création du binaire .bin.
  - Compiler et exécuter avec succès un programme de test simple (ex: boucle while et clignotement de RAM).
- **Réalisation** :
  - Conception et développement intégral du module de génération de code (`codegen.rs`) traduisant l'ensemble de l'AST (variables, boucles, conditions, fonctions inlinées et opérations binaires) en assembleur.
  - Gestion de la table des symboles pour l'allocation automatique de la RAM utilisateur (limitée à l'adresse 119 pour préserver l'espace de l'OS et des entrées-sorties Memory-Mapped).
  - Mise en place du *backpatching* et de l'optimisation des sauts via les instructions de l'ALU (`SHL`, `INC`).
  - Automatisation complète du pipeline de compilation en invoquant l'assembleur via un sous-processus `Command` directement depuis le compilateur.
- **Difficultés** :
  - Gestion de la limite stricte de la ROM (127 instructions maximales) causée par le codage sur 7 bits de l'instruction `VAL`.
  - Résolution du paradoxe des sauts conditionnels où le calcul de l'adresse cible écrasait par inadvertance le registre `D` (qui stockait le résultat de la condition).
  - Structuration de la logique pour mapper proprement les opérateurs du langage de haut niveau (`JUMP`) vers les opcodes spécifiques de l'ALU sans corrompre la pile ou l'état des registres.
- **Apprentissages** :
  - Maîtrise de la contrainte matérielle de la "Zero-Page" et de la segmentation mémoire.
  - Apprentissage concret de la rigueur nécessaire lors de l'écriture d'un compilateur de bout en bout, où chaque instruction générée a un impact direct et immédiat sur la machine virtuelle et le comportement matériel.
  - Application du principe DRY (*Don't Repeat Yourself*) en architecturant une toolchain modulaire interconnectée.

Voici la complétion de ta documentation pour ce sprint mémorable. J'ai mis l'accent sur cette confrontation brutale (mais géniale) entre tes ambitions logicielles et la réalité physique de ton processeur, et comment tu as contourné le problème avec le Télécran.

---

## [2026-08-22] NAND to Game - Le Jeu Snake et Sprint 9

- **Objectif** : Utiliser notre propre langage de haut niveau et notre compilateur pour programmer le jeu Snake et y jouer sur le CPU simulé.
- **Tâches** :
  - Programmer l'initialisation du jeu (variables de la grille, serpent, spawn de la pomme).
  - Coder la boucle de jeu principale (logique de déplacement, collisions, lecture des inputs).
  - Optimiser le code et les variables pour respecter les contraintes de mémoire (RAM sur 8 bits).
  - Corriger les potentiels bugs finaux (dans le jeu ou dans le compilateur).
  - Exécuter le Snake sur la VM et faire une première partie fonctionnelle.
- **Réalisation** :
  - **Abandon du jeu Snake (et de Pong)** : Le développement de jeux avec une logique spatiale complexe (gestion de la queue dynamique du serpent, multiples collisions X/Y de Pong) générait un code assembleur beaucoup trop lourd (plus de 150 instructions), entraînant des crashs matériels (le processeur lisant dans le vide).
  - **Pivot vers le "Télécran" (Etch-A-Sketch)** : Conception et exécution réussie d'une application de dessin interactive en remplacement. Le programme lit en temps réel les frappes du clavier (touches Z, Q, S, D) via `peek(127)` et déplace un curseur pour dessiner sur l'écran en utilisant la commande `poke(..., 4)` avec un nouveau caractère matériel optimisé.
  - L'astuce du Télécran (ne pas coder l'effacement du curseur pour laisser une trace) a permis de réduire drastiquement la taille du programme à moins de 90 instructions assembleur.
- **Difficultés** :
  - **Le mur de la ROM (127 instructions)** : Découverte de la limite physique absolue de l'architecture. L'instruction `VAL` étant encodée sur 7 bits, la mémoire morte (ROM) ne peut pas adresser plus de 127 lignes de code.
  - **Le paradoxe du Registre D** : Impossibilité mathématique de faire un saut conditionnel au-delà de la ligne 127. Pour calculer une adresse >127, l'ALU doit être utilisée. Or, l'entrée A de l'ALU est physiquement câblée au registre `D`. Puisque le registre `D` stocke le drapeau (Vrai/Faux) de la condition lors d'un `if`, utiliser l'ALU pour calculer l'adresse de saut écrase instantanément le résultat de la condition, corrompant le programme.
  - **Le coût de la logique de haut niveau** : Chaque bloc `if` en langage JUMP génère une douzaine d'instructions assembleur. Il a fallu se rendre à l'évidence qu'un jeu comme Snake est inatteignable sans instructions natives plus complexes ou une mémoire ROM plus vaste.
- **Apprentissages** :
  - **Le matériel dicte le logiciel** : Expérience concrète des contraintes de l'informatique des années 70 (la fameuse "Zero-Page Constraint"). Quand le hardware ne peut plus suivre, c'est le *Game Design* qui doit s'adapter pour économiser des octets.
  - **Maîtrise du Memory-Mapped I/O** : Validation totale du système d'entrées/sorties en haut niveau. La lecture asynchrone du clavier avec `peek(127)` et la réinitialisation matérielle avec `poke(127, 0)` fonctionnent à la perfection de bout en bout.
  - Une boucle complète a été bouclée : de la porte NAND jusqu'à une application interactive avec contrôles au clavier, prouvant que l'architecture ISA personnalisée, bien que restreinte, est parfaitement fonctionnelle.
