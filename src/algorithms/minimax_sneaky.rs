use crate::basic::*;
use crate::board::{Board, CloneBoard, MutBoard};

pub fn minimax_sneaky<B: CloneBoard>(board: B, depth: usize, curr: Token) -> Option<Token> {
    if depth == 0 {
        return None;
    }

    let mut losing = true;

    for col in column::IDXS_CENTRED_FIRST {
        let (next_board, pos) = match board.clone_and_place(&col, &curr) {
            Some(v) => v,
            None => continue,
        };

        if next_board.won_at(&pos) {
            return Some(curr);
        }

        if let Some(winner) = minimax_sneaky(next_board, depth - 1, curr.next()) {
            if winner == curr {
                return Some(curr);
            }
        } else {
            losing = false;
        }
    }

    if losing { Some(curr.next()) } else { None }
}