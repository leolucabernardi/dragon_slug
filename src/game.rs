use crate::board::Boardstate;
use crate::board::Bitboard;
use crate::board::PieceKind;
enum Event {
     Move,
     End
}
enum Turnstyle {
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


pub fn game() {
    use Event::*;
    use Turnstyle::*;
    let turn = Turnstyle::White;
    let board: Boardstate = Boardstate::setup();
    match turn {
        White => {
            let next_move = get_next_move();
            board.update(next_move);
            println!("w");
            turn.next(Move);
            }
        Black => {
            let next_move = get_next_move();
            board.update(next_move);
            println!("b");
            turn.next(End);
        }
        Ending =>{
            println!("It's over!")
        }
    }
}

fn get_next_move() -> Bitboard {
    let next_move: Bitboard = Bitboard {bit: 0b100000001, color: true, kind: PieceKind::Rook};
    return next_move;
} 