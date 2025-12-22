use crate::basic::*;
use crate::board::CloneBoard;
use hashbrown::HashMap;
use std::hash::{Hash, RandomState};

pub fn minimax_cached<B: CloneBoard + Hash>(board: B, depth: usize, curr: Token) -> Option<Token> {
    let mut cache = HashMap::new();
    minimax_cached_helper(board, depth, curr, &mut cache)
}

pub fn minimax_cached_helper<B: CloneBoard + Hash>(
    board: B,
    depth: usize,
    curr: Token,
    cache: &mut HashMap<B, Option<Token>>,
) -> Option<Token> {
    if depth == 0 {
        return None;
    }

    if let Some(cached_result) = cache.get(&board) {
        return *cached_result;
    }

    let mut out = None;

    let mut losing = true;

    for (next_board, cell) in board.next_boards(&curr) {
        if next_board.won_at(&cell) {
            out = Some(curr);
            break;
        }

        match minimax_cached_helper(next_board, depth - 1, curr.next(), cache) {
            None => losing = false,
            Some(winner) if winner == curr => {
                out = Some(winner);
                break;
            }
            _ => (),
        }
    }
    if out.is_none() && losing {
        out = Some(curr.next());
    }

    cache.insert(board, out);
    out
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
            minimax_cached(b, d, curr)
        },
        ArrayBoard,
        BitBoard,
        SymmBoard
    );

    make_medium_tests!(
        |mut b, d| {
            let curr = b.curr_player();
            minimax_cached(b, d, curr)
        },
        SymmBoard,
        BitBoard
    );


}
