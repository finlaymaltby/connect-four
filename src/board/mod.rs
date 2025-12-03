use crate::basic::{Position, Token, column, row};
use std::fmt::Debug;

pub mod array_board;
pub mod moves_board;

/// Trait containing common board functionality.
pub trait Board: Debug {
    /// An empty starting board.
    const EMPTY: Self;

    /// Returns the token at the given position, or None if the position is empty.
    fn get(&self, pos: &Position) -> Option<Token>;

    /// Returns true if a token can be placed in the given column.
    /// i.e. the column is not full.
    fn can_place(&self, col: &column::Idx) -> bool;

    /// Places a token in the given column, modifying the board in place and
    /// returning the position the token was placed. Does not check if the column is full.
    /// Must be preceded by a call to `can_place`.
    /// # Panics
    fn force_place(&mut self, col: &column::Idx, token: &Token) -> Position;

    /// Tries to place a token in the given column, checking `can_place` and then calling
    /// `force_place`. Returns `Some(Position)` if successful, `None` if the column is full.
    fn try_place(&mut self, col: &column::Idx, token: &Token) -> Option<Position> {
        if self.can_place(col) {
            Some(self.force_place(col, token))
        } else {
            None
        }
    }

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
    fn display(&self) {
        for row in row::IDXS.rev() {
            print!("|");
            for col in column::IDXS {
                let pos = Position { col, row };
                match self.get(&pos) {
                    Some(Token::Red) => print!("R"),
                    Some(Token::Yellow) => print!("Y"),
                    None => print!(" "),
                }
            }
            println!("|");
        }
        println!("+--------+");
    }
}

/// Trait for board implementations that have a cheap clone operation.
/// Must opt-in to this trait.
pub trait CloneBoard: Board + Clone {
    /// Clones the given board and calls `try_place` with the given column,
    /// returning the new board and position if successful.
    fn clone_and_place(&self, col: &column::Idx, token: &Token) -> Option<(Self, Position)> {
        if self.can_place(col) {
            let mut new_board = self.clone();
            let pos = new_board.force_place(col, token);
            Some((new_board, pos))
        } else {
            None
        }
    }

    /// Returns an iterator over every possible subsequent board state
    /// after placing the given token in each non-full column.
    fn next_boards(&self, token: &Token) -> impl Iterator<Item = (Self, Position)> {
        column::IDXS.filter_map(move |col| self.clone_and_place(&col, token))
    }
}

/// Trait for board implementations that don't have a cheap clone operation
/// and instead place and unplace tokens on the same board.
pub trait MutBoard: Board {
    /// Removes a token from the given position, modifying the board in place.
    /// Does not check that there is a token at the position.
    fn unplace(&mut self, pos: &Position);

    /// Removes a token from the given position, modifying the board in place.
    /// Checks that there is a token at the position before unplacing.
    /// # Panics
    fn unplace_checked(&mut self, pos: &Position) {
        if self.get(pos).is_some() {
            self.unplace(pos);
        } else {
            panic!("Tried to unplace from an empty position: {:?}", pos);
        }
    }
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

#[cfg(test)]
mod mut_board_tests {
    use super::*;
    use crate::basic::{column, row};
    use crate::board::array_board::ArrayBoard;

    #[test]
    fn test() {
        won_at_horizontal::<ArrayBoard>();
    }

    fn won_at_horizontal<B: MutBoard>() {
        let mut board = B::EMPTY;

        for col in column::Idx::ZERO..=column::Idx::try_from(3).unwrap() {
            board.force_place(&col, &Token::Red);
        }

        let pos = Position {
            col: column::Idx::try_from(3).unwrap(),
            row: row::Idx::try_from(0).unwrap(),
        };
        assert!(board.won_at(&pos));
    }
}
