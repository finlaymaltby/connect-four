use crate::{basic::{Position, Token, column, row}, board::moves::Moves};
use std::fmt::{Debug, Display};

#[macro_use]
pub mod testing;
pub mod array_board;
pub mod bit_board;
mod bit_col;
pub mod moves;
pub mod symm_board;

/// Trait containing common board functionality.
pub trait Board: Debug + Sized + Eq {
    /// An empty starting board.
    const EMPTY: Self;

    /// Returns the token at the given position, or None if the position is empty.
    fn get(&self, pos: &Position) -> Option<Token>;

    /// Compute the current player based on the number of tokens on the board.
    fn curr_player(&self) -> Token {
        let mut red_count = 0;
        let mut yellow_count = 0;

        for row in row::BOTTOM_UP {
            for col in column::IDXS {
                let pos = Position { col, row };
                match self.get(&pos) {
                    Some(Token::Red) => red_count += 1,
                    Some(Token::Yellow) => yellow_count += 1,
                    None => {}
                }
            }
        }

        if red_count < yellow_count {
            Token::Red
        } else {
            Token::Yellow
        }
    }

    /// Returns true if a token can be placed in the given column.
    /// i.e. the column is not full.
    fn can_place(&self, col: &column::Idx) -> bool;

    /// Tries to place the given token in the given column.
    /// Returns `Some(Position)` if successful, `None` if the column is full.
    /// `token` should equal the current player, as given by `curr_player`.
    fn place(&mut self, col: &column::Idx, token: &Token) -> Option<Position>;

    /// Checks there is a win, a sequence of four same-colour tokens, that includes the given
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
        for row in row::BOTTOM_UP.rev() {
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

    /// Read a board from a string representation.
    fn read(string: &str) -> Self {

        let mut board = Self::EMPTY;

        for line in string.split('|').rev() {
            if line.trim().is_empty() {
                continue;
            }
            debug_assert!(line.len() == 7);

            for (i, ch) in line.chars().enumerate() {
                let token = match ch {
                    'R' => Token::Red,
                    'Y' => Token::Yellow,
                    '.' | ' ' => continue,
                    '+' | '-' => return board, // end of board representation
                    _ => panic!("Invalid character in board string: {}", ch),
                };
                let pos = board.place(&column::Idx::try_from(i).unwrap(), &token);
                debug_assert!(pos.is_some());
            }
        }

        board
    }

    fn from_moves(moves: &Moves) -> Self {
        let mut board = Self::EMPTY;
        for (col, token) in moves.moves.iter() {
            board.place(&col, &token).unwrap();
        }
        board
    }
}

/// Trait for board implementations that have a cheap clone operation.
/// Must opt-in to this trait.
pub trait CloneBoard: Board + Clone {
    /// Clones the given board and calls `try_place` with the given column,
    /// returning the new board and position if successful.
    /// `token` should equal the current player, as given by `curr_player`.
    fn clone_and_place(&self, col: &column::Idx, token: &Token) -> Option<(Self, Position)> {
        // check if we can place first to avoid cloning unnecessarily
        if self.can_place(col) {
            let mut new_board = self.clone();
            if let Some(pos) = new_board.place(col, token) {
                Some((new_board, pos))
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Returns an iterator over every possible subsequent board state
    /// after placing the given token in each non-full column.
    /// Ordered by column::IDXS_CENTRED_FIRST.
    fn next_boards(&self, token: &Token) -> impl Iterator<Item = (Self, Position)> {
        // a simple optimisation to try the centre columns first
        column::IDXS_CENTRED_FIRST
            .iter()
            .filter_map(move |col| self.clone_and_place(&col, token))
    }

    /// TODO
    fn flipped(&self) -> Self {
        let mut board = Self::EMPTY;
        for col in column::IDXS {
            for row in row::BOTTOM_UP {
                if let Some(token) = self.get(&Position {col, row}) {
                    board.place(&col.flipped(), &token);
                }
            }
        }

        board
    }
}

/// Trait for board implementations that don't have a cheap clone operation
/// and instead place and unplace tokens on the same board.
pub trait MutBoard: Board {
    /// Removes the token at the given position, modifying the board in place.
    /// Does not check if there is a token at the position.
    fn unplace(&mut self, pos: &Position);
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
            board.place(&col, &Token::Red);
        }

        let pos = Position {
            col: column::Idx::try_from(3).unwrap(),
            row: row::Idx::try_from(0).unwrap(),
        };
        assert!(board.won_at(&pos));
    }
}
