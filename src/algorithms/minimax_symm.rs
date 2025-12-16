use crate::basic::*;
use crate::board::{Board, CloneBoard};
use std::collections::HashMap;
use std::hash::{Hash, RandomState};

/// Type to store the difference in height of each column with its reflection,
/// to efficiently compute when board is symmetrical.
/// 
/// i.e. symm_diff[i] = column[6-i] - column[i].height()
type SymmDiff = Option<[isize; 3]>;

fn make_diffs<B: Board>(board: &B) -> SymmDiff {
    let mut diffs: [isize; 3] = [0; 3];
    for &col_l in column::IDXS[0..3].iter() {
        let col_r = col_l.flipped();

        for row in row::BOTTOM_UP {
            let token_l = board.get(&Position {col: col_l, row});
            let token_r = board.get(&Position {col: col_r, row});
            match (token_l, token_r) {
                (None, None) => break,
                (None, Some(_)) => {
                    diffs[usize::from(col_l)] += 1
                },
                (Some(_), None) => {
                    diffs[usize::from(col_l)] -= 1
                }
                (Some(token_l), Some(token_r)) => {
                    if token_l != token_r {
                        return None;
                    }
                }
            }
        }
    }

    Some(diffs)

}   

pub fn minimax_symm<B: CloneBoard + Hash>(board: B, depth: usize, curr: Token) -> Option<Token> {
    let mut cache = HashMap::new();
    let diffs = make_diffs(&board);
    minimax_symm_helper(board, depth, curr, &mut cache, diffs)
}

fn next_diffs<B: Board>(board: &B, pos: &Position, diffs: SymmDiff) -> SymmDiff {
    if pos.col == column::Idx::CENTRE {
        return diffs;
    }

    let mut diffs = diffs?;

    let placed = board.get(pos)?;
    let flipped = Position {col: pos.col.flipped(), row: pos.row};

    if Some(placed) != board.get(&flipped) {
        return None;
    }

    if usize::from(pos.col) < 3 {
        diffs[usize::from(pos.col)] -= 1;
    }  else {
        diffs[usize::from(flipped.col)] += 1;
    }

    Some(diffs)
}

fn next_boards<B: CloneBoard + Hash>(board: &B, curr: &Token, diffs: SymmDiff) -> Vec<(SymmDiff, B, Position)> {
    match diffs {
        None => board.next_boards(curr)
            .map(|(b, p)| (None, b, p))
            .collect(),

        Some([0,0,0]) => column::IDXS[0..=3].iter()
            .filter_map(|col| board.clone_and_place(&col, curr))
            .map(|(b, p)| (next_diffs(&b, &p, diffs), b, p))
            .collect(),

        _ => board.next_boards(curr) 
            .map(|(b, p)| (next_diffs(&b, &p, diffs), b, p))
            .collect(),

    }
}

fn minimax_symm_helper<B: CloneBoard + Hash>(
    board: B,
    depth: usize,
    curr: Token,
    cache: &mut HashMap<B, Option<Token>, RandomState>,
    diffs: SymmDiff
) -> Option<Token> {
    if depth == 0 {
        return None;
    }

    if let Some(cached_result) = cache.get(&board) {
        return *cached_result;
    }

    let mut out = None;

    let mut losing = true;

    for (diffs, next_board, pos) in next_boards(&board, &curr, diffs) {
        if next_board.won_at(&pos) {
            out = Some(curr);
            break;
        }

        if let Some(winner) = minimax_symm_helper(next_board, depth - 1, curr.next(), cache, diffs) {
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
    use crate::board::{bit_board::BitBoard, symm_board::SymmBoard};
    use crate::test_boards;

    use super::*;

    fn test_board<B: CloneBoard + Hash>(board_str: &str, outcome: Option<Token>, depth: usize) {
        let board = B::read(board_str);
        let curr = board.curr_player();
        
        assert_eq!(minimax_symm(board, depth, curr), outcome, "{}", board_str);
    }

    #[test]
    fn test_boards() {
        for (board_str, outcome, depth) in test_boards::MEDIUM_TEST_BOARDS {
            test_board::<BitBoard>(board_str, outcome, depth);
            test_board::<SymmBoard>(board_str, outcome, depth);
        }
    }

    fn board_is_symm(board: &BitBoard) -> bool {
        for col in column::IDXS {
            for row in row::BOTTOM_UP {
                let pos = Position {col, row};
                if board.get(&pos) != board.get(&pos.flipped()) {
                    return false;
                }
            }
        }

        true
    }

    #[test]
    fn test_symm() {
        let board = BitBoard::EMPTY;
        let mut cache = HashMap::new();
        let diffs = make_diffs(&board);
        minimax_symm_helper(board, 8, Token::START, &mut cache, diffs);

        for board in cache.keys() {
            if board_is_symm(board) {
                continue;
            }

            assert!(!cache.contains_key(&board.flipped()), "minimax_symm visited this board and its reflection:\n{}", board.to_string());
        }

    }
}