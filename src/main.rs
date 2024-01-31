mod board;
mod helper;
mod game;
use game::*;
fn main() {
   game();

}
fn pretty_print(bit: u64){
    let mask: u64 = 0b11111111;
    for shift in (0..8).rev() {
        println!("{:08b}", bit>>shift*8 & mask)
    }
}