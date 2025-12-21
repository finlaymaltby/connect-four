use crate::basic::*;

pub struct Position {
    pub board: &'static str,
    pub winner: Option<Token>,
    pub depth: usize,
}

pub const EASY_0: Position = Position {
    board: "|.......|
            |.......|
            |.......|
            |.......|
            |RRRR...|
            |YYYY...|",
    winner: Some(Token::Yellow),
    depth: 1,
};

pub const EASY_1: Position = Position {
    board: "|RRY.R.Y|
            |YYR.Y.R|
            |RYR.R.R|
            |YRRYY.Y|
            |RRYRY.Y|
            |YYRYR.Y|",
    winner: Some(Token::Yellow),
    depth: 10,
};

pub const EASY_2: Position = Position {
    board: "|RRY.R..|
            |YYR.Y..|
            |RYR.R..|
            |YRRYYY.|
            |RRYRYR.|
            |YYRYRYY|",
    winner: Some(Token::Red),
    depth: 11,
};

pub const MEDIUM_0: Position = Position {
    board: "|.......|
            |Y.Y.R..|
            |R.Y.R..|
            |YRR.Y..|
            |YRYYR..|
            |YYRYR.R|",
    winner: None,
    depth: 20,
};

pub const MEDIUM_1: Position = Position {
    board: "|.......|
            |..Y.Y..|
            |..Y.R..|
            |..RRY..|
            |.RYRR.Y|
            |YYRYR.R|",
    winner: Some(Token::Red),
    depth: 24,
};
