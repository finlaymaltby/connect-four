use std::ops::{Index, IndexMut};

use crate::{basic::*, board::MutBoard};


/// MutBoard implementation using a 2D array of Option<Token>.
/// An array of columns, where the 0th element is the bottom of the column.
#[derive(Debug, Clone)]
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

impl MutBoard for ArrayBoard {
    const EMPTY: Self = ArrayBoard {
        grid: [[None; row::COUNT]; column::COUNT],
    };

    fn can_place(&self, col: &column::Idx) -> bool {
        self[(*col, row::Idx::TOP)].is_none()
    }

    fn force_place(&mut self, col: &column::Idx, token: &Token) -> Position {
        for row in row::ROWIDXS.iter() {
            if self[(*col, *row)].is_none() {
                self[(*col, *row)] = Some(*token);
                return Position {
                    col: *col,
                    row: *row,
                };
            }
        }
        panic!("Must check `can_place` before calling `force_place`");
    }

    fn unplace(&mut self, pos: &Position) {
        self[(pos.col, pos.row)] = None;
    }

    fn get_token_at(&self, pos: &Position) -> Option<Token> {
        self[(pos.col, pos.row)]
    }

    fn display(&self) {
        for row in (row::ROWIDXS.iter()).rev() {
            print!("|");
            for col in column::COLIDXS.iter() {
                match self[(*col, *row)] {
                    Some(token) => print!("{}", token),
                    None => print!(" "),
                }
            }
            println!("|");
        }
    }
}