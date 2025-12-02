use crate::basic::*;
use crate::board::{Board, MutBoard};

pub fn minimax_basic<B: MutBoard>(board: &mut B, depth: usize, curr: Token) -> Option<Token> {
    if depth == 0 { 
        return None; 
    }
    for col in column::COLIDXS.iter() {
        if board.can_place(col) {
            let pos = board.force_place(col, &curr);

            if board.won_at(&pos) {
                return Some(curr);
            }

            if let Some(winner) = minimax_basic(board, depth - 1, curr.next()) {
                if winner == curr {
                    return Some(curr);
                }
            }
            board.unplace(&pos);
        }
    }
    None
}