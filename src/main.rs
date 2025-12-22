#![feature(step_trait, new_range_api)]
#![allow(unused)]

use crate::algorithms::minimax_basic::{minimax_clone, minimax_mut};
use crate::algorithms::minimax_cached::minimax_cached;
use crate::algorithms::minimax_counting::minimax_counting;
use crate::algorithms::minimax_symm::minimax_symm;

use crate::basic::*;
use crate::board::Board;
use crate::board::array_board::ArrayBoard;
use crate::board::bit_board::BitBoard;
use crate::board::symm_board::SymmBoard;
use crate::test_positions::{MEDIUM_0, MEDIUM_1};
use std::time::Instant;

mod algorithms;
mod basic;
mod board;
mod finite_index;
mod test_positions;

fn speed_test() {
    let depth = 24;

    let board_str = MEDIUM_1.board;

    let mut results = Vec::new();

    // ArrayBoard with minimax_copy
    let board = ArrayBoard::read(board_str);
    let start = Instant::now();
    println!("{:?}", minimax_clone(board, depth, Token::START));
    results.push(("ArrayBoard + minimax_copy", start.elapsed()));

    // ArrayBoard with minimax_mut
    let mut board = ArrayBoard::read(board_str);
    let start = Instant::now();
    println!("{:?}", minimax_mut(&mut board, depth, Token::START));
    results.push(("ArrayBoard + minimax_mut", start.elapsed()));

    // BitBoard with minimax_copy
    let board = BitBoard::read(board_str);
    let start = Instant::now();
    println!("{:?}", minimax_clone(board, depth, Token::START));
    results.push(("BitBoard + minimax_copy", start.elapsed()));

    // BitBoard with minimax_mut
    let mut board = BitBoard::read(board_str);
    let start = Instant::now();
    println!("{:?}", minimax_mut(&mut board, depth, Token::START));
    results.push(("BitBoard + minimax_mut", start.elapsed()));

    // ArrayBoard with minimax_cached
    let board = ArrayBoard::read(board_str);
    let start = Instant::now();
    println!("{:?}", minimax_cached(board, depth, Token::START));
    results.push(("ArrayBoard + minimax_cached", start.elapsed()));

    // BitBoard with minimax_cached
    let board = BitBoard::read(board_str);
    let start = Instant::now();
    println!("{:?}", minimax_cached(board, depth, Token::START));
    results.push(("BitBoard + minimax_cached", start.elapsed()));

    // SymmetricBitBoard with minimax_copy
    let board = SymmBoard::read(board_str);
    let start = Instant::now();
    println!("{:?}", minimax_clone(board, depth, Token::START));
    results.push(("SymmetricBitBoard + minimax_copy", start.elapsed()));

    // SymmetricBitBoard with minimax_mut
    let mut board = SymmBoard::read(board_str);
    let start = Instant::now();
    println!("{:?}", minimax_mut(&mut board, depth, Token::START));
    results.push(("SymmetricBitBoard + minimax_mut", start.elapsed()));

    // SymmetricBitBoard with minimax_cached
    let board = SymmBoard::read(board_str);
    let start = Instant::now();
    println!("{:?}", minimax_cached(board, depth, Token::START));
    results.push(("SymmetricBitBoard + minimax_cached", start.elapsed()));

    println!("\n{:<30} {:>15}", "Configuration", "Time (ms)");
    println!("{:-<47}", "");
    for (name, duration) in results {
        println!("{:<30} {:>15.2}", name, duration.as_secs_f64() * 1000.0);
    }
}

fn main() {
    let depth = 14;

    let board_str = "|.......|
                     |.......|
                     |.......|
                     |.......|
                     |...R...|
                     |..YYR..|";

    let board = BitBoard::read(board_str);
    let start = Instant::now();
    minimax_cached(board, depth, Token::START);
    println!("bitboard + cached: {:?}", start.elapsed());

    let board = BitBoard::read(board_str);
    let start = Instant::now();
    minimax_symm(board, depth, Token::START);
    println!("bitboard + symm: {:?}", start.elapsed());

    let board = SymmBoard::read(board_str);
    let start = Instant::now();
    minimax_cached(board, depth, Token::START);
    println!("symboard + cached: {:?}", start.elapsed());

    let board = SymmBoard::read(board_str);
    let start = Instant::now();
    minimax_symm(board, depth, Token::START);
    println!("symmboard + symm: {:?}", start.elapsed());

    let board = BitBoard::read(board_str);
    let start = Instant::now();
    minimax_counting(board, depth, Token::START);
    println!("bitboard + counting: {:?}", start.elapsed());

    let board = SymmBoard::read(board_str);
    let start = Instant::now();
    minimax_counting(board, depth, Token::START);
    println!("symboard + counting: {:?}", start.elapsed());
}
