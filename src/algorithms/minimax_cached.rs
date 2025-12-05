use crate::basic::*;
use crate::board::{Board, CloneBoard, MutBoard};
use std::hash::{Hash, RandomState};
use std::collections::HashMap;

pub fn minimax_cached<B: CloneBoard + Hash>(board: B, depth: usize, curr: Token) -> Option<Token> {
    let mut cache = HashMap::new();
    minimax_cached_helper(board, depth, curr, &mut cache)
}

fn minimax_cached_helper<B: CloneBoard + Hash>(board: B, depth: usize, curr: Token, cache: &mut HashMap<B, Option<Token>, RandomState>) -> Option<Token> {
    if depth == 0 {
        cache.insert(board, None);
        return None;
    }

    if let Some(cached_result) = cache.get(&board) {
        return *cached_result;
    }

    let mut out = None;

    let mut losing = true;

    for (next_board, pos) in board.next_boards(&curr) {
        if next_board.won_at(&pos) {
            out = Some(curr);
            break;
        }

        if let Some(winner) = minimax_cached_helper(next_board, depth - 1, curr.next(), cache) {
            if winner == curr {
                out = Some(curr);
                break;
            }
        } else {
            losing = false;
        }
    }

    if out.is_none() && losing { 
        out = Some(curr.next());
    } 

    cache.insert(board, out);
    out
}   