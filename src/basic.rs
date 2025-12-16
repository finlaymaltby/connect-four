use crate::finite_index::FiniteIndex;
use std::cmp::min;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::RangeInclusive;

/// A Token in the game, either Yellow or Red.
/// (Yellow starts)
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Token {
    Yellow,
    Red,
}

impl Token {
    /// The starting token (Yellow)
    pub const START: Token = Token::Yellow;

    /// Get the next, opposite, other token
    pub fn next(&self) -> Token {
        match self {
            Token::Yellow => Token::Red,
            Token::Red => Token::Yellow,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Yellow => write!(f, "Y"),
            Token::Red => write!(f, "R"),
        }
    }
}

pub mod column {
    

    pub const COUNT: usize = 7;

    pub type Idx = super::FiniteIndex<6>;

    pub const IDXS: [Idx; COUNT] = [
        Idx::raw(0),
        Idx::raw(1),
        Idx::raw(2),
        Idx::raw(3),
        Idx::raw(4),
        Idx::raw(5),
        Idx::raw(6),
    ];

    pub const IDXS_CENTRED_FIRST: [Idx; COUNT] = [
        Idx::raw(3),
        Idx::raw(2),
        Idx::raw(4),
        Idx::raw(1),
        Idx::raw(5),
        Idx::raw(0),
        Idx::raw(6),
    ];

    impl Idx {
        pub const CENTRE: Self = Idx::raw(3);
        /// Returns the column on the opposite side of the board, based on symmetry.
        pub fn flipped(&self) -> Self {
            Self::raw(usize::from(Self::MAX) - usize::from(*self))
        }
    }
}

pub mod row {
    use super::*;

    pub const COUNT: usize = 6;

    pub type Idx = super::FiniteIndex<5>;

    impl Idx {
        pub const BOTTOM: Self = Self::ZERO;
        pub const TOP: Self = Self::MAX;
    }
    /// bottom to top
    pub const BOTTOM_UP: RangeInclusive<Idx> = Idx::ZERO..=Idx::MAX;
}

/// A Position on the board, defined by a column and row index.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Position {
    pub col: column::Idx,
    pub row: row::Idx,
}

impl Position {
    /// Returns an iterator over the positions in the same row as this position,
    /// from 3 columns to the left to 3 columns to the right (capped at the board edges).
    pub fn row_neighbourhood(&self) -> impl Iterator<Item = Position> {
        (self.col.shift(-3)..=self.col.shift(3)).map(move |col| Position { col, row: self.row })
    }

    /// Returns an iterator over the positions in the same column as this position,
    /// from 3 rows below to 3 rows above (capped at the board edges).
    pub fn col_neighbourhood(&self) -> impl Iterator<Item = Position> {
        (self.row.shift(-3)..=self.row.shift(3)).map(move |row| Position { col: self.col, row })
    }

    /// Returns an iterator over the positions in the same diagonal (bottom-left to top-right)
    /// as this position within a distance of 3 (capped at the board edges).
    pub fn diag1_neighbourhood(&self) -> impl Iterator<Item = Position> {
        let start_offset = -min(
            3,
            min(
                isize::try_from(self.col).unwrap(),
                isize::try_from(self.row).unwrap(),
            ),
        );

        let end_offset = min(
            3,
            min(
                isize::try_from(column::Idx::MAX).unwrap() - isize::try_from(self.col).unwrap(),
                isize::try_from(row::Idx::MAX).unwrap() - isize::try_from(self.row).unwrap(),
            ),
        );

        (start_offset..=end_offset).map(move |offset| Position {
            col: self.col.shift(offset),
            row: self.row.shift(offset),
        })
    }

    /// Returns an iterator over the positions in the same diagonal (top-left to bottom-right)
    /// as this position within a distance of 3 (capped at the board edges).
    pub fn diag2_neighbourhood(&self) -> impl Iterator<Item = Position> {
        let start_offset = -min(
            3,
            min(
                isize::try_from(self.col).unwrap(),
                isize::try_from(row::Idx::MAX).unwrap() - isize::try_from(self.row).unwrap(),
            ),
        );

        let end_offset = min(
            3,
            min(
                isize::try_from(column::Idx::MAX).unwrap() - isize::try_from(self.col).unwrap(),
                isize::try_from(self.row).unwrap(),
            ),
        );

        (start_offset..=end_offset).map(move |offset| Position {
            col: self.col.shift(offset),
            row: self.row.shift(-offset),
        })
    }

    pub fn flipped(&self) -> Self {
        Position {col: self.col.flipped(), row: self.row}
    }

    /// TODO. (left, right)
    pub fn symm_pair(&self) -> Option<(Self, Self)> {
        if self.col == column::Idx::CENTRE {
            return None;
        }
        
        let flipped = Position { col: self.col.flipped(), row: self.row };
        if usize::from(self.col) < 3 {
            Some((*self, flipped))
        } else {
            Some((flipped, *self))
        }
    }

}
