pub mod board;
pub mod eval_01;
pub mod algo_01;

use crate::{algo_01::Algo, board::{Board, Coord}, eval_01::Eval};

fn main() {
    let board: Board = Board::new(); 
    let eval: Eval = Eval::new();
    let algo: Algo = Algo::new();
    while board.winner() == None {
        let decision: Option<Coord> = algo.decide(&board, &eval);
        println!("{:?}", decision);
    }
    
    match board.winner() {
        Some(winner) => println!("{:?}", winner),
        None => {}, 
    }
}