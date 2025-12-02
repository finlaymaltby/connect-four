use std::fmt::Formatter;
use std::fmt::Display;

/// A Token in the game, either Yellow or Red.
/// (Yellow starts)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Token {
    Yellow, 
    Red
}

impl Token {
    /// The starting token (Yellow)
    pub const START: Token = Token::Yellow;

    /// Get the next, opposite, other token
    pub fn next(&self) -> Token {
        match self {
            Token::Yellow => Token::Red,
            Token::Red => Token::Yellow
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Yellow => write!(f, "Y"),
            Token::Red => write!(f, "R")
        }
    }
}


pub mod column {
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
        Seven
    }

    impl Idx {
        /// Shifts this column index by the given amount, 
        /// staying within bounds by capping at the edges.
        pub fn shift(&self, by: isize) -> Self {
            let mut val = usize::from(*self) as isize + by;
            val = val.clamp(0, (COUNT - 1) as isize);
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
                Idx::Seven => 7
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
                _ => Err("Column index out of bounds")
            }
        }
    }


    /// Number of columns on the board.
    pub const COUNT: usize = 8;

    /// Ordered array of all column indices.
    pub const COLIDXS: [Idx; COUNT] = [
        Idx::Zero,
        Idx::One,
        Idx::Two,
        Idx::Three,
        Idx::Four,
        Idx::Five,
        Idx::Six,
        Idx::Seven
    ];
}

pub mod row {
    /// Type to index a row of the board.
    #[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
    pub enum Idx {
        Zero,
        One,
        Two,
        Three,
        Four,
        Five,
        Six
    }
    
    impl Idx {
        /// The row index of the top of a column.
        pub const TOP: Idx = Idx::Six;
        /// The row index of the bottom of a column.
        pub const BOTTOM: Idx = Idx::Zero;

        /// Shifts this row index by the given amount,
        /// staying within bounds by capping at the edges.
        /// Positive values shift up, negative values shift down.
        pub fn shift(&self, by: isize) -> Self {
            let mut val = usize::from(*self) as isize + by;
            val = val.clamp(0, (COUNT - 1) as isize);
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
                Idx::Six => 6
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
                _ => Err("Row index out of bounds")
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
        Idx::Six
    ];
}

/// A Position on the board, defined by a column and row index.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Position {
    pub col: column::Idx,
    pub row: row::Idx
}
