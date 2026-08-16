// ==========================================
// PROGRAMME : HELLO WORLD
// ==========================================

// --- 1. Preuve de vie mathématique ---
// Calcul de 17 + 3, stocké dans RAM[3]
VAL 17
PASS_B D
VAL 3
ADD RAM


// --- 2. Mot "HELLO" ---

// 'H' (72) -> RAM[100]
VAL 72
PASS_B D
VAL 100
PASS_A RAM

// 'E' (69) -> RAM[101]
VAL 69
PASS_B D
VAL 101
PASS_A RAM

// 'L' (76) -> RAM[102]
VAL 76
PASS_B D
VAL 102
PASS_A RAM

// 'L' (76) -> RAM[103]
VAL 76
PASS_B D
VAL 103
PASS_A RAM

// 'O' (79) -> RAM[104]
VAL 79
PASS_B D
VAL 104
PASS_A RAM


// --- 3. Espace " " ---

// ' ' (32) -> RAM[105]
VAL 32
PASS_B D
VAL 105
PASS_A RAM


// --- 4. Mot "WORLD" ---

// 'W' (87) -> RAM[106]
VAL 87
PASS_B D
VAL 106
PASS_A RAM

// 'O' (79) -> RAM[107]
VAL 79
PASS_B D
VAL 107
PASS_A RAM

// 'R' (82) -> RAM[108]
VAL 82
PASS_B D
VAL 108
PASS_A RAM

// 'L' (76) -> RAM[109]
VAL 76
PASS_B D
VAL 109
PASS_A RAM

// 'D' (68) -> RAM[110]
VAL 68
PASS_B D
VAL 110
PASS_A RAM