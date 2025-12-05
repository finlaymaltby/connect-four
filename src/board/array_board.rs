use std::ops::{Index, IndexMut};

use crate::basic::*;
use crate::board::{Board, CloneBoard, MutBoard};

/// MutBoard implementation using a 2D array of Option<Token>.
/// An array of columns, where the 0th element is the bottom of the column.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayBoard {
    grid: [[Option<Token>; row::COUNT]; column::COUNT],
}

impl Index<(column::Idx, row::Idx)> for ArrayBoard {
    type Output = Option<Token>;

    fn index(&self, index: (column::Idx, row::Idx)) -> &Self::Output {
        &self.grid[usize::from(index.0)][usize::from(index.1)]
    }
}

impl IndexMut<(column::Idx, row::Idx)> for ArrayBoard {
    fn index_mut(&mut self, index: (column::Idx, row::Idx)) -> &mut Self::Output {
        &mut self.grid[usize::from(index.0)][usize::from(index.1)]
    }
}

impl Board for ArrayBoard {
    const EMPTY: Self = ArrayBoard {
        grid: [[None; row::COUNT]; column::COUNT],
    };

    fn can_place(&self, col: &column::Idx) -> bool {
        self[(*col, row::Idx::TOP)].is_none()
    }

    fn get(&self, pos: &Position) -> Option<Token> {
        self[(pos.col, pos.row)]
    }

    fn place_unchecked(&mut self, col: &column::Idx, token: &Token) -> Position {
        for row in row::IDXS {
            if self[(*col, row)].is_none() {
                self[(*col, row)] = Some(*token);
                return Position {
                    col: *col,
                    row: row,
                };
            }
        }
        panic!("Must check `can_place` before calling `force_place`");
    }
}

impl CloneBoard for ArrayBoard {}

impl MutBoard for ArrayBoard {
    fn unplace_unchecked(&mut self, col: &column::Idx) {
        for row in (row::Idx::ZERO..=row::Idx::TOP).rev() {
            if self[(*col, row)].is_some() {
                self[(*col, row)] = None;
                return;
            }
        }
        panic!("Tried to unplace from an empty column: {:?}", col);
    }
}
