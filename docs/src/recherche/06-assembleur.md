# Couche 5 : Langage d'Assemblage
La couche 5 a permis de créer un assembleur, premier outil logiciel permettant de programmer le CPU sans écrire directement les instructions en binaire.

### Résultats de la recherche :

1. **Assembleur** : implémentation en Rust d'un parseur traduisant des mnémoniques textuels (assembleur) en instructions binaires 8 bits, conformes à l'ISA défini en Couche 3.
2. **Premier programme exécuté** : validation de la chaîne complète (assembleur → binaire → VM → CPU simulé) par l'exécution réussie d'un programme *Hello World*, écrivant une chaîne ASCII en mémoire.
3. **Facilité de mise en œuvre** : contrairement aux couches précédentes, la traduction assembleur → binaire et son exécution se sont mises en place sans difficulté majeure, la logique de correspondance mnémonique → opcode/mode découlant directement de l'ISA déjà spécifié.

### Apprentissages clés

Manipulation concrète du fonctionnement d'un assembleur : comment un texte lisible par un humain (mnémoniques) se traduit mécaniquement en instructions binaires exploitables par le CPU, et comment ce processus s'articule avec le cycle d'exécution bas niveau déjà construit.
