pub fn create_attack_patterns() {
    let WHITE_PAWN_ATTACKS: [u64; 64] = [0; 64];
    let BLACK_PAWN_ATTACKS: [u64; 64] = [0; 64];
    let KNIGHT_ATTACKS: [u64; 64] = [0; 64];
    let mut BISHOP_ATTACKS: [u64; 64] = [0; 64];
    let mut ROOK_ATTACKS: [u64; 64] = [0; 64];
    let mut QUEEN_ATTACKS: [u64; 64] = [0; 64];
    let KING_ATTACKS: [u64; 64] = [0; 64];
   
    for square in 0..64 {
        ROOK_ATTACKS[square] = (0b11111111) << ((square / 8)*8) ^ (0x101010101010101) << (square % 8);
        BISHOP_ATTACKS[square] =  1<<square;
        QUEEN_ATTACKS[square] = ROOK_ATTACKS | BISHOP_ATTACKS;
    }
    
}

fn pretty_print(bitboard: u64) {
    for i in (0..64).rev() {
        if bitboard & 1<<i > 0 {print!("1")} else {print!("0")}
        if i%8 == 0 {
            print!("\n")
        }
    }
    print!("\n")
}

// const ROOK_ATTACKS: [u64; 64] = [0b11111111 0x101010101010101];
