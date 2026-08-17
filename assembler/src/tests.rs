    use super::*;

    #[test]
    fn val_valeur_simple() {
        assert_eq!(parse_instruction("VAL 5"), 5);
    }

    #[test]
    fn val_valeur_maximale_sur_7_bits() {
        assert_eq!(parse_instruction("VAL 127"), 127);
    }

    #[test]
    fn val_valeur_hors_plage_est_masquee_sur_7_bits() {
        assert_eq!(parse_instruction("VAL 200"), 72);
    }

    #[test]
    fn val_valeur_non_numerique_retombe_a_zero() {
        assert_eq!(parse_instruction("VAL abc"), 0);
    }

    #[test]
    #[should_panic]
    fn val_sans_argument_panique() {
        parse_instruction("VAL");
    }

    #[test]
    fn add_sans_mode_utilise_le_mode_d_par_defaut() {
        assert_eq!(parse_instruction("ADD"), 0b1000_0000);
    }

    #[test]
    fn add_mode_d() {
        assert_eq!(parse_instruction("ADD D"), 0b1000_0000);
    }

    #[test]
    fn add_mode_d_b() {
        assert_eq!(parse_instruction("ADD D_B"), 0b1000_0001);
    }

    #[test]
    fn add_mode_a() {
        assert_eq!(parse_instruction("ADD A"), 0b1000_0010);
    }

    #[test]
    fn add_mode_ram() {
        assert_eq!(parse_instruction("ADD RAM"), 0b1000_0011);
    }

    #[test]
    fn add_mode_ram_b() {
        assert_eq!(parse_instruction("ADD RAM_B"), 0b1000_0100);
    }

    #[test]
    fn add_mode_jmp() {
        assert_eq!(parse_instruction("ADD JMP"), 0b1000_0101);
    }

    #[test]
    fn add_mode_jmp_b() {
        assert_eq!(parse_instruction("ADD JMP_B"), 0b1000_0110);
    }

    #[test]
    fn add_mode_ad() {
        assert_eq!(parse_instruction("ADD AD"), 0b1000_0111);
    }

    #[test]
    fn opcode_sub() {
        assert_eq!(parse_instruction("SUB D"), 0b1000_1000);
    }

    #[test]
    fn opcode_rsub() {
        assert_eq!(parse_instruction("RSUB D"), 0b1001_0000);
    }

    #[test]
    fn opcode_inc() {
        assert_eq!(parse_instruction("INC D"), 0b1001_1000);
    }

    #[test]
    fn opcode_dec() {
        assert_eq!(parse_instruction("DEC D"), 0b1010_0000);
    }

    #[test]
    fn opcode_and() {
        assert_eq!(parse_instruction("AND D"), 0b1010_1000);
    }

    #[test]
    fn opcode_or() {
        assert_eq!(parse_instruction("OR D"), 0b1011_0000);
    }

    #[test]
    fn opcode_xor() {
        assert_eq!(parse_instruction("XOR D"), 0b1011_1000);
    }

    #[test]
    fn opcode_not() {
        assert_eq!(parse_instruction("NOT D"), 0b1100_0000);
    }

    #[test]
    fn opcode_shl() {
        assert_eq!(parse_instruction("SHL D"), 0b1100_1000);
    }

    #[test]
    fn opcode_shr() {
        assert_eq!(parse_instruction("SHR D"), 0b1101_0000);
    }

    #[test]
    fn opcode_cmp_eq() {
        assert_eq!(parse_instruction("CMP_EQ D"), 0b1101_1000);
    }

    #[test]
    fn opcode_cmp_lt_et_cmp_gt_partagent_le_meme_code() {
        assert_eq!(parse_instruction("CMP_LT D"), 0b1110_1000);
        assert_eq!(parse_instruction("CMP_GT D"), 0b1110_1000);
    }

    #[test]
    fn opcode_pass_a() {
        assert_eq!(parse_instruction("PASS_A D"), 0b1111_0000);
    }

    #[test]
    fn opcode_pass_b() {
        assert_eq!(parse_instruction("PASS_B D"), 0b1111_1000);
    }

    #[test]
    fn ligne_vide_retourne_zero() {
        assert_eq!(parse_instruction(""), 0);
    }

    #[test]
    fn ligne_avec_seulement_des_espaces_retourne_zero() {
        assert_eq!(parse_instruction("   "), 0);
    }

    #[test]
    fn operation_inconnue_retourne_zero() {
        assert_eq!(parse_instruction("FOO D"), 0);
    }

    #[test]
    fn mode_inconnu_retombe_sur_mode_d_par_defaut() {
        assert_eq!(parse_instruction("ADD FOO"), parse_instruction("ADD D"));
    }

    #[test]
    fn espaces_multiples_entre_les_tokens_sont_ignores() {
        assert_eq!(parse_instruction("ADD    RAM"), parse_instruction("ADD RAM"));
    }