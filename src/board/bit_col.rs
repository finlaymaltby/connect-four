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

/// A column of the BitBoard, stored as a u8.
/// Formatted with a leading 1 bit, followed by the rows from bottom to top,
/// The top tile is the LSB and the bottom tile is the MSB after the leading 1.
// Examples:
// 0b01abcdef : col is full, a at bottom, f at top
// 0b00000001 : col is empty
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BitCol(u8);

impl BitCol {
    /// An empty BitCol.
    pub const EMPTY: Self = BitCol(0b00000001);

    /// Counts the number of tokens in the column.
    /// Also the bit index of the leading one.
    pub fn count(&self) -> usize {
        7 - self.0.leading_zeros() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.count() == 0
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
            Some(bit_to_token(self.0 & bit_mask))
        } else {
            None
        }
    }

    /// Pop the top token from the column.  
    /// Debug asserts that the column is not empty.
    pub fn force_pop(&mut self) {
        debug_assert!(!self.is_empty(), "Tried to pop from an empty column.");
        self.0 >>= 1;
    }

    /// Push a token onto the column.
    /// Debug asserts that the column is not full.
    pub fn force_push(&mut self, token: &Token) {
        debug_assert!(!self.is_full(), "Tried to push onto a full column.");

        let token_bit = token_to_bit(token);
        self.0 <<= 1;
        self.0 |= token_bit;
    }

    /// Returns the underlying u8 value of the column.
    pub fn as_u8(&self) -> u8 {
        self.0
    }
}
