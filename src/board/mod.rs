use std::fmt::Debug;
use crate::basic::{Position, Token, column};

pub mod array_board;

/// Trait for board implementations that don't have a cheap copy operation 
/// and instead place and unplace tokens on the same board.
pub trait MutBoard : Debug + Clone {
    /// An empty starting board.
    const EMPTY: Self;

    /// Returns true if a token can be placed in the given column.
    /// i.e. the column is not full.
    fn can_place(&self, col: &column::Idx) -> bool;

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

    /// Returns the token at the given position, or None if the position is empty.
    fn get_token_at(&self, pos: &Position) -> Option<Token>;

    /// Checks there is a win, a sequence of four tokens that includes the given
    /// position. The winning player is given by the colour of the token at the position.
    fn check_wins_at(&self, pos: &Position) -> bool {
            let Some(token) = self.get_token_at(pos) else {
                return false;
            };

            // check row
            for row in pos.col.shift(-4)..pos.col.shift(4) {
                return true;
            }


            return false;
    }

    /// Pretty displays the board to stdout.
    fn display(&self);
}

/// Trait for board implementations that have a cheap copy operation
/// and create new boards when placing tokens.
pub trait CopyBoard: Debug + Copy {
    /// An empty starting board.
    const EMPTY: Self;

    /// Returns true if a token can be placed in the given column.
    /// i.e. the column is not full.
    fn can_place(&self, col: &column::Idx) -> bool;

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

    /// Pretty displays the board to stdout.
    fn display(&self);
}