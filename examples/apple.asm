// let x = ...
VAL 1
PASS_B D
VAL 0
PASS_A RAM
// let direction = ...
VAL 1
PASS_B D
VAL 1
PASS_A RAM
// --- LOOP ---
// poke(124, ...)
VAL 3
PASS_B D
VAL 124
PASS_A RAM
VAL 1
PASS_B D_B
VAL 1
SUB D
// --- IF ---
VAL 1
PASS_B D
// --- Cache droite (adresse 120) ---
VAL 120
PASS_A RAM
VAL 1
PASS_B D_B
// --- Opération (Equal) ---
VAL 120
CMP_EQ D_B
VAL 0
CMP_EQ D
VAL 34
VAL 34
VAL 34
PASS_A JMP
// x += 1 (Opti INC)
VAL 0
PASS_B D_B
VAL 0
INC RAM
VAL 1
PASS_B D_B
VAL 0
SUB D
// --- IF ---
VAL 0
PASS_B D
// --- Cache droite (adresse 120) ---
VAL 120
PASS_A RAM
VAL 1
PASS_B D_B
// --- Opération (Equal) ---
VAL 120
CMP_EQ D_B
VAL 0
CMP_EQ D
VAL 56
VAL 56
VAL 56
PASS_A JMP
// x -= 1 (Opti DEC)
VAL 0
PASS_B D_B
VAL 0
DEC RAM
VAL 0
PASS_B D_B
VAL 15
SUB D
// --- IF ---
VAL 15
PASS_B D
// --- Cache droite (adresse 120) ---
VAL 120
PASS_A RAM
VAL 0
PASS_B D_B
// --- Opération (Equal) ---
VAL 120
CMP_EQ D_B
VAL 0
CMP_EQ D
VAL 78
VAL 78
VAL 78
PASS_A JMP
// direction = ...
VAL 0
PASS_B D
VAL 1
PASS_A RAM
VAL 0
PASS_B D_B
VAL 0
SUB D
// --- IF ---
VAL 0
PASS_B D
// --- Cache droite (adresse 120) ---
VAL 120
PASS_A RAM
VAL 0
PASS_B D_B
// --- Opération (Equal) ---
VAL 120
CMP_EQ D_B
VAL 0
CMP_EQ D
VAL 100
VAL 100
VAL 100
PASS_A JMP
// direction = ...
VAL 1
PASS_B D
VAL 1
PASS_A RAM
// poke(125, ...)
VAL 0
PASS_B D_B
VAL 125
PASS_A RAM
// poke(126, ...)
VAL 0
PASS_B D_B
VAL 126
PASS_A RAM
// poke(124, ...)
VAL 2
PASS_B D
VAL 124
PASS_A RAM
VAL 255
PASS_B D
VAL 8
PASS_A JMP