pub mod board;

use crate::board::{Board, Coordinate};

fn main() {
    println!("Hello, world!");
    let mut board: Board = Board::new(); 
    board.play(Coordinate::new(1, 1));
}
