use std::cmp::min;
use std::fmt::Display;
use std::fmt::Formatter;
use std::range::Step;

/// A Token in the game, either Yellow or Red.
/// (Yellow starts)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
    use super::*;

    /// Type to index a column of the board.
    #[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
    pub enum Idx {
        Zero,
        One,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
    }

    impl Idx {
        pub const MAX: Idx = Idx::Seven;

        /// Shifts this column index by the given amount,
        /// staying within bounds by capping at the edges.
        pub fn shift(&self, by: isize) -> Self {
            let mut val = isize::from(*self) + by;
            val = val.clamp(0, 7);
            Idx::try_from(val as usize).unwrap()
        }
    }

    impl From<Idx> for usize {
        fn from(value: Idx) -> Self {
            match value {
                Idx::Zero => 0,
                Idx::One => 1,
                Idx::Two => 2,
                Idx::Three => 3,
                Idx::Four => 4,
                Idx::Five => 5,
                Idx::Six => 6,
                Idx::Seven => 7,
            }
        }
    }

    impl From<Idx> for isize {
        fn from(value: Idx) -> Self {
            match value {
                Idx::Zero => 0,
                Idx::One => 1,
                Idx::Two => 2,
                Idx::Three => 3,
                Idx::Four => 4,
                Idx::Five => 5,
                Idx::Six => 6,
                Idx::Seven => 7,
            }
        }
    }

    impl TryFrom<usize> for Idx {
        type Error = &'static str;

        fn try_from(value: usize) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Idx::Zero),
                1 => Ok(Idx::One),
                2 => Ok(Idx::Two),
                3 => Ok(Idx::Three),
                4 => Ok(Idx::Four),
                5 => Ok(Idx::Five),
                6 => Ok(Idx::Six),
                7 => Ok(Idx::Seven),
                _ => Err("Column index out of bounds"),
            }
        }
    }

    impl Step for Idx {
        fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
            usize::steps_between(&usize::from(*start), &usize::from(*end))
        }

        fn forward_checked(start: Self, count: usize) -> Option<Self> {
            if (usize::from(start) + count) < COUNT {
                start.shift(count as isize).into()
            } else {
                None
            }
        }

        fn backward_checked(start: Self, count: usize) -> Option<Self> {
            if (isize::from(start) - (count as isize)) >= 0 {
                start.shift(-(count as isize)).into()
            } else {
                None
            }
        }
    }

    /// Number of columns on the board.
    pub const COUNT: usize = 8;

    pub const COLIDXS: [Idx; COUNT] = [
        Idx::Zero,
        Idx::One,
        Idx::Two,
        Idx::Three,
        Idx::Four,
        Idx::Five,
        Idx::Six,
        Idx::Seven,
    ];
}

pub mod row {
    use super::*;
    /// Type to index a row of the board.
    #[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
    pub enum Idx {
        Zero,
        One,
        Two,
        Three,
        Four,
        Five,
        Six,
    }

    impl Idx {
        pub const MAX: Idx = Idx::Six;

        /// The row index of the top of a column.
        pub const TOP: Idx = Idx::Six;
        /// The row index of the bottom of a column.
        pub const BOTTOM: Idx = Idx::Zero;

        /// Shifts this row index by the given amount,
        /// staying within bounds by capping at the edges.
        /// Positive values shift up, negative values shift down.
        pub fn shift(&self, by: isize) -> Self {
            let mut val = isize::from(*self) + by;
            val = val.clamp(0, 6);
            Idx::try_from(val as usize).unwrap()
        }
    }

    impl From<Idx> for usize {
        fn from(value: Idx) -> Self {
            match value {
                Idx::Zero => 0,
                Idx::One => 1,
                Idx::Two => 2,
                Idx::Three => 3,
                Idx::Four => 4,
                Idx::Five => 5,
                Idx::Six => 6,
            }
        }
    }

    impl From<Idx> for isize {
        fn from(value: Idx) -> Self {
            match value {
                Idx::Zero => 0,
                Idx::One => 1,
                Idx::Two => 2,
                Idx::Three => 3,
                Idx::Four => 4,
                Idx::Five => 5,
                Idx::Six => 6,
            }
        }
    }

    impl TryFrom<usize> for Idx {
        type Error = &'static str;

        fn try_from(value: usize) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Idx::Zero),
                1 => Ok(Idx::One),
                2 => Ok(Idx::Two),
                3 => Ok(Idx::Three),
                4 => Ok(Idx::Four),
                5 => Ok(Idx::Five),
                6 => Ok(Idx::Six),
                _ => Err("Row index out of bounds"),
            }
        }
    }

    impl Step for Idx {
        fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
            usize::steps_between(&usize::from(*start), &usize::from(*end))
        }

        fn forward_checked(start: Self, count: usize) -> Option<Self> {
            if (usize::from(start) + count) < COUNT {
                start.shift(count as isize).into()
            } else {
                None
            }
        }

        fn backward_checked(start: Self, count: usize) -> Option<Self> {
            if (isize::from(start) - count as isize) >= 0 {
                start.shift(-(count as isize)).into()
            } else {
                None
            }
        }
    }

    /// Number of rows on the board.
    pub const COUNT: usize = 7;

    /// Ordered array of all row indices.
    pub const ROWIDXS: [Idx; COUNT] = [
        Idx::Zero,
        Idx::One,
        Idx::Two,
        Idx::Three,
        Idx::Four,
        Idx::Five,
        Idx::Six,
    ];
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
        let start_offset = -min(3, min(isize::from(self.col), isize::from(self.row)));

        let end_offset = min(
            3,
            min(
                isize::from(column::Idx::MAX) - isize::from(self.col),
                isize::from(row::Idx::MAX) - isize::from(self.row),
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
                isize::from(self.col),
                isize::from(row::Idx::MAX) - isize::from(self.row),
            ),
        );

        let end_offset = min(
            3,
            min(
                isize::from(column::Idx::MAX) - isize::from(self.col),
                isize::from(self.row),
            ),
        );

        (start_offset..=end_offset).map(move |offset| Position {
            col: self.col.shift(offset),
            row: self.row.shift(-offset),
        })
    }
}
