pub mod board;
pub mod eval_01;
pub mod inter_01;

use crate::{board::{Board, Coord}, eval_01::Eval, inter_01::Inter};

fn main() {
    let mut board: Board = Board::new(); 
    let evaluator: Eval = Eval::new();
    let interpretor: Inter = Inter::new();

    board.play(Coord::new(1, 1)); prints(&board, &evaluator); // o
    board.play(Coord::new(2, 2)); prints(&board, &evaluator); // o
    board.play(Coord::new(1, 0)); prints(&board, &evaluator); // x
    board.play(Coord::new(0, 1)); prints(&board, &evaluator); // x
    board.play(Coord::new(3, 3)); prints(&board, &evaluator); // o
    board.play(Coord::new(4, 4)); prints(&board, &evaluator); // o
    board.play(Coord::new(2, 0)); prints(&board, &evaluator); // x
    board.play(Coord::new(0, 2)); prints(&board, &evaluator); // x
    board.play(Coord::new(5, 5)); prints(&board, &evaluator); // o
    board.play(Coord::new(6, 6)); prints(&board, &evaluator); // o
}

fn prints(board: &Board, evaluator: &Eval) {
    println!("{:?}", board.winner());
    println!("{:?}", evaluator.evaluate(&board));
}