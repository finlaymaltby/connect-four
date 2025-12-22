use crate::basic::*;
use crate::board::CloneBoard;
use binary_heap_plus::*;
use std::{
    collections::HashMap,
    hash::{Hash, RandomState},
};

pub fn minimax_counting<B: CloneBoard + Hash>(
    board: B,
    depth: usize,
    curr: Token,
) -> Option<Token> {
    let mut cache = HashMap::new();
    minimax_counting_helper(board, depth, curr, &mut cache)
}

pub fn minimax_counting_helper<B: CloneBoard + Hash>(
    board: B,
    depth: usize,
    curr: Token,
    cache: &mut HashMap<B, Option<Token>, RandomState>,
) -> Option<Token> {
    if depth == 0 {
        return None;
    }

    if let Some(cached_result) = cache.get(&board) {
        return *cached_result;
    }

    // BinaryHeap to sort possible boards by heuristic
    let mut nexts =
        BinaryHeap::with_capacity_by_key(column::COUNT, |(next_board, cell, info)| *info);

    for (next_board, cell) in board.next_boards(&curr) {
        match next_board.count_adjacent_at(&cell) {
            // found a win
            None => {
                cache.insert(next_board, Some(curr));
                return Some(curr);
            }
            // add the  board
            Some(info) => nexts.push((next_board, cell, info)),
        }
    }

    let mut losing = true;

    for (next_board, cell, _) in nexts.into_iter_sorted() {
        match minimax_counting_helper(next_board, depth - 1, curr.next(), cache) {
            None => losing = false,
            Some(winner) if winner == curr => {
                cache.insert(board, Some(winner));
                return Some(winner);
            }
            _ => (),
        }
    }

    let result = if losing { Some(curr.next()) } else { None };
    cache.insert(board, result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::{
        Board, array_board::ArrayBoard, bit_board::BitBoard, symm_board::SymmBoard,
    };

    make_easy_tests!(
        |mut b, d| {
            let curr = b.curr_player();
            minimax_counting(b, d, curr)
        },
        ArrayBoard,
        BitBoard,
        SymmBoard
    );

    make_medium_tests!(
        |mut b, d| {
            let curr = b.curr_player();
            minimax_counting(b, d, curr)
        },
        SymmBoard,
        BitBoard
    );

}
