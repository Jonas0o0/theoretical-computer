// let x = ...
VAL 7
PASS_B D
VAL 0
PASS_A RAM
// let y = ...
VAL 7
PASS_B D
VAL 1
PASS_A RAM
// let touche = ...
VAL 0
PASS_B D
VAL 2
PASS_A RAM
// --- LOOP ---
// touche = ...
VAL 127
PASS_B D
PASS_A A
PASS_B D_B
VAL 2
PASS_A RAM
VAL 2
PASS_B D_B
VAL 122
SUB D
// --- IF ---
VAL 122
PASS_B D
// --- Cache droite (adresse 120) ---
VAL 120
PASS_A RAM
VAL 2
PASS_B D_B
// --- Opération (Equal) ---
VAL 120
CMP_EQ D_B
VAL 0
CMP_EQ D
VAL 40
VAL 40
VAL 40
PASS_A JMP
// y -= 1 (Opti DEC)
VAL 1
PASS_B D_B
VAL 1
DEC RAM
VAL 2
PASS_B D_B
VAL 115
SUB D
// --- IF ---
VAL 115
PASS_B D
// --- Cache droite (adresse 120) ---
VAL 120
PASS_A RAM
VAL 2
PASS_B D_B
// --- Opération (Equal) ---
VAL 120
CMP_EQ D_B
VAL 0
CMP_EQ D
VAL 62
VAL 62
VAL 62
PASS_A JMP
// y += 1 (Opti INC)
VAL 1
PASS_B D_B
VAL 1
INC RAM
VAL 2
PASS_B D_B
VAL 113
SUB D
// --- IF ---
VAL 113
PASS_B D
// --- Cache droite (adresse 120) ---
VAL 120
PASS_A RAM
VAL 2
PASS_B D_B
// --- Opération (Equal) ---
VAL 120
CMP_EQ D_B
VAL 0
CMP_EQ D
VAL 84
VAL 84
VAL 84
PASS_A JMP
// x -= 1 (Opti DEC)
VAL 0
PASS_B D_B
VAL 0
DEC RAM
VAL 2
PASS_B D_B
VAL 100
SUB D
// --- IF ---
VAL 100
PASS_B D
// --- Cache droite (adresse 120) ---
VAL 120
PASS_A RAM
VAL 2
PASS_B D_B
// --- Opération (Equal) ---
VAL 120
CMP_EQ D_B
VAL 0
CMP_EQ D
VAL 106
VAL 106
VAL 106
PASS_A JMP
// x += 1 (Opti INC)
VAL 0
PASS_B D_B
VAL 0
INC RAM
// poke(127, ...)
VAL 0
PASS_B D
VAL 127
PASS_A RAM
// poke(125, ...)
VAL 0
PASS_B D_B
VAL 125
PASS_A RAM
// poke(126, ...)
VAL 1
PASS_B D_B
VAL 126
PASS_A RAM
// poke(124, ...)
VAL 4
PASS_B D
VAL 124
PASS_A RAM
VAL 255
PASS_B D
VAL 12
PASS_A JMP