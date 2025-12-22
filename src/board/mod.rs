use crate::{
    basic::{Cell, Token, column, row},
    board::moves::Moves,
};
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

    /// Returns the token at the given cell, or None if the cell is empty.
    fn get(&self, cell: &Cell) -> Option<Token>;

    /// Compute the current player based on the number of tokens on the board.
    fn curr_player(&self) -> Token {
        let mut red_count = 0;
        let mut yellow_count = 0;

        for row in row::BOTTOM_UP {
            for col in column::IDXS {
                let cell = Cell { col, row };
                match self.get(&cell) {
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
    /// Returns `Some(Cell)` if successful, `None` if the column is full.
    /// `token` should equal the current player, as given by `curr_player`.
    fn place(&mut self, col: &column::Idx, token: &Token) -> Option<Cell>;

    /// Checks there is a win, a sequence of four same-colour tokens, that includes the given
    /// cell. The winning player is given by the colour of the token at the cell.
    fn won_at(&self, cell: &Cell) -> bool {
        self.count_adjacent_at(cell).is_none()
    }

    /// For every direction (horizontal, vertical, and both diagonals),
    /// it calculates the length of line of same tokens in that direction,
    /// including the given cell.
    /// Returns None if a win is found (a row of four), else
    /// Some((count of adjacent triples, count of adjacent pairs))
    fn count_adjacent_at(&self, cell: &Cell) -> Option<(usize, usize)> {
        fn count_line(board: &impl Board, cell: &Cell, dir: (isize, isize)) -> usize {
            let Some(token) = board.get(cell) else {
                panic!("Tried to count adjacent at an empty cell.");
            };

            let mut next = cell.try_shift(dir);
            let mut count = 1;

            while let Some(curr) = next {
                if board.get(&curr) == Some(token) {
                    count += 1;
                    if (count >= 4) {
                        return count;
                    }
                } else {
                    break;
                }
                next = curr.try_shift(dir);
            }
            let dir = (-dir.0, -dir.1);
            let mut next = cell.try_shift(dir);

            while let Some(curr) = next {
                if board.get(&curr) == Some(token) {
                    count += 1;
                    if (count >= 4) {
                        return count;
                    }
                } else {
                    break;
                }
                next = curr.try_shift(dir);
            }
            count
        }

        let Some(token) = self.get(cell) else {
            panic!("Tried to count adjacent at an empty cell")
        };
        // right, down, to bottom-right, to bottom-left
        let dirs = [(1, 0), (0, -1), (1, -1), (-1, -1)];
        let mut counts = (0, 0);

        for dir in dirs {
            match count_line(self, cell, dir) {
                2 => counts.1 += 1,
                3 => counts.0 += 1,
                4 => return None,
                _ => (),
            }
        }
        Some(counts)
    }
    /// String pretty display
    fn to_string(&self) -> String {
        let mut string = "\n".to_owned();

        for row in row::BOTTOM_UP.rev() {
            string.push('|');
            for col in column::IDXS {
                let cell = Cell { col, row };
                match self.get(&cell) {
                    Some(Token::Red) => string.push('R'),
                    Some(Token::Yellow) => string.push('Y'),
                    None => string.push(' '),
                }
            }
            string.push_str("|\n");
        }
        string.push_str("+--------+\n");

        string
    }

    /// Pretty displays the board to stdout.
    fn display(&self) {
        print!("{}", self.to_string());
    }

    /// Read a board from a string representation.
    fn read(string: &str) -> Self {
        let mut board = Self::EMPTY;
        let mut diff = 0;

        for line in string.split('|').rev() {
            if line.trim().is_empty() {
                continue;
            }
            debug_assert!(line.len() == 7);

            for (i, ch) in line.chars().enumerate() {
                let token = match ch {
                    'Y' => {
                        diff += 1;
                        Token::Yellow
                    }
                    'R' => {
                        diff -= 1;
                        Token::Red
                    }
                    '.' | ' ' => continue,
                    '+' | '-' => return board, // end of board representation
                    _ => panic!("Invalid character in board string: {}", ch),
                };
                let cell = board.place(&column::Idx::try_from(i).unwrap(), &token);
                debug_assert!(cell.is_some());
            }
        }

        assert!(diff == 0 || diff == 1, "incorrect difference in yellow and red tokens");
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
    /// returning the new board and cell if successful.
    /// `token` should equal the current player, as given by `curr_player`.
    fn clone_and_place(&self, col: &column::Idx, token: &Token) -> Option<(Self, Cell)> {
        // check if we can place first to avoid cloning unnecessarily
        if self.can_place(col) {
            let mut new_board = self.clone();
            if let Some(cell) = new_board.place(col, token) {
                Some((new_board, cell))
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
    fn next_boards(&self, token: &Token) -> impl Iterator<Item = (Self, Cell)> {
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
                if let Some(token) = self.get(&Cell { col, row }) {
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
    /// Removes the token at the given cell, modifying the board in place.
    /// Does not check if there is a token at the cell.
    fn unplace(&mut self, cell: &Cell);
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

        let cell = Cell {
            col: column::Idx::try_from(3).unwrap(),
            row: row::Idx::try_from(0).unwrap(),
        };
        assert!(board.won_at(&cell));
    }
}
