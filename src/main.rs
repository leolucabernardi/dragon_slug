mod board;
mod helper;
mod game;
use board::Boardstate;
use game::*;
fn main() {
   let mut board: Boardstate = Boardstate::setup();
   let turn = Turnstyle::White;
    game(&mut board, &turn);
}

fn pretty_print(bits: u64){
    print!("  a b c d e f g h   \n*-----------------*\n| ");
    for i in (0..64).rev() {
      if bits & 1<<i > 0 {
            print!("{}", '👾')
        } else {
            print!("b ")
        }
        if i>0 && (i) % 8 == 0 && i!=63{
            print!("| {}" ,1 + i/ 8);
            print!("\n| ")
        }
    }

    println!("| 1\n*-----------------*")
}