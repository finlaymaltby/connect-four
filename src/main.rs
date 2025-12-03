#![feature(step_trait, new_range_api)]

use crate::basic::*;
use crate::board::array_board::ArrayBoard;
use crate::board::moves_board::MovesBoard;
use crate::board::*;

use crate::algorithms::minimax_basic::{minimax_copy, minimax_mut};
use crate::algorithms::minimax_sneaky::minimax_sneaky;
use std::time::Instant;

mod algorithms;
mod basic;
mod board;
mod finite_index;

fn main() {
    let depth = 8;

    let start_mut = Instant::now();
    let t1 = minimax_mut(&mut MovesBoard::EMPTY, depth, Token::START);
    let duration_mut = start_mut.elapsed();

    let start_copy = Instant::now();
    let t2 = minimax_copy(MovesBoard::EMPTY, depth, Token::START);
    let duration_copy = start_copy.elapsed();


    let start_mut_a = Instant::now();
    let a1 = minimax_mut(&mut ArrayBoard::EMPTY, depth, Token::START);
    let duration_mut_a = start_mut_a.elapsed();

    let start_copy_a = Instant::now();
    let a2 = minimax_copy(ArrayBoard::EMPTY, depth, Token::START);
    let duration_copy_a = start_copy_a.elapsed();

    assert_eq!(a1, a2, "minimax_mut and minimax_copy produced different results for ArrayBoard");

    // test minimax_sneaky for MovesBoard (use a fresh board to avoid moves/consumption issues)
    let start_sneaky = Instant::now();
    let s1 = minimax_sneaky(MovesBoard::EMPTY, depth, Token::START);
    let duration_sneaky = start_sneaky.elapsed();
    assert_eq!(s1, t1, "minimax_sneaky produced a different result for MovesBoard");

    // test minimax_sneaky for ArrayBoard (fresh board)
    let start_sneaky_a = Instant::now();
    let sa1 = minimax_sneaky(ArrayBoard::EMPTY, depth, Token::START);
    let duration_sneaky_a = start_sneaky_a.elapsed();
    assert_eq!(sa1, a1, "minimax_sneaky produced a different result for ArrayBoard");

    println!("Time taken by minimax_mut (MovesBoard): {:?}", duration_mut);
    println!("Time taken by minimax_copy (MovesBoard): {:?}", duration_copy);
    println!("Time taken by minimax_sneaky (MovesBoard): {:?}", duration_sneaky);
    println!("Time taken by minimax_mut (ArrayBoard): {:?}", duration_mut_a);
    println!("Time taken by minimax_copy (ArrayBoard): {:?}", duration_copy_a);
    println!("Time taken by minimax_sneaky (ArrayBoard): {:?}", duration_sneaky_a);
}
