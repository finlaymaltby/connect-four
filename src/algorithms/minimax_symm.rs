use crate::algorithms::minimax_cached::minimax_cached_helper;
use crate::basic::*;
use crate::board::{Board, CloneBoard};
use std::collections::HashMap;
use std::hash::{Hash, RandomState};

/// Type to store the difference in height of each column with its reflection,
/// to efficiently compute when board is symmetrical.
///
/// i.e. symm_diff[i] = column[6-i] - column[i].height()
type SymmDiff = [isize; 3];

/// Creates a SymmDiffs for the given board.
/// Returns None if the board is irreversably asymmetrical.
fn make_diffs<B: Board>(board: &B) -> Option<SymmDiff> {
    let mut diffs: [isize; 3] = [0; 3];
    for &col_l in column::IDXS[0..3].iter() {
        let col_r = col_l.flipped();

        for row in row::BOTTOM_UP {
            let token_l = board.get(&Cell { col: col_l, row });
            let token_r = board.get(&Cell { col: col_r, row });
            match (token_l, token_r) {
                (None, None) => break,
                (None, Some(_)) => diffs[usize::from(col_l)] += 1,
                (Some(_), None) => diffs[usize::from(col_l)] -= 1,
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
    if let Some(diffs) = make_diffs(&board) {
        minimax_symm_helper(board, depth, curr, &mut cache, diffs)
    } else {
        minimax_cached_helper(board, depth, curr, &mut cache)
    }
}

/// Updates the given diff considering the token just placed at `cell`.
/// Returns None if the board is irreversibly asymmetrical
fn next_diffs<B: Board>(board: &B, cell: &Cell, diffs: SymmDiff) -> Option<SymmDiff> {
    if cell.col == column::Idx::CENTRE {
        return Some(diffs);
    }

    let mut new_diffs = diffs;
    let token = board.get(cell)?;
    let flipped = Cell {
        col: cell.col.flipped(),
        row: cell.row,
    };

    if let Some(flipped) = board.get(&flipped)
        && token != flipped
    {
        return None;
    }

    if usize::from(cell.col) < 3 {
        new_diffs[usize::from(cell.col)] -= 1;
    } else {
        new_diffs[usize::from(flipped.col)] += 1;
    }

    Some(new_diffs)
}

fn next_boards<B: CloneBoard + Hash>(
    board: &B,
    curr: &Token,
    diffs: SymmDiff,
) -> Vec<(Option<SymmDiff>, B, Cell)> {
    match diffs {
        [0, 0, 0] => column::IDXS[0..=3]
            .iter()
            .filter_map(|col| board.clone_and_place(&col, curr))
            .map(|(b, p)| (next_diffs(&b, &p, diffs), b, p))
            .collect(),

        _ => board
            .next_boards(curr)
            .map(|(b, p)| (next_diffs(&b, &p, diffs), b, p))
            .collect(),
    }
}

fn minimax_symm_helper<B: CloneBoard + Hash>(
    board: B,
    depth: usize,
    curr: Token,
    cache: &mut HashMap<B, Option<Token>, RandomState>,
    diffs: SymmDiff,
) -> Option<Token> {
    if depth == 0 {
        return None;
    }

    if let Some(cached_result) = cache.get(&board) {
        return *cached_result;
    }

    let mut out = None;

    let mut losing = true;

    for (diffs, next_board, cell) in next_boards(&board, &curr, diffs) {
        if next_board.won_at(&cell) {
            out = Some(curr);
            break;
        }

        let result = match diffs {
            None => minimax_cached_helper(next_board, depth - 1, curr.next(), cache),
            Some(diffs) => minimax_symm_helper(next_board, depth - 1, curr.next(), cache, diffs),
        };

        if let Some(winner) = result {
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
    use super::*;
    use crate::board::{
        Board, array_board::ArrayBoard, bit_board::BitBoard, symm_board::SymmBoard,
    };

    make_easy_tests!(
        |mut b, d| {
            let curr = b.curr_player();
            minimax_symm(b, d, curr)
        },
        ArrayBoard,
        BitBoard,
        SymmBoard
    );

    make_medium_tests!(
        |mut b, d| {
            let curr = b.curr_player();
            minimax_symm(b, d, curr)
        },
        SymmBoard,
        ArrayBoard
    );
}
