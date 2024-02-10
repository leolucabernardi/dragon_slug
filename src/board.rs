// bitboards for the initial position of the white pieces that will be flipped to initialise blacks position
pub const PAWN_INITIAL: u64 = 0xFF00;
pub const KNIGHT_INITIAL: u64 = 0b1000010;
pub const BISHOP_INITIAL: u64 = 0b100100;
pub const ROOK_INITIAL: u64 = 0b10000001;
pub const QUEEN_INITIAL: u64 = 0b10000;
pub const KING_INITIAL: u64 = 0b00001000;

static SYMBOLS: [(char, char); 6] = [
    ('♙', '♟'),
    ('♘', '♞'),
    ('♗', '♝'),
    ('♖', '♜'),
    ('♕', '♛'),
    ('♔', '♚'),
];

// const WHITE_PAWN_ATTACKS: [u64; 64]
// const BLACK_PAWN_ATTACKS: [u64; 64];
// const KNIGHT_ATTACKS: [u64; 64];
// const BISHOP_ATTACKS: [u64; 64];
// const ROOK_ATTACKS: [u64; 64] = [0b11111111 0x101010101010101];
// const QUEEN_ATTACKS: [u64; 64];
// const KING_ATTACKS: [u64; 64];
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    All,
}
impl PieceKind {
    // allows us to use numbers as the piece names which is nice
    pub fn index(self) -> usize {
        match self {
            PieceKind::Pawn => 0,
            PieceKind::Knight => 1,
            PieceKind::Bishop => 2,
            PieceKind::Rook => 3,
            PieceKind::Queen => 4,
            PieceKind::King => 5,
            PieceKind::All => 6,
        }
    }
}
pub const fn pawn(color: bool) -> u64 {
    return if color {
        PAWN_INITIAL
    } else {
        PAWN_INITIAL << 40
    };
}
pub const fn knight(color: bool) -> u64 {
    return if color {
        KNIGHT_INITIAL
    } else {
        KNIGHT_INITIAL << 56
    };
}
pub const fn bishop(color: bool) -> u64 {
    return if color {
        BISHOP_INITIAL
    } else {
        BISHOP_INITIAL << 56
    };
}
pub const fn rook(color: bool) -> u64 {
    return if color {
        ROOK_INITIAL
    } else {
        ROOK_INITIAL << 56
    };
}
pub const fn queen(color: bool) -> u64 {
    return if color {
        QUEEN_INITIAL
    } else {
        QUEEN_INITIAL << 56
    };
}
pub const fn king(color: bool) -> u64 {
    return if color {
        KING_INITIAL
    } else {
        KING_INITIAL << 56
    };
}
pub const fn all(color: bool) -> u64 {
    return if color {
        KING_INITIAL | QUEEN_INITIAL | ROOK_INITIAL | BISHOP_INITIAL | KNIGHT_INITIAL | PAWN_INITIAL
    } else {
        (KING_INITIAL | QUEEN_INITIAL | ROOK_INITIAL | BISHOP_INITIAL | KNIGHT_INITIAL) << 56
            ^ PAWN_INITIAL << 40
    };
}

#[derive(Debug)]
pub struct Boardstate {
    pub white: [u64; 7],
    pub black: [u64; 7],

    pub white_castling_allowed: bool,
    pub black_castling_allowed: bool,

    pub ply_count: i32,
    pub checkmate: bool,
    pub draw: bool,
}
impl Boardstate {
    pub fn setup() -> Self {
        Self {
            white: [
                pawn(true),
                knight(true),
                bishop(true),
                rook(true),
                queen(true),
                king(true),
                all(true),
            ],

            black: [
                pawn(false),
                knight(false),
                bishop(false),
                rook(false),
                queen(false),
                king(false),
                all(false),
            ],

            white_castling_allowed: true,
            black_castling_allowed: true,

            ply_count: 0,
            checkmate: false,
            draw: false,
        }
    }
    pub fn update(
        &mut self,
        new_move: u64,
        color: bool,
        kind: PieceKind,
        double_move: bool,
    ) -> &mut Self {
        if self.ply_count > 10 {
            self.draw = true;
        }
        if !double_move {
            match color {
                true => self.white[kind.index()] ^= new_move,
                false => self.black[kind.index()] ^= new_move,
            }
        }
        self.ply_count += 1;
        return self;
    }
    pub fn ending(&self) -> bool {
        return self.draw | self.checkmate;
    }
    pub fn populate_squares(&self, squares: &mut [char; 64]) {
        let mut p = 0;
        for piece_bits in self.white {
            if p < 6 {
                for i in (0..64).rev() {
                    if piece_bits & 1 << i > 0 {
                        squares[i] = SYMBOLS[p].0;
                    }
                }
            }
            p += 1;
        }
        p = 0;
        for piece_bits in self.black {
            if p < 6 {
                for i in (0..64).rev() {
                    if piece_bits & 1 << i > 0 {
                        squares[i] = SYMBOLS[p].1;
                    }
                }
            }
            p += 1;
        }
    }
    pub fn get_legal_moves() {}
}
