#![feature(step_trait, new_range_api)]
#[allow(unused)]
use crate::basic::*;

use crate::board::array_board::ArrayBoard;
use crate::board::*;

use crate::algorithms::minimax_basic::{minimax_copy, minimax_mut};
use std::time::Instant;

mod algorithms;
mod basic;
mod board;
mod finite_index;

fn main() {
    let mut board: ArrayBoard = ArrayBoard::EMPTY;

    let depth = 10;

    let start_mut = Instant::now();
    let t1 = minimax_mut(&mut board, depth, Token::START);
    let duration_mut = start_mut.elapsed();

    let start_copy = Instant::now();
    let t2 = minimax_copy(board, depth, Token::START);
    let duration_copy = start_copy.elapsed();

    println!("Time taken by minimax_mut: {:?}", duration_mut);
    println!("Time taken by minimax_copy: {:?}", duration_copy);
}
