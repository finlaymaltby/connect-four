use crate::basic::*;
use crate::board::{Board, CloneBoard, MutBoard};

pub fn minimax_mut<B: MutBoard>(board: &mut B, depth: usize, curr: Token) -> Option<Token> {
    if depth == 0 {
        return None;
    }

    let mut losing = true;

    for col in column::IDXS {
        if let Some(pos) = board.try_place(&col, &curr) {
            if board.won_at(&pos) {
                board.unplace(&pos);
                return Some(curr);
            }

            if let Some(winner) = minimax_mut(board, depth - 1, curr.next()) {
                if winner == curr {
                    board.unplace(&pos);
                    return Some(curr);
                }
            } else {
                losing = false;
            }
            board.unplace(&pos);
        }
    }

    if losing { Some(curr.next()) } else { None }
}

pub fn minimax_copy<B: CloneBoard>(board: B, depth: usize, curr: Token) -> Option<Token> {
    if depth == 0 {
        return None;
    }

    let mut losing = true;

    for (next_board, pos) in board.next_boards(&curr) {
        if next_board.won_at(&pos) {
            return Some(curr);
        }

        if let Some(winner) = minimax_copy(next_board, depth - 1, curr.next()) {
            if winner == curr {
                return Some(curr);
            }
        } else {
            losing = false;
        }
    }

    if losing { Some(curr.next()) } else { None }
}

