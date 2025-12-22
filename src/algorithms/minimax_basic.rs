use crate::algorithms;
use crate::basic::*;
use crate::board::{CloneBoard, MutBoard};

pub fn minimax_mut<B: MutBoard>(board: &mut B, depth: usize, curr: Token) -> Option<Token> {
    if depth == 0 {
        return None;
    }

    let mut losing = true;

    for col in column::IDXS {
        if let Some(cell) = board.place(&col, &curr) {
            if board.won_at(&cell) {
                board.unplace(&cell);
                return Some(curr);
            }

            if let Some(winner) = minimax_mut(board, depth - 1, curr.next()) {
                if winner == curr {
                    board.unplace(&cell);
                    return Some(curr);
                }
            } else {
                losing = false;
            }
            board.unplace(&cell);
        }
    }

    if losing { Some(curr.next()) } else { None }
}

pub fn minimax_clone<B: CloneBoard>(board: B, depth: usize, curr: Token) -> Option<Token> {
    if depth == 0 {
        return None;
    }

    let mut losing = true;

    for (next_board, cell) in board.next_boards(&curr) {
        if next_board.won_at(&cell) {
            return Some(curr);
        }

        if let Some(winner) = minimax_clone(next_board, depth - 1, curr.next()) {
            if winner == curr {
                return Some(curr);
            }
        } else {
            losing = false;
        }
    }

    if losing { Some(curr.next()) } else { None }
}

#[cfg(test)]
mod mut_tests {
    use super::*;
    use crate::board::{
        Board, array_board::ArrayBoard, bit_board::BitBoard, symm_board::SymmBoard,
    };

    fn run_minimax_mut<B: MutBoard>(mut board: B, depth: usize) -> Option<Token> {
        let curr = board.curr_player();
        minimax_mut(&mut board, depth, curr)
    }

    make_easy_tests!(
        |mut b, d| {
            let curr = b.curr_player();
            minimax_mut(&mut b, d, curr)
        },
        ArrayBoard,
        BitBoard,
        SymmBoard
    );
}

#[cfg(test)]
mod clone_tests {
    use super::*;
    use crate::board::{
        Board, array_board::ArrayBoard, bit_board::BitBoard, symm_board::SymmBoard,
    };

    make_easy_tests!(
        |mut b, d| {
            let curr = b.curr_player();
            minimax_clone(b, d, curr)
        },
        ArrayBoard,
        BitBoard,
        SymmBoard
    );
}
