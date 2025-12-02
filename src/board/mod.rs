use crate::basic::{Position, Token, column};
use std::fmt::Debug;

pub mod array_board;

/// Trait containing common board functionality.
pub trait Board: Debug + Clone {
    /// An empty starting board.
    const EMPTY: Self;

    /// Returns true if a token can be placed in the given column.
    /// i.e. the column is not full.
    fn can_place(&self, col: &column::Idx) -> bool;

    /// Returns the token at the given position, or None if the position is empty.
    fn get(&self, pos: &Position) -> Option<Token>;

    /// Checks there is a win, a sequence of four same-colour tokens that includes the given
    /// position. The winning player is given by the colour of the token at the position.
    fn won_at(&self, pos: &Position) -> bool {
        let Some(token) = self.get(pos) else {
            return false;
        };

        // Check horizontal
        if check_line(pos.row_neighbourhood().map(|pos| self.get(&pos)), &token) {
            return true;
        }

        // Check vertical
        if check_line(pos.col_neighbourhood().map(|pos| self.get(&pos)), &token) {
            return true;
        }

        // Check diagonals
        if check_line(pos.diag1_neighbourhood().map(|pos| self.get(&pos)), &token) {
            return true;
        }
        if check_line(pos.diag2_neighbourhood().map(|pos| self.get(&pos)), &token) {
            return true;
        }

        return false;
    }

    /// Pretty displays the board to stdout.
    fn display(&self);
}

/// Trait for board implementations that don't have a cheap copy operation
/// and instead place and unplace tokens on the same board.
pub trait MutBoard: Board {
    /// Tries to place a token in the given column, checking `can_place` and then calling
    /// `force_place`. Returns `Some(Position)` if successful, `None` if the column is full.
    fn try_place(&mut self, col: &column::Idx, token: &Token) -> Option<Position> {
        if self.can_place(col) {
            Some(self.force_place(col, token))
        } else {
            None
        }
    }

    /// Places a token in the given column, modifying the board in place and
    /// returning the position the token was placed. Does not check if the column is full.
    /// Must be preceded by a call to `can_place`.
    /// # Panics
    fn force_place(&mut self, col: &column::Idx, token: &Token) -> Position;

    /// Removes a token from the given position, modifying the board in place.
    fn unplace(&mut self, pos: &Position);
}

/// Trait for board implementations that have a cheap copy operation
/// and create new boards when placing tokens.
pub trait CopyBoard: Board + Copy {
    /// Tries to place a token in the given column, checking `can_place` and then calling
    /// `force_place`. Returns `Some((Self, Position))` if successful, `None` if the column is full.
    fn try_place(&self, col: &column::Idx, token: &Token) -> Option<(Self, Position)> {
        if self.can_place(col) {
            Some(self.force_place(col, token))
        } else {
            None
        }
    }

    /// Returns a new board with the token placed in the given column and
    /// the position the token was placed. Does not check if the column is full.
    /// Must be preceded by a call to `can_place`.
    /// # Panics
    fn force_place(&self, col: &column::Idx, token: &Token) -> (Self, Position);
}

fn check_line<T: Iterator<Item = Option<Token>>>(line: T, token: &Token) -> bool {
    let mut count = 0;
    for cell in line {
        if cell == Some(*token) {
            count += 1;
            if count >= 4 {
                return true;
            }
        } else {
            count = 0;
        }
    }
    false
}
