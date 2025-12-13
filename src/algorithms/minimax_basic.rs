use crate::basic::*;
use crate::board::{CloneBoard, MutBoard};

pub fn minimax_mut<B: MutBoard>(board: &mut B, depth: usize, curr: Token) -> Option<Token> {
    if depth == 0 {
        return None;
    }

    let mut losing = true;

    for col in column::IDXS {
        if let Some(pos) = board.place(&col, &curr) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::{array_board::ArrayBoard, bit_board::BitBoard, symm_board::SymmBoard};
    use crate::test_boards;

    fn test_board_mut<B: MutBoard>(board_str: &str, outcome: Option<Token>, depth: usize) {
        let mut board = B::read(board_str);
        let curr = board.curr_player();
        
        assert_eq!(minimax_mut(&mut board, depth, curr), outcome, "{}", board_str);
    }

    fn test_board_clone<B: CloneBoard>(board_str: &str, outcome: Option<Token>, depth: usize) {
        let board = B::read(board_str);
        let curr = board.curr_player();
        
        assert_eq!(minimax_copy(board, depth, curr), outcome, "{}", board_str);
    }

    #[test]
    fn test_minimax_mut() {
        for (board_str, outcome, depth) in test_boards::EASY_TEST_BOARDS {
            test_board_mut::<ArrayBoard>(board_str, outcome, depth);
            test_board_mut::<BitBoard>(board_str, outcome, depth);
            test_board_mut::<SymmBoard>(board_str, outcome, depth);
        }
    }

    #[test]
    fn test_minimax_clone() {

        for (board_str, outcome, depth) in test_boards::EASY_TEST_BOARDS {
            test_board_clone::<ArrayBoard>(board_str, outcome, depth);
            test_board_clone::<BitBoard>(board_str, outcome, depth);
            test_board_clone::<SymmBoard>(board_str, outcome, depth);
        }
    }
}
