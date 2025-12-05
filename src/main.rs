#![feature(step_trait, new_range_api)]
use crate::algorithms::minimax_basic::{minimax_copy, minimax_mut};
use crate::algorithms::minimax_cached::minimax_cached;

use crate::basic::*;
use crate::board::Board;
use crate::board::array_board::ArrayBoard;
use crate::board::bit_board::BitBoard;
use crate::board::count_bit_board::CountBitBoard;
use std::time::Instant;

mod algorithms;
mod basic;
mod board;
mod finite_index;

fn main() {
    let depth = 8;

    let board_str = "|.......|
                     |.......|
                     |.......|
                     |...R...|
                     |...Y...|
                     |..RYYR.|";

    let mut results = Vec::new();

    // ArrayBoard with minimax_copy
    let board = ArrayBoard::read(board_str);
    let start = Instant::now();
    println!("{:?}", minimax_copy(board, depth, Token::START));
    results.push(("ArrayBoard + minimax_copy", start.elapsed()));

    // ArrayBoard with minimax_mut
    let mut board = ArrayBoard::read(board_str);
    let start = Instant::now();
    println!("{:?}", minimax_mut(&mut board, depth, Token::START));
    results.push(("ArrayBoard + minimax_mut", start.elapsed()));

    // BitBoard with minimax_copy
    let board = BitBoard::read(board_str);
    let start = Instant::now();
    println!("{:?}", minimax_copy(board, depth, Token::START));
    results.push(("BitBoard + minimax_copy", start.elapsed()));

    // BitBoard with minimax_mut
    let mut board = BitBoard::read(board_str);
    let start = Instant::now();
    println!("{:?}", minimax_mut(&mut board, depth, Token::START));
    results.push(("BitBoard + minimax_mut", start.elapsed()));

    // CountBitBoard with minimax_copy
    let board = CountBitBoard::read(board_str);
    let start = Instant::now();
    println!("{:?}", minimax_copy(board, depth, Token::START));
    results.push(("CountBitBoard + minimax_copy", start.elapsed()));

    // CountBitBoard with minimax_mut
    let mut board = CountBitBoard::read(board_str);
    let start = Instant::now();
    println!("{:?}", minimax_mut(&mut board, depth, Token::START));
    results.push(("CountBitBoard + minimax_mut", start.elapsed()));

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

    // CountBitBoard with minimax_cached
    let board = CountBitBoard::read(board_str);
    let start = Instant::now();
    println!("{:?}", minimax_cached(board, depth, Token::START));
    results.push(("CountBitBoard + minimax_cached", start.elapsed()));

    println!("\n{:<30} {:>15}", "Configuration", "Time (ms)");
    println!("{:-<47}", "");
    for (name, duration) in results {
        println!("{:<30} {:>15.2}", name, duration.as_secs_f64() * 1000.0);
    }
}
