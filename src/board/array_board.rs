use std::ops::{Index, IndexMut};

use crate::basic::*;
use crate::board::{Board, CloneBoard, MutBoard};

use crate::board::testing;

/// MutBoard implementation using a 2D array of Option<Token>.
/// An array of columns, where the 0th element is the bottom of the column.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayBoard {
    grid: [[Option<Token>; row::COUNT]; column::COUNT],
}

impl Index<&Position> for ArrayBoard {
    type Output = Option<Token>;

    fn index(&self, index: &Position) -> &Self::Output {
        &self.grid[usize::from(index.col)][usize::from(index.row)]
    }
}

impl IndexMut<&Position> for ArrayBoard {
    fn index_mut(&mut self, index: &Position) -> &mut Self::Output {
        &mut self.grid[usize::from(index.col)][usize::from(index.row)]
    }
}

impl Board for ArrayBoard {
    const EMPTY: Self = ArrayBoard {
        grid: [[None; row::COUNT]; column::COUNT],
    };

    fn can_place(&self, col: &column::Idx) -> bool {
        self[&Position {
            col: *col,
            row: row::Idx::TOP,
        }]
            .is_none()
    }

    fn get(&self, pos: &Position) -> Option<Token> {
        self[pos]
    }

    fn place(&mut self, col: &column::Idx, token: &Token) -> Option<Position> {
        for row in row::IDXS {
            if self[&Position { col: *col, row }].is_none() {
                self[&Position { col: *col, row }] = Some(*token);
                return Some(Position {
                    col: *col,
                    row: row,
                });
            }
        }
        None
    }
}

impl CloneBoard for ArrayBoard {}

impl MutBoard for ArrayBoard {
    fn unplace(&mut self, pos: &Position) {
        self[pos] = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    make_board_tests!(ArrayBoard);
    make_mut_board_tests!(ArrayBoard);
}
