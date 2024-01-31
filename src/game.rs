use crate::board::Boardstate;
use crate::board::PieceKind;
use crate::pretty_print;
enum Event {
     Move,
     End
}
# [derive(Clone, Copy)]
pub enum Turnstyle {
    White,
    Black,
    Ending,
}

impl Turnstyle {
fn next(self, event:Event) -> Turnstyle {
        match(self, event) {
            (Turnstyle::White, Event::Move) => Turnstyle::Black,
            (Turnstyle::Black, Event::Move) => Turnstyle::White,
            (_, Event::End) => Turnstyle::Ending,
            (Turnstyle::Ending, Event::Move) => Turnstyle::Ending,
       }
    }
}


pub fn game(board: &mut Boardstate, turn: &Turnstyle) {
    use Event::*;
    use Turnstyle::*;
    match turn {
        White => {
            let (next_move, kind) = get_next_move(true, board);
           board.update(next_move, true, kind, false);
           let turn: Turnstyle = if board.ending() {turn.next(End)} else {turn.next(Move)};
            pretty_print(board.white[PieceKind::Pawn.index()]);
            game(board, &turn);
        }
        Black => {
            let (next_move, kind) = get_next_move(false, board);
           board.update(next_move, true, kind, false);
           let turn: Turnstyle = if board.ending() {turn.next(End)} else {turn.next(Move)}; 
            pretty_print(board.white[PieceKind::Pawn.index()]);
            game(board, &turn);
        }
        Ending =>{
            println!("It's over!");
        }
    }
}

fn get_next_move(color: bool, board: &mut Boardstate) -> (u64, PieceKind) {
    let next_move:u64  = 0b1000000010000000000;
    let kind:PieceKind = PieceKind::Pawn;
    return (next_move, kind);
} 