mod board;
mod helper;
mod game;
mod player;
mod bot;
mod moves;
use board::Boardstate;
use game::*;
fn main() {
    let mut board: Boardstate = Boardstate::setup();
    let turn = Turnstyle::White;
    moves::create_attack_patterns();
    // game(&mut board, &turn);
}



fn pretty_print(board: &mut Boardstate){
    let mut squares: [char; 64] =  ['░', '▓', '░', '▓', '░', '▓', '░', '▓',
                                    '▓', '░', '▓', '░', '▓', '░', '▓', '░',
                                    '░', '▓', '░', '▓', '░', '▓', '░', '▓',
                                    '▓', '░', '▓', '░', '▓', '░', '▓', '░',
                                    '░', '▓', '░', '▓', '░', '▓', '░', '▓',
                                    '▓', '░', '▓', '░', '▓', '░', '▓', '░',
                                    '░', '▓', '░', '▓', '░', '▓', '░', '▓',
                                    '▓', '░', '▓', '░', '▓', '░', '▓', '░'];
    board.populate_squares(&mut squares);
    println!("abcdefgh");
    for i in (0..64).rev() {
        print!("{}", squares[i]);
        if i < 63 && (i % 8 == 0) {
            print!(" | {} \n", i/8 + 1)
        }
    }
    print!("\n\n")
}