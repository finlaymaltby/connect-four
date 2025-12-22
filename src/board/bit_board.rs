use crate::basic::*;
use crate::board::{Board, CloneBoard, MutBoard, bit_col};


/// A board implementation using bit manipulation for storage.
/// Each column is stored as a BitCol.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BitBoard {
    cols: [bit_col::BitCol; column::COUNT],
}
impl Board for BitBoard {
    const EMPTY: Self = BitBoard {
        cols: [bit_col::BitCol::EMPTY; column::COUNT],
    };

    fn get(&self, cell: &Cell) -> Option<Token> {
        self.cols[usize::from(cell.col)].get(&cell.row)
    }

    fn can_place(&self, col: &column::Idx) -> bool {
        !self.cols[usize::from(*col)].is_full()
    }

    fn place(&mut self, col: &column::Idx, token: &Token) -> Option<Cell> {
        if self.can_place(col) == false {
            return None;
        }

        let col_idx = usize::from(*col);
        self.cols[col_idx].force_push(token);
        Some(Cell {
            col: *col,
            row: row::Idx::try_from(self.cols[col_idx].count() - 1).unwrap(),
        })
    }
}

impl CloneBoard for BitBoard {}

impl MutBoard for BitBoard {
    fn unplace(&mut self, cell: &Cell) {
        self.cols[usize::from(cell.col)].force_pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    make_board_tests!(BitBoard);
    make_mut_board_tests!(BitBoard);
}
