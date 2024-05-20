use crate::board::Boardstate;
use crate::board::PieceKind;

pub fn get_move(color: bool, board: &mut Boardstate) -> (u64, PieceKind) {
    return (2, PieceKind::Rook);
}
