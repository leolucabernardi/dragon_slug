
// bitboards for the initial position of the white pieces that will be flipped to initialise blacks position
pub const PAWN_INITIAL: u64 = 0xFF00;
pub const KNIGHT_INITIAL: u64 = 0b1000010;
pub const BISHOP_INITIAL: u64 = 0b100100;
pub const ROOK_INITIAL: u64 = 0b10000001;
pub const QUEEN_INITIAL: u64 = 0b10000;
pub const KING_INITIAL: u64 = 0b00001000;

pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
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
        }
    }
}
pub struct Bitboard {
    pub bit: u64,
    pub color: bool,
    pub kind: PieceKind,
}
impl Bitboard {

    pub const fn pawn(color: bool) -> Self {
        Self { bit: if color {PAWN_INITIAL} else {PAWN_INITIAL<<40}, color, kind: PieceKind::Pawn}
    }
    pub const fn knight(color: bool) -> Self {
        Self { bit: if color {KNIGHT_INITIAL} else {KNIGHT_INITIAL<<56}, color, kind: PieceKind::Knight}
    }
    pub const fn bishop(color: bool) -> Self {
        Self { bit: if color {BISHOP_INITIAL} else {BISHOP_INITIAL<<56}, color, kind: PieceKind::Bishop}
    }
    pub const fn rook(color: bool) -> Self {
        Self { bit: if color {ROOK_INITIAL} else {ROOK_INITIAL<<56}, color, kind: PieceKind::Rook}
    }
    pub const fn queen(color: bool) -> Self {
        Self { bit: if color {QUEEN_INITIAL} else {QUEEN_INITIAL<<56}, color, kind: PieceKind::Queen}
    }
    pub const fn king(color: bool) -> Self {
        Self { bit: if color {KING_INITIAL} else {KING_INITIAL<<56}, color, kind: PieceKind::King}
    }
    pub const fn all(color: bool) -> Self {
        Self { bit: if color {KING_INITIAL ^ QUEEN_INITIAL ^ ROOK_INITIAL ^ BISHOP_INITIAL ^ KNIGHT_INITIAL ^ PAWN_INITIAL } 
            else {(KING_INITIAL ^ QUEEN_INITIAL ^ ROOK_INITIAL ^ BISHOP_INITIAL ^ KNIGHT_INITIAL)<<56 ^PAWN_INITIAL<<40},
            color, kind: PieceKind::King}
    }
}

pub struct Boardstate {
    pub white_king: Bitboard,
    pub black_king: Bitboard,

    pub white_queen: Bitboard,
    pub black_queen: Bitboard,

    pub white_rooks: Bitboard,
    pub black_rooks: Bitboard,

    pub white_bishops: Bitboard,
    pub black_bishops: Bitboard,

    pub white_knights: Bitboard,
    pub black_knights: Bitboard,

    pub white_pawns: Bitboard,
    pub black_pawns: Bitboard,

    pub white_pieces: Bitboard,
    pub black_pieces: Bitboard,

    pub white_castling_allowed: bool,
    pub black_castling_allowed: bool,

    pub ply_count: i32,
    pub white_to_play: bool,
    pub checkmate: bool
}
impl Boardstate {
    pub fn setup() -> Self {
        
        Self {  
            white_king: Bitboard::king(true), 
            black_king: Bitboard::king(false),

            white_queen: Bitboard::queen(true),
            black_queen: Bitboard::queen(false),

            white_rooks: Bitboard::rook(true),
            black_rooks: Bitboard::rook(false),

            white_bishops: Bitboard::bishop(true), 
            black_bishops: Bitboard::bishop(false),

            white_knights: Bitboard::knight(true), 
            black_knights: Bitboard::knight(false),

            white_pawns: Bitboard::pawn(true), 
            black_pawns: Bitboard::pawn(false),

            white_pieces:  Bitboard::all(true),
            black_pieces: Bitboard::all(false),

            white_castling_allowed: true, 
            black_castling_allowed: true,

            ply_count: 0,
            white_to_play: true,
            checkmate: false,
            }
    }
    pub fn update(mut self, target_board: Bitboard) -> Self {
        match target_board.kind.index() {
            0 => if target_board.color {self.white_pawns.bit ^= target_board.bit;} else {self.black_pawns.bit ^= target_board.bit;}
            1 => if target_board.color {self.white_knights.bit ^= target_board.bit;} else {self.black_knights.bit ^= target_board.bit;}
            2 => if target_board.color {self.white_bishops.bit ^= target_board.bit;} else {self.black_bishops.bit ^= target_board.bit;}
            3 => if target_board.color {self.white_rooks.bit ^= target_board.bit;} else {self.black_rooks.bit ^= target_board.bit;}
            4 => if target_board.color {self.white_queen.bit ^= target_board.bit;} else {self.black_queen.bit ^= target_board.bit;}
            5 => if target_board.color {self.white_king.bit ^= target_board.bit;} else {self.black_king.bit ^= target_board.bit;}
            _ => println!("how?")
        }
        self.ply_count += 1;
        return self;
    }
}
