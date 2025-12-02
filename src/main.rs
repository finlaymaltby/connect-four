#![feature(step_trait, new_range_api)]

use crate::basic::*;

use crate::board::array_board::ArrayBoard;
use crate::board::*;

use crate::algorithms::minimax_basic::minimax_basic;

mod basic;
mod board;
mod algorithms;

fn main() {
    let mut board: ArrayBoard = ArrayBoard::EMPTY;

    let t = minimax_basic(&mut board, 42, Token::Red);

    println!("{:?}", t);
}
