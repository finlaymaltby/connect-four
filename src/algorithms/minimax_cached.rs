use crate::basic::*;
use crate::board::CloneBoard;
use std::collections::HashMap;
use std::hash::{Hash, RandomState};

pub fn minimax_cached<B: CloneBoard + Hash>(board: B, depth: usize, curr: Token) -> Option<Token> {
    let mut cache = HashMap::new();
    minimax_cached_helper(board, depth, curr, &mut cache)
}

fn minimax_cached_helper<B: CloneBoard + Hash>(
    board: B,
    depth: usize,
    curr: Token,
    cache: &mut HashMap<B, Option<Token>, RandomState>,
) -> Option<Token> {
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

#[cfg(test)]
mod tests {
    use crate::board::{bit_board::BitBoard, symmetric_bit_board::SymmetricBitBoard};
    use crate::test_boards;

    use super::*;

    fn test_board<B: CloneBoard + Hash>(board_str: &str, outcome: Option<Token>, depth: usize) {
        let board = B::read(board_str);
        let curr = board.curr_player();
        
        assert_eq!(minimax_cached(board, depth, curr), outcome, "{}", board_str);
    }

    #[test]
    fn test_boards() {
        for (board_str, outcome, depth) in test_boards::MEDIUM_TEST_BOARDS {
            test_board::<BitBoard>(board_str, outcome, depth);
            test_board::<SymmetricBitBoard>(board_str, outcome, depth);
        }
    }
}