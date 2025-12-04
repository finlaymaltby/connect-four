use crate::basic::*;
use crate::board::{Board, CloneBoard, MutBoard};

fn token_to_bit(token: &Token) -> u8 {
    match token {
        Token::Yellow => 0,
        Token::Red => 1,
    }
}

fn bit_to_token(bit: u8) -> Token {
    match bit {
        0 => Token::Yellow,
        _ => Token::Red,
    }
}

mod bit_col {
    use super::*;

    /// A column of the CountBoard, stored as a u8.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct CountBitCol {
        col: u8,
        count: usize,
    }

    impl CountBitCol {
        /// An empty BitCol.
        pub const EMPTY: Self = CountBitCol {
            col: 0b00000000,
            count: 0,
        };

        pub fn count(&self) -> usize {
            self.count as usize
        }

        pub fn is_empty(&self) -> bool {
            self.count == 0
        }

        pub fn is_full(&self) -> bool {
            self.count() >= row::COUNT
        }

        /// Gets the token at the given row in the column.
        pub fn get(&self, row: &row::Idx) -> Option<Token> {
            if self.count() > usize::from(*row) {
                // The bit index of the desired row
                let bit_idx = self.count() - usize::from(*row) - 1;
                let bit_mask = 1 << bit_idx;
                Some(bit_to_token(self.col & bit_mask))
            } else {
                None
            }
        }

        /// Pop the top token from the column.  
        /// Panics if the column is empty.
        pub fn force_pop(&mut self) {
            if self.is_empty() {
                panic!("Tried to pop from an empty column");
            }
            self.col >>= 1;
            self.count -= 1;
        }

        /// Push a token onto the column.
        /// Panics if the column is full.
        pub fn force_push(&mut self, token: &Token) {
            if self.is_full() {
                panic!("Tried to push onto a full column");
            }
            let token_bit = token_to_bit(token);
            self.col <<= 1;
            self.col |= token_bit;
            self.count += 1;
        }
    }
}

/// A board implementation using bit manipulation for storage.
/// Each column is stored as a BitCol.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CountBitBoard {
    cols: [bit_col::CountBitCol; column::COUNT],
}
impl Board for CountBitBoard {
    const EMPTY: Self = CountBitBoard {
        cols: [bit_col::CountBitCol::EMPTY; column::COUNT],
    };

    fn get(&self, pos: &Position) -> Option<Token> {
        self.cols[usize::from(pos.col)].get(&pos.row)
    }

    fn can_place(&self, col: &column::Idx) -> bool {
        !self.cols[usize::from(*col)].is_full()
    }

    fn place_unchecked(&mut self, col: &column::Idx, token: &Token) -> Position {
        let col_idx = usize::from(*col);
        self.cols[col_idx].force_push(token);
        Position {
            col: *col,
            row: row::Idx::try_from(self.cols[col_idx].count() - 1).unwrap(),
        }
    }
}

impl CloneBoard for CountBitBoard {}

impl MutBoard for CountBitBoard {
    fn unplace_unchecked(&mut self, col: &column::Idx) {
        self.cols[usize::from(*col)].force_pop();
    }
}
