pub mod board;
pub mod bot_01;
pub mod bot_core;

use crate::{board::{Board, Coordinate}, bot_01::Bot};

fn main() {
    let mut board: Board = Board::new(); 
    let bot: Bot = Bot::new();
    board.play(Coordinate::new(1, 1)); prints(&board, &bot); // o
    board.play(Coordinate::new(2, 2)); prints(&board, &bot); // o
    board.play(Coordinate::new(1, 0)); prints(&board, &bot); // x
    board.play(Coordinate::new(0, 1)); prints(&board, &bot); // x
    board.play(Coordinate::new(3, 3)); prints(&board, &bot); // o
    board.play(Coordinate::new(4, 4)); prints(&board, &bot); // o
    board.play(Coordinate::new(2, 0)); prints(&board, &bot); // x
    board.play(Coordinate::new(0, 2)); prints(&board, &bot); // x
    board.play(Coordinate::new(5, 5)); prints(&board, &bot); // o
    board.play(Coordinate::new(6, 6)); prints(&board, &bot); // o
}

fn prints(board: &Board, bot: &Bot) {
    println!("{:?}", board.winner());
    println!("{:?}", bot.evaluate(&board));
}