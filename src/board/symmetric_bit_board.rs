use crate::basic::*;
use crate::board::{Board, CloneBoard, MutBoard, bit_col};
use std::hash::Hash;

/// A board implementation using bit manipulation for storage with
/// customised equality and hashing for symmetry.
/// Each column is stored as a BitCol.
#[derive(Clone, Debug)]
pub struct SymmetricBitBoard {
    cols: [bit_col::BitCol; column::COUNT],
}
impl Board for SymmetricBitBoard {
    const EMPTY: Self = SymmetricBitBoard {
        cols: [bit_col::BitCol::EMPTY; column::COUNT],
    };

    fn get(&self, pos: &Position) -> Option<Token> {
        self.cols[usize::from(pos.col)].get(&pos.row)
    }

    fn can_place(&self, col: &column::Idx) -> bool {
        !self.cols[usize::from(*col)].is_full()
    }

    fn place(&mut self, col: &column::Idx, token: &Token) -> Option<Position> {
        if self.can_place(col) == false {
            return None;
        }

        let col_idx = usize::from(*col);
        self.cols[col_idx].force_push(token);
        Some(Position {
            col: *col,
            row: row::Idx::try_from(self.cols[col_idx].count() - 1).unwrap(),
        })
    }
}

impl CloneBoard for SymmetricBitBoard {}

impl MutBoard for SymmetricBitBoard {
    fn unplace(&mut self, pos: &Position) {
        self.cols[usize::from(pos.col)].force_pop();
    }
}

impl PartialEq for SymmetricBitBoard {
    fn eq(&self, other: &Self) -> bool {
        self.cols == other.cols
            || self
                .cols
                .iter()
                .zip(other.cols.iter().rev())
                .all(|(a, b)| a == b)
    }
}

impl Eq for SymmetricBitBoard {}

impl Hash for SymmetricBitBoard {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.cols[3].hash(state);
        for i in 0..3 {
            let col_a = self.cols[i];
            let col_b = self.cols[6 - i];
            state.write_u8(col_a.as_u8() | col_b.as_u8());
            state.write_u8(col_a.as_u8() & col_b.as_u8());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    make_board_tests!(SymmetricBitBoard);
    make_mut_board_tests!(SymmetricBitBoard);
}